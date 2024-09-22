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
//! ## Stream Data Structure
//!
//! ```rust
//! struct Stream {
//!     video_id: String,
//!     title: String,
//!     view_count: String,
//!     streamed_date: String,
//!     duration: String,
//! }
//! ```
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
//!
//! ## Note
//!
//! The system is designed to handle one or more archived streams per run.
//! Different services may be responsible for various stages of the workflow.

use anyhow::Result;
use stream_pulse::{extract_json_from_script, parse_streams};
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

lazy_static::lazy_static! {
    static ref CLIENT: Result<reqwest::Client, reqwest::Error> = {
        reqwest::Client::builder()
            // Because sometimes, even bots want to feel like humans.
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
            .build()
    };
}

//  Parliament of Kenya Channel Stream URL
const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";
// Should run every 12 hours
const CRON_EXPR: &str = "0 0 */12 * * *";
// const CRON_EXPR: &str = "*/15 * * * * *";

async fn fetch_and_process_streams() -> Result<()> {
    let client = CLIENT.as_ref()?;
    let response = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;

    match extract_json_from_script(&response) {
        Ok(json) => {
            let streams = parse_streams(&json)?;

            for stream in streams {
                // Process the new stream
                println!("Processing new stream: {}", stream.video_id);
                // TODO: Implement stream processing logic here
                // 1. Download and store the full transcript.
                // 2. Generate a structured summary using an LLM service.
                // 3. Create notifications for subscribers and update the web interface.
            }
        }
        Err(e) => {
            eprintln!("Error parsing streams: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
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
