use std::{path::PathBuf, sync::Arc};

use crate::{extract_json_from_script, parse_streams};
use anyhow::anyhow;
use futures::executor::block_on;
use rayon::prelude::*;
use stream_datastore::DataStore;
use stream_digest::summarize_with_sliding_window;
use ytdlp_bindings::{AudioProcessor, VttProcessor, YtDlp, YtDlpError};

//  Parliament of Kenya Channel Stream URL
lazy_static::lazy_static! {
    static ref CLIENT: Result<reqwest::Client, reqwest::Error> = {
        reqwest::Client::builder()
            // Because sometimes, even bots want to feel like humans.
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
            .build()
    };

    static ref DB: Result<DataStore, anyhow::Error> = {
        block_on(DataStore::new("/var/lib/bunge-bits/store.db"))
    };

    static ref YTDLP: Result<YtDlp, YtDlpError> = {
        YtDlp::new()
    };

    static ref OPENAI: openai_dive::v1::api::Client = {
        openai_dive::v1::api::Client::new_from_env()
    };
}

const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";

pub async fn fetch_and_process_streams() -> anyhow::Result<()> {
    let client = CLIENT.as_ref()?;
    let db = DB
        .as_ref()
        .map_err(|_| anyhow!("Failed to connect to database"))?;
    let ytdlp = YTDLP.as_ref()?;

    let response = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;
    let autosub_path = PathBuf::from("/var/tmp/bunge-bits/autosub");

    match extract_json_from_script(&response) {
        Ok(json) => {
            let mut streams = parse_streams(&json)?;

            // This is where initially downloaded audio by yt-dlp is saved
            let audio_download_path = PathBuf::from("/tmp/bunge-bits/audio");

            streams.par_iter().try_for_each(|stream| {
                let mut audio_out_path = audio_download_path.join(&stream.video_id);

                // Download audio file with yt-dlp
                ytdlp.download_audio(
                    &format!("https://youtube.com/watch?v={}", stream.video_id),
                    &audio_out_path,
                )?;

                // set mp3 extension
                audio_out_path.set_extension("mp3");

                // This is the directory we store the chunked audio files
                let chunked_audio_path =
                    PathBuf::from(format!("/tmp/bunge-bits/{}", stream.video_id));

                // Split downloaded audio to chunks
                ytdlp.split_audio_to_chunks(
                    audio_out_path,
                    1800,
                    chunked_audio_path.join("file_%03d.mp3"),
                )?;

                Ok::<_, anyhow::Error>(())
            })?;

            // TODO: Iterate through each newly created directory with chunked
            //       audio files and call LLM service to transcribe and summarize
            //       Can this be achived in parallel as well?
            //       Are there any rate limiting constraints with OpenAI's API?

            for stream in streams.iter_mut() {
                // Process the new stream
                println!("Processing new stream: {}", stream.video_id);

                // check if stream exists in db i.e it was already processed
                let existing_stream = db.get_stream(&stream.video_id).await?;
                if existing_stream.is_some() {
                    continue;
                }

                // TODO: Rather than fetching the stream's closed captions
                //       We use yt-dlp to download the stream's audio
                //       Then use ffmpeeg via the ytdlp bindings to split the audio
                //       into chunks.
                //       Once this is done; process each audio chunk and send to the
                //       LLM service for transcription & summarization still making use
                //       of the sliding window technique
                //       In terms of performance, this could be expensive indeed
                //       Perhaps we could parallelize the download and audio extraction
                //       process to improve runtime

                // get video closed captions via yt-dlp
                let vtt_output_path = autosub_path.join(&stream.video_id);
                ytdlp.download_auto_sub(
                    &format!("https://youtube.com/watch?v={}", stream.video_id),
                    &vtt_output_path,
                )?;

                // XXX: Assumes that the stream getting processed contains English
                //      closed captions
                let vtt_file_path = PathBuf::from(format!("{:?}.en.vtt", vtt_output_path));
                let vtt_string = ytdlp.read_vtt_file(&vtt_file_path)?;

                let summary = summarize_with_sliding_window(
                    &vtt_string,
                    |chunk, context| Box::pin(async move { summarize_chunk(chunk, context).await }),
                    |summaries| Box::pin(async move { combine_summaries(summaries).await }),
                )
                .await?;

                stream.closed_captions_summary = summary;

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
    let summary_str = summaries.join("\n");
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai() {
        let client = &OPENAI;
        let models = client.models().list().await.unwrap();
        println!("{:?}", models);
    }
}
