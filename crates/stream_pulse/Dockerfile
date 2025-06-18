FROM rust:1.87 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# --- First stage: gather dependency graph ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Second stage: cache dependencies only ---
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy actual source code
COPY . .
RUN cargo build --release --package stream-pulse-cron

# --- Final stage: runtime image ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/stream-pulse-cron /usr/local/bin/stream-pulse-cron
WORKDIR /app
ENV OPENAI_API_KEY= \
  DATABASE_URL= \
  SENTRY_DSN=
CMD ["stream-pulse-cron"]
