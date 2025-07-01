//! # Cron Job Workflow for Parliament of Kenya Channel Streams
//!
//! A cron job that runs every 4 hours to fetch and process archived streams from the Parliament of Kenya's YouTube channel.
//!
//! Potential panics from the `fetch_and_process_streams` entry-point are handled gracefully

use futures::FutureExt;
use stream_pulse::{fetch_and_process_streams, tracing::init_tracing_subscriber};
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").unwrap_or_else(|_| String::new()),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some("production".into()),
            ..Default::default()
        },
    ));

    init_tracing_subscriber()?;

    let mut scheduler = JobScheduler::new().await?;
    let cron_schedule =
        std::env::var("CRON_SCHEDULE").unwrap_or_else(|_| "0 0 */4 * * *".to_string());

    tracing::info!(%cron_schedule);

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Africa::Nairobi)
        .with_cron_job_type()
        .with_schedule(&cron_schedule)?
        .with_run_async(Box::new(|uuid, _| {
            Box::pin(async move {
                // Maximum streams that can be processed in a run
                let max_streams = std::env::var("MAX_STREAMS_TO_PROCESS")
                    .ok()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(3);

                tracing::info!(job_id = %uuid, max_streams, "Running cron job: {}", uuid);

                let result = std::panic::AssertUnwindSafe(fetch_and_process_streams(max_streams))
                    .catch_unwind()
                    .await;

                if let Err(err) = result {
                    tracing::error!(error = ?err, "Job panicked");
                }
            })
        }))
        .build()?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down scheduler...");
    scheduler.shutdown().await?;

    Ok(())
}
