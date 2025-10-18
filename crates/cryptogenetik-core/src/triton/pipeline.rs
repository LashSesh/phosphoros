//! Triton search pipeline

use crate::optimizer::OperatorSet;
use crate::score_hooks::HookSet;
use crate::{Error, Result};
use phosphoros_core::{Evaluation, ResonanceEngine};
use rand::SeedableRng;

/// Search progress callback
pub type ProgressCallback = Box<dyn Fn(usize, f64) + Send + Sync>;

/// Triton search pipeline
pub struct TritonPipeline<'a, E: ResonanceEngine> {
    engine: &'a mut E,
    hooks: HookSet,
    operators: OperatorSet,
    progress_callback: Option<ProgressCallback>,
}

impl<'a, E: ResonanceEngine> TritonPipeline<'a, E> {
    /// Create new pipeline
    pub fn new(engine: &'a mut E, hooks: HookSet, operators: OperatorSet) -> Self {
        Self {
            engine,
            hooks,
            operators,
            progress_callback: None,
        }
    }

    /// Set progress callback
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }

    /// Run search for specified steps
    pub fn search(&mut self, seed_words: &[String], max_steps: usize, seed: u64) -> Result<SearchResult> {
        use rand::Rng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut best_score = 0.0;
        let mut best_vector = [0.0; 5];
        let mut gated_count = 0;
        let mut output_count = 0;

        for step in 0..max_steps {
            // 1. Generate search vector (simplified)
            let v: [f64; 5] = [
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ];

            // 2. Early score via hooks
            let data = format!("{:?}", v).as_bytes().to_vec();
            let early_score = self.hooks.early_score(&data, seed_words);

            // 3. Evaluate with resonance engine
            let t = step as f64 * 0.01;
            let perception = v;
            let intention = v;
            let gradient = v.map(|x| x * 0.1);

            let eval = self.engine.evaluate(t, perception, intention, gradient, 0.5);

            match eval {
                Evaluation::Gated { .. } => {
                    gated_count += 1;
                }
                Evaluation::Output { vector, score } => {
                    output_count += 1;

                    // 4. Apply operators
                    if let Some(transformed) = self.operators.apply(vector, score, gradient) {
                        let combined_score = (early_score + score) / 2.0;

                        if combined_score > best_score {
                            best_score = combined_score;
                            best_vector = transformed;
                        }
                    }
                }
            }

            // Progress callback
            if let Some(ref callback) = self.progress_callback {
                if step % 100 == 0 {
                    callback(step, best_score);
                }
            }
        }

        Ok(SearchResult {
            best_score,
            best_vector,
            steps_taken: max_steps,
            gated_count,
            output_count,
        })
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Best score found
    pub best_score: f64,
    /// Best vector found
    pub best_vector: [f64; 5],
    /// Total steps taken
    pub steps_taken: usize,
    /// Number of gated evaluations
    pub gated_count: usize,
    /// Number of output evaluations
    pub output_count: usize,
}

impl SearchResult {
    /// Calculate pruning factor (ratio of gated to total)
    pub fn pruning_factor(&self) -> f64 {
        self.gated_count as f64 / self.steps_taken as f64
    }

    /// Calculate early exit rate
    pub fn early_exit_rate(&self) -> f64 {
        self.gated_count as f64 / (self.gated_count + self.output_count) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phosphoros_core::HolisticMatrix;

    #[test]
    fn test_pipeline_search() {
        let mut engine = HolisticMatrix::default_config();
        let hooks = HookSet::new();
        let operators = OperatorSet::new();
        let mut pipeline = TritonPipeline::new(&mut engine, hooks, operators);

        let words = vec!["test".to_string()];
        let result = pipeline.search(&words, 100, 12345);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.steps_taken, 100);
    }
}
