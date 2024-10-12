use crate::Stream;
use anyhow::Context;
use sqlx::{Sqlite, SqlitePool, Transaction};

#[derive(Debug, Clone)]
pub struct DataStore(SqlitePool);

impl DataStore {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .context("Failed to connect to database")?;

        // Create the streams table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS streams (
                video_id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                view_count TEXT NOT NULL,
                streamed_date TEXT NOT NULL,
                stream_timestamp DATETIME NOT NULL,
                duration TEXT NOT NULL,
                closed_captions_summary TEXT,
            UNIQUE(video_id)
            )
            "#,
        )
        .execute(&pool)
        .await
        .context("Failed to create streams table")?;

        Ok(DataStore(pool))
    }

    pub async fn insert_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        let result = sqlx::query(
            r#"
            INSERT INTO streams (
                video_id,
                title,
                view_count,
                streamed_date,
                stream_timestamp,
                duration,
                closed_captions_summary
            )
           VALUES (?, ?, ?, ?, ?, ?, ?)
           "#,
        )
        .bind(&stream.video_id)
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.streamed_date)
        .bind(
            stream
                .timestamp_from_time_ago()
                .context("Failed to get timestamp")?
                .to_string(),
        )
        .bind(&stream.duration)
        .bind(&stream.closed_captions_summary)
        .execute(&self.0)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(db_err.into())
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn stream_exists(&self, video_id: &str) -> anyhow::Result<bool> {
        let existing_stream = self.get_stream(video_id).await?;
        Ok(existing_stream.is_some())
    }

    pub async fn bulk_insert_streams(
        &self,
        streams: &[Stream],
    ) -> anyhow::Result<BulkInsertResult> {
        let mut transaction = self
            .0
            .begin()
            .await
            .context("Failed to start transaction")?;

        let result = self.bulk_insert_streams_tx(&mut transaction, streams).await;

        match result {
            Ok(insert_result) => {
                transaction
                    .commit()
                    .await
                    .context("Failed to commit transaction")?;
                Ok(insert_result)
            }
            Err(e) => {
                transaction
                    .rollback()
                    .await
                    .context("Failed to rollback transaction")?;
                Err(e)
            }
        }
    }

    async fn bulk_insert_streams_tx(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
        streams: &[Stream],
    ) -> anyhow::Result<BulkInsertResult> {
        let mut successful_inserts = 0;
        let mut failed_inserts = Vec::new();

        for stream in streams {
            let result = sqlx::query(
                r#"
                INSERT INTO streams (
                    video_id,
                    title,
                    view_count,
                    streamed_date,
                    stream_timestamp,
                    duration,
                    closed_captions_summary
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&stream.video_id)
            .bind(&stream.title)
            .bind(&stream.view_count)
            .bind(&stream.streamed_date)
            .bind(
                stream
                    .timestamp_from_time_ago()
                    .context("Failed to get timestamp")?
                    .to_string(),
            )
            .bind(&stream.duration)
            .bind(&stream.closed_captions_summary)
            .execute(&mut **transaction)
            .await;

            match result {
                Ok(_) => successful_inserts += 1,
                Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
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

        Ok(BulkInsertResult {
            successful_inserts,
            failed_inserts,
        })
    }

    pub async fn get_stream(&self, video_id: &str) -> anyhow::Result<Option<Stream>> {
        let stream = sqlx::query_as::<_, Stream>("SELECT * FROM streams WHERE video_id = ?")
            .bind(video_id)
            .fetch_optional(&self.0)
            .await
            .context("Failed to get stream")?;

        Ok(stream)
    }

    pub async fn update_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE streams 
                SET title = ?, 
                view_count = ?,
                duration = ?,
                closed_captions_summary = ?
            WHERE video_id = ?
            "#,
        )
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.duration)
        .bind(&stream.closed_captions_summary)
        .bind(&stream.video_id)
        .execute(&self.0)
        .await
        .context("Failed to update stream")?;

        Ok(())
    }

    pub async fn delete_stream(&self, video_id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM streams WHERE video_id = ?")
            .bind(video_id)
            .execute(&self.0)
            .await
            .context("Failed to delete stream")?;

        Ok(())
    }

    pub async fn list_streams(&self) -> anyhow::Result<Vec<Stream>> {
        let streams =
            sqlx::query_as::<_, Stream>("SELECT * FROM streams ORDER BY stream_timestamp DESC")
                .fetch_all(&self.0)
                .await
                .context("Failed to list streams")?;

        Ok(streams)
    }
}

/// Represents the result of a bulk insert operation.
#[derive(Debug)]
pub struct BulkInsertResult {
    pub successful_inserts: usize,
    pub failed_inserts: Vec<FailedInsert>,
}

/// Represents a failed insert during a bulk insert operation.
#[derive(Debug)]
pub struct FailedInsert {
    pub video_id: String,
    pub reason: InsertFailReason,
}

/// Enumerates the possible reasons for a failed insert.
#[derive(Debug)]
pub enum InsertFailReason {
    DuplicateEntry,
    OtherError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stream;

    #[sqlx::test]
    async fn test_datastore_crud_operations() -> anyhow::Result<()> {
        let db = DataStore::new("sqlite::memory:").await?;

        // Test inserting a stream
        let stream = Stream {
            video_id: "abc123".to_string(),
            title: "Test Stream".to_string(),
            view_count: "1000 views".to_string(),
            streamed_date: "4 days ago".to_string(),
            duration: "1:30:00".to_string(),
            closed_captions_summary: "Test summary".to_string(),
        };
        db.insert_stream(&stream).await?;

        // Test retrieving the stream
        let retrieved = db.get_stream("abc123").await?.expect("Stream should exist");
        assert_eq!(retrieved.video_id, "abc123");
        assert_eq!(retrieved.title, "Test Stream");

        // Test updating the stream
        let mut updated_stream = retrieved;
        updated_stream.title = "Updated Test Stream".to_string();
        db.update_stream(&updated_stream).await?;

        let updated = db.get_stream("abc123").await?.expect("Stream should exist");
        assert_eq!(updated.title, "Updated Test Stream");

        // Test listing streams
        let streams = db.list_streams().await?;
        assert_eq!(streams.len(), 1);
        assert_eq!(streams[0].video_id, "abc123");

        // Test deleting the stream
        db.delete_stream("abc123").await?;
        let deleted = db.get_stream("abc123").await?;
        assert!(deleted.is_none(), "Stream should have been deleted");

        Ok(())
    }

    #[sqlx::test]
    async fn test_bulk_insert_streams_transaction() -> anyhow::Result<()> {
        let db = DataStore::new("sqlite::memory:").await?;

        let streams = vec![
            Stream {
                video_id: "unique1".to_string(),
                title: "Test Stream 1".to_string(),
                view_count: "1000 views".to_string(),
                streamed_date: "1 hour ago".to_string(),
                duration: "1:30:00".to_string(),
                closed_captions_summary: "Test summary".to_string(),
            },
            Stream {
                video_id: "unique2".to_string(),
                title: "Test Stream 2".to_string(),
                view_count: "2000 views".to_string(),
                streamed_date: "2 hours ago".to_string(),
                duration: "2:00:00".to_string(),
                closed_captions_summary: "Test summary".to_string(),
            },
            // Add a duplicate to test error handling
            Stream {
                video_id: "unique1".to_string(),
                title: "Duplicate Stream".to_string(),
                view_count: "3000 views".to_string(),
                streamed_date: "3 hours ago".to_string(),
                duration: "1:45:00".to_string(),
                closed_captions_summary: "Test summary".to_string(),
            },
        ];

        let result = db.bulk_insert_streams(&streams).await?;

        assert_eq!(result.successful_inserts, 2);
        assert_eq!(result.failed_inserts.len(), 1);
        assert!(matches!(
            result.failed_inserts[0].reason,
            InsertFailReason::DuplicateEntry
        ));

        // Verify that both non-duplicate streams were inserted
        let all_streams = db.list_streams().await?;
        assert_eq!(all_streams.len(), 2);

        Ok(())
    }

    #[sqlx::test]
    async fn test_datastore_error_handling() -> anyhow::Result<()> {
        let db = DataStore::new("sqlite::memory:").await?;

        // Test getting a non-existent stream
        let non_existent = db.get_stream("non_existent").await?;
        assert!(
            non_existent.is_none(),
            "Non-existent stream should return None"
        );

        // Test inserting a duplicate stream
        let stream = Stream {
            video_id: "duplicate".to_string(),
            title: "Duplicate Stream".to_string(),
            view_count: "100 views".to_string(),
            streamed_date: "1 month ago".to_string(),
            duration: "0:30:00".to_string(),
            ..Default::default()
        };
        db.insert_stream(&stream).await?;

        let result = db.insert_stream(&stream).await;
        assert!(result.is_err(), "Inserting a duplicate stream should fail");

        // Test updating a non-existent stream
        let non_existent_stream = Stream {
            video_id: "non_existent".to_string(),
            title: "Non-existent Stream".to_string(),
            view_count: "0 views".to_string(),
            streamed_date: "2 months ago".to_string(),
            duration: "0:15:00".to_string(),
            ..Default::default()
        };
        let result = db.update_stream(&non_existent_stream).await;
        assert!(
            result.is_ok(),
            "Updating a non-existent stream should not fail"
        );

        // Test deleting a non-existent stream
        let result = db.delete_stream("non_existent").await;
        assert!(
            result.is_ok(),
            "Deleting a non-existent stream should not fail"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_datastore_multiple_streams() -> anyhow::Result<()> {
        let db = DataStore::new("sqlite::memory:").await?;

        // Insert multiple streams
        for i in 1..=5 {
            let stream = Stream {
                video_id: format!("video{}", i),
                title: format!("Stream {}", i),
                view_count: format!("{} views", i * 100),
                streamed_date: if i == 1 {
                    format!("{} hour ago", i)
                } else {
                    format!("{} hours ago", i)
                },
                duration: format!("0:{}:00", i * 15),
                ..Default::default()
            };
            db.insert_stream(&stream).await?;
        }

        // Test listing all streams
        let streams = db.list_streams().await?;
        assert_eq!(streams.len(), 5, "Should have 5 streams");

        // Test retrieving specific streams
        for i in 1..=5 {
            let stream = db
                .get_stream(&format!("video{}", i))
                .await?
                .expect("Stream should exist");
            assert_eq!(stream.title, format!("Stream {}", i));
        }

        // Test updating a specific stream
        let mut stream3 = db
            .get_stream("video3")
            .await?
            .expect("Stream 3 should exist");
        stream3.title = "Updated Stream 3".to_string();
        db.update_stream(&stream3).await?;

        let updated_stream3 = db
            .get_stream("video3")
            .await?
            .expect("Updated Stream 3 should exist");
        assert_eq!(updated_stream3.title, "Updated Stream 3");

        // Test deleting a specific stream
        db.delete_stream("video2").await?;
        let streams = db.list_streams().await?;
        assert_eq!(streams.len(), 4, "Should have 4 streams after deletion");
        assert!(
            db.get_stream("video2").await?.is_none(),
            "Stream 2 should have been deleted"
        );

        Ok(())
    }
}
