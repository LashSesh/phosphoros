//! QDASH Explorer - Complete exploration engine

use crate::alchemy::{InformationAlchemyEvaluator, SolveCoagulaDecision};
use crate::geometry::Point5D;
use crate::resonance::{Infogenom, SpectralSignature};
use crate::spiral::TritonSpiralGenerator;

/// TIC (Temporal Invariant Crystal) structure
pub struct TICCrystal {
    /// Crystal identifier
    pub id: String,
    /// Center point in 5D space
    pub center: Point5D,
    /// Stability measure
    pub stability: f64,
    /// Formation step number
    pub formation_step: usize,
}

/// QDASH exploration engine
pub struct QDASHExplorer {
    pub(crate) infogenom: Infogenom,
    spiral: TritonSpiralGenerator,
    evaluator: InformationAlchemyEvaluator,
    decision: SolveCoagulaDecision,
    max_steps: usize,
    exploration_path: Vec<(Point5D, SpectralSignature)>,
    crystals: Vec<TICCrystal>,
}

/// Result of an exploration run
pub struct ExplorationResult {
    /// Number of steps taken
    pub steps: usize,
    /// Best resonance encountered
    pub best_resonance: f64,
    /// Number of crystals formed
    pub num_crystals: usize,
    /// Final position in 5D space
    pub final_position: Point5D,
}

impl QDASHExplorer {
    /// Create a new QDASH explorer
    pub fn new(infogenom: Infogenom, seed: u64, max_steps: usize) -> Self {
        Self {
            infogenom,
            spiral: TritonSpiralGenerator::new(seed),
            evaluator: InformationAlchemyEvaluator::new(None),
            decision: SolveCoagulaDecision::new(),
            max_steps,
            exploration_path: Vec::new(),
            crystals: Vec::new(),
        }
    }

    /// Execute exploration
    pub fn explore(&mut self) -> ExplorationResult {
        for _i in 0..self.max_steps {
            let coords = self.spiral.generate_next();
            let signature = self.evaluator.evaluate(&coords);

            let gate_open = self.decision.check_merkaba_gate(signature);

            if gate_open {
                self.attempt_crystallization(&coords, signature);
            }

            self.spiral.update_ouroboros(signature);
            self.exploration_path.push((coords, signature));
        }

        ExplorationResult {
            steps: self.exploration_path.len(),
            best_resonance: self.spiral.get_best_resonance(),
            num_crystals: self.crystals.len(),
            final_position: self.spiral.current_pos,
        }
    }

    fn attempt_crystallization(&mut self, center: &Point5D, signature: SpectralSignature) {
        if signature.resonance() > 0.7 {
            let crystal = TICCrystal {
                id: format!("TIC_{}", self.crystals.len()),
                center: *center,
                stability: signature.resonance(),
                formation_step: self.exploration_path.len(),
            };
            self.crystals.push(crystal);
        }
    }

    /// Get reference to formed crystals
    pub fn get_crystals(&self) -> &[TICCrystal] {
        &self.crystals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qdash_explorer() {
        let infogenom = Infogenom::new("test".to_string(), 4);
        let mut explorer = QDASHExplorer::new(infogenom, 42, 100);
        let result = explorer.explore();
        assert_eq!(result.steps, 100);
        assert!(result.best_resonance >= 0.0);
    }

    #[test]
    fn test_exploration_deterministic() {
        let infogenom1 = Infogenom::new("test1".to_string(), 4);
        let infogenom2 = Infogenom::new("test2".to_string(), 4);

        let mut explorer1 = QDASHExplorer::new(infogenom1, 42, 50);
        let mut explorer2 = QDASHExplorer::new(infogenom2, 42, 50);

        let result1 = explorer1.explore();
        let result2 = explorer2.explore();

        assert_eq!(result1.steps, result2.steps);
    }
}
