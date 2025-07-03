//! # Cron Job Runner for Bunge Bits
//!
//! This module defines the main cron job logic for Bunge Bits
//!
//! ## Overview
//!
//! A scheduled job runs every 4 hours (by default), fetching and processing newly archived
//! livestreams. The job uses the `tokio-cron-scheduler` crate with timezone awareness
//! (`Africa/Nairobi`) to match the Kenyan parliamentary schedule.
//!
//! Each job:
//! - Determines how many streams to process based on an environment variable
//! - Calls [`fetch_and_process_streams`] to perform the full pipeline
//! - Handles errors and panics gracefully with structured `tracing` logs
//!
//! The module also exposes a live status mechanism:
//! - A background loop updates `AppState.next_tick_for_job` every 5 seconds
//! - This is used by the HTTP server's `/status` endpoint to report upcoming job ticks
//!
//! ## Shutdown
//!
//! The cron runner listens for a `SIGINT` (`Ctrl+C`) signal and shuts down cleanly.
//!
//! ## Environment Variables
//!
//! - `CRON_SCHEDULE`: Custom cron string (optional, defaults to "0 0 */4 * * *")
//! - `MAX_STREAMS_TO_PROCESS`: Limits how many streams are processed per run

use std::{sync::Arc, time::Duration};

use chrono_tz::Africa::Nairobi;
use futures::FutureExt;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};
use uuid::Uuid;

use crate::fetch_and_process_streams;

use super::AppState;

pub async fn start_cron(cron_schedule: &str, app_state: Arc<AppState>) -> anyhow::Result<()> {
    tracing::info!(%cron_schedule);

    let mut scheduler = JobScheduler::new().await?;

    let job_id = Uuid::new_v4();

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Africa::Nairobi)
        .with_job_id(job_id.into())
        .with_cron_job_type()
        .with_schedule(cron_schedule)?
        .with_run_async(Box::new(|uuid, _| {
            Box::pin(async move {
                // Maximum streams that can be processed in a run
                let max_streams = std::env::var("MAX_STREAMS_TO_PROCESS")
                    .ok()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(3);

                tracing::info!(job_id = %uuid, max_streams, "Running cron job...");

                let result = std::panic::AssertUnwindSafe(fetch_and_process_streams(max_streams))
                    .catch_unwind()
                    .await;

                match result {
                    Ok(Ok(_)) => {
                        tracing::info!(job_id = %uuid, "Cron job completed successfully");
                    }
                    Ok(Err(err)) => {
                        tracing::error!(job_id = %uuid, error = ?err, "Fetch and process streams pipeline failed");
                    }
                    Err(panic_err) => {
                        tracing::error!(job_id = %uuid, error = ?panic_err, "Fetch and process streams pipeline failed");
                    }
                }
            })
        }))
        .build()?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutting down scheduler...");
            scheduler.shutdown().await?;
        }
        _ = check_time_till_next_job(&mut scheduler, job_id, app_state) => {}
    }

    Ok(())
}

async fn check_time_till_next_job(
    scheduler: &mut JobScheduler,
    job_id: Uuid,
    app_state: Arc<AppState>,
) -> anyhow::Result<()> {
    loop {
        let time = scheduler.next_tick_for_job(job_id).await?;

        if let Some(next) = time {
            let tz_time = next.with_timezone(&Nairobi);
            if let Ok(mut lock) = app_state.next_tick_for_job.lock() {
                *lock = Some(tz_time);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
