//! Gabriel Cell - Metatron Resonite with (ψ, ρ, ω) dynamics

use crate::geometry::Point5D;
use serde::{Deserialize, Serialize};

/// Spectral signature σ = (ψ, ρ, ω) representing coherence, density, and frequency
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpectralSignature {
    /// Coherence/Semantics (0.0 to 1.0)
    pub psi: f64,
    /// Density/Structure (0.0 to 1.0)
    pub rho: f64,
    /// Frequency/Phase (0.0 to 1.0)
    pub omega: f64,
}

impl SpectralSignature {
    /// Create a new spectral signature (values clamped to [0, 1])
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
        }
    }

    /// Calculate resonance score: D = ψ·ρ·ω (INVARIANT - do not change)
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Calculate total energy (alternative metric)
    pub fn energy(&self) -> f64 {
        (self.psi.powi(2) + self.rho.powi(2) + self.omega.powi(2)).sqrt()
    }
}

/// Gabriel Cell - A resonite unit with Hebbian-like learning dynamics
pub struct GabrielCell {
    /// Cell identifier
    pub id: String,
    /// Psi component (coherence)
    pub psi: f64,
    /// Rho component (density)
    pub rho: f64,
    /// Omega component (frequency)
    pub omega: f64,
    /// Current output value
    pub output: f64,
    /// Learning rate for adaptation
    pub learn_rate: f64,
    history: Vec<f64>,
}

impl GabrielCell {
    /// Create a new Gabriel Cell with default initialization
    pub fn new(id: String) -> Self {
        Self {
            id,
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            output: 1.0,
            learn_rate: 0.12,
            history: Vec::new(),
        }
    }

    /// Evaluate cell on 5D embedding
    pub fn evaluate(&mut self, embedding: &Point5D) -> f64 {
        let input = embedding.coords.iter().sum::<f64>();

        // GabrielCell activation
        self.psi = (1.0 - self.learn_rate) * self.psi + self.learn_rate * input;
        self.output = self.psi * self.rho * self.omega;
        self.history.push(self.output);

        (self.output / 10.0).clamp(0.0, 1.0)
    }

    /// Apply Hebbian-like feedback
    pub fn feedback(&mut self, target: f64) {
        let err = target - self.output;
        self.psi += self.learn_rate * err;
        self.rho += self.learn_rate * err.tanh();
        self.omega += self.learn_rate * err.sin();

        self.psi = self.psi.clamp(0.01, 10.0);
        self.rho = self.rho.clamp(0.01, 10.0);
        self.omega = self.omega.clamp(0.01, 10.0);
    }

    /// Get current spectral signature
    pub fn get_signature(&self) -> SpectralSignature {
        SpectralSignature::new(self.psi / 10.0, self.rho / 10.0, self.omega / 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_signature_resonance() {
        let sig = SpectralSignature::new(0.8, 0.9, 0.7);
        assert!((sig.resonance() - 0.504).abs() < 1e-3);
    }

    #[test]
    fn test_spectral_signature_clamping() {
        let sig = SpectralSignature::new(1.5, -0.5, 0.5);
        assert_eq!(sig.psi, 1.0);
        assert_eq!(sig.rho, 0.0);
        assert_eq!(sig.omega, 0.5);
    }

    #[test]
    fn test_gabriel_cell_evaluation() {
        let mut cell = GabrielCell::new("test".to_string());
        let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let output = cell.evaluate(&point);
        assert!(output >= 0.0 && output <= 1.0);
    }
}
