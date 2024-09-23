use std::path::PathBuf;

use crate::{extract_json_from_script, parse_streams};
use anyhow::Result;
use futures::executor::block_on;
use stream_datastore::DataStore;
use ytdlp_bindings::{VttProcessor, YtDlp, YtDlpError};

//  Parliament of Kenya Channel Stream URL
lazy_static::lazy_static! {
    static ref CLIENT: Result<reqwest::Client, reqwest::Error> = {
        reqwest::Client::builder()
            // Because sometimes, even bots want to feel like humans.
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
            .build()
    };

    static ref DB: Result<DataStore> = {
        block_on(DataStore::new("/var/lib/bunge-bits/store.db"))
    };

    static ref YTDLP: Result<YtDlp, YtDlpError> = {
        YtDlp::new()
    };
}

const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";

pub async fn fetch_and_process_streams() -> Result<()> {
    let client = CLIENT.as_ref()?;
    let db = DB.as_ref().unwrap();
    let ytdlp = YTDLP.as_ref()?;

    let response = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;
    let auto_sub_path = PathBuf::from("var")
        .join("lib")
        .join("bunge-bits")
        .join("auto_subs");

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

                // check if stream exists in db i.e it was already processed
                let existing_stream = db.get_stream(&stream.video_id).await?;
                if existing_stream.is_some() {
                    continue;
                }
                // get video closed captions via yt-dlp
                ytdlp.download_auto_sub(
                    &format!("https://youtube.com/watch?v={}", stream.video_id),
                    &auto_sub_path,
                )?;
            }
        }
        Err(e) => {
            eprintln!("Error parsing streams: {}", e);
        }
    }

    Ok(())
}
