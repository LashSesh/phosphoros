//! Quantum-Enhanced Forensic Analysis
//!
//! Provides quantum algorithms for Monero forensic analysis:
//! - Ring signature decomposition using QAOA
//! - Link analysis using quantum walks

mod ring_qaoa;

pub use ring_qaoa::{RingSignatureQAOA, QuantumRingAnalysis};
