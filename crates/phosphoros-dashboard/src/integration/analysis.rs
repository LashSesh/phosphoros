//! Analysis integration using cryptogenetik-core
//!
//! Provides search and optimization with topological operators.

use cryptogenetik_core::{PhosphorosCore, OperatorSet, HookSet};
use phosphoros_core::HolisticMatrix;
use thiserror::Error;

/// Analysis integration error
#[derive(Debug, Error)]
pub enum AnalysisError {
    /// Search error
    #[error("Search error: {0}")]
    SearchError(String),
}

/// Analysis result
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Best resonance found
    pub best_resonance: f64,
    /// Pruning factor
    pub pruning_factor: f64,
    /// Steps taken
    pub steps: usize,
}

/// Analysis integration
pub struct AnalysisIntegration {
    core: PhosphorosCore<HolisticMatrix>,
}

impl Default for AnalysisIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisIntegration {
    /// Create a new analysis integration
    pub fn new() -> Self {
        let engine = HolisticMatrix::default_config();
        let hooks = HookSet::default();
        let operators = OperatorSet::all_default();
        let core = PhosphorosCore::new(engine, hooks, operators);
        
        Self { core }
    }
    
    /// Create with custom operator configuration
    pub fn with_operators(operators: OperatorSet) -> Self {
        let engine = HolisticMatrix::default_config();
        let hooks = HookSet::default();
        let core = PhosphorosCore::new(engine, hooks, operators);
        
        Self { core }
    }
    
    /// Explore seed space
    pub fn explore(&mut self, seed_words: Vec<String>, max_steps: usize) -> Result<AnalysisResult, AnalysisError> {
        // Use a deterministic seed for reproducibility
        let seed = 42u64;
        
        let result = self.core.explore(seed_words, max_steps, seed)
            .map_err(|e| AnalysisError::SearchError(e.to_string()))?;
        
        Ok(AnalysisResult {
            best_resonance: result.best_resonance,
            pruning_factor: result.pruning_factor,
            steps: result.steps,
        })
    }
    
    /// Configure operators
    pub fn configure_operators(&mut self, _wt: f64, _sw: f64, _dk: f64, _pi: f64) {
        // For now, just recreate with default operators
        // In the future, we can create custom operator instances
        let engine = HolisticMatrix::default_config();
        let hooks = HookSet::default();
        let operators = OperatorSet::all_default();
        self.core = PhosphorosCore::new(engine, hooks, operators);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_explore() {
        let mut integration = AnalysisIntegration::new();
        let words = vec!["test".to_string(), "seed".to_string()];
        
        let result = integration.explore(words, 100);
        assert!(result.is_ok());
        
        let res = result.unwrap();
        assert!(res.best_resonance >= 0.0);
        assert!(res.steps > 0);
    }
}
