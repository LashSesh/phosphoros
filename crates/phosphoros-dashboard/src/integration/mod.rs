//! Integration module for PHOSPHOROS core functionality
//!
//! This module provides integration between the dashboard and
//! the core PHOSPHOROS components.

pub mod wallet;
pub mod resonance;
pub mod analysis;
pub mod infogenetic;

pub use wallet::WalletIntegration;
pub use resonance::ResonanceIntegration;
pub use analysis::AnalysisIntegration;
pub use infogenetic::{InfoGeneticIntegration, InfoGeneticResult, CellState, InfoGeneticError};
