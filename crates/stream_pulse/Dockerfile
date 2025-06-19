# base image
FROM rust:1.87 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# copy only the manifests to generate "workspace-aware" recipe 
COPY Cargo.toml Cargo.lock ./
COPY crates/stream_pulse/Cargo.toml crates/stream_pulse/Cargo.toml
COPY crates/stream_datastore/Cargo.toml crates/stream_datastore/Cargo.toml
COPY crates/stream_digest/Cargo.toml crates/stream_digest/Cargo.toml
COPY crates/ytdlp_bindings/Cargo.toml crates/ytdlp_bindings/Cargo.toml
# generate reproducible build plan
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# copy the full workspace for final build
COPY . .
RUN cargo build --release --bin stream-pulse-cron

# runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
  ca-certificates \
  ffmpeg \
  python3 \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/stream-pulse-cron /usr/local/bin/stream-pulse-cron
# copy vendored yt-dlp binary from build output
COPY --from=builder /app/target/release/build/ytdlp_bindings-*/out/yt-dlp /usr/local/bin/yt-dlp
WORKDIR /app
ENV OPENAI_API_KEY= \
  DATABASE_URL= \
  SENTRY_DSN= \
  MAX_STREAMS_TO_PROCESS= \
  YTDLP_COOKIES_PATH=
CMD ["stream-pulse-cron"]
