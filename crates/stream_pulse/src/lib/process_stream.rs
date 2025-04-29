use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use anyhow::{anyhow, Context};
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
use ytdlp_bindings::{AudioProcessor, YtDlp, YtDlpError};

use crate::{extract_json_from_script, parse_streams};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

static YTDLP: LazyLock<Result<YtDlp, YtDlpError>> = LazyLock::new(YtDlp::new);

static OPENAI: LazyLock<OpenAiClient> = LazyLock::new(openai_dive::v1::api::Client::new_from_env);

//  Parliament of Kenya Channel Stream URL
const YOUTUBE_STREAM_URL: &str = "https://www.youtube.com/@ParliamentofKenyaChannel/streams";

#[tracing::instrument]
pub async fn fetch_and_process_streams() -> anyhow::Result<()> {
    let client = &CLIENT;
    let ytdlp = YTDLP.as_ref()?;
    let openai = &OPENAI;

    let db = DataStore::new("bunge-bits-store.db").context("Failed to connect to database")?;

    let response = client.get(YOUTUBE_STREAM_URL).send().await?.text().await?;

    match extract_json_from_script(&response) {
        Ok(json) => {
            let mut streams = parse_streams(&json)?;

            // This is where initially downloaded audio by yt-dlp is saved
            let audio_download_path = PathBuf::from("/var/tmp/bunge-bits/audio");
            // This is were ausosubs are stored
            let autosub_path = PathBuf::from("/var/tmp/bunge-bits/autosub");

            // sort by upload date
            streams.sort_by(|a, b| {
                b.timestamp_from_time_ago()
                    .cmp(&a.timestamp_from_time_ago())
            });

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
                let chunked_audio_path =
                    PathBuf::from(format!("/var/tmp/bunge-bits/audio/{}", stream.video_id));

                // Download autosub
                ytdlp.download_auto_sub(&youtube_stream, &vtt_output_path)?;

                // Download audio file with yt-dlp
                ytdlp.download_audio(&youtube_stream, &audio_out_path)?;

                // set mp3 extension
                audio_out_path.set_extension("mp3");

                // create nested `/stream.id/` dir
                create_dir_all(&chunked_audio_path).expect("Failed to create directories");

                // Split downloaded audio to chunks
                ytdlp.split_audio_to_chunks(
                    audio_out_path,
                    1800,
                    chunked_audio_path.join(format!("{}_%03d.mp3", stream.video_id)),
                )?;

                Ok::<_, anyhow::Error>(())
            })?;

            for stream in streams.iter() {
                let audio_chunks_path =
                    PathBuf::from(format!("/var/tmp/bunge-bits/audio/{}", stream.video_id));
                let mut transcript_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("/var/tmp/bunge-bits/{}.txt", stream.video_id))?;

                for file in std::fs::read_dir(&audio_chunks_path).context("Failed to read dir")? {
                    let file = file.context("Failed to get file")?;
                    let params = AudioTranscriptionParametersBuilder::default()
                        .file(FileUpload::File(format!("{:?}", file.path())))
                        .model(WhisperEngine::Whisper1.to_string())
                        .response_format(AudioOutputFormat::Srt)
                        .build()?;
                    let transcription = openai.audio().create_transcription(params).await?;
                    write!(transcript_file, "{}", transcription)?;
                    writeln!(transcript_file, "----END_OF_CHUNK----")?;
                }
            }

            for stream in streams {
                let transcript = std::fs::read_to_string(format!(
                    "/var/tmp/bunge-bits/{}.txt",
                    stream.video_id
                ))?;
                summarize_linear(
                    &transcript,
                    "----END_OF_CHUNK----",
                    |chunk, ctx| Box::pin(async move { summarize_chunk(chunk, ctx, openai).await }),
                    |summaries| Box::pin(async move { combine_summaries(summaries, openai).await }),
                )
                .await?;
            }
        }
        Err(e) => {
            eprintln!("Error parsing streams: {}", e);
        }
    }

    Ok(())
}

#[tracing::instrument]
async fn summarize_chunk(
    chunk: String,
    context: Option<Arc<String>>,
    openai: &OpenAiClient,
) -> Result<String, anyhow::Error> {
    let user_prompt = context.map(|ctx| {
        format!(
            r#"
Context:
{}

VTT Chunk:
{}

Based on the transcript chunk and the provided context, please summarize it based on the instructions you received previously.
        "#,
            ctx, chunk
        )
    }).unwrap_or_else(|| {
        format!(
            r#"
VTT Chunk:
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
    let response = openai.chat().create(parameters).await?;

    chat_completions_text_from_response(response)
}

#[tracing::instrument]
async fn combine_summaries(
    summaries: Vec<String>,
    openai: &OpenAiClient,
) -> Result<String, anyhow::Error> {
    let summaries = summaries.join("\n");

    let parameters = ChatCompletionParametersBuilder::default()
        .model(Gpt4Engine::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    format!(
                        r#"
Given the following summaries of a video live stream chunks. Combine them into a single coherent summary:

Summaries:
{}
                        "#, 
                            summaries
                    )
                ),
                name: None,
            },
        ])
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let response = openai.chat().create(parameters).await?;

    chat_completions_text_from_response(response)
}

#[tracing::instrument]
pub fn chat_completions_text_from_response(
    response: ChatCompletionResponse,
) -> Result<String, anyhow::Error> {
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
                    c => return Err(anyhow!("Unexpcted chat message content: {:?}", c)),
                }
            } else {
                return Err(anyhow!("Unexpected absence of chage message content"));
            }
        }
        c => return Err(anyhow!("Unexpcted chat message response: {:?}", c)),
    };

    Ok(response)
}
