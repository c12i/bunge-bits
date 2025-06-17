# stream_pulse

The core engine powering `bunge-bits`. This crate handles all logic related to the summarization pipeline.

## Development setup

To run the `stream_pulse_cron` binary, set the following environment variables:

```bash
OPENAI_API_KEY="<your_openai_api_key>"
DATABASE_URL="<your_postgres_database_url>"
SENTRY_DSN="<optional_sentry_dsn>" # can be omitted for local development
```

You can define these variables directly in your shell or in a `.env` file placed at the root of the Cargo workspace.

Running the binary:

```bash
cargo run --package stream_pulse_cron
```
