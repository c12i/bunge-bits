use crate::Stream;
use anyhow::Context;
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
            .inspect_err(
                |e| tracing::error!(error = ?e, "Failed to establish connection to database"),
            )
            .context("Failed to connect to database")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS streams (
                video_id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                view_count TEXT NOT NULL,
                stream_timestamp TIMESTAMPTZ NOT NULL,
                duration TEXT NOT NULL,
                summary_md TEXT,
                timestamp_md TEXT,
                is_published BOOLEAN NOT NULL DEFAULT FALSE
            )
            "#,
        )
        .execute(&pool)
        .await
        .inspect_err(|e| tracing::error!(error = ?e, "Failed to create streams table"))
        .context("Failed to create streams table")?;

        Ok(DataStore { pool })
    }

    pub async fn stream_exists(&self, video_id: &str) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM streams WHERE video_id = $1")
            .bind(video_id)
            .fetch_one(&self.pool)
            .await
            .inspect_err(
                |e| tracing::error!(error = ?e, video_id, "Failed to check if stream exists"),
            )
            .context("Failed to check if stream exists")?;
        Ok(count.0 > 0)
    }

    #[tracing::instrument(skip(self, streams))]
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
                    video_id, title, view_count,
                    stream_timestamp, duration, summary_md, timestamp_md
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(&stream.video_id)
            .bind(&stream.title)
            .bind(&stream.view_count)
            .bind(timestamp)
            .bind(&stream.duration)
            .bind(&stream.summary_md)
            .bind(&stream.timestamp_md)
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

        tx.commit()
            .await
            .inspect_err(|e| {
                tracing::error!(
                    error = ?e,
                    successful = successful_inserts,
                    failed = failed_inserts.len(),
                    "Failed to commit transaction during bulk insert of streams"
                )
            })
            .context("Failed to commit transaction")?;

        Ok(BulkInsertResult {
            successful_inserts,
            failed_inserts,
        })
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
