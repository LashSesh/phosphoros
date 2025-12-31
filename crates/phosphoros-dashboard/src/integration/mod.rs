//! Integration module for PHOSPHOROS core functionality
//!
//! This module provides integration between the dashboard and
//! the core PHOSPHOROS components including:
//! - Wallet integration for seed/address management
//! - Resonance analysis (ψ, ρ, ω)
//! - InfoGenetic DNA analysis and visualization
//! - 5D Spiral topology from Klemm's axiomatic framework
//! - MEF Ledger for persistent hash-chained storage

pub mod wallet;
pub mod resonance;
pub mod analysis;
pub mod infogenetic;
pub mod spiral_topology;
pub mod mef_ledger;

pub use wallet::WalletIntegration;
pub use resonance::ResonanceIntegration;
pub use analysis::AnalysisIntegration;
pub use infogenetic::{InfoGeneticIntegration, InfoGeneticResult, CellState, InfoGeneticError};
pub use spiral_topology::{
    State5D, SpiralPath, SpiralParams, SpiralType,
    ResonanceFields, SpiralDynamics, SpiralCoordinates,
    segment_weight, spiral_action,
};
pub use mef_ledger::{
    InfogeneticLedger, InfogeneticBlock, CompactInfogeneticEntry,
    CompactSignature, CompactCellState, StrandStatistics,
    LedgerError, LedgerStatistics, ProofOfResonance,
};
