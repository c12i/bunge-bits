//! # DataStore Module
//!
//! This module provides functionality for interacting with a SQLite database
//! to store and retrieve information about YouTube streams and their closed captions.
//!
//! The module uses sqlx for database operations and provides an abstraction layer
//! for CRUD operations on streams and their associated closed captions.

use anyhow::Context;
use sqlx::{Sqlite, SqlitePool, Transaction};

use crate::{error::YtScrapeError, Stream};

#[derive(Debug, Clone)]
pub struct DataStore(SqlitePool);

/// Represents the closed captions associated with a YouTube stream.
#[derive(Debug, sqlx::FromRow)]
pub struct StreamClosedCaptions {
    pub video_id: String,
    pub closed_caption_text: String,
    pub closed_caption_summary: Option<String>,
}

impl DataStore {
    pub async fn new(database_url: &str) -> Result<Self, YtScrapeError> {
        let pool = SqlitePool::connect(database_url)
            .await
            .context("Failed to connect to database")?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS streams (
            video_id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            view_count TEXT NOT NULL,
            streamed_date TEXT NOT NULL,
            duration TEXT NOT NULL,
            UNIQUE(video_id)
        )"#,
        )
        .execute(&pool)
        .await
        .context("Failed to create streams table")?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS stream_closed_captions (
              video_id TEXT PRIMARY KEY,
              closed_caption_text TEXT NOT NULL,
              closed_caption_summary TEXT,
              FOREIGN KEY (video_id) REFERENCES streams(video_id)
          )"#,
        )
        .execute(&pool)
        .await
        .context("Failed to create stream_closed_captions table")?;

        Ok(DataStore(pool))
    }

    pub async fn insert_stream(&self, stream: &Stream) -> Result<(), YtScrapeError> {
        let result = sqlx::query(
            "INSERT INTO streams (video_id, title, view_count, streamed_date, duration)
           VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&stream.video_id)
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.streamed_date)
        .bind(&stream.duration)
        .execute(&self.0)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(YtScrapeError::UniqueConstraintViolation(db_err.into()))
            }
            Err(err) => Err(YtScrapeError::InternalError(err.into())),
        }
    }

    pub async fn bulk_insert_streams(
        &self,
        streams: &[Stream],
    ) -> Result<BulkInsertResult, YtScrapeError> {
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
    ) -> Result<BulkInsertResult, YtScrapeError> {
        let mut successful_inserts = 0;
        let mut failed_inserts = Vec::new();

        for stream in streams {
            let result = sqlx::query(
                "INSERT INTO streams (video_id, title, view_count, streamed_date, duration)
              VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&stream.video_id)
            .bind(&stream.title)
            .bind(&stream.view_count)
            .bind(&stream.streamed_date)
            .bind(&stream.duration)
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

    pub async fn get_stream(&self, video_id: &str) -> Result<Option<Stream>, YtScrapeError> {
        let stream = sqlx::query_as::<_, Stream>("SELECT * FROM streams WHERE video_id = ?")
            .bind(video_id)
            .fetch_optional(&self.0)
            .await
            .context("Failed to get stream")?;

        Ok(stream)
    }

    pub async fn update_stream(&self, stream: &Stream) -> Result<(), YtScrapeError> {
        sqlx::query(
            "UPDATE streams SET title = ?, view_count = ?, streamed_date = ?, duration = ?
           WHERE video_id = ?",
        )
        .bind(&stream.title)
        .bind(&stream.view_count)
        .bind(&stream.streamed_date)
        .bind(&stream.duration)
        .bind(&stream.video_id)
        .execute(&self.0)
        .await
        .context("Failed to update stream")?;

        Ok(())
    }

    pub async fn delete_stream(&self, video_id: &str) -> Result<(), YtScrapeError> {
        sqlx::query("DELETE FROM streams WHERE video_id = ?")
            .bind(video_id)
            .execute(&self.0)
            .await
            .context("Failed to delete stream")?;

        Ok(())
    }

    pub async fn list_streams(&self) -> Result<Vec<Stream>, YtScrapeError> {
        let streams = sqlx::query_as::<_, Stream>("SELECT * FROM streams")
            .fetch_all(&self.0)
            .await
            .context("Failed to list streams")?;

        Ok(streams)
    }

    pub async fn insert_closed_captions(
        &self,
        closed_captions: &StreamClosedCaptions,
    ) -> Result<(), YtScrapeError> {
        sqlx::query(
          r#"INSERT INTO stream_closed_captions (video_id, closed_caption_text, closed_caption_summary)
             VALUES (?, ?, ?)"#,
      )
      .bind(&closed_captions.video_id)
      .bind(&closed_captions.closed_caption_text)
      .bind(&closed_captions.closed_caption_summary)
      .execute(&self.0)
      .await
      .context("Failed to insert closed captions")?;

        Ok(())
    }

    pub async fn get_closed_captions(
        &self,
        video_id: &str,
    ) -> Result<Option<StreamClosedCaptions>, YtScrapeError> {
        let closed_captions = sqlx::query_as::<_, StreamClosedCaptions>(
            "SELECT * FROM stream_closed_captions WHERE video_id = ?",
        )
        .bind(video_id)
        .fetch_optional(&self.0)
        .await
        .context("Failed to get closed captions")?;

        Ok(closed_captions)
    }

    pub async fn update_closed_captions(
        &self,
        closed_captions: &StreamClosedCaptions,
    ) -> Result<(), YtScrapeError> {
        sqlx::query(
            r#"UPDATE stream_closed_captions 
             SET closed_caption_text = ?, closed_caption_summary = ?
             WHERE video_id = ?"#,
        )
        .bind(&closed_captions.closed_caption_text)
        .bind(&closed_captions.closed_caption_summary)
        .bind(&closed_captions.video_id)
        .execute(&self.0)
        .await
        .context("Failed to update closed captions")?;

        Ok(())
    }

    pub async fn delete_closed_captions(&self, video_id: &str) -> Result<(), YtScrapeError> {
        sqlx::query("DELETE FROM stream_closed_captions WHERE video_id = ?")
            .bind(video_id)
            .execute(&self.0)
            .await
            .context("Failed to delete closed captions")?;

        Ok(())
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
    use anyhow::Result;

    #[sqlx::test]
    async fn test_datastore_crud_operations() -> Result<(), YtScrapeError> {
        let db = DataStore::new("sqlite::memory:").await?;

        // Test inserting a stream
        let stream = Stream {
            video_id: "abc123".to_string(),
            title: "Test Stream".to_string(),
            view_count: "1000 views".to_string(),
            streamed_date: "2023-05-01".to_string(),
            duration: "1:30:00".to_string(),
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
    async fn test_bulk_insert_streams_transaction() -> Result<(), YtScrapeError> {
        let db = DataStore::new("sqlite::memory:").await?;

        let streams = vec![
            Stream {
                video_id: "unique1".to_string(),
                title: "Test Stream 1".to_string(),
                view_count: "1000 views".to_string(),
                streamed_date: "2023-05-01".to_string(),
                duration: "1:30:00".to_string(),
            },
            Stream {
                video_id: "unique2".to_string(),
                title: "Test Stream 2".to_string(),
                view_count: "2000 views".to_string(),
                streamed_date: "2023-05-02".to_string(),
                duration: "2:00:00".to_string(),
            },
            // Add a duplicate to test error handling
            Stream {
                video_id: "unique1".to_string(),
                title: "Duplicate Stream".to_string(),
                view_count: "3000 views".to_string(),
                streamed_date: "2023-05-03".to_string(),
                duration: "1:45:00".to_string(),
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
    async fn test_datastore_error_handling() -> Result<(), YtScrapeError> {
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
            streamed_date: "2023-05-02".to_string(),
            duration: "0:30:00".to_string(),
        };
        db.insert_stream(&stream).await?;

        let result = db.insert_stream(&stream).await;
        assert!(result.is_err(), "Inserting a duplicate stream should fail");

        // Test updating a non-existent stream
        let non_existent_stream = Stream {
            video_id: "non_existent".to_string(),
            title: "Non-existent Stream".to_string(),
            view_count: "0 views".to_string(),
            streamed_date: "2023-05-03".to_string(),
            duration: "0:15:00".to_string(),
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
    async fn test_datastore_multiple_streams() -> Result<(), YtScrapeError> {
        let db = DataStore::new("sqlite::memory:").await?;

        // Insert multiple streams
        for i in 1..=5 {
            let stream = Stream {
                video_id: format!("video{}", i),
                title: format!("Stream {}", i),
                view_count: format!("{} views", i * 100),
                streamed_date: format!("2023-05-{:02}", i),
                duration: format!("0:{}:00", i * 15),
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

    #[sqlx::test]
    async fn test_closed_captions_crud() -> Result<(), YtScrapeError> {
        let db = DataStore::new("sqlite::memory:").await?;

        // First, insert a stream
        let stream = Stream {
            video_id: "test123".to_string(),
            title: "Test Stream".to_string(),
            view_count: "1000 views".to_string(),
            streamed_date: "2023-05-01".to_string(),
            duration: "1:30:00".to_string(),
        };
        db.insert_stream(&stream).await?;

        // Test inserting closed captions
        let closed_captions = StreamClosedCaptions {
            video_id: "test123".to_string(),
            closed_caption_text: "Test transcript".to_string(),
            closed_caption_summary: Some("Test summary".to_string()),
        };
        db.insert_closed_captions(&closed_captions).await?;

        // Test retrieving closed captions
        let retrieved = db.get_closed_captions("test123").await?.unwrap();
        assert_eq!(retrieved.closed_caption_text, "Test transcript");

        // Test updating closed captions
        let updated_captions = StreamClosedCaptions {
            video_id: "test123".to_string(),
            closed_caption_text: "Updated transcript".to_string(),
            closed_caption_summary: Some("Updated summary".to_string()),
        };
        db.update_closed_captions(&updated_captions).await?;

        let updated = db.get_closed_captions("test123").await?.unwrap();
        assert_eq!(updated.closed_caption_text, "Updated transcript");

        // Test deleting closed captions
        db.delete_closed_captions("test123").await?;
        let deleted = db.get_closed_captions("test123").await?;
        assert!(deleted.is_none());

        Ok(())
    }
}
