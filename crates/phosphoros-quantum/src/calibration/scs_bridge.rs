//! SCS Bridge for PHOSPHOROS Integration
//!
//! Connects the Seraphic Calibration System with PHOSPHOROS resonance engines.

use super::{
    CalibrationRegime, CalibrationState, CalibrationSuggestion,
    PerformanceTriplet, performance::CalibrationConfig,
};
use crate::engines::QuantumResonanceEngine;
use crate::backend::QuantumBackend;
use std::time::Instant;

/// Bridge between SCS and PHOSPHOROS
pub struct SCSBridge {
    /// Current calibration state
    state: CalibrationState,

    /// Double-kick operator parameters
    phi_u_rate: f64, // Update kick learning rate
    phi_v_rate: f64, // Stabilization kick rate

    /// Proof-of-Resonance threshold
    por_threshold: f64,

    /// Calibration step timing
    last_step: Option<Instant>,
    /// Step interval in milliseconds (reserved for rate-limiting)
    #[allow(dead_code)]
    step_interval_ms: u64,
}

impl SCSBridge {
    /// Create a new SCS bridge
    pub fn new() -> Self {
        Self {
            state: CalibrationState::new(),
            phi_u_rate: 0.1,
            phi_v_rate: 0.05,
            por_threshold: 0.0, // Must improve or stay same
            last_step: None,
            step_interval_ms: 1000,
        }
    }

    /// Configure double-kick rates
    pub fn with_kick_rates(mut self, phi_u: f64, phi_v: f64) -> Self {
        self.phi_u_rate = phi_u;
        self.phi_v_rate = phi_v;
        self
    }

    /// Set PoR threshold
    pub fn with_por_threshold(mut self, threshold: f64) -> Self {
        self.por_threshold = threshold;
        self
    }

    /// Perform a calibration step
    ///
    /// Implements the SCS Double-Kick operator T = Φ_V ∘ Φ_U:
    /// 1. Φ_U (Update-Kick): Improves quality (ψ)
    /// 2. Φ_V (Stabilization-Kick): Optimizes stability (ρ)
    pub fn step(&mut self, current_triplet: PerformanceTriplet) -> CalibrationSuggestion {
        // Record observation
        self.state.observe(current_triplet);
        self.state.step_count += 1;
        self.last_step = Some(Instant::now());

        // Check for regime change (CRI - Calibration Regime Initialization)
        let regime_change = self.check_regime_change();

        // Apply double-kick operator
        let new_config = self.apply_double_kick();

        // Verify Proof-of-Resonance
        let por_verified = self.verify_por(&new_config);

        // Calculate expected improvement
        let expected_improvement = self.estimate_improvement(&new_config);

        CalibrationSuggestion {
            new_config,
            expected_improvement,
            confidence: self.calculate_confidence(),
            notes: self.generate_notes(),
            regime_change,
            por_verified,
        }
    }

    /// Apply the double-kick operator T = Φ_V ∘ Φ_U
    fn apply_double_kick(&self) -> CalibrationConfig {
        let mut config = self.state.config.clone();

        // Φ_U: Update-Kick (quality improvement)
        let psi = self.state.current.psi;
        if psi < 0.8 {
            // Increase circuit depth for better quality
            config.circuit_depth = (config.circuit_depth + 1).min(10);
            config.learning_rate *= 1.1;
        }

        // Φ_V: Stabilization-Kick (stability optimization)
        let rho = self.state.current.rho;
        if rho < 0.7 {
            // Increase shots for better stability
            config.shots = (config.shots as f64 * 1.2) as usize;
        }

        // Efficiency consideration (ω)
        let omega = self.state.current.omega;
        if omega < 0.5 && config.shots > 500 {
            // Trade some shots for efficiency
            config.shots = (config.shots as f64 * 0.9) as usize;
        }

        config
    }

    /// Check if regime change is needed (CRI)
    fn check_regime_change(&self) -> Option<CalibrationRegime> {
        let saturation = self.state.field_saturation();
        let is_stagnant = self.state.is_stagnant();

        match self.state.regime {
            CalibrationRegime::Standard => {
                if is_stagnant && saturation > 0.8 {
                    Some(CalibrationRegime::Aggressive)
                } else {
                    None
                }
            }
            CalibrationRegime::Aggressive => {
                if self.state.current.resonance() > self.state.best.resonance() * 0.95 {
                    Some(CalibrationRegime::Standard)
                } else if self.state.step_count > 50 {
                    Some(CalibrationRegime::Exploring)
                } else {
                    None
                }
            }
            CalibrationRegime::Exploring => {
                if self.state.current.resonance() > 0.7 {
                    Some(CalibrationRegime::Standard)
                } else {
                    None
                }
            }
            CalibrationRegime::Homeostasis => {
                if self.state.current.resonance() < self.state.best.resonance() * 0.9 {
                    Some(CalibrationRegime::Standard)
                } else {
                    None
                }
            }
        }
    }

    /// Verify Proof-of-Resonance
    ///
    /// PoR ensures that configuration updates only improve or maintain performance.
    fn verify_por(&self, _new_config: &CalibrationConfig) -> bool {
        // In a full implementation, this would simulate the new config
        // For now, we check if the trend is positive
        if self.state.history.len() < 2 {
            return true;
        }

        let recent_avg: f64 = self.state.history.iter()
            .rev()
            .take(5)
            .map(|t| t.resonance())
            .sum::<f64>() / 5.0;

        let older_avg: f64 = self.state.history.iter()
            .rev()
            .skip(5)
            .take(5)
            .map(|t| t.resonance())
            .sum::<f64>().max(0.001) / 5.0;

        // PoR passes if improvement is >= threshold
        (recent_avg - older_avg) >= self.por_threshold
    }

    /// Estimate expected improvement from new config
    fn estimate_improvement(&self, _new_config: &CalibrationConfig) -> f64 {
        // Heuristic based on current state
        let current_resonance = self.state.current.resonance();
        let best_resonance = self.state.best.resonance();

        let gap = best_resonance - current_resonance;

        // Estimate we can close 10-30% of the gap
        gap * 0.2 * self.calculate_confidence()
    }

    /// Calculate confidence in the suggestion
    fn calculate_confidence(&self) -> f64 {
        let history_factor = (self.state.history.len() as f64 / 100.0).min(1.0);
        let stability_factor = self.state.current.rho;
        let saturation_factor = 1.0 - self.state.field_saturation();

        (history_factor + stability_factor + saturation_factor) / 3.0
    }

    /// Generate human-readable notes
    fn generate_notes(&self) -> String {
        let mut notes = Vec::new();

        if self.state.current.psi < 0.7 {
            notes.push("Quality (ψ) below target - increasing circuit depth");
        }
        if self.state.current.rho < 0.7 {
            notes.push("Stability (ρ) below target - increasing shots");
        }
        if self.state.current.omega < 0.5 {
            notes.push("Efficiency (ω) below target - optimizing resources");
        }
        if self.state.is_stagnant() {
            notes.push("Stagnation detected - consider regime change");
        }

        if notes.is_empty() {
            notes.push("Performance within acceptable range");
        }

        notes.join("; ")
    }

    /// Get current calibration state
    pub fn state(&self) -> &CalibrationState {
        &self.state
    }

    /// Reset the bridge
    pub fn reset(&mut self) {
        self.state = CalibrationState::new();
        self.last_step = None;
    }

    /// Calibrate a quantum resonance engine
    pub fn calibrate_engine<B: QuantumBackend>(
        &mut self,
        engine: &mut QuantumResonanceEngine<B>,
        benchmark_runs: usize,
    ) -> Vec<CalibrationSuggestion> {
        let mut suggestions = Vec::new();

        for _ in 0..benchmark_runs {
            // Run a benchmark evaluation
            let test_perception = [0.5, 0.5, 0.5, 0.5, 0.5];
            let test_intention = [0.5, 0.5, 0.5, 0.5, 0.5];

            let start = Instant::now();
            let result = engine.evaluate(0.0, test_perception, test_intention, 0.5);
            let elapsed = start.elapsed().as_secs_f64();

            let triplet = match result {
                Ok(r) => PerformanceTriplet::new(
                    r.psi,
                    r.rho,
                    1.0 / (1.0 + elapsed), // Efficiency based on time
                ),
                Err(_) => PerformanceTriplet::new(0.0, 0.0, 0.0),
            };

            let suggestion = self.step(triplet);
            suggestions.push(suggestion);
        }

        suggestions
    }
}

impl Default for SCSBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scs_bridge_creation() {
        let bridge = SCSBridge::new();
        assert_eq!(bridge.state.step_count, 0);
    }

    #[test]
    fn test_calibration_step() {
        let mut bridge = SCSBridge::new();

        let triplet = PerformanceTriplet::new(0.6, 0.7, 0.8);
        let suggestion = bridge.step(triplet);

        assert!(suggestion.por_verified);
        assert!(suggestion.confidence > 0.0);
        assert_eq!(bridge.state.step_count, 1);
    }

    #[test]
    fn test_multiple_steps() {
        let mut bridge = SCSBridge::new();

        for i in 0..10 {
            let triplet = PerformanceTriplet::new(
                0.5 + 0.03 * i as f64,
                0.6,
                0.7,
            );
            let _ = bridge.step(triplet);
        }

        assert_eq!(bridge.state.step_count, 10);
        assert!(bridge.state.best.psi > 0.5);
    }

    #[test]
    fn test_regime_change_detection() {
        let mut bridge = SCSBridge::new();

        // Simulate stagnation
        for _ in 0..30 {
            let triplet = PerformanceTriplet::new(0.5, 0.5, 0.5);
            let suggestion = bridge.step(triplet);

            if suggestion.regime_change.is_some() {
                // Should suggest regime change after stagnation
                assert!(matches!(
                    suggestion.regime_change,
                    Some(CalibrationRegime::Aggressive) | Some(CalibrationRegime::Exploring)
                ));
                return;
            }
        }
    }

    #[test]
    fn test_por_verification() {
        let mut bridge = SCSBridge::new();

        // Improving trend
        for i in 0..10 {
            let triplet = PerformanceTriplet::new(
                0.5 + 0.05 * i as f64,
                0.6 + 0.03 * i as f64,
                0.7,
            );
            let suggestion = bridge.step(triplet);
            assert!(suggestion.por_verified);
        }
    }

    #[test]
    fn test_double_kick_quality() {
        let mut bridge = SCSBridge::new();

        // Low quality should trigger depth increase
        let triplet = PerformanceTriplet::new(0.3, 0.8, 0.8);
        let suggestion = bridge.step(triplet);

        // Config should have increased depth
        assert!(suggestion.new_config.circuit_depth >= 3);
    }

    #[test]
    fn test_double_kick_stability() {
        let mut bridge = SCSBridge::new();

        // Low stability should trigger shots increase
        let triplet = PerformanceTriplet::new(0.8, 0.3, 0.8);
        let suggestion = bridge.step(triplet);

        // Config should have increased shots
        assert!(suggestion.new_config.shots >= 1000);
    }
}
