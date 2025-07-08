use std::sync::{Arc, Mutex};

use stream_pulse::{start_cron, start_server, tracing::init_tracing_subscriber, AppState};

/// Every 4 hours
const DEFAULT_CRON_SCHEDULE: &str = "0 0 */4 * * *";

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

    let cron_schedule =
        std::env::var("CRON_SCHEDULE").unwrap_or_else(|_| DEFAULT_CRON_SCHEDULE.to_string());

    let app_state = Arc::new(AppState {
        next_tick_for_job: Mutex::new(None),
    });

    tokio::select! {
        _ = start_cron(&cron_schedule, app_state.clone()) => {}
        _ = start_server(app_state.clone()) => {}
    }

    Ok(())
}
