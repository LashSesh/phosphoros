//! Monolith - Action node for singularity-triggered decisions

/// Monolith - Action node that triggers on geometric singularity conditions
///
/// The Monolith evaluates whether the gradient field and resonance cone align
/// sufficiently to trigger an action vector (excalibration).
#[derive(Debug, Clone)]
pub struct Monolith {
    /// Action vector E⃗
    pub action_vector: [f64; 5],
    /// Stability metric
    pub stability: f64,
    /// Timestamp of last evaluation
    pub timestamp: f64,
    /// Whether singularity was triggered
    pub singularity_triggered: bool,
}

impl Monolith {
    /// Create a new Monolith
    pub fn new() -> Self {
        Self {
            action_vector: [0.0; 5],
            stability: 0.0,
            timestamp: 0.0,
            singularity_triggered: false,
        }
    }

    /// Evaluate Monolith criterion: M(t) = δ(∇(ψ,ρ,ω) · C⃗_Konus - Θ)
    ///
    /// Returns true if the singularity condition is met and action should be taken.
    pub fn evaluate_criterion(
        &mut self,
        gradient: &[f64; 5],
        konus_field: &[f64; 5],
        theta: f64,
        t: f64,
    ) -> bool {
        // Compute dot product of gradient and konus field
        let dot: f64 = gradient
            .iter()
            .zip(konus_field.iter())
            .map(|(g, k)| g * k)
            .sum();

        self.timestamp = t;

        // Singularity triggered when dot > theta
        if dot > theta {
            self.action_vector = *gradient;
            self.stability = dot - theta;
            self.singularity_triggered = true;
            true
        } else {
            self.singularity_triggered = false;
            false
        }
    }

    /// Get the excalibration vector (action to take)
    ///
    /// Returns the action vector if singularity was triggered, zero otherwise.
    pub fn excalibrate(&self) -> [f64; 5] {
        if self.singularity_triggered {
            self.action_vector
        } else {
            [0.0; 5]
        }
    }

    /// Reset the Monolith to initial state
    pub fn reset(&mut self) {
        self.action_vector = [0.0; 5];
        self.stability = 0.0;
        self.timestamp = 0.0;
        self.singularity_triggered = false;
    }
}

impl Default for Monolith {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monolith_creation() {
        let monolith = Monolith::new();
        assert!(!monolith.singularity_triggered);
        assert_eq!(monolith.stability, 0.0);
    }

    #[test]
    fn test_evaluate_criterion_triggered() {
        let mut monolith = Monolith::new();
        let gradient = [1.0, 2.0, 3.0, 4.0, 5.0];
        let konus = [5.0, 4.0, 3.0, 2.0, 1.0];
        let theta = 30.0;
        // dot = 1*5 + 2*4 + 3*3 + 4*2 + 5*1 = 35
        let result = monolith.evaluate_criterion(&gradient, &konus, theta, 1.0);
        assert!(result);
        assert!(monolith.singularity_triggered);
        assert_eq!(monolith.stability, 35.0 - theta);
    }

    #[test]
    fn test_evaluate_criterion_not_triggered() {
        let mut monolith = Monolith::new();
        let gradient = [1.0, 1.0, 1.0, 1.0, 1.0];
        let konus = [1.0, 1.0, 1.0, 1.0, 1.0];
        let theta = 10.0;
        // dot = 5.0
        let result = monolith.evaluate_criterion(&gradient, &konus, theta, 1.0);
        assert!(!result);
        assert!(!monolith.singularity_triggered);
    }

    #[test]
    fn test_excalibrate() {
        let mut monolith = Monolith::new();
        let gradient = [1.0, 2.0, 3.0, 4.0, 5.0];
        let konus = [5.0, 4.0, 3.0, 2.0, 1.0];
        monolith.evaluate_criterion(&gradient, &konus, 30.0, 1.0);
        let action = monolith.excalibrate();
        assert_eq!(action, gradient);
    }
}
