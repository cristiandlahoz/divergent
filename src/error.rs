use crate::vcs::VcsError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DivergentError {
    #[error("{0}")]
    Vcs(#[from] VcsError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    InvalidInput(String),
}
