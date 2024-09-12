#[derive(thiserror::Error, Debug)]
pub enum YtScrapeError {
    #[error("{0}")]
    ParseError(&'static str),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}
