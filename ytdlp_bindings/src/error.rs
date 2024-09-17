#[derive(thiserror::Error, Debug)]
pub enum YtDlpError {
    #[error("Failed to execute yt-dlp: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("yt-dlp exited with non-zero status: {0}")]
    NonZeroExit(i32),
    #[error("Invalid output path: {0}")]
    InvalidOutputPath(String),
    #[error("Failed to locate yt-dlp binary")]
    BinaryNotFound,
    #[error("Failed to read VTT file: {0}")]
    VttReadError(#[source] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Invalid input path: {0}")]
    InvalidInputPath(String),
}
