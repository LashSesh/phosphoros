//! Performance Triplet and Calibration Types
//!
//! Defines the (ψ, ρ, ω) performance metrics used by SCS,
//! directly mapping to PHOSPHOROS SpectralSignature.

use serde::{Deserialize, Serialize};
use super::CalibrationRegime;

/// Performance triplet Φ(c) = (ψ, ρ, ω)
///
/// This is the core metric used by SCS for calibration:
/// - ψ (psi): Quality/coherence metric [0, 1]
/// - ρ (rho): Stability/density metric [0, 1]
/// - ω (omega): Efficiency/frequency metric [0, 1]
///
/// Combined score: D = ψ · ρ · ω
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerformanceTriplet {
    /// Quality/coherence (how good is the result?)
    pub psi: f64,

    /// Stability/density (how robust is it?)
    pub rho: f64,

    /// Efficiency/frequency (how fast/cheap is it?)
    pub omega: f64,
}

impl PerformanceTriplet {
    /// Create a new performance triplet
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
        }
    }

    /// Calculate combined resonance score D = ψ · ρ · ω
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Check if this triplet dominates another (Pareto dominance)
    pub fn dominates(&self, other: &PerformanceTriplet) -> bool {
        self.psi >= other.psi && self.rho >= other.rho && self.omega >= other.omega
            && (self.psi > other.psi || self.rho > other.rho || self.omega > other.omega)
    }

    /// Calculate Euclidean distance to another triplet
    pub fn distance(&self, other: &PerformanceTriplet) -> f64 {
        ((self.psi - other.psi).powi(2)
            + (self.rho - other.rho).powi(2)
            + (self.omega - other.omega).powi(2))
        .sqrt()
    }

    /// Create from PHOSPHOROS SpectralSignature values
    pub fn from_spectral(psi: f64, rho: f64, omega: f64) -> Self {
        Self::new(psi, rho, omega)
    }
}

impl Default for PerformanceTriplet {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

/// Current calibration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationState {
    /// Current performance triplet
    pub current: PerformanceTriplet,

    /// Best observed performance
    pub best: PerformanceTriplet,

    /// Current calibration regime
    pub regime: CalibrationRegime,

    /// Number of calibration steps taken
    pub step_count: usize,

    /// Mandorla field values (16-dimensional resonance accumulator)
    pub mandorla_field: Vec<f64>,

    /// History of recent performance values
    pub history: Vec<PerformanceTriplet>,

    /// Current configuration parameters
    pub config: CalibrationConfig,
}

impl CalibrationState {
    /// Create a new calibration state
    pub fn new() -> Self {
        Self {
            current: PerformanceTriplet::default(),
            best: PerformanceTriplet::default(),
            regime: CalibrationRegime::Standard,
            step_count: 0,
            mandorla_field: vec![0.0; 16],
            history: Vec::new(),
            config: CalibrationConfig::default(),
        }
    }

    /// Update with new performance observation
    pub fn observe(&mut self, triplet: PerformanceTriplet) {
        self.current = triplet;
        self.history.push(triplet);

        // Keep history bounded
        if self.history.len() > 100 {
            self.history.remove(0);
        }

        // Update best if improved
        if triplet.resonance() > self.best.resonance() {
            self.best = triplet;
        }

        // Update Mandorla field (accumulator)
        self.update_mandorla_field(&triplet);
    }

    /// Update Mandorla field with new observation
    fn update_mandorla_field(&mut self, triplet: &PerformanceTriplet) {
        // Encode triplet into field dimensions
        let alpha = 0.1; // Learning rate

        // Distribute triplet values across field dimensions
        for i in 0..16 {
            let signal = match i % 3 {
                0 => triplet.psi,
                1 => triplet.rho,
                _ => triplet.omega,
            };

            // Hebbian-like update
            self.mandorla_field[i] = (1.0 - alpha) * self.mandorla_field[i] + alpha * signal;
        }
    }

    /// Calculate field saturation (for regime change detection)
    pub fn field_saturation(&self) -> f64 {
        let mean: f64 = self.mandorla_field.iter().sum::<f64>() / 16.0;
        let variance: f64 = self.mandorla_field
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / 16.0;

        // Low variance = saturated
        1.0 - variance.sqrt().min(1.0)
    }

    /// Check if stuck in local optimum
    pub fn is_stagnant(&self) -> bool {
        if self.history.len() < 10 {
            return false;
        }

        let recent: Vec<_> = self.history.iter().rev().take(10).collect();
        let recent_avg: f64 = recent.iter().map(|t| t.resonance()).sum::<f64>() / 10.0;
        let older_avg: f64 = self.history.iter().rev().skip(10).take(10)
            .map(|t| t.resonance()).sum::<f64>().max(1.0) / 10.0;

        // Stagnant if less than 1% improvement
        (recent_avg - older_avg).abs() < 0.01
    }
}

impl Default for CalibrationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Calibration configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationConfig {
    /// Quantum circuit depth
    pub circuit_depth: usize,

    /// Number of measurement shots
    pub shots: usize,

    /// Learning rate for parameter updates
    pub learning_rate: f64,

    /// Ansatz type
    pub ansatz: String,

    /// Backend selection
    pub backend: String,

    /// Custom parameters
    pub custom: std::collections::HashMap<String, f64>,
}

impl Default for CalibrationConfig {
    fn default() -> Self {
        Self {
            circuit_depth: 3,
            shots: 1000,
            learning_rate: 0.1,
            ansatz: "hardware_efficient".to_string(),
            backend: "local".to_string(),
            custom: std::collections::HashMap::new(),
        }
    }
}

/// Calibration suggestion from SCS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationSuggestion {
    /// Suggested new configuration
    pub new_config: CalibrationConfig,

    /// Expected improvement
    pub expected_improvement: f64,

    /// Confidence in the suggestion
    pub confidence: f64,

    /// Explanation of the suggestion
    pub notes: String,

    /// Whether a regime change is suggested
    pub regime_change: Option<CalibrationRegime>,

    /// Proof-of-Resonance verified
    pub por_verified: bool,
}

impl CalibrationSuggestion {
    /// Create a new suggestion
    pub fn new(config: CalibrationConfig) -> Self {
        Self {
            new_config: config,
            expected_improvement: 0.0,
            confidence: 0.5,
            notes: String::new(),
            regime_change: None,
            por_verified: false,
        }
    }

    /// Set expected improvement
    pub fn with_improvement(mut self, improvement: f64) -> Self {
        self.expected_improvement = improvement;
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    /// Add notes
    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = notes.to_string();
        self
    }

    /// Suggest regime change
    pub fn with_regime_change(mut self, regime: CalibrationRegime) -> Self {
        self.regime_change = Some(regime);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_triplet() {
        let triplet = PerformanceTriplet::new(0.8, 0.9, 0.7);
        assert_eq!(triplet.psi, 0.8);
        assert_eq!(triplet.rho, 0.9);
        assert_eq!(triplet.omega, 0.7);

        let resonance = triplet.resonance();
        assert!((resonance - 0.504).abs() < 0.01);
    }

    #[test]
    fn test_dominance() {
        let a = PerformanceTriplet::new(0.8, 0.8, 0.8);
        let b = PerformanceTriplet::new(0.7, 0.7, 0.7);

        assert!(a.dominates(&b));
        assert!(!b.dominates(&a));
    }

    #[test]
    fn test_calibration_state() {
        let mut state = CalibrationState::new();

        // Add observations
        for i in 0..20 {
            let triplet = PerformanceTriplet::new(
                0.5 + 0.02 * i as f64,
                0.5,
                0.5,
            );
            state.observe(triplet);
        }

        assert_eq!(state.history.len(), 20);
        assert!(state.best.psi > 0.5);
    }

    #[test]
    fn test_stagnation_detection() {
        let mut state = CalibrationState::new();

        // Add constant observations
        for _ in 0..20 {
            state.observe(PerformanceTriplet::new(0.5, 0.5, 0.5));
        }

        assert!(state.is_stagnant());
    }

    #[test]
    fn test_suggestion() {
        let suggestion = CalibrationSuggestion::new(CalibrationConfig::default())
            .with_improvement(0.1)
            .with_confidence(0.8)
            .with_notes("Increase circuit depth");

        assert_eq!(suggestion.expected_improvement, 0.1);
        assert_eq!(suggestion.confidence, 0.8);
    }
}
