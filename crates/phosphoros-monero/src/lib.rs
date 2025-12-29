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
//!
//! ## Quantum Enhancement
//!
//! Enable the `quantum` feature for QAOA-based ring signature analysis:
//! ```toml
//! phosphoros-monero = { features = ["quantum"] }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod rpc;
pub mod ring_analysis;
pub mod heuristics;
pub mod quantum;
pub mod graph_analysis;

mod error;

pub use error::{Error, Result};
pub use rpc::{MoneroRpcClient, MoneroRpcConfig};
pub use ring_analysis::{RingAnalyzer, RingAnalysisResult, RingMemberInfo};
pub use heuristics::{TemporalHeuristic, DecoySelectionHeuristic, HeuristicResult};
pub use quantum::{QuantumRingAnalyzer, QuantumEnhancedResult};
pub use graph_analysis::{TransactionGraph, TxNode, TxEdge, NodeType, EdgeType, AnomalyReport};

// Re-export key types from phosphoros-bip39
pub use phosphoros_bip39::monero::{
    KeyImage, RingMember as MoneroRingMember, TxOutputRef, MoneroNetwork,
};
