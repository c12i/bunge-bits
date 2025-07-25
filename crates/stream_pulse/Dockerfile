# base image
FROM rust:1.88 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# copy only the manifests to generate "workspace-aware" recipe 
COPY Cargo.toml Cargo.lock ./
COPY crates/stream_pulse/Cargo.toml crates/stream_pulse/Cargo.toml
COPY crates/stream_datastore/Cargo.toml crates/stream_datastore/Cargo.toml
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
  curl \
  xz-utils \
  python3 \
  && rm -rf /var/lib/apt/lists/*
# install ffmpeg
RUN curl -L https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz \
  | tar xJ --strip-components=1 -C /usr/local/bin --wildcards '*/ffmpeg' '*/ffprobe'
COPY --from=builder /app/target/release/stream-pulse-cron /usr/local/bin/stream-pulse-cron
WORKDIR /app
ENV OPENAI_API_KEY= \
  DATABASE_URL= \
  SENTRY_DSN= \
  MAX_STREAMS_TO_PROCESS= \
  YTDLP_COOKIES_PATH= \
  CRON_SCHEDULE= 

EXPOSE 8001

CMD ["stream-pulse-cron"]
