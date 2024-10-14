use crate::Stream;
use anyhow::{anyhow, Context};
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn new<P: AsRef<Path>>(database_path: P) -> anyhow::Result<Self> {
        let conn = Connection::open(database_path)
            .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;

        // Create the streams table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS streams (
                video_id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                view_count TEXT NOT NULL,
                streamed_date TEXT NOT NULL,
                stream_timestamp DATETIME NOT NULL,
                duration TEXT NOT NULL,
                closed_captions_summary TEXT
            )
            "#,
            [],
        )
        .map_err(|e| anyhow!("Failed to create streams table: {}", e))?;

        Ok(DataStore { conn })
    }

    pub fn insert_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        let timestamp = stream
            .timestamp_from_time_ago()
            .context("Failed to get timestamp")?;

        let result = self.conn.execute(
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
            params![
                &stream.video_id,
                &stream.title,
                &stream.view_count,
                &stream.streamed_date,
                timestamp.to_string(),
                &stream.duration,
                &stream.closed_captions_summary,
            ],
        );

        match result {
            Ok(_) => Ok(()),
            Err(rusqlite::Error::SqliteFailure(error, _))
                if error.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                Err(anyhow!("Duplicate entry"))
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn stream_exists(&self, video_id: &str) -> anyhow::Result<bool> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM streams WHERE video_id = ?",
                params![video_id],
                |row| row.get(0),
            )
            .context("Failed to check if stream exists")?;

        Ok(count > 0)
    }

    pub fn bulk_insert_streams(&mut self, streams: &[Stream]) -> anyhow::Result<BulkInsertResult> {
        let mut successful_inserts = 0;
        let mut failed_inserts = Vec::new();

        let tx = self.conn.transaction()?;

        for stream in streams {
            let timestamp = stream
                .timestamp_from_time_ago()
                .context("Failed to get timestamp")?;

            let result = tx.execute(
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
                params![
                    &stream.video_id,
                    &stream.title,
                    &stream.view_count,
                    &stream.streamed_date,
                    timestamp.to_string(),
                    &stream.duration,
                    &stream.closed_captions_summary,
                ],
            );

            match result {
                Ok(_) => successful_inserts += 1,
                Err(rusqlite::Error::SqliteFailure(error, _))
                    if error.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
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

        tx.commit()?;

        Ok(BulkInsertResult {
            successful_inserts,
            failed_inserts,
        })
    }

    pub fn get_stream(&self, video_id: &str) -> anyhow::Result<Option<Stream>> {
        let result = self.conn.query_row(
            "SELECT * FROM streams WHERE video_id = ?",
            params![video_id],
            |row| {
                Ok(Stream {
                    video_id: row.get(0)?,
                    title: row.get(1)?,
                    view_count: row.get(2)?,
                    streamed_date: row.get(3)?,
                    duration: row.get(5)?,
                    closed_captions_summary: row.get(6)?,
                })
            },
        );

        match result {
            Ok(stream) => Ok(Some(stream)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn update_stream(&self, stream: &Stream) -> anyhow::Result<()> {
        self.conn
            .execute(
                r#"
            UPDATE streams 
            SET title = ?, 
                view_count = ?,
                duration = ?,
                closed_captions_summary = ?
            WHERE video_id = ?
            "#,
                params![
                    &stream.title,
                    &stream.view_count,
                    &stream.duration,
                    &stream.closed_captions_summary,
                    &stream.video_id,
                ],
            )
            .context("Failed to update stream")?;

        Ok(())
    }

    pub fn delete_stream(&self, video_id: &str) -> anyhow::Result<()> {
        self.conn
            .execute("DELETE FROM streams WHERE video_id = ?", params![video_id])
            .context("Failed to delete stream")?;

        Ok(())
    }

    pub fn list_streams(&self) -> anyhow::Result<Vec<Stream>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM streams ORDER BY stream_timestamp DESC")?;
        let stream_iter = stmt.query_map([], |row| {
            Ok(Stream {
                video_id: row.get(0)?,
                title: row.get(1)?,
                view_count: row.get(2)?,
                streamed_date: row.get(3)?,
                duration: row.get(5)?,
                closed_captions_summary: row.get(6)?,
            })
        })?;

        let streams: Result<Vec<_>, _> = stream_iter.collect();
        Ok(streams?)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> anyhow::Result<DataStore> {
        DataStore::new(":memory:")
    }

    #[test]
    fn test_datastore_crud_operations() -> anyhow::Result<()> {
        let db = create_test_db()?;

        // Test inserting a stream
        let stream = Stream {
            video_id: "abc123".to_string(),
            title: "Test Stream".to_string(),
            view_count: "1000 views".to_string(),
            streamed_date: "4 days ago".to_string(),
            duration: "1:30:00".to_string(),
            closed_captions_summary: "Test summary".to_string(),
        };
        db.insert_stream(&stream)?;

        // Test retrieving the stream
        let retrieved = db.get_stream("abc123")?.expect("Stream should exist");
        assert_eq!(retrieved.video_id, "abc123");
        assert_eq!(retrieved.title, "Test Stream");

        // Test updating the stream
        let mut updated_stream = retrieved;
        updated_stream.title = "Updated Test Stream".to_string();
        db.update_stream(&updated_stream)?;

        let updated = db.get_stream("abc123")?.expect("Stream should exist");
        assert_eq!(updated.title, "Updated Test Stream");

        // Test listing streams
        let streams = db.list_streams()?;
        assert_eq!(streams.len(), 1);
        assert_eq!(streams[0].video_id, "abc123");

        // Test deleting the stream
        db.delete_stream("abc123")?;
        let deleted = db.get_stream("abc123")?;
        assert!(deleted.is_none(), "Stream should have been deleted");

        Ok(())
    }

    #[test]
    fn test_bulk_insert_streams() -> anyhow::Result<()> {
        let mut db = create_test_db()?;

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

        let result = db.bulk_insert_streams(&streams)?;

        assert_eq!(result.successful_inserts, 2);
        assert_eq!(result.failed_inserts.len(), 1);
        assert!(matches!(
            result.failed_inserts[0].reason,
            InsertFailReason::DuplicateEntry
        ));

        // Verify that both non-duplicate streams were inserted
        let all_streams = db.list_streams()?;
        assert_eq!(all_streams.len(), 2);

        Ok(())
    }
}
