//! Error types for Monero forensics

use thiserror::Error;

/// Monero forensics error type
#[derive(Error, Debug)]
pub enum Error {
    /// RPC connection error
    #[error("RPC connection error: {0}")]
    RpcConnection(String),

    /// RPC request error
    #[error("RPC request failed: {0}")]
    RpcRequest(String),

    /// Invalid response from daemon
    #[error("Invalid RPC response: {0}")]
    InvalidResponse(String),

    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    /// Block not found
    #[error("Block not found: {0}")]
    BlockNotFound(String),

    /// Invalid key image format
    #[error("Invalid key image: {0}")]
    InvalidKeyImage(String),

    /// Analysis error
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

/// Result type for Monero forensics
pub type Result<T> = std::result::Result<T, Error>;
