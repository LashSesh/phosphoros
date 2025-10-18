//! Resonance Channel - Individual resonance unit with temporal dynamics

use std::f64::consts::PI;

/// Resonance Channel with (ψ, ρ, ω) dynamics and temporal envelope
///
/// Each channel maintains its own spectral components and phase dynamics,
/// evaluating resonance over time with an envelope function.
#[derive(Debug, Clone)]
pub struct ResonanceChannel {
    /// Channel identifier
    pub id: usize,
    /// Semantic density (ψ)
    pub psi: f64,
    /// Structural coherence (ρ)
    pub rho: f64,
    /// Rhythmic phase (ω)
    pub omega: f64,
    /// Envelope value (computed)
    pub lambda: f64,
    /// Circular frequency for envelope
    pub omega_circ: f64,
    /// Phase offset
    pub phi: f64,
}

impl ResonanceChannel {
    /// Create a new resonance channel
    pub fn new(id: usize) -> Self {
        Self {
            id,
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            lambda: 1.0,
            omega_circ: 2.0 * PI,
            phi: 0.0,
        }
    }

    /// Create a channel with custom phase offset
    pub fn with_phase(id: usize, phi: f64) -> Self {
        Self {
            id,
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            lambda: 1.0,
            omega_circ: 2.0 * PI,
            phi,
        }
    }

    /// Calculate envelope: Λ(t) = 1/2(1 + sin(ω_circ·t + φ))
    pub fn envelope(&self, t: f64) -> f64 {
        0.5 * (1.0 + (self.omega_circ * t + self.phi).sin())
    }

    /// Evaluate channel at time t: D(t) = ψ·ρ·ω·Λ(t)
    pub fn evaluate(&mut self, t: f64) -> f64 {
        self.lambda = self.envelope(t);
        self.psi * self.rho * self.omega * self.lambda
    }

    /// Update channel parameters with feedback
    pub fn update(&mut self, delta_psi: f64, delta_rho: f64, delta_omega: f64) {
        self.psi += delta_psi;
        self.rho += delta_rho;
        self.omega += delta_omega;

        // Clamp to valid range
        self.psi = self.psi.clamp(0.01, 10.0);
        self.rho = self.rho.clamp(0.01, 10.0);
        self.omega = self.omega.clamp(0.01, 10.0);
    }

    /// Reset channel to initial state
    pub fn reset(&mut self) {
        self.psi = 1.0;
        self.rho = 1.0;
        self.omega = 1.0;
        self.lambda = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let ch = ResonanceChannel::new(0);
        assert_eq!(ch.id, 0);
        assert_eq!(ch.psi, 1.0);
        assert_eq!(ch.rho, 1.0);
        assert_eq!(ch.omega, 1.0);
    }

    #[test]
    fn test_envelope_oscillates() {
        let ch = ResonanceChannel::new(0);
        let env0 = ch.envelope(0.0);
        let env_quarter = ch.envelope(0.25);
        assert!((env0 - 0.5).abs() < 1e-10);
        assert!(env_quarter > 0.5); // Should increase
    }

    #[test]
    fn test_evaluate() {
        let mut ch = ResonanceChannel::new(0);
        let result = ch.evaluate(0.0);
        // At t=0, envelope is 0.5, so result should be 1.0*1.0*1.0*0.5 = 0.5
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_update_clamps() {
        let mut ch = ResonanceChannel::new(0);
        ch.update(10.0, -5.0, 0.5);
        assert!(ch.psi <= 10.0);
        assert!(ch.rho >= 0.01);
    }
}
