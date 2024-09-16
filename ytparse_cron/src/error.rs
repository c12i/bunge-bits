#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ParseError: {0}")]
    ParseError(&'static str),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}
