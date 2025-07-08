use std::sync::Mutex;

use chrono::DateTime;
use chrono_tz::Tz;

pub mod cron;
pub mod server;

/// Shared application state for coordinating between the cron scheduler and the HTTP server.
#[derive(Debug)]
pub struct AppState {
    pub next_tick_for_job: Mutex<Option<DateTime<Tz>>>,
}
