//! Seraphic Calibration System (SCS) Bridge
//!
//! Integrates the QSO Seraphic Calibration System with PHOSPHOROS
//! for automatic hyperparameter tuning and resonance optimization.

mod scs_bridge;
mod performance;

pub use scs_bridge::SCSBridge;
pub use performance::{PerformanceTriplet, CalibrationState, CalibrationSuggestion};

/// SCS Calibration regimes
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CalibrationRegime {
    /// Standard conservative optimization
    Standard,
    /// Aggressive exploration for escaping local optima
    Aggressive,
    /// Wide exploration of parameter space
    Exploring,
    /// Maintain current configuration (homeostasis)
    Homeostasis,
}

impl Default for CalibrationRegime {
    fn default() -> Self {
        Self::Standard
    }
}
