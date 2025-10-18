//! Spectral Signature - Core resonance invariant (ψ, ρ, ω)
//!
//! This module defines the fundamental spectral signature used throughout PHOSPHOROS.
//! The resonance formula D = ψ·ρ·ω is **INVARIANT** and must not be changed.

use serde::{Deserialize, Serialize};

/// Spectral signature σ = (ψ, ρ, ω) representing coherence, density, and frequency
///
/// This is the core invariant of the PHOSPHOROS system. All components must use
/// this signature type and preserve the resonance calculation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

    /// Create a zero signature (no resonance)
    pub fn zero() -> Self {
        Self {
            psi: 0.0,
            rho: 0.0,
            omega: 0.0,
        }
    }

    /// Create a maximum signature (full resonance)
    pub fn max() -> Self {
        Self {
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
        }
    }

    /// Calculate resonance score: D = ψ·ρ·ω
    ///
    /// **INVARIANT**: This formula must not be changed. It is the core
    /// resonance calculation used throughout PHOSPHOROS.
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Calculate total energy (alternative metric)
    pub fn energy(&self) -> f64 {
        (self.psi.powi(2) + self.rho.powi(2) + self.omega.powi(2)).sqrt()
    }

    /// Calculate the norm (L2 distance from origin)
    pub fn norm(&self) -> f64 {
        self.energy()
    }
}

impl Default for SpectralSignature {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clamps_values() {
        let sig = SpectralSignature::new(1.5, -0.5, 0.5);
        assert_eq!(sig.psi, 1.0);
        assert_eq!(sig.rho, 0.0);
        assert_eq!(sig.omega, 0.5);
    }

    #[test]
    fn test_resonance_invariant() {
        let sig = SpectralSignature::new(0.8, 0.6, 0.5);
        let expected = 0.8 * 0.6 * 0.5;
        assert!((sig.resonance() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_zero_resonance() {
        let sig = SpectralSignature::zero();
        assert_eq!(sig.resonance(), 0.0);
    }

    #[test]
    fn test_max_resonance() {
        let sig = SpectralSignature::max();
        assert_eq!(sig.resonance(), 1.0);
    }

    #[test]
    fn test_energy() {
        let sig = SpectralSignature::new(0.6, 0.8, 0.0);
        let expected = (0.6f64.powi(2) + 0.8f64.powi(2)).sqrt();
        assert!((sig.energy() - expected).abs() < 1e-10);
    }
}
