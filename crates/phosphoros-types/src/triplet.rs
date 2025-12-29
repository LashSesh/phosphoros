//! Performance Triplet (ψ, ρ, ω)
//!
//! The fundamental performance metric used throughout PHOSPHOROS.

use serde::{Deserialize, Serialize};

/// Performance Triplet representing (ψ, ρ, ω)
///
/// - ψ (psi): Coherence/Quality - measures semantic coherence [0, 1]
/// - ρ (rho): Stability/Density - measures structural stability [0, 1]
/// - ω (omega): Efficiency/Frequency - measures operational efficiency [0, 1]
///
/// The resonance score D = ψ · ρ · ω is the INVARIANT product used for ranking.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTriplet {
    /// Coherence/Quality component
    pub psi: f64,
    /// Stability/Density component
    pub rho: f64,
    /// Efficiency/Frequency component
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

    /// Create a triplet with all components equal
    pub fn uniform(value: f64) -> Self {
        Self::new(value, value, value)
    }

    /// Create a zero triplet
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create a unit triplet (all 1s)
    pub fn unit() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Calculate the resonance score (INVARIANT)
    ///
    /// D = ψ · ρ · ω
    #[inline]
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Calculate the harmonic mean
    pub fn harmonic_mean(&self) -> f64 {
        if self.psi == 0.0 || self.rho == 0.0 || self.omega == 0.0 {
            return 0.0;
        }
        3.0 / (1.0 / self.psi + 1.0 / self.rho + 1.0 / self.omega)
    }

    /// Calculate the geometric mean (cube root of resonance)
    pub fn geometric_mean(&self) -> f64 {
        self.resonance().cbrt()
    }

    /// Calculate the arithmetic mean
    pub fn arithmetic_mean(&self) -> f64 {
        (self.psi + self.rho + self.omega) / 3.0
    }

    /// Calculate the L2 norm
    pub fn norm(&self) -> f64 {
        (self.psi.powi(2) + self.rho.powi(2) + self.omega.powi(2)).sqrt()
    }

    /// Normalize to unit norm
    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n < 1e-10 {
            Self::zero()
        } else {
            Self::new(self.psi / n, self.rho / n, self.omega / n)
        }
    }

    /// Blend two triplets with weight α ∈ [0, 1]
    ///
    /// result = (1 - α) · self + α · other
    pub fn blend(&self, other: &Self, alpha: f64) -> Self {
        let a = alpha.clamp(0.0, 1.0);
        Self::new(
            (1.0 - a) * self.psi + a * other.psi,
            (1.0 - a) * self.rho + a * other.rho,
            (1.0 - a) * self.omega + a * other.omega,
        )
    }

    /// Component-wise minimum
    pub fn min(&self, other: &Self) -> Self {
        Self::new(
            self.psi.min(other.psi),
            self.rho.min(other.rho),
            self.omega.min(other.omega),
        )
    }

    /// Component-wise maximum
    pub fn max(&self, other: &Self) -> Self {
        Self::new(
            self.psi.max(other.psi),
            self.rho.max(other.rho),
            self.omega.max(other.omega),
        )
    }

    /// Check if all components exceed threshold
    pub fn all_above(&self, threshold: f64) -> bool {
        self.psi >= threshold && self.rho >= threshold && self.omega >= threshold
    }

    /// Check if any component exceeds threshold
    pub fn any_above(&self, threshold: f64) -> bool {
        self.psi >= threshold || self.rho >= threshold || self.omega >= threshold
    }

    /// Get components as array
    pub fn as_array(&self) -> [f64; 3] {
        [self.psi, self.rho, self.omega]
    }

    /// Create from array
    pub fn from_array(arr: [f64; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }

    /// Distance to another triplet (Euclidean)
    pub fn distance(&self, other: &Self) -> f64 {
        let dp = self.psi - other.psi;
        let dr = self.rho - other.rho;
        let dw = self.omega - other.omega;
        (dp.powi(2) + dr.powi(2) + dw.powi(2)).sqrt()
    }
}

impl Default for PerformanceTriplet {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

impl std::fmt::Display for PerformanceTriplet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ψ={:.3}, ρ={:.3}, ω={:.3}) → D={:.4}",
            self.psi,
            self.rho,
            self.omega,
            self.resonance()
        )
    }
}

impl std::ops::Add for PerformanceTriplet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.psi + other.psi,
            self.rho + other.rho,
            self.omega + other.omega,
        )
    }
}

impl std::ops::Sub for PerformanceTriplet {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            (self.psi - other.psi).max(0.0),
            (self.rho - other.rho).max(0.0),
            (self.omega - other.omega).max(0.0),
        )
    }
}

impl std::ops::Mul<f64> for PerformanceTriplet {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.psi * scalar, self.rho * scalar, self.omega * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplet_creation() {
        let t = PerformanceTriplet::new(0.8, 0.9, 0.7);
        assert!((t.psi - 0.8).abs() < 1e-10);
        assert!((t.rho - 0.9).abs() < 1e-10);
        assert!((t.omega - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_clamping() {
        let t = PerformanceTriplet::new(1.5, -0.5, 0.5);
        assert!((t.psi - 1.0).abs() < 1e-10);
        assert!((t.rho - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_resonance() {
        let t = PerformanceTriplet::new(0.5, 0.5, 0.5);
        assert!((t.resonance() - 0.125).abs() < 1e-10);

        let u = PerformanceTriplet::unit();
        assert!((u.resonance() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_means() {
        let t = PerformanceTriplet::new(1.0, 1.0, 1.0);
        assert!((t.harmonic_mean() - 1.0).abs() < 1e-10);
        assert!((t.geometric_mean() - 1.0).abs() < 1e-10);
        assert!((t.arithmetic_mean() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_blend() {
        let a = PerformanceTriplet::new(0.0, 0.0, 0.0);
        let b = PerformanceTriplet::new(1.0, 1.0, 1.0);

        let mid = a.blend(&b, 0.5);
        assert!((mid.psi - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let t = PerformanceTriplet::new(0.8, 0.9, 0.7);
        let s = format!("{}", t);
        assert!(s.contains("ψ=0.800"));
        assert!(s.contains("ρ=0.900"));
        assert!(s.contains("ω=0.700"));
    }
}
