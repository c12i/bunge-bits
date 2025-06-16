use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

const DEFAULT_LOG_LEVEL: &str = "INFO";
const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn init_tracing_subscriber() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL));

    let formatting_layer = BunyanFormattingLayer::new(CRATE_NAME.to_string(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(sentry_tracing::layer())
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
}
