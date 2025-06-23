use crate::domain::TIME_AGO_REGEX;
use crate::Stream;
use anyhow::Context;
use itertools::Either;
use itertools::Itertools;
use sqlx::migrate::Migrator;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashSet, sync::LazyLock};
#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: PgPool,
}

static MIGRATOR: Migrator = sqlx::migrate!();

impl DataStore {
    /// Establish connection to database and create the streams table
    /// if not exists
    pub async fn init(database_url: &str) -> anyhow::Result<Self> {
        LazyLock::force(&TIME_AGO_REGEX);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .inspect_err(
                |e| tracing::error!(error = ?e, "Failed to establish connection to database"),
            )
            .context("Failed to connect to database")?;

        MIGRATOR
            .run(&pool)
            .await
            .inspect_err(|e| tracing::error!(error = ?e, "Failed to run database migrations"))
            .context("Failed to run database migrations")?;

        Ok(DataStore { pool })
    }

    pub async fn get_existing_stream_ids(
        &self,
        video_ids: &[&str],
    ) -> anyhow::Result<HashSet<String>> {
        #[derive(sqlx::FromRow)]
        struct VideoId {
            video_id: String,
        }
        let streams =
            sqlx::query_as::<_, VideoId>("SELECT video_id FROM streams WHERE video_id = ANY($1)")
                .bind(video_ids)
                .fetch_all(&self.pool)
                .await
                .inspect_err(|e| {
                    tracing::error!(error = ?e, "Failed to fetch existing streams");
                })
                .context("Failed to fetch existing streams")?;

        Ok(streams.into_iter().map(|s| s.video_id).collect())
    }

    #[tracing::instrument(skip(self, streams))]
    pub async fn bulk_insert_streams(
        &self,
        streams: &[Stream],
    ) -> anyhow::Result<BulkInsertResult> {
        let (valid_streams, invalid_stream_date_errors): (Vec<_>, Vec<_>) =
            streams.iter().partition_map(|stream| {
                if let Some(timestamp) = stream.timestamp_from_time_ago() {
                    Either::Left((stream.clone(), timestamp))
                } else {
                    let reason = InsertFailReason::InvalidStreamedDate {
                        malformed_date: stream.streamed_date.clone(),
                    };
                    Either::Right(FailedInsert {
                        video_id: stream.video_id.clone(),
                        reason,
                    })
                }
            });

        let (video_ids, title, view_counts, streamed_dates, durations, summaries, timestamp_md): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = valid_streams
            .iter()
            .map(|(stream, stream_date)| {
                (
                    stream.video_id.clone(),
                    stream.title.clone(),
                    stream.view_count.clone(),
                    *stream_date,
                    stream.duration.clone(),
                    stream.summary_md.clone(),
                    stream.timestamp_md.clone(),
                )
            })
            .multiunzip();

        let pg_result = sqlx::query(
            "
            INSERT INTO streams (video_id, title, view_count,stream_timestamp, duration, summary_md, timestamp_md)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::timestamptz[], $5::text[], $6::text[], $7::text[]) ON CONFLICT DO NOTHING
            "
        )
        .bind(&video_ids[..])
        .bind(&title[..])
        .bind(&view_counts[..])
        .bind(&streamed_dates[..])
        .bind(&durations[..])
        .bind(&summaries[..])
        .bind(&timestamp_md[..])
        .execute(&self.pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                error = ?err,
                "Failed to execute bulk insert for streams"
            )
        })
        .context("Failed to execute bulk insert for streams")?;

        let successful_inserts = pg_result.rows_affected() as usize;

        if !invalid_stream_date_errors.is_empty() {
            tracing::warn!(
                invalid_stream_date_errors = ?invalid_stream_date_errors,
                "Some streams had invalid streamed_date formats and were not inserted"
            )
        }

        Ok(BulkInsertResult {
            successful_inserts,
            failed_inserts: invalid_stream_date_errors,
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
    InvalidStreamedDate { malformed_date: String },
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_bulk_insert_and_check_existing_streams_works(pool: PgPool) {
        let datastore = DataStore { pool };

        let streams = vec![
            Stream {
                video_id: "test_video_1".to_string(),
                title: "Test Video 1".to_string(),
                view_count: 100.to_string(),
                streamed_date: "1 hour ago".to_string(),
                duration: chrono::Duration::seconds(3600).to_string(),
                summary_md: Some("This is a test video summary".to_owned()),
                timestamp_md: Some(Utc::now().to_string()),
            },
            Stream {
                video_id: "test_video_2".to_string(),
                title: "Test Video 2".to_string(),
                view_count: 200.to_string(),
                streamed_date: "2 days ago".to_string(),
                duration: chrono::Duration::seconds(7200).to_string(),
                summary_md: Some("This is another test video summary".to_owned()),
                timestamp_md: Some(Utc::now().to_string()),
            },
            Stream {
                video_id: "test_video_3".to_string(),
                title: "Test Video 3".to_string(),
                view_count: 300.to_string(),
                streamed_date: "4 weeks ago".to_string(),
                duration: chrono::Duration::seconds(1800).to_string(),
                summary_md: None,
                timestamp_md: None,
            },
            // This stream has an invalid streamed_date format
            Stream {
                video_id: "test_video_invalid".to_string(),
                title: "Invalid Stream".to_string(),
                view_count: 50.to_string(),
                streamed_date: "invalid date format".to_string(),
                duration: chrono::Duration::seconds(600).to_string(),
                summary_md: Some("This stream has an invalid date format".to_owned()),
                timestamp_md: Some(Utc::now().to_string()),
            },
        ];

        // Insert streams
        let result = datastore.bulk_insert_streams(&streams).await.unwrap();

        // Check results
        assert_eq!(result.successful_inserts, 3);
        assert_eq!(result.failed_inserts.len(), 1);

        let invalid_stream_ids = result
            .failed_inserts
            .iter()
            .map(|f| f.video_id.clone())
            .collect::<HashSet<_>>();

        let existing_steams = datastore
            .get_existing_stream_ids(&streams.iter().map(|s| s.video_id.as_str()).collect_vec())
            .await
            .unwrap();
        // Verify that valid streams were inserted
        for stream in streams
            .iter()
            .filter(|s| !invalid_stream_ids.contains(&s.video_id))
        {
            assert!(existing_steams.contains(&stream.video_id));
        }

        // verify that the invalid streams were in the failed_inserts list
        let expected_invalid_streams = vec!["test_video_invalid".to_string()];
        for invalid_stream in result.failed_inserts {
            assert!(expected_invalid_streams.contains(&invalid_stream.video_id));
        }
    }
}
