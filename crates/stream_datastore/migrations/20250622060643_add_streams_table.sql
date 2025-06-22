-- Add migration script here
CREATE TABLE IF NOT EXISTS streams (
    video_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    view_count TEXT NOT NULL,
    stream_timestamp TIMESTAMPTZ NOT NULL,
    duration TEXT NOT NULL,
    summary_md TEXT,
    timestamp_md TEXT,
    is_published BOOLEAN NOT NULL DEFAULT FALSE
)
