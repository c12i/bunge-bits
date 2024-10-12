#[derive(thiserror::Error, Debug)]
pub enum YtDlpError {
    #[error("Failed to execute yt-dlp: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("{command} exited with status '{status}' {output}")]
    NonZeroExit {
        command: String,
        status: i32,
        output: String,
    },
    #[error("Invalid output path: {0}")]
    InvalidOutputPath(String),
    #[error("Failed to locate {0} binary")]
    BinaryNotFound(String),
    #[error("Failed to read VTT file: {0}")]
    VttReadError(#[source] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Invalid input path: {0}")]
    InvalidInputPath(String),
}
