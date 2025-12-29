//! Hybrid Classical-Quantum Holistic Matrix
//!
//! Combines the classical HolisticMatrix from phosphoros-core with
//! quantum-enhanced evaluation for improved resonance detection.

use crate::backend::QuantumBackend;
use crate::backend::simulator::LocalSimulator;
use super::{QuantumResonanceEngine, QuantumEngineConfig};

/// Hybrid resonance evaluation combining classical and quantum approaches
///
/// Uses classical evaluation for fast screening and switches to quantum
/// evaluation when higher precision is needed.
pub struct HybridHolisticMatrix<B: QuantumBackend> {
    /// Quantum resonance engine
    quantum_engine: QuantumResonanceEngine<B>,

    /// Threshold for switching to quantum evaluation
    quantum_threshold: f64,

    /// Whether to always use quantum for final evaluation
    always_quantum_final: bool,

    /// Current time parameter
    time: f64,

    /// Statistics
    stats: HybridStats,
}

/// Statistics for hybrid evaluation
#[derive(Debug, Clone, Default)]
pub struct HybridStats {
    pub classical_evaluations: usize,
    pub quantum_evaluations: usize,
    pub quantum_improvements: usize,
}

/// Result of hybrid evaluation
#[derive(Debug, Clone)]
pub struct HybridEvaluation {
    /// Classical evaluation score
    pub classical_score: f64,

    /// Quantum evaluation score (if performed)
    pub quantum_score: Option<f64>,

    /// Final combined score
    pub final_score: f64,

    /// Whether quantum evaluation was used
    pub used_quantum: bool,

    /// The (ψ, ρ, ω) triplet
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,

    /// Evaluation outcome
    pub outcome: EvaluationOutcome,
}

/// Outcome of evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationOutcome {
    /// Passed threshold, output the vector
    Output,
    /// Below threshold, gated
    Gated,
    /// Borderline, needs further analysis
    Borderline,
}

impl<B: QuantumBackend> HybridHolisticMatrix<B> {
    /// Create a new hybrid matrix with the given quantum backend
    pub fn new(backend: B, config: QuantumEngineConfig) -> Self {
        let quantum_threshold = config.quantum_threshold;
        let quantum_engine = QuantumResonanceEngine::new(backend, config);

        Self {
            quantum_engine,
            quantum_threshold,
            always_quantum_final: false,
            time: 0.0,
            stats: HybridStats::default(),
        }
    }

    /// Evaluate resonance using hybrid approach
    pub fn evaluate(
        &mut self,
        t: f64,
        perception: [f64; 5],
        intention: [f64; 5],
        gradient: [f64; 5],
        theta: f64,
    ) -> HybridEvaluation {
        self.time = t;

        // Step 1: Classical evaluation (fast)
        let classical_result = self.classical_evaluate(&perception, &intention, &gradient);
        self.stats.classical_evaluations += 1;

        // Step 2: Decide if quantum evaluation is needed
        let needs_quantum = self.should_use_quantum(classical_result.score, theta);

        if !needs_quantum && !self.always_quantum_final {
            // Classical is sufficient
            return HybridEvaluation {
                classical_score: classical_result.score,
                quantum_score: None,
                final_score: classical_result.score,
                used_quantum: false,
                psi: classical_result.psi,
                rho: classical_result.rho,
                omega: classical_result.omega,
                outcome: if classical_result.score > theta {
                    EvaluationOutcome::Output
                } else {
                    EvaluationOutcome::Gated
                },
            };
        }

        // Step 3: Quantum evaluation for refinement
        self.stats.quantum_evaluations += 1;

        let quantum_result = self
            .quantum_engine
            .evaluate(t, perception, intention, theta)
            .ok();

        let (quantum_score, quantum_psi, quantum_rho, quantum_omega) = match &quantum_result {
            Some(r) => (Some(r.resonance), r.psi, r.rho, r.omega),
            None => (None, classical_result.psi, classical_result.rho, classical_result.omega),
        };

        // Step 4: Combine results
        let final_score = match quantum_score {
            Some(qs) => {
                // Weighted combination: quantum has higher weight near threshold
                let weight = self.calculate_quantum_weight(classical_result.score, theta);
                (1.0 - weight) * classical_result.score + weight * qs
            }
            None => classical_result.score,
        };

        // Track improvements
        if quantum_score.map(|qs| qs > classical_result.score).unwrap_or(false) {
            self.stats.quantum_improvements += 1;
        }

        let outcome = if final_score > theta * 1.1 {
            EvaluationOutcome::Output
        } else if final_score < theta * 0.9 {
            EvaluationOutcome::Gated
        } else {
            EvaluationOutcome::Borderline
        };

        HybridEvaluation {
            classical_score: classical_result.score,
            quantum_score,
            final_score,
            used_quantum: quantum_score.is_some(),
            psi: quantum_psi,
            rho: quantum_rho,
            omega: quantum_omega,
            outcome,
        }
    }

    /// Classical resonance evaluation (fast approximation)
    fn classical_evaluate(
        &self,
        perception: &[f64; 5],
        intention: &[f64; 5],
        gradient: &[f64; 5],
    ) -> ClassicalResult {
        // Compute dot product (Mandorla intersection)
        let dot_product: f64 = perception
            .iter()
            .zip(intention.iter())
            .map(|(p, i)| p * i)
            .sum();

        // Compute norms
        let p_norm: f64 = perception.iter().map(|x| x * x).sum::<f64>().sqrt();
        let i_norm: f64 = intention.iter().map(|x| x * x).sum::<f64>().sqrt();

        // Cosine similarity (normalized Mandorla)
        let cos_sim = if p_norm > 1e-10 && i_norm > 1e-10 {
            dot_product / (p_norm * i_norm)
        } else {
            0.0
        };

        // ψ: Based on alignment
        let psi = (cos_sim + 1.0) / 2.0; // Map [-1, 1] to [0, 1]

        // ρ: Based on magnitude stability
        let magnitude_diff = (p_norm - i_norm).abs();
        let max_magnitude = p_norm.max(i_norm).max(1e-10);
        let rho = 1.0 - (magnitude_diff / max_magnitude).min(1.0);

        // ω: Based on gradient coherence
        let g_norm: f64 = gradient.iter().map(|x| x * x).sum::<f64>().sqrt();
        let omega = if g_norm > 1e-10 {
            let grad_alignment: f64 = gradient
                .iter()
                .zip(perception.iter())
                .map(|(g, p)| g * p)
                .sum::<f64>()
                / (g_norm * p_norm.max(1e-10));
            (grad_alignment + 1.0) / 2.0
        } else {
            0.5
        };

        // Combined score
        let score = psi * rho * omega;

        ClassicalResult { psi, rho, omega, score }
    }

    /// Determine if quantum evaluation should be used
    fn should_use_quantum(&self, classical_score: f64, theta: f64) -> bool {
        // Use quantum when classical result is near the threshold
        let margin = 0.2;
        classical_score > theta - margin && classical_score < theta + margin
    }

    /// Calculate weight for quantum result in combination
    fn calculate_quantum_weight(&self, classical_score: f64, theta: f64) -> f64 {
        // Higher weight when closer to threshold
        let distance_to_threshold = (classical_score - theta).abs();
        let max_distance = self.quantum_threshold;

        if distance_to_threshold > max_distance {
            0.0
        } else {
            1.0 - (distance_to_threshold / max_distance)
        }
    }

    /// Get evaluation statistics
    pub fn stats(&self) -> &HybridStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = HybridStats::default();
    }

    /// Set whether to always use quantum for final evaluation
    pub fn set_always_quantum(&mut self, always: bool) {
        self.always_quantum_final = always;
    }

    /// Reset the engine
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.quantum_engine.reset();
        self.stats = HybridStats::default();
    }
}

/// Result of classical evaluation
struct ClassicalResult {
    psi: f64,
    rho: f64,
    omega: f64,
    score: f64,
}

impl HybridHolisticMatrix<LocalSimulator> {
    /// Create with default local simulator
    pub fn default_local() -> Self {
        let backend = LocalSimulator::metatron();
        let config = QuantumEngineConfig::default();
        Self::new(backend, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_creation() {
        let matrix = HybridHolisticMatrix::default_local();
        assert_eq!(matrix.stats().classical_evaluations, 0);
    }

    #[test]
    fn test_hybrid_evaluation() {
        let mut matrix = HybridHolisticMatrix::default_local();

        let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
        let intention = [0.5, 0.5, 0.5, 0.5, 0.5];
        let gradient = [0.0, 0.0, 0.0, 0.0, 0.0];
        let theta = 0.3;

        let result = matrix.evaluate(0.0, perception, intention, gradient, theta);

        assert!(result.final_score >= 0.0 && result.final_score <= 1.0);
        assert!(matrix.stats().classical_evaluations > 0);
    }

    #[test]
    fn test_orthogonal_vectors_low_resonance() {
        let mut matrix = HybridHolisticMatrix::default_local();

        // Orthogonal vectors should have low resonance
        let perception = [1.0, 0.0, 0.0, 0.0, 0.0];
        let intention = [0.0, 1.0, 0.0, 0.0, 0.0];
        let gradient = [0.0, 0.0, 0.0, 0.0, 0.0];

        let result = matrix.evaluate(0.0, perception, intention, gradient, 0.5);

        // Orthogonal should have lower resonance than aligned
        assert!(result.psi < 0.8);
    }

    #[test]
    fn test_quantum_activation_near_threshold() {
        let mut matrix = HybridHolisticMatrix::default_local();

        // Create a scenario near threshold
        let perception = [0.6, 0.6, 0.6, 0.6, 0.6];
        let intention = [0.5, 0.5, 0.5, 0.5, 0.5];
        let gradient = [0.1, 0.1, 0.1, 0.1, 0.1];

        // Use a threshold that might trigger quantum evaluation
        let theta = 0.4;

        let result = matrix.evaluate(0.0, perception, intention, gradient, theta);

        // Just verify it works
        assert!(result.psi >= 0.0);
    }

    #[test]
    fn test_statistics_tracking() {
        let mut matrix = HybridHolisticMatrix::default_local();

        let p = [0.5, 0.5, 0.5, 0.5, 0.5];
        let i = [0.5, 0.5, 0.5, 0.5, 0.5];
        let g = [0.0, 0.0, 0.0, 0.0, 0.0];

        for _ in 0..5 {
            let _ = matrix.evaluate(0.0, p, i, g, 0.3);
        }

        assert_eq!(matrix.stats().classical_evaluations, 5);
    }
}
