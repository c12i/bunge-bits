#[derive(thiserror::Error, Debug)]
pub enum YtScrapeError {
    #[error("ParseError: {0}")]
    ParseError(&'static str),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
    #[error("UniqueConstraintViolation: {0}")]
    UniqueConstraintViolation(#[source] anyhow::Error),
}
