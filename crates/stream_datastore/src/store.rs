use crate::Stream;
use anyhow::{anyhow, Context};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: PgPool,
}

impl DataStore {
    /// Establish connection to database and create the streams table
    /// if not exists
    pub async fn init(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .context("Failed to connect to database")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS streams (
                video_id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                view_count TEXT NOT NULL,
                streamed_date TEXT NOT NULL,
                stream_timestamp TIMESTAMPTZ NOT NULL,
                duration TEXT NOT NULL,
                closed_captions_summary TEXT
            )
            "#,
        )
        .execute(&pool)
        .await
        .context("Failed to create streams table")?;

        Ok(DataStore { pool })
    }

    pub async fn insert_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        let timestamp = stream
            .timestamp_from_time_ago()
            .context("Failed to get timestamp")?;

        let result = sqlx::query(
            r#"
            INSERT INTO streams (
                video_id, title, view_count, streamed_date,
                stream_timestamp, duration, closed_captions_summary
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&stream.video_id)
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.streamed_date)
        .bind(timestamp)
        .bind(&stream.duration)
        .bind(&stream.closed_captions_summary)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(e)) if e.constraint() == Some("streams_pkey") => {
                Err(anyhow!("Duplicate entry"))
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn stream_exists(&self, video_id: &str) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM streams WHERE video_id = $1")
            .bind(video_id)
            .fetch_one(&self.pool)
            .await
            .context("Failed to check if stream exists")?;
        Ok(count.0 > 0)
    }

    pub async fn bulk_insert_streams(
        &self,
        streams: &[Stream],
    ) -> anyhow::Result<BulkInsertResult> {
        let mut successful_inserts = 0;
        let mut failed_inserts = Vec::new();
        let mut tx = self.pool.begin().await?;

        for stream in streams {
            let timestamp = match stream.timestamp_from_time_ago() {
                Some(t) => t,
                None => {
                    failed_inserts.push(FailedInsert {
                        video_id: stream.video_id.clone(),
                        reason: InsertFailReason::OtherError("Invalid timestamp".into()),
                    });
                    continue;
                }
            };

            let result = sqlx::query(
                r#"
                INSERT INTO streams (
                    video_id, title, view_count, streamed_date,
                    stream_timestamp, duration, closed_captions_summary
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(&stream.video_id)
            .bind(&stream.title)
            .bind(&stream.view_count)
            .bind(&stream.streamed_date)
            .bind(timestamp)
            .bind(&stream.duration)
            .bind(&stream.closed_captions_summary)
            .execute(&mut *tx)
            .await;

            match result {
                Ok(_) => successful_inserts += 1,
                Err(sqlx::Error::Database(e)) if e.constraint() == Some("streams_pkey") => {
                    failed_inserts.push(FailedInsert {
                        video_id: stream.video_id.clone(),
                        reason: InsertFailReason::DuplicateEntry,
                    });
                }
                Err(e) => {
                    failed_inserts.push(FailedInsert {
                        video_id: stream.video_id.clone(),
                        reason: InsertFailReason::OtherError(e.to_string()),
                    });
                }
            }
        }

        tx.commit().await.context("Failed to commit transaction")?;

        Ok(BulkInsertResult {
            successful_inserts,
            failed_inserts,
        })
    }

    pub async fn get_stream(&self, video_id: &str) -> anyhow::Result<Option<Stream>> {
        let result = sqlx::query_as::<_, Stream>(
            "SELECT video_id, title, view_count, streamed_date, duration, closed_captions_summary FROM streams WHERE video_id = $1"
        )
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!(error = ?e, "Failed to fetch stream"))?;

        Ok(result)
    }

    pub async fn update_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE streams
            SET title = $1,
                view_count = $2,
                duration = $3,
                closed_captions_summary = $4
            WHERE video_id = $5
            "#,
        )
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.duration)
        .bind(&stream.closed_captions_summary)
        .bind(&stream.video_id)
        .execute(&self.pool)
        .await
        .context("Failed to update stream")?;

        Ok(())
    }

    pub async fn delete_stream(&self, video_id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM streams WHERE video_id = $1")
            .bind(video_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete stream")?;
        Ok(())
    }

    pub async fn list_streams(&self) -> anyhow::Result<Vec<Stream>> {
        let streams = sqlx::query_as::<_, Stream>(
            "SELECT video_id, title, view_count, streamed_date, duration, closed_captions_summary FROM streams ORDER BY stream_timestamp DESC"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to list streams")?;
        Ok(streams)
    }
}

#[derive(Debug)]
pub struct BulkInsertResult {
    pub successful_inserts: usize,
    pub failed_inserts: Vec<FailedInsert>,
}

#[derive(Debug)]
pub struct FailedInsert {
    pub video_id: String,
    pub reason: InsertFailReason,
}

#[derive(Debug)]
pub enum InsertFailReason {
    DuplicateEntry,
    OtherError(String),
}
