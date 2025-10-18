//! Error types for phosphoros-bip39

use thiserror::Error;

/// Errors that can occur in phosphoros-bip39
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid mnemonic phrase
    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    /// Invalid word count
    #[error("Invalid word count: {0} (must be 12, 15, 18, 21, or 24)")]
    InvalidWordCount(usize),

    /// Mnemonic generation failed
    #[error("Mnemonic generation failed: {0}")]
    MnemonicGeneration(String),

    /// Invalid derivation path
    #[error("Invalid derivation path: {0}")]
    InvalidDerivationPath(String),

    /// Derivation failed
    #[error("Derivation failed: {0}")]
    DerivationFailed(String),

    /// Unsupported blockchain
    #[error("Unsupported blockchain: {0}")]
    UnsupportedBlockchain(String),

    /// Address generation failed
    #[error("Address generation failed: {0}")]
    AddressGenerationFailed(String),
}

/// Result type for phosphoros-bip39
pub type Result<T> = std::result::Result<T, Error>;
