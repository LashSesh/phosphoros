//! Mandorla Field - Perception and Intention intersection

/// Mandorla Field - Intersection of Gabriel Funnel (perception) and Oriphiel Field (intention)
///
/// The Mandorla represents the sacred intersection where perception meets intention,
/// creating a singularity condition for action generation.
#[derive(Debug, Clone)]
pub struct MandorlaField {
    /// Gabriel Funnel - Perception vector P⃗
    pub gabriel_funnel: [f64; 5],
    /// Oriphiel Field - Intention vector I⃗
    pub oriphiel_field: [f64; 5],
    /// Singularity threshold η
    pub eta_threshold: f64,
}

impl MandorlaField {
    /// Create a new Mandorla Field with specified threshold
    pub fn new(eta_threshold: f64) -> Self {
        Self {
            gabriel_funnel: [0.0; 5],
            oriphiel_field: [0.0; 5],
            eta_threshold,
        }
    }

    /// Calculate Mandorla scalar: S_Mandorla = P⃗ · I⃗
    pub fn scalar_product(&self) -> f64 {
        self.gabriel_funnel
            .iter()
            .zip(self.oriphiel_field.iter())
            .map(|(p, i)| p * i)
            .sum()
    }

    /// Get the magnitude of the Mandorla scalar
    pub fn magnitude(&self) -> f64 {
        self.scalar_product().abs()
    }

    /// Check singularity condition: magnitude ≥ η ∧ |d/dt| < ε
    pub fn check_singularity(&self, derivative: f64) -> bool {
        let mag = self.magnitude();
        mag >= self.eta_threshold && derivative.abs() < 0.01
    }

    /// Update Gabriel Funnel (perception)
    pub fn update_gabriel(&mut self, perception: [f64; 5]) {
        self.gabriel_funnel = perception;
    }

    /// Update Oriphiel Field (intention)
    pub fn update_oriphiel(&mut self, intention: [f64; 5]) {
        self.oriphiel_field = intention;
    }

    /// Reset the field to zero
    pub fn reset(&mut self) {
        self.gabriel_funnel = [0.0; 5];
        self.oriphiel_field = [0.0; 5];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandorla_creation() {
        let field = MandorlaField::new(0.5);
        assert_eq!(field.eta_threshold, 0.5);
    }

    #[test]
    fn test_scalar_product() {
        let mut field = MandorlaField::new(0.5);
        field.update_gabriel([1.0, 2.0, 3.0, 4.0, 5.0]);
        field.update_oriphiel([5.0, 4.0, 3.0, 2.0, 1.0]);
        let product = field.scalar_product();
        // 1*5 + 2*4 + 3*3 + 4*2 + 5*1 = 5 + 8 + 9 + 8 + 5 = 35
        assert_eq!(product, 35.0);
    }

    #[test]
    fn test_check_singularity() {
        let mut field = MandorlaField::new(10.0);
        field.update_gabriel([3.0, 4.0, 0.0, 0.0, 0.0]);
        field.update_oriphiel([4.0, 3.0, 0.0, 0.0, 0.0]);
        // magnitude = |3*4 + 4*3| = 24.0
        assert!(field.check_singularity(0.005));
        assert!(!field.check_singularity(0.02));
    }
}
