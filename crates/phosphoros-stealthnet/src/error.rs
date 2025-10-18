//! Error types for the stealthnet subsystem.

use thiserror::Error;

/// Result type for stealthnet operations.
pub type Result<T> = std::result::Result<T, StealthError>;

/// Error types for stealth networking operations.
#[derive(Debug, Error)]
pub enum StealthError {
    /// Invalid configuration
    #[error("Invalid stealth configuration: {0}")]
    InvalidConfig(String),

    /// Payload transformation failed
    #[error("Payload transformation failed: {0}")]
    TransformationFailed(String),

    /// Steganography encoding failed
    #[error("Steganography encoding failed: {0}")]
    EncodingFailed(String),

    /// Proxy configuration error
    #[error("Proxy configuration error: {0}")]
    ProxyError(String),

    /// Network operation failed
    #[cfg(feature = "network")]
    #[error("Network operation failed: {0}")]
    NetworkError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<serde_json::Error> for StealthError {
    fn from(err: serde_json::Error) -> Self {
        StealthError::SerializationError(err.to_string())
    }
}
