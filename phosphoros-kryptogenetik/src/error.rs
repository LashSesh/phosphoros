//! Error types for phosphoros-kryptogenetik

use thiserror::Error;

/// Error type for phosphoros-kryptogenetik operations
#[derive(Error, Debug)]
pub enum Error {
    /// Infogenom not found
    #[error("Infogenom '{0}' not found")]
    InfogenomNotFound(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Mathematical error (overflow, underflow, etc.)
    #[error("Mathematical error: {0}")]
    MathError(String),

    /// Feature not enabled
    #[error("Feature '{0}' is not enabled")]
    FeatureNotEnabled(String),
}

/// Result type for phosphoros-kryptogenetik operations
pub type Result<T> = std::result::Result<T, Error>;
