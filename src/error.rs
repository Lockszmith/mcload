//! Shared error type.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("config error: {0}")]
    Config(String),

    #[error("metadata error: {0}")]
    Metadata(String),

    #[error("queue error: {0}")]
    Queue(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
