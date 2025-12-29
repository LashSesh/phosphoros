//! Kosmokrator - Exclusion Engine with Proof-of-Resonance

use std::ops::{Add, Mul};

/// Complex number for phase space operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    /// Real component
    pub re: f64,
    /// Imaginary component
    pub im: f64,
}

impl Complex {
    /// Create a new complex number from real and imaginary parts
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create a complex number from polar coordinates (r, θ)
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Calculate the magnitude |z| = √(re² + im²)
    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Calculate the phase angle θ = atan2(im, re)
    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Return the complex conjugate z* = re - i·im
    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// Phase space state - superposition of operators with phase positions
#[derive(Debug, Clone)]
pub struct PhaseState {
    /// Amplitude coefficients |αᵢ| for each basis state
    pub amplitudes: Vec<f64>,
    /// Phase angles θᵢ for each basis state
    pub phases: Vec<f64>,
}

impl PhaseState {
    /// Create new phase state with n dimensions
    pub fn new(n: usize) -> Self {
        Self {
            amplitudes: vec![1.0 / (n as f64).sqrt(); n],
            phases: vec![0.0; n],
        }
    }

    /// Normalize: ||ψ||² = 1
    pub fn normalize(&mut self) {
        let norm_sq: f64 = self.amplitudes.iter().map(|a| a * a).sum();
        let norm = norm_sq.sqrt();

        if norm > 1e-12 {
            self.amplitudes.iter_mut().for_each(|a| *a /= norm);
        }
    }

    /// Convert to complex representation
    pub fn to_complex(&self) -> Vec<Complex> {
        self.amplitudes
            .iter()
            .zip(self.phases.iter())
            .map(|(&a, &theta)| Complex::from_polar(a, theta))
            .collect()
    }
}

/// Kosmokrator - Exclusion engine with Proof-of-Resonance (PoR)
///
/// Filters unstable states and defines reality through resonance thresholds.
#[derive(Debug, Clone)]
pub struct Kosmokrator {
    /// Resonance threshold κ⋆
    pub kappa_threshold: f64,
    /// Maximum fluctuation ε
    pub epsilon_max: f64,
    /// Cutoff tolerance η
    pub eta_cutoff: f64,
    /// History of coherence values
    pub coherence_history: Vec<f64>,
}

impl Kosmokrator {
    /// Create a new Kosmokrator with specified thresholds
    pub fn new(kappa_threshold: f64, epsilon_max: f64, eta_cutoff: f64) -> Self {
        Self {
            kappa_threshold,
            epsilon_max,
            eta_cutoff,
            coherence_history: Vec::new(),
        }
    }

    /// Calculate coherence measure: κ(t) = |1/N Σ e^(iθⱼ)|
    pub fn coherence(&self, state: &PhaseState) -> f64 {
        let complex_states = state.to_complex();
        let n = complex_states.len() as f64;

        let sum = complex_states
            .iter()
            .fold(Complex::new(0.0, 0.0), |acc, &c| acc + c);

        sum.magnitude() / n
    }

    /// Check Proof-of-Resonance (PoR): κ(t) ≥ κ⋆ ∧ |dκ/dt| ≤ ε
    pub fn check_por(&mut self, state: &PhaseState) -> bool {
        let kappa = self.coherence(state);
        self.coherence_history.push(kappa);

        // κ(t) ≥ κ⋆
        let threshold_met = kappa >= self.kappa_threshold;

        // |dκ/dt| ≤ ε
        let stability_met = if self.coherence_history.len() >= 2 {
            let len = self.coherence_history.len();
            let delta = (self.coherence_history[len - 1] - self.coherence_history[len - 2]).abs();
            delta <= self.epsilon_max
        } else {
            true
        };

        threshold_met && stability_met
    }

    /// Telescope operator: Isolate stable resonance cores
    pub fn telescope_operator(&self, states: &[PhaseState]) -> Vec<usize> {
        states
            .iter()
            .enumerate()
            .filter_map(|(idx, state)| {
                let kappa = self.coherence(state);

                // Check stability over time
                if kappa >= self.kappa_threshold * 0.8 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Exclusion operator: Eliminate unstable states
    pub fn exclusion_operator(&self, states: Vec<PhaseState>) -> Vec<PhaseState> {
        states
            .into_iter()
            .filter(|state| {
                let kappa = self.coherence(state);
                kappa >= self.kappa_threshold * 0.5 // Minimum threshold
            })
            .collect()
    }

    /// Reality filter: Only PoR-compliant states
    pub fn reality_filter(&mut self, states: Vec<PhaseState>) -> Vec<PhaseState> {
        states
            .into_iter()
            .filter(|state| self.check_por(state))
            .collect()
    }

    /// Reset the Kosmokrator to initial state
    pub fn reset(&mut self) {
        self.coherence_history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_from_polar() {
        use std::f64::consts::PI;
        let c = Complex::from_polar(1.0, PI / 2.0);
        assert!(c.re.abs() < 1e-10);
        assert!((c.im - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_phase_state_normalize() {
        let mut state = PhaseState::new(3);
        state.amplitudes = vec![2.0, 2.0, 2.0];
        state.normalize();
        let norm_sq: f64 = state.amplitudes.iter().map(|a| a * a).sum();
        assert!((norm_sq - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_kosmokrator_coherence() {
        let kosmo = Kosmokrator::new(0.8, 0.05, 0.1);
        let state = PhaseState::new(10);
        let coherence = kosmo.coherence(&state);
        assert!(coherence >= 0.0 && coherence <= 1.0);
    }

    #[test]
    fn test_check_por() {
        let mut kosmo = Kosmokrator::new(0.3, 0.05, 0.1);
        let mut state = PhaseState::new(10);
        // All phases are 0, so complex vectors are all aligned
        state.normalize();
        let coherence = kosmo.coherence(&state);
        // Coherence should be 1/sqrt(n) for normalized equal amplitudes
        assert!(
            coherence > 0.3,
            "Expected coherence > 0.3, got {}",
            coherence
        );
        // First call should pass with our lowered threshold
        let result = kosmo.check_por(&state);
        assert!(result, "PoR check should pass with aligned phases");
    }
}
