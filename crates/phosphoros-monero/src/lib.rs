//! # PHOSPHOROS Monero Forensics
//!
//! Specialized Monero blockchain forensics and ring signature analysis.
//!
//! ## Features
//!
//! - **RPC Client**: Connect to Monero daemon for blockchain data
//! - **Ring Analysis**: Heuristics for identifying real transaction inputs
//! - **Key Image Tracking**: Detect spent outputs
//! - **Temporal Analysis**: Timing-based ring member identification
//!
//! ## IRS Compliance
//!
//! This module implements methodologies aligned with cryptocurrency tracing
//! requirements for law enforcement and regulatory agencies.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod rpc;
pub mod ring_analysis;
pub mod heuristics;

mod error;

pub use error::{Error, Result};
pub use rpc::{MoneroRpcClient, MoneroRpcConfig};
pub use ring_analysis::{RingAnalyzer, RingAnalysisResult};
pub use heuristics::{TemporalHeuristic, DecoySelectionHeuristic, HeuristicResult};

// Re-export key types from phosphoros-bip39
pub use phosphoros_bip39::monero::{
    KeyImage, RingMember, TxOutputRef, MoneroNetwork,
};
