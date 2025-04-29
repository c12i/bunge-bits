//! # Cron Job Workflow for Parliament of Kenya Channel Streams
//!
//! This module implements a cron job that runs every 12 hours to fetch and process
//! archived streams from the Parliament of Kenya YouTube channel.
//!
//! ## Workflow Overview
//!
//! 1. **Initial Run**: Fetches and saves data for all 30 archived streams.
//! 2. **Subsequent Runs**: Checks for new streams and processes them.
//!
//! ## Process for New Streams
//!
//! When a new stream is detected:
//!
//! 1. Download and store the full transcript.
//! 2. Generate a structured summary using an LLM service.
//! 3. Create notifications for subscribers and update the web interface.
//!
//! ## Implementation Details
//!
//! - The cron job fetches the `ytInitialData` object from the YouTube channel.
//! - New streams are identified by comparing the most recent stream with the database.
//! - The initial run processes all 30 archived streams sequentially.
//! - Subsequent runs focus on identifying and processing new streams.

use anyhow::Result;
use stream_pulse::fetch_and_process_streams;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

// Should run every 12 hours
const CRON_EXPR: &str = "0 0 */12 * * *";
// const CRON_EXPR: &str = "*/15 * * * * *";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut scheduler = JobScheduler::new().await?;

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Africa::Nairobi)
        .with_cron_job_type()
        .with_schedule(CRON_EXPR)?
        .with_run_async(Box::new(|uuid, _| {
            Box::pin(async move {
                println!("Running cron job: {}", uuid);
                if let Err(e) = fetch_and_process_streams().await {
                    eprintln!("Error in cron job: {}", e);
                }
            })
        }))
        .build()?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    println!("Shutting down scheduler...");
    scheduler.shutdown().await?;

    Ok(())
}
