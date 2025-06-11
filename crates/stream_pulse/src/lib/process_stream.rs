use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use anyhow::{bail, Context};
use openai_dive::v1::{
    api::Client as OpenAiClient,
    models::Gpt4Engine,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponse, ChatCompletionResponseFormat,
        ChatMessage, ChatMessageContent,
    },
};
use openai_dive::v1::{
    models::WhisperEngine,
    resources::{
        audio::{AudioOutputFormat, AudioTranscriptionParametersBuilder},
        shared::FileUpload,
    },
};
use rayon::prelude::*;
use stream_datastore::{DataStore, Stream};
use stream_digest::summarize_linear;
use ytdlp_bindings::{AudioProcessor, YtDlp};

use crate::{extract_json_from_script, parse_streams};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
static YTDLP: LazyLock<YtDlp> = LazyLock::new(|| YtDlp::new().expect("Failed to initialize YtDlp"));
static OPENAI: LazyLock<OpenAiClient> = LazyLock::new(openai_dive::v1::api::Client::new_from_env);

//  Parliament of Kenya Channel Stream URL
const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";
// Work directory - basically where all artifacts will be stored
const WORKDIR: &str = "/var/tmp/bunge-bits";
const TRANSCRIPT_CHUNK_DELIMITER: &str = "----END_OF_CHUNK----";

#[tracing::instrument]
pub async fn fetch_and_process_streams() -> anyhow::Result<()> {
    let client = &CLIENT;
    let ytdlp = &YTDLP;
    let openai = &OPENAI;

    let db = DataStore::new("bunge-bits-store.db").context("Failed to connect to database")?;

    let yt_html_document = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;
    match extract_json_from_script(&yt_html_document) {
        Ok(json) => {
            let mut streams = parse_streams(&json)?;
            tracing::info!("Processing `{}` streams", streams.len());

            // This is where initially downloaded audio by yt-dlp is saved
            let audio_download_path = PathBuf::from(format!("{WORKDIR}/audio"));

            // sort by upload date
            streams.sort_by(|a, b| {
                b.timestamp_from_time_ago()
                    .cmp(&a.timestamp_from_time_ago())
            });

            let mut streams = streams
                .into_iter()
                .filter(|stream| !db.stream_exists(&stream.video_id).unwrap_or(false))
                .collect::<Vec<Stream>>();

            // XXX: Revert to take all
            streams.par_iter_mut().take(1).try_for_each(|stream| {
                handle_stream_audio(stream, audio_download_path.clone(), ytdlp)
            })?;

            transcribe_streams(&streams, openai).await?;

            summarize_streams(&streams, Arc::new(OPENAI.clone())).await?;
        }
        Err(e) => {
            tracing::error!(error = ?e,  "Error parsing streams");
        }
    }

    Ok(())
}

#[tracing::instrument(skip(stream, ytdlp))]
fn handle_stream_audio(
    stream: &mut Stream,
    audio_download_path: PathBuf,
    ytdlp: &YtDlp,
) -> anyhow::Result<()> {
    let youtube_stream = format!("https://youtube.com/watch?v={}", stream.video_id);

    let mut audio_out_path = audio_download_path.join(&stream.video_id);

    // This is the directory we store the chunked audio files
    let chunked_audio_path = PathBuf::from(format!("{WORKDIR}/audio/{}", stream.video_id));

    // Download audio file with yt-dlp
    ytdlp.download_audio(&youtube_stream, &audio_out_path)?;

    // set mp3 extension
    audio_out_path.set_extension("mp3");

    // create nested `/stream.id/` dir
    create_dir_all(&chunked_audio_path).expect("Failed to create directories");

    // Split downloaded audio to chunks
    ytdlp.split_audio_to_chunks(
        audio_out_path,
        900,
        chunked_audio_path.join(format!("{}_%03d.mp3", stream.video_id)),
    )?;

    Ok(())
}

#[tracing::instrument(skip(streams, openai))]
async fn transcribe_streams(streams: &[Stream], openai: &OpenAiClient) -> anyhow::Result<()> {
    // XXX: Revert to take all
    for stream in streams.iter().take(1) {
        let audio_chunks_path = PathBuf::from(format!("{WORKDIR}/audio/{}", stream.video_id));
        let mut transcript_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{WORKDIR}/{}.txt", stream.video_id))?;

        let mut entries: Vec<_> = std::fs::read_dir(&audio_chunks_path)
            .context("Failed to read dir")?
            .collect::<Result<_, _>>()
            .context("Failed to collect dir entries")?;

        // fs::read_dir doesn't guarantee sorted dir contents, hence the need to
        // perform lexicographic sorting
        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            match transcribe_audio(entry.path(), openai).await {
                Ok(transcription) => {
                    write!(transcript_file, "{}", transcription)?;
                    writeln!(transcript_file, "{}", TRANSCRIPT_CHUNK_DELIMITER)?;
                }
                Err(err) => {
                    tracing::error!(error = ?err, "Skipping failed chunk {}", entry.path().display());
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

#[tracing::instrument(skip(openai))]
async fn transcribe_audio(audio_path: PathBuf, openai: &OpenAiClient) -> anyhow::Result<String> {
    let params = AudioTranscriptionParametersBuilder::default()
        .file(FileUpload::File(format!("{}", audio_path.display())))
        .model(WhisperEngine::Whisper1.to_string())
        .response_format(AudioOutputFormat::Text)
        .build()?;

    let max_retries = 3;
    let mut attempts = 0;

    loop {
        tracing::info!(
            "Transcribing audio from source {}. Attempt = {}",
            audio_path.display(),
            attempts
        );

        attempts += 1;
        match openai.audio().create_transcription(params.clone()).await {
            Ok(result) => {
                //XXX: Very basic check that it’s not a JSON error disguised as a string
                if result.trim_start().starts_with('{') {
                    tracing::warn!("Received unexpected JSON: {result}");
                    if attempts >= max_retries {
                        return Err(anyhow::anyhow!("Received JSON error instead of transcription after {attempts} attempts"));
                    }
                } else {
                    tracing::info!("Transcription success: {}", audio_path.display());
                    return Ok(result);
                }
            }
            Err(err) => {
                tracing::warn!("Attempt {attempts} failed for {:?}: {err:?}", audio_path);
                if attempts >= max_retries {
                    bail!("Failed after {attempts} attempts: {err}");
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(attempts))).await;
    }
}

#[tracing::instrument(skip(streams, openai))]
async fn summarize_streams(streams: &[Stream], openai: Arc<OpenAiClient>) -> anyhow::Result<()> {
    // XXX: Revert to take all
    for stream in streams.iter().take(1) {
        let transcript_path = format!("{WORKDIR}/{}.txt", stream.video_id);
        let transcript = std::fs::read_to_string(&transcript_path)
            .with_context(|| format!("Failed to read transcript at {}", transcript_path))?;

        summarize_linear(
            &transcript,
            TRANSCRIPT_CHUNK_DELIMITER,
            {
                let openai = openai.clone();
                move |chunk, ctx| {
                    let openai = openai.clone();
                    Box::pin(async move { summarize_chunk(chunk, ctx, &openai).await })
                }
            },
            {
                let openai = openai.clone();
                move |summaries| {
                    let openai = openai.clone();
                    Box::pin(async move { combine_summaries(summaries, &openai).await })
                }
            },
        )
        .await
        .with_context(|| format!("Failed to summarize stream {}", stream.video_id))?;
    }

    Ok(())
}

#[tracing::instrument(skip(chunk, context, openai))]
async fn summarize_chunk(
    chunk: String,
    context: Option<Arc<String>>,
    openai: &OpenAiClient,
) -> anyhow::Result<String> {
    let user_prompt = context
        .map(|ctx| {
            format!(
                r#"
Context:
{}

Transcript Chunk:
{}

Based on the transcript chunk and the provided context, please summarize it based on the instructions you received previously.
                "#,
                ctx, chunk
            )
        })
        .unwrap_or_else(|| {
            format!(
                r#"
Transcript Chunk:
{}

Based on the transcript chunk, please summarize it based on the instructions you received previously.
                "#,
                chunk
            )
        });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(include_str!("../prompts/system_0.txt").into()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(user_prompt),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let mut attempts = 0;
    let max_attempts = 5;

    loop {
        match openai.chat().create(parameters.clone()).await {
            Ok(response) => break chat_completions_text_from_response(response),
            Err(err) => {
                attempts += 1;
                let err_str = format!("{:?}", err);
                // In case of a 429 response, OpenAI will recommend a wait time
                // we try to use the recommended wait time here, otherwise the fallback is used
                let wait_ms = extract_wait_time_ms_from_error(&err_str).unwrap_or_else(|| {
                    let fallback = 2_u64.pow(attempts) * 1000;
                    tracing::warn!(
                        attempts,
                        "No wait time found, using fallback {}ms",
                        fallback
                    );
                    fallback
                });

                if attempts >= max_attempts {
                    tracing::error!(error = ?err, "Failed after {} attempts", attempts);
                    return Err(err.into());
                }

                tracing::warn!(
                    attempts,
                    wait_ms,
                    "Rate limit hit or other error. Retrying after {}ms",
                    wait_ms
                );

                tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;
            }
        }
    }
}

/// Try to extract wait time from potential 429 error response
fn extract_wait_time_ms_from_error(err_msg: &str) -> Option<u64> {
    let marker = "Please try again in ";
    if let Some(start) = err_msg.find(marker) {
        let after = &err_msg[start + marker.len()..];
        if let Some(end) = after.find("ms") {
            return after[..end].trim().parse::<u64>().ok();
        }
    }
    None
}

#[tracing::instrument(skip(openai))]
async fn combine_summaries(
    summaries: Vec<String>,
    openai: &OpenAiClient,
) -> anyhow::Result<String> {
    let summaries = summaries.join("\n");

    let prompt = format!(
        r#"
Given the following summaries of a video live stream chunks, combine them into a single coherent summary:

Summaries:
{}
        "#,
        summaries
    );

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![ChatMessage::User {
            content: ChatMessageContent::Text(prompt),
            name: None,
        }])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let mut attempts = 0;
    let max_attempts = 5;

    loop {
        match openai.chat().create(parameters.clone()).await {
            Ok(response) => break chat_completions_text_from_response(response),
            Err(err) => {
                attempts += 1;

                let err_str = format!("{:?}", err);
                let wait_ms = extract_wait_time_ms_from_error(&err_str).unwrap_or_else(|| {
                    let fallback = 2_u64.pow(attempts) * 1000;
                    tracing::warn!(
                        attempts,
                        "No wait time found, using fallback {}ms",
                        fallback
                    );
                    fallback
                });

                if attempts >= max_attempts {
                    tracing::error!(error = ?err, "combine_summaries failed after {} attempts", attempts);
                    return Err(err.into());
                }

                tracing::warn!(
                    error = ?err,
                    wait_ms,
                    attempts,
                    "Retrying combine_summaries after {}ms (attempt {}/{})",
                    wait_ms,
                    attempts,
                    max_attempts
                );

                tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;
            }
        }
    }
}

#[tracing::instrument(skip(response))]
pub fn chat_completions_text_from_response(
    response: ChatCompletionResponse,
) -> anyhow::Result<String> {
    let response = response
        .choices
        .first()
        .map(|c| c.to_owned())
        .context("response.choices is unexpectedly empty")?;

    let response = match response.message {
        ChatMessage::Assistant { content, .. } => {
            if let Some(content) = content {
                match content {
                    ChatMessageContent::Text(text) => text,
                    c => bail!("Unexpcted chat message content: {:?}", c),
                }
            } else {
                bail!("Unexpected absence of chage message content");
            }
        }
        c => bail!("Unexpcted chat message response: {:?}", c),
    };

    Ok(response)
}
