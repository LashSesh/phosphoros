//! Error types for cryptogenetik-core

use thiserror::Error;

/// Errors that can occur in cryptogenetik-core
#[derive(Error, Debug)]
pub enum Error {
    /// Exploration failed
    #[error("Exploration failed: {0}")]
    ExplorationFailed(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Operator failed
    #[error("Operator failed: {0}")]
    OperatorFailed(String),

    /// Resonance engine error
    #[error("Resonance engine error: {0}")]
    ResonanceEngineError(String),

    /// BIP39 integration error (when feature is enabled)
    #[cfg(feature = "bip39-integration")]
    #[error("BIP39 error: {0}")]
    Bip39Error(#[from] phosphoros_bip39::Error),
}

/// Result type for cryptogenetik-core
pub type Result<T> = std::result::Result<T, Error>;
