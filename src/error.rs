use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Http client error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Hunter not found: {0}")]
    HunterNotFound(String),
    #[error("Processor not found: {0}")]
    ProcessorNotFound(String),
    #[error("Processor error: {0}")]
    ProcessorError(String),
    // #[error("unknown error")]
    // Unknown,
}
