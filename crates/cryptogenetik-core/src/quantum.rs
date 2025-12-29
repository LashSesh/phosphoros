//! Quantum-Enhanced Search
//!
//! Integrates Grover's algorithm for √N speedup in search operations.
//!
//! # Features
//!
//! Enable with the `quantum` feature flag:
//! ```toml
//! [dependencies]
//! cryptogenetik-core = { path = "...", features = ["quantum"] }
//! ```

#[cfg(feature = "quantum")]
use phosphoros_quantum::algorithms::grover::{GroverSearch, GroverOracle, GroverResult};
#[cfg(feature = "quantum")]
use phosphoros_quantum::backend::simulator::LocalSimulator;
#[cfg(feature = "quantum")]
use phosphoros_quantum::backend::BackendError;

/// Quantum search mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumSearchMode {
    /// Classical search only
    Classical,
    /// Quantum-accelerated search using Grover
    GroverAccelerated,
    /// Hybrid: use quantum for pruning, classical for evaluation
    Hybrid,
}

impl Default for QuantumSearchMode {
    fn default() -> Self {
        #[cfg(feature = "quantum")]
        return Self::Hybrid;
        #[cfg(not(feature = "quantum"))]
        return Self::Classical;
    }
}

/// Quantum-enhanced search configuration
#[cfg(feature = "quantum")]
pub struct QuantumSearchConfig {
    /// Search mode
    pub mode: QuantumSearchMode,
    /// Number of qubits for search space (2^n candidates)
    pub search_qubits: usize,
    /// Grover iteration count (optimal: π/4 * √N)
    pub grover_iterations: Option<usize>,
    /// Threshold for target identification
    pub target_threshold: f64,
}

#[cfg(feature = "quantum")]
impl Default for QuantumSearchConfig {
    fn default() -> Self {
        Self {
            mode: QuantumSearchMode::Hybrid,
            search_qubits: 8, // 256 candidates per quantum batch
            grover_iterations: None, // Auto-calculate
            target_threshold: 0.7,
        }
    }
}

#[cfg(feature = "quantum")]
impl QuantumSearchConfig {
    /// Create new config with specified qubits
    pub fn new(qubits: usize) -> Self {
        Self {
            search_qubits: qubits,
            ..Default::default()
        }
    }

    /// Set the search mode
    pub fn with_mode(mut self, mode: QuantumSearchMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set target threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.target_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Calculate optimal Grover iterations for N candidates
    pub fn optimal_iterations(n: usize) -> usize {
        // π/4 * √N
        ((std::f64::consts::PI / 4.0) * (n as f64).sqrt()).round() as usize
    }
}

/// Oracle that marks candidates above a score threshold
#[cfg(feature = "quantum")]
pub struct ScoreThresholdOracle<F>
where
    F: Fn(usize) -> f64 + Send + Sync,
{
    score_fn: F,
    threshold: f64,
    n: usize,
    cached_marked: Vec<usize>,
}

#[cfg(feature = "quantum")]
impl<F> ScoreThresholdOracle<F>
where
    F: Fn(usize) -> f64 + Send + Sync,
{
    /// Create new score threshold oracle
    pub fn new(n: usize, threshold: f64, score_fn: F) -> Self {
        // Pre-compute marked states
        let cached_marked: Vec<usize> = (0..n)
            .filter(|&idx| score_fn(idx) >= threshold)
            .collect();

        Self {
            score_fn,
            threshold,
            n,
            cached_marked,
        }
    }
}

#[cfg(feature = "quantum")]
impl<F> GroverOracle for ScoreThresholdOracle<F>
where
    F: Fn(usize) -> f64 + Send + Sync,
{
    fn is_marked(&self, state: usize) -> bool {
        (self.score_fn)(state) >= self.threshold
    }

    fn marked_states(&self) -> Vec<usize> {
        self.cached_marked.clone()
    }

    fn search_space_size(&self) -> usize {
        self.n
    }
}

/// Quantum-enhanced search engine
#[cfg(feature = "quantum")]
pub struct QuantumSearchEngine {
    config: QuantumSearchConfig,
}

#[cfg(feature = "quantum")]
impl QuantumSearchEngine {
    /// Create new quantum search engine
    pub fn new(config: QuantumSearchConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default_engine() -> Self {
        Self::new(QuantumSearchConfig::default())
    }

    /// Run Grover search to find candidates above threshold
    pub fn grover_search<F>(&self, search_space: usize, score_fn: F) -> Result<QuantumSearchResult, BackendError>
    where
        F: Fn(usize) -> f64 + Send + Sync,
    {
        let n = search_space.min(1 << self.config.search_qubits);
        let backend = LocalSimulator::new(self.config.search_qubits);

        let oracle = ScoreThresholdOracle::new(n, self.config.target_threshold, score_fn);

        let iterations = self.config.grover_iterations
            .unwrap_or_else(|| QuantumSearchConfig::optimal_iterations(n));

        let grover = GroverSearch::new(backend, self.config.search_qubits);
        let result = grover.search(&oracle, iterations)?;

        // Solutions from Grover result
        let candidates: Vec<usize> = result.solutions.clone();

        // Calculate amplification from probabilities
        let amplification = if !result.probabilities.is_empty() {
            result.probabilities[0] * (n as f64)
        } else {
            1.0
        };

        Ok(QuantumSearchResult {
            candidates,
            quantum_amplification: amplification,
            grover_result: Some(result),
            speedup_factor: (n as f64).sqrt(),
        })
    }

    /// Hybrid search: use quantum pruning with classical evaluation
    pub fn hybrid_search<F, E>(
        &self,
        search_space: usize,
        quantum_score_fn: F,
        classical_eval: E,
    ) -> Result<HybridSearchResult, BackendError>
    where
        F: Fn(usize) -> f64 + Send + Sync,
        E: Fn(usize) -> f64,
    {
        // Step 1: Quantum pruning
        let quantum_result = self.grover_search(search_space, quantum_score_fn)?;

        // Step 2: Classical evaluation on quantum candidates
        let mut evaluated: Vec<(usize, f64)> = quantum_result
            .candidates
            .iter()
            .map(|&idx| (idx, classical_eval(idx)))
            .collect();

        // Sort by score
        evaluated.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let best_index = evaluated.first().map(|(i, _)| *i).unwrap_or(0);
        let best_score = evaluated.first().map(|(_, s)| *s).unwrap_or(0.0);

        Ok(HybridSearchResult {
            best_index,
            best_score,
            candidates_evaluated: evaluated.len(),
            quantum_pruning_factor: 1.0 - (evaluated.len() as f64 / search_space as f64),
            quantum_result,
            evaluated_scores: evaluated,
        })
    }
}

/// Result of quantum search
#[cfg(feature = "quantum")]
#[derive(Debug, Clone)]
pub struct QuantumSearchResult {
    /// High-probability candidate indices
    pub candidates: Vec<usize>,
    /// Quantum amplification achieved
    pub quantum_amplification: f64,
    /// Full Grover result
    pub grover_result: Option<GroverResult>,
    /// Theoretical speedup factor
    pub speedup_factor: f64,
}

/// Result of hybrid search
#[cfg(feature = "quantum")]
#[derive(Debug, Clone)]
pub struct HybridSearchResult {
    /// Best candidate index
    pub best_index: usize,
    /// Best score found
    pub best_score: f64,
    /// Number of candidates evaluated classically
    pub candidates_evaluated: usize,
    /// Fraction of search space pruned by quantum
    pub quantum_pruning_factor: f64,
    /// Quantum search result
    pub quantum_result: QuantumSearchResult,
    /// All evaluated scores
    pub evaluated_scores: Vec<(usize, f64)>,
}

#[cfg(feature = "quantum")]
impl HybridSearchResult {
    /// Calculate effective speedup
    pub fn effective_speedup(&self, total_space: usize) -> f64 {
        total_space as f64 / self.candidates_evaluated.max(1) as f64
    }
}

// Stub implementations when quantum feature is disabled

/// Quantum search config (stub)
#[cfg(not(feature = "quantum"))]
pub struct QuantumSearchConfig;

#[cfg(not(feature = "quantum"))]
impl Default for QuantumSearchConfig {
    fn default() -> Self {
        Self
    }
}

/// Quantum search engine (stub)
#[cfg(not(feature = "quantum"))]
pub struct QuantumSearchEngine;

#[cfg(not(feature = "quantum"))]
impl QuantumSearchEngine {
    /// Create new engine (quantum disabled)
    pub fn new(_config: QuantumSearchConfig) -> Self {
        Self
    }

    /// Create default engine (quantum disabled)
    pub fn default_engine() -> Self {
        Self
    }
}

#[cfg(all(test, feature = "quantum"))]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_search_config() {
        let config = QuantumSearchConfig::new(6)
            .with_mode(QuantumSearchMode::GroverAccelerated)
            .with_threshold(0.8);

        assert_eq!(config.search_qubits, 6);
        assert_eq!(config.mode, QuantumSearchMode::GroverAccelerated);
        assert!((config.target_threshold - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_optimal_iterations() {
        // For N=256, optimal is ~12
        let iters = QuantumSearchConfig::optimal_iterations(256);
        assert!(iters >= 10 && iters <= 14);

        // For N=1024, optimal is ~25
        let iters = QuantumSearchConfig::optimal_iterations(1024);
        assert!(iters >= 23 && iters <= 27);
    }

    #[test]
    fn test_grover_search() {
        let engine = QuantumSearchEngine::new(
            QuantumSearchConfig::new(4).with_threshold(0.5)
        );

        // Score function: mark indices divisible by 4 as high-scoring
        let score_fn = |idx: usize| {
            if idx % 4 == 0 { 0.8 } else { 0.2 }
        };

        let result = engine.grover_search(16, score_fn);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.speedup_factor >= 3.0); // √16 = 4
    }

    #[test]
    fn test_hybrid_search() {
        let engine = QuantumSearchEngine::new(
            QuantumSearchConfig::new(4).with_threshold(0.4)
        );

        // Quantum score: approximate indicator
        let quantum_fn = |idx: usize| {
            if idx < 8 { 0.6 } else { 0.2 }
        };

        // Classical score: precise evaluation
        let classical_fn = |idx: usize| {
            1.0 / (1.0 + (idx as f64 - 3.0).abs())
        };

        let result = engine.hybrid_search(16, quantum_fn, classical_fn);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.best_score > 0.0);
        assert!(result.quantum_pruning_factor >= 0.0);
    }
}
