# stream_pulse

The core engine powering `bunge-bits`. This crate handles all logic related to the summarization pipeline.

## Development setup

To run the `stream_pulse_cron` binary, set the following environment variables:

```bash
OPENAI_API_KEY="<your_openai_api_key>"
DATABASE_URL="<your_postgres_database_url>"
YTDLP_COOKIES_PATH="<path to your cookies.txt file>" # required in order to authenticate to yt, especially in a cloud env, this is required
SENTRY_DSN="<optional_sentry_dsn>" # can be omitted for local development
MAX_STREAMS_TO_PROCESS=3 # optional config of the maxim number of streams that can be processed in a given run
```

Please read [this guide](../ytdlp_bindings/README.md#using-cookiestxt-for-authenticated-youtube-downloads) on how to setup your `cookies.txt` file.

You can define these variables directly in your shell or in a `.env` file placed at the root of the Cargo workspace.

## Running the CLI

For quick local testing and development, run the `dev-cli` example:

```bash
cargo run --example dev-cli -- fetch-and-process-streams --max-streams 2
```

The `--max-streams` flag is optional (default: 3). This CLI is intended for local development, prototyping, or ad-hoc tasks. It is not used in production.

# Running the Production Cron Workflow

To run the actual scheduled production workflow:

```bash
cargo run --package stream-pulse-cron
```

This binary is designed to run as a background job (e.g. via cron or systemd timer) and handles automated stream fetching and summarization.
