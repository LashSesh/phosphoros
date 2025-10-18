//! Triton Spiral Generator - 5D Golden Spiral with Ouroboros feedback

use crate::geometry::Point5D;
use crate::resonance::SpectralSignature;

/// Triton Spiral Generator with Golden Ratio and Ouroboros momentum
pub struct TritonSpiralGenerator {
    step: usize,
    /// Base radius for spiral generation
    pub radius_base: f64,
    phi_golden: f64,
    /// Learning rate for Ouroboros feedback
    pub alpha_learning: f64,
    /// Current position in 5D space
    pub current_pos: Point5D,
    ouroboros_momentum: Point5D,
    best_signature: Option<SpectralSignature>,
    projection_matrix: [[f64; 5]; 5],
}

impl TritonSpiralGenerator {
    /// Create a new Triton spiral generator with deterministic seed
    pub fn new(seed: u64) -> Self {
        let phi_golden = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());

        // Deterministically generated projection matrix
        let mut projection_matrix = [[0.0; 5]; 5];
        let mut h = seed;
        for row in &mut projection_matrix {
            for cell in row.iter_mut() {
                h = h.wrapping_mul(0x9e3779b97f4a7c15);
                *cell = ((h as f64 / u64::MAX as f64) - 0.5) * 2.0;
            }
        }

        Self {
            step: 0,
            radius_base: 0.015,
            phi_golden,
            alpha_learning: 0.12,
            current_pos: Point5D::zero(),
            ouroboros_momentum: Point5D::zero(),
            best_signature: None,
            projection_matrix,
        }
    }

    /// Generate next point in the spiral
    pub fn generate_next(&mut self) -> Point5D {
        let r = self.radius_base * ((self.step + 1) as f64).sqrt();
        let theta = self.step as f64 * self.phi_golden;

        // 2D base
        let u = [r * theta.cos(), r * theta.sin()];

        // Extend to 5D
        let extended = [
            u[0],
            u[1],
            0.5 * (theta * 0.5).sin(),
            0.5 * (theta * 0.5).cos(),
            (self.step as f64 / 100.0) - 0.5,
        ];

        // Apply projection matrix
        let mut base_coords = [0.0; 5];
        for (i, coord) in base_coords.iter_mut().enumerate() {
            for (j, &ext_val) in extended.iter().enumerate() {
                *coord += ext_val * self.projection_matrix[i][j];
            }
        }

        // Apply Ouroboros drift
        let ouroboros_drift = self.ouroboros_momentum.scale(self.alpha_learning);
        let mut coords = Point5D {
            coords: base_coords,
        };
        coords = coords.add(&ouroboros_drift);
        coords = coords.normalize();

        self.current_pos = coords;
        self.step += 1;

        coords
    }

    /// Update Ouroboros momentum via spectral signature feedback
    pub fn update_ouroboros(&mut self, signature: SpectralSignature) {
        let gradient = Point5D::new(
            signature.psi - 0.5,
            signature.rho - 0.5,
            signature.omega - 0.5,
            signature.resonance() - 0.5,
            (self.step as f64 * 0.1).sin() * signature.resonance(),
        );

        let decay = 0.9;
        let old = self.ouroboros_momentum.scale(decay);
        let new = gradient.scale(1.0 - decay);
        self.ouroboros_momentum = old.add(&new);

        // Track best signature
        if self.best_signature.is_none()
            || signature.resonance() > self.best_signature.unwrap().resonance()
        {
            self.best_signature = Some(signature);
        }
    }

    /// Get the best resonance encountered so far
    pub fn get_best_resonance(&self) -> f64 {
        self.best_signature.map(|s| s.resonance()).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triton_spiral_deterministic() {
        let mut spiral1 = TritonSpiralGenerator::new(42);
        let mut spiral2 = TritonSpiralGenerator::new(42);

        let p1 = spiral1.generate_next();
        let p2 = spiral2.generate_next();

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_triton_spiral_progression() {
        let mut spiral = TritonSpiralGenerator::new(42);
        let p1 = spiral.generate_next();
        let p2 = spiral.generate_next();

        assert!(p1.distance(&p2) > 0.0);
    }

    #[test]
    fn test_ouroboros_update() {
        let mut spiral = TritonSpiralGenerator::new(42);
        let sig = SpectralSignature::new(0.8, 0.9, 0.7);
        spiral.update_ouroboros(sig);
        assert!(spiral.get_best_resonance() > 0.0);
    }
}
