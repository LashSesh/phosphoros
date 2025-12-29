//! PhosphorosCore bridge - Main API

use crate::optimizer::OperatorSet;
use crate::score_hooks::HookSet;
use crate::triton::{ProgressCallback, TritonPipeline};
use crate::Result;
use phosphoros_core::ResonanceEngine;

/// PhosphorosCore - Main bridge integrating resonance engine with search
pub struct PhosphorosCore<E: ResonanceEngine> {
    engine: E,
    hooks: HookSet,
    operators: OperatorSet,
}

impl<E: ResonanceEngine> PhosphorosCore<E> {
    /// Create a new PhosphorosCore with custom engine
    pub fn new(engine: E, hooks: HookSet, operators: OperatorSet) -> Self {
        Self {
            engine,
            hooks,
            operators,
        }
    }

    /// Create with default configuration
    pub fn default_config(engine: E) -> Self {
        Self {
            engine,
            hooks: HookSet::all(),
            operators: OperatorSet::all_default(),
        }
    }

    /// Explore keyspace with seed words
    pub fn explore(&mut self, seed_words: Vec<String>, steps: usize, seed: u64) -> Result<Outcome> {
        self.explore_with_callback(seed_words, steps, seed, None)
    }

    /// Explore with progress callback
    pub fn explore_with_callback(
        &mut self,
        seed_words: Vec<String>,
        steps: usize,
        seed: u64,
        callback: Option<ProgressCallback>,
    ) -> Result<Outcome> {
        let mut pipeline =
            TritonPipeline::new(&mut self.engine, self.hooks.clone(), self.operators.clone());

        if let Some(cb) = callback {
            pipeline.set_progress_callback(cb);
        }

        let result = pipeline.search(&seed_words, steps, seed)?;

        Ok(Outcome {
            best_resonance: result.best_score,
            best_vector: result.best_vector,
            steps: result.steps_taken,
            pruning_factor: result.pruning_factor(),
            early_exit_rate: result.early_exit_rate(),
            topk: vec![], // Would be populated in full implementation
        })
    }

    /// Get the current engine state
    pub fn engine_state(&self) -> &E::State {
        self.engine.state()
    }
}

/// Exploration outcome
#[derive(Debug, Clone)]
pub struct Outcome {
    /// Best resonance score found
    pub best_resonance: f64,
    /// Best 5D vector found
    pub best_vector: [f64; 5],
    /// Steps taken
    pub steps: usize,
    /// Pruning factor (gated ratio)
    pub pruning_factor: f64,
    /// Early exit rate
    pub early_exit_rate: f64,
    /// Top-k results
    pub topk: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use phosphoros_core::HolisticMatrix;

    #[test]
    fn test_phosphoros_core_creation() {
        let engine = HolisticMatrix::default_config();
        let hooks = HookSet::all();
        let operators = OperatorSet::all_default();
        let _core = PhosphorosCore::new(engine, hooks, operators);
    }

    #[test]
    fn test_explore() {
        let engine = HolisticMatrix::default_config();
        let mut core = PhosphorosCore::default_config(engine);

        let words = vec!["test".to_string(), "words".to_string()];
        let result = core.explore(words, 100, 42);
        assert!(result.is_ok());
    }
}
