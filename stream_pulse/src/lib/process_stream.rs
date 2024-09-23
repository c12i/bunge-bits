use std::{path::PathBuf, sync::Arc};

use crate::{extract_json_from_script, parse_streams};
use anyhow::Result;
use futures::executor::block_on;
use stream_datastore::DataStore;
use vtt_digest::summarize_with_sliding_window;
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
    let autosub_path = PathBuf::from("var")
        .join("lib")
        .join("bunge-bits")
        .join("autosub");

    match extract_json_from_script(&response) {
        Ok(json) => {
            let mut streams = parse_streams(&json)?;

            for stream in streams.iter_mut() {
                // Process the new stream
                println!("Processing new stream: {}", stream.video_id);

                // check if stream exists in db i.e it was already processed
                let existing_stream = db.get_stream(&stream.video_id).await?;
                if existing_stream.is_some() {
                    continue;
                }

                // get video closed captions via yt-dlp
                let vtt_output_path = autosub_path.join(&stream.video_id);
                ytdlp.download_auto_sub(
                    &format!("https://youtube.com/watch?v={}", stream.video_id),
                    &vtt_output_path,
                )?;

                let vtt_file_path = PathBuf::from(format!("{:?}.en.vtt", vtt_output_path));
                let vtt_string = ytdlp.read_vtt_file(&vtt_file_path)?;

                let summary = summarize_with_sliding_window(
                    &vtt_string,
                    |chunk, context| Box::pin(async move { summarize_chunk(chunk, context).await }),
                    |summaries| Box::pin(async move { combine_summaries(summaries).await }),
                )
                .await?;

                stream.closed_captions_vtt = Some(vtt_string);
                stream.closed_captions_summary = Some(summary);

                db.insert_stream(stream).await?;
            }
        }
        Err(e) => {
            eprintln!("Error parsing streams: {}", e);
        }
    }

    Ok(())
}

#[allow(unused)]
async fn summarize_chunk(chunk: String, context: Option<Arc<String>>) -> anyhow::Result<String> {
    // TODO: Make an API call to OpenAI to summarize a chunk of vtt content
    //       if there is context from a previous summary, provide it in the prompt
    todo!()
}

#[allow(unused)]
async fn combine_summaries(summaries: Vec<String>) -> anyhow::Result<String> {
    // TODO: Make an API call to OpenAI to coherently summarize the contents of
    //       previous chunked summaries into a single one
    todo!()
}
