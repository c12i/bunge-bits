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
//!     id: String,
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
use reqwest;
use yt_scrape::{extract_json_from_script, parse_streams};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";
    let response = reqwest::get(url).await?.text().await?;

    match extract_json_from_script(&response) {
        Ok(json) => {
            let dat = parse_streams(&json);
            println!("{:#?}", dat);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    Ok(())
}
