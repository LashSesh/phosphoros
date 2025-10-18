//! Information Alchemy Evaluator - Spectral signature measurement

use crate::geometry::Point5D;
use crate::resonance::SpectralSignature;

/// Evaluator for spectral signatures with coherence tracking
pub struct InformationAlchemyEvaluator {
    target_embedding: Option<Point5D>,
    coherence_window: usize,
    history: Vec<Point5D>,
}

impl InformationAlchemyEvaluator {
    /// Create a new evaluator with optional target embedding
    pub fn new(target_embedding: Option<Point5D>) -> Self {
        Self {
            target_embedding,
            coherence_window: 8,
            history: Vec::new(),
        }
    }

    /// Evaluate spectral signature of a point
    pub fn evaluate(&mut self, point: &Point5D) -> SpectralSignature {
        // ψ (Coherence): Similarity to target vector
        let psi = if let Some(target) = &self.target_embedding {
            let dot = point.dot(target);
            let norm_product = point.norm() * target.norm() + 1e-12;
            (dot / norm_product).max(0.0)
        } else {
            1.0 - point
                .coords
                .iter()
                .map(|&x| (x - 0.5).powi(2))
                .sum::<f64>()
                .sqrt()
        };

        // ρ (Density): Structural coherence over history
        let rho = if self.history.len() >= 2 {
            let recent: Vec<_> = self
                .history
                .iter()
                .rev()
                .take(self.coherence_window)
                .collect();
            let distances: Vec<_> = recent.iter().map(|p| point.distance(p)).collect();
            let mean_dist = distances.iter().sum::<f64>() / distances.len() as f64;
            1.0 / (1.0 + mean_dist)
        } else {
            point.norm()
        };

        // ω (Frequency): Phase matching
        let omega = if self.history.len() >= 2 {
            let last = &self.history[self.history.len() - 1];
            let prev = &self.history[self.history.len() - 2];

            let direction = point.add(&last.scale(-1.0));
            let prev_direction = last.add(&prev.scale(-1.0));

            let corr =
                direction.dot(&prev_direction) / (direction.norm() * prev_direction.norm() + 1e-12);
            (corr + 1.0) / 2.0
        } else {
            0.5
        };

        self.history.push(*point);
        if self.history.len() > 2 * self.coherence_window {
            self.history.drain(0..self.coherence_window);
        }

        SpectralSignature::new(psi, rho, omega)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_creation() {
        let evaluator = InformationAlchemyEvaluator::new(None);
        assert_eq!(evaluator.coherence_window, 8);
    }

    #[test]
    fn test_evaluator_signature() {
        let mut evaluator = InformationAlchemyEvaluator::new(None);
        let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let sig = evaluator.evaluate(&point);
        assert!(sig.psi >= 0.0 && sig.psi <= 1.0);
        assert!(sig.rho >= 0.0 && sig.rho <= 1.0);
        assert!(sig.omega >= 0.0 && sig.omega <= 1.0);
    }
}
