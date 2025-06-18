use clap::{Parser, Subcommand};
use futures::FutureExt;
use stream_pulse::{fetch_and_process_streams, tracing::init_tracing_subscriber};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct StreamPulseCli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Fetch and process new livestreams
    ///
    /// This will call the summarization pipeline and store results in the database.
    FetchAndProcessStreams {
        /// Maximum number of new streams to handle in this run (default: 3)
        #[arg(long, default_value_t = 3)]
        max_streams: usize,
    },

    /// Generate navigation timestamps for a stream
    ///
    /// Provide a YouTube video ID to generate highlight timestamps from its transcript.
    GenerateStreamTimestamps {
        /// The YouTube video ID of the stream (e.g. p40gmygQL2c)
        video_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    init_tracing_subscriber()?;

    let cli = StreamPulseCli::parse();

    match cli.command {
        Commands::FetchAndProcessStreams { max_streams } => {
            let result = std::panic::AssertUnwindSafe(fetch_and_process_streams(max_streams))
                .catch_unwind()
                .await;

            if let Err(err) = result {
                tracing::error!(error = ?err, "Job panicked");
            }
        }

        Commands::GenerateStreamTimestamps { video_id } => {
            todo!(
                "Implement generate_stream_timestamps for video_id = {}",
                video_id
            );
        }
    }

    Ok(())
}
