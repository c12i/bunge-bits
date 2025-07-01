# stream_pulse

The core engine powering `bunge-bits`. This crate handles all logic related to the summarization pipeline.

## Development setup

To run the `stream_pulse_cron` binary, set the following environment variables:

```bash
OPENAI_API_KEY="<your_openai_api_key>"
DATABASE_URL="<your_postgres_database_url>"
YTDLP_COOKIES_PATH="<path to your cookies.txt file>" # required in order to authenticate to yt, especially in a cloud env
SENTRY_DSN="<optional_sentry_dsn>" # can be omitted for local development
MAX_STREAMS_TO_PROCESS=3 # optional config of the maxim number of streams that can be processed in a given run
CRON_SCHEDULE="<cron_expression>" # optional cron schedule to run the pipeline. Defaults to "0 0 */4 * * *" (every 4 hours)
```

Please read [this guide](../ytdlp_bindings/README.md#using-cookiestxt-for-authenticated-youtube-downloads) on how to setup your `cookies.txt` file.

You can define these variables directly in your shell or in a `.env` file placed at the root of the Cargo workspace.

## Running the CLI

For quick local testing and development, run the `dev-cli` example:

```bash
cargo run --example dev-cli -- fetch-and-process-streams --max-streams 2
```

The `--max-streams` flag is optional (default: 3). This CLI is intended for local development, prototyping, or ad-hoc tasks. It is not used in production.

## Running the Production Cron Workflow

To run the actual scheduled production workflow:

```bash
cargo run --package stream-pulse-cron
```

This binary is designed to run as a background job (e.g. via cron or systemd timer) and handles automated stream fetching and summarization.

## Running with Docker

To run `stream-pulse-cron` reliably with environment configuration and persistent file storage, use the following command:

```bash
docker run -d \
  --name stream-pulse-cron \
  --restart unless-stopped \
  -e OPENAI_API_KEY="..." \
  -e DATABASE_URL="..." \
  -e SENTRY_DSN="..." \
  -e CRON_SCHEDULE="..." \
  -e MAX_STREAMS_TO_PROCESS=2 \
  -e YTDLP_COOKIES_PATH=/app/cookies.txt \
  -v /path/to/cookies.txt:/app/cookies.txt \
  -v /var/tmp/bunge-bits:/var/tmp/bunge-bits \
  ghcr.io/c12i/bunge-bits/stream-pulse-cron:latest
```

### Explanation of Volume Mounts

| Mount                                        | Purpose                                                                                                                                                                                                                                                                                               |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-v /path/to/cookies.txt:/app/cookies.txt`   | Provides the container with a valid YouTube `cookies.txt` file. This allows `yt-dlp` to access age-restricted or authenticated videos using a pre-exported session from a browser (see [yt-dlp cookies.txt setup](../ytdlp_bindings/README.md#using-cookiestxt-for-authenticated-youtube-downloads)). |
| `-v /var/tmp/bunge-bits:/var/tmp/bunge-bits` | Ensures that downloaded audio and intermediate files are persisted between runs. Without this, Docker would discard the files after each container restart.                                                                                                                                           |
