use std::{path::PathBuf, sync::LazyLock};

use anyhow::Context;
use rayon::prelude::*;
use stream_datastore::{DataStore, Stream};
use ytdlp_bindings::{AudioProcessor, YtDlp, YtDlpError};

use crate::{extract_json_from_script, parse_streams};

static CLIENT: LazyLock<Result<reqwest::Client, reqwest::Error>> = LazyLock::new(|| {
    reqwest::Client::builder()
    // Because sometimes, even bots want to feel like humans.
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
    .build()
});

static YTDLP: LazyLock<Result<YtDlp, YtDlpError>> = LazyLock::new(YtDlp::new);

#[allow(unused)]
static OPENAI: LazyLock<openai_dive::v1::api::Client> =
    LazyLock::new(openai_dive::v1::api::Client::new_from_env);

//  Parliament of Kenya Channel Stream URL
const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";

pub async fn fetch_and_process_streams() -> anyhow::Result<()> {
    let client = CLIENT.as_ref()?;
    let ytdlp = YTDLP.as_ref()?;

    let db = DataStore::new("bunge-bits-store.db").context("Failed to connect to database")?;

    let response = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;

    match extract_json_from_script(&response) {
        Ok(json) => {
            let streams = parse_streams(&json)?;

            // This is where initially downloaded audio by yt-dlp is saved
            let audio_download_path = PathBuf::from("/var/tmp/bunge-bits/audio");
            // This is were ausosubs are stored
            let autosub_path = PathBuf::from("/var/tmp/bunge-bits/autosub");

            let mut streams = streams
                .into_iter()
                .filter(|stream| match db.stream_exists(&stream.video_id) {
                    Ok(exists) => !exists,
                    Err(_) => false,
                })
                .collect::<Vec<Stream>>();

            streams.par_iter_mut().try_for_each(|stream| {
                let youtube_stream = format!("https://youtube.com/watch?v={}", stream.video_id);

                let mut audio_out_path = audio_download_path.join(&stream.video_id);
                let vtt_output_path = autosub_path.join(&stream.video_id);
                // This is the directory we store the chunked audio files
                let chunked_audio_path = PathBuf::from(format!(
                    "/var/tmp/bunge-bits/chunked-audio/{}",
                    stream.video_id
                ));

                // Download autosub
                ytdlp.download_auto_sub(&youtube_stream, &vtt_output_path)?;

                // Download audio file with yt-dlp
                ytdlp.download_audio(&youtube_stream, &audio_out_path)?;

                // set mp3 extension
                audio_out_path.set_extension("mp3");

                // Split downloaded audio to chunks
                ytdlp.split_audio_to_chunks(
                    audio_out_path,
                    1800,
                    chunked_audio_path.join("file_%03d.mp3"),
                )?;

                Ok::<_, anyhow::Error>(())
            })?;
        }
        Err(e) => {
            eprintln!("Error parsing streams: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai() {
        fetch_and_process_streams()
            .await
            .inspect_err(|e| eprintln!("Error: {}", e))
            .unwrap();
    }
}
