//! # Cron Job Workflow for Parliament of Kenya Channel Streams
//!
//! A cron job that runs every 12 hours to fetch and process
//! archived streams from the Parliament of Kenya YouTube channel.

use futures::FutureExt;
use stream_pulse::{fetch_and_process_streams, tracing::init_tracing_subscriber};
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

// Should run every ~12~ n hours
const CRON_EXPR: &str = "0 0 */2 * * *";

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN env var not set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some("production".into()),
            ..Default::default()
        },
    ));

    init_tracing_subscriber()?;

    let mut scheduler = JobScheduler::new().await?;

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Africa::Nairobi)
        .with_cron_job_type()
        .with_schedule(CRON_EXPR)?
        .with_run_async(Box::new(|uuid, _| {
            Box::pin(async move {
                tracing::info!(job_id = %uuid, "Running cron job: {}", uuid);
                let result = std::panic::AssertUnwindSafe(fetch_and_process_streams())
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
