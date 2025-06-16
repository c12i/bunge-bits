use std::{
    fs::{create_dir_all, remove_dir_all, OpenOptions},
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
// Maximum streams that can be processed in a run
const MAX_STREAMS_TO_PROCESS: usize = 3;

#[tracing::instrument]
pub async fn fetch_and_process_streams() -> anyhow::Result<()> {
    let client = &CLIENT;
    let ytdlp = &YTDLP;
    let openai = &OPENAI;

    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let db = DataStore::init(&db_url)
        .await
        .context("Failed to initialize database")?;

    let yt_html_document = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;
    match extract_json_from_script(&yt_html_document) {
        Ok(json) => {
            let mut streams = parse_streams(&json)?;
            tracing::info!(count = streams.len(), "Processing streams");

            // This is where initially downloaded audio by yt-dlp is saved
            let audio_download_path = PathBuf::from(format!("{WORKDIR}/audio"));

            let mut streams = sort_and_filter_existing_streams(&db, &mut streams).await;

            if streams.is_empty() {
                tracing::info!("No streams to process at this time");
                return Ok(());
            }

            streams.par_iter_mut().try_for_each(|stream| {
                handle_stream_audio(stream, audio_download_path.clone(), ytdlp)
            })?;

            transcribe_streams(&streams, openai).await?;

            summarize_streams(&mut streams, Arc::new(OPENAI.clone()), &db).await?;

            // TODO: Generate timestamp_md
        }
        Err(e) => {
            tracing::error!(error = ?e,  "Error extracing ytInitialData from the html document");
            bail!(
                "Failed to extract ytInitialData from html document: {:?}",
                e
            );
        }
    }

    cleanup_audio_dir();

    Ok(())
}

#[tracing::instrument(skip(stream, ytdlp))]
fn handle_stream_audio(
    stream: &mut Stream,
    audio_download_path: PathBuf,
    ytdlp: &YtDlp,
) -> anyhow::Result<()> {
    let youtube_stream = format!("https://youtube.com/watch?v={}", stream.video_id);

    // Pass path without extension to yt-dlp
    let audio_base_path = audio_download_path.join(&stream.video_id);
    let audio_mp3_path = audio_base_path.with_extension("mp3");

    let chunked_audio_path = PathBuf::from(format!("{WORKDIR}/audio/{}", stream.video_id));

    // Skip download if .mp3 already exists
    if !audio_mp3_path.exists() {
        ytdlp.download_audio(&youtube_stream, &audio_base_path)?;
    } else {
        tracing::debug!("Audio already exists at {:?}", audio_mp3_path);
    }

    // Skip splitting if chunk files already exist
    let chunk_exists = std::fs::read_dir(&chunked_audio_path)
        .map(|mut entries| entries.any(|e| e.is_ok()))
        .unwrap_or(false);

    if !chunk_exists {
        create_dir_all(&chunked_audio_path)?;
        ytdlp.split_audio_to_chunks(
            &audio_mp3_path,
            900,
            chunked_audio_path.join(format!("{}_%03d.mp3", stream.video_id)),
        )?;
    } else {
        tracing::debug!("Chunks already exist at {:?}", chunked_audio_path);
    }

    Ok(())
}

#[tracing::instrument(skip(streams, openai))]
async fn transcribe_streams(streams: &[Stream], openai: &OpenAiClient) -> anyhow::Result<()> {
    for stream in streams {
        let audio_chunks_path = PathBuf::from(format!("{WORKDIR}/audio/{}", stream.video_id));
        let mut transcript_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{WORKDIR}/{}.txt", stream.video_id))?;

        let mut entries = std::fs::read_dir(&audio_chunks_path)
            .context("Failed to read dir")?
            .collect::<Result<Vec<_>, _>>()
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

    let max_retries = 5;
    let mut attempt = 0;

    loop {
        tracing::info!(attempt, audio_path = %audio_path.display(), "Transcribing audio from source",);

        attempt += 1;
        match openai.audio().create_transcription(params.clone()).await {
            Ok(result) => {
                //XXX: Very basic check that it’s not a JSON error disguised as a string
                if result.trim_start().starts_with('{') {
                    tracing::warn!("Received unexpected JSON: {result}");
                    if attempt >= max_retries {
                        bail!(
                            "Received JSON error instead of transcription after {attempt} attempts"
                        );
                    }
                } else {
                    tracing::info!("Transcription success for {}", audio_path.display());
                    return Ok(result);
                }
            }
            Err(err) => {
                tracing::warn!(attempt, error = ?err, "Transcription failed for {} (attempt ({}/{}))", audio_path.display(), attempt, max_retries);
                if attempt >= max_retries {
                    bail!("Failed after {attempt} attempts: {err}");
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(attempt))).await;
    }
}

// TODO: Stream resume support
#[tracing::instrument(skip(streams, openai, db))]
async fn summarize_streams(
    streams: &mut [Stream],
    openai: Arc<OpenAiClient>,
    db: &DataStore,
) -> anyhow::Result<()> {
    for stream in streams.iter_mut() {
        let transcript_path = format!("{WORKDIR}/{}.txt", stream.video_id);
        let transcript = std::fs::read_to_string(&transcript_path)
            .with_context(|| format!("Failed to read transcript at {}", transcript_path))?;

        let result = summarize_linear(
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

        // TODO: Add guard to detect malformed or incomplete LLM output
        stream.summary_md = Some(result);
    }

    db.bulk_insert_streams(streams).await?;

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

    // TODO: Add websearch capability
    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(include_str!("../prompts/system_0.txt").into()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(include_str!("../prompts/user_0.txt").into()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(user_prompt),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let mut attempt = 0;
    let max_attempts = 5;

    loop {
        tracing::info!(attempt, "Summarizing chunk");

        match openai.chat().create(parameters.clone()).await {
            Ok(response) => break chat_completions_text_from_response(response),
            Err(err) => {
                attempt += 1;
                let err_str = format!("{:?}", err);
                // In case of a 429 response, OpenAI will recommend a wait time
                // we try to use the recommended wait time here, otherwise the fallback is used
                let wait_ms = extract_wait_time_ms_from_error(&err_str).unwrap_or_else(|| {
                    let fallback = 2_u64.pow(attempt) * 1000;
                    tracing::warn!(attempt, "No wait time found, using fallback {}ms", fallback);
                    fallback
                });

                if attempt >= max_attempts {
                    tracing::error!(error = ?err, "Failed after {} attempts", attempt);
                    return Err(err.into());
                }

                tracing::warn!(
                    error = ?err,
                    attempt,
                    wait_ms,
                    "Rate limit hit or other error. Retrying after {}ms (attempt {}/{})",
                    wait_ms,
                    attempt,
                    max_attempts
                );

                tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;
            }
        }
    }
}

#[tracing::instrument(skip(openai, summaries))]
async fn combine_summaries(
    summaries: Vec<String>,
    openai: &OpenAiClient,
) -> anyhow::Result<String> {
    let summaries = summaries.join("\n");

    let prompt = format!(
        r#"
Given the following markdown summaries of individual transcript chunks from a Kenyan Parliament stream, combine them into one single, coherent markdown summary. Follow the same formatting instructions you were given previously, including front matter, summary paragraph, and all structured sections.

Summaries:
{}
        "#,
        summaries
    );

    // TODO: Add websearch capability
    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(include_str!("../prompts/system_0.txt").into()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(include_str!("../prompts/user_0.txt").into()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(prompt),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let mut attempt = 0;
    let max_attempts = 5;

    loop {
        tracing::info!(attempt, "Combining summaries");

        match openai.chat().create(parameters.clone()).await {
            Ok(response) => break chat_completions_text_from_response(response),
            Err(err) => {
                attempt += 1;

                let err_str = format!("{:?}", err);
                let wait_ms = extract_wait_time_ms_from_error(&err_str).unwrap_or_else(|| {
                    let fallback = 2_u64.pow(attempt) * 1000;
                    tracing::warn!(attempt, "No wait time found, using fallback {}ms", fallback);
                    fallback
                });

                if attempt >= max_attempts {
                    tracing::error!(error = ?err, "combine_summaries failed after {} attempts", attempt);
                    return Err(err.into());
                }

                tracing::warn!(
                    error = ?err,
                    wait_ms,
                    attempt,
                    "Retrying combine_summaries after {}ms (attempt {}/{})",
                    wait_ms,
                    attempt,
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
                    c => bail!("Unexpected chat message content: {:?}", c),
                }
            } else {
                bail!("Unexpected absence of chat message content");
            }
        }
        c => bail!("Unexpected chat message response: {:?}", c),
    };

    Ok(response)
}

/// Filter and sort streams that already exist in the database based on their `video_id`.
pub async fn sort_and_filter_existing_streams(
    db: &DataStore,
    streams: &mut [Stream],
) -> Vec<Stream> {
    // Initially filter out streams already processed
    // from raw stream data from yt document
    let mut filtered = Vec::new();

    for stream in streams.iter() {
        match db.stream_exists(&stream.video_id).await {
            Ok(false) => filtered.push(stream.clone()),
            Ok(true) => {} // skip existing
            Err(e) => {
                tracing::warn!(video_id = %stream.video_id, error = %e, "Failed to check stream existence");
            }
        }
    }

    // sort filtered streams by timestamp ascending (older streams first)
    // newer streams will “wait their turn” behind older unprocessed ones.
    filtered.sort_by(|a, b| {
        a.timestamp_from_time_ago()
            .cmp(&b.timestamp_from_time_ago())
    });

    // return the first 3 streams to avoid overloading system
    filtered.into_iter().take(MAX_STREAMS_TO_PROCESS).collect()
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

/// Deletes the /audio directory inside the working directory.
/// Logs a warning if the cleanup fails but does not panic.
pub fn cleanup_audio_dir() {
    let audio_path = PathBuf::from(format!("{WORKDIR}/audio"));

    if audio_path.exists() {
        if let Err(e) = remove_dir_all(&audio_path) {
            tracing::warn!(error = ?e, path = ?audio_path, "Failed to clean up audio directory");
        } else {
            tracing::info!(path = ?audio_path, "Cleaned up audio directory");
        }
    }
}
