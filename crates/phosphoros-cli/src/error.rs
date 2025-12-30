use thiserror::Error;

pub type Result<T> = std::result::Result<T, OuroborosError>;

#[derive(Debug, Error)]
pub enum OuroborosError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<anyhow::Error> for OuroborosError {
    fn from(value: anyhow::Error) -> Self {
        OuroborosError::Crypto(value.to_string())
    }
}
