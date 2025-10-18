//! Error types for the Satellite subsystem.

use thiserror::Error;

/// Result type for Satellite operations.
pub type Result<T> = std::result::Result<T, SatelliteError>;

/// Errors that can occur in the Satellite subsystem.
#[derive(Debug, Error)]
pub enum SatelliteError {
    /// Invalid input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Snapshot not found
    #[error("Snapshot {0} not found")]
    SnapshotNotFound(String),

    /// Analysis error
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// YAML serialization error
    #[error("YAML error: {0}")]
    YamlError(String),
}
