//! # Status Server for Bunge Bits Cron System
//!
//! This Axum-based HTTP server exposes a simple health check and status endpoint to monitor the cron job scheduler.
//!
//! ## Endpoints
//!
//! - `GET /status`: Returns the next scheduled cron job tick as an ISO 8601 timestamp
//!   and a `healthy` flag indicating if the service is up.
//!
//! Example response:
//!
//! ```json
//! {
//!   "healthy": true,
//!   "next_tick": "2025-07-03T18:00:00+03:00"
//! }
//! ```
//!
//! The `next_tick` value is updated every few seconds based on the scheduler state.

use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};
use chrono::SecondsFormat;
use serde::Serialize;
use tokio::net::TcpListener;

use super::AppState;

pub async fn start_server(app_state: Arc<AppState>) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/status", get(status))
        .with_state(app_state);

    let addr = "127.0.0.1:8001";
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("HTTP server started at http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct StatusResponse {
    healthy: bool,
    next_tick: Option<String>,
}

#[tracing::instrument(skip(app_state))]
async fn status(State(app_state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let next = app_state
        .next_tick_for_job
        .lock()
        .ok()
        .and_then(|guard| guard.clone());

    Json(StatusResponse {
        healthy: true,
        next_tick: next.map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true)),
    })
}
