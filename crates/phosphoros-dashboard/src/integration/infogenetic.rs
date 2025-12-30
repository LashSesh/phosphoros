//! InfoGenetic integration module
//!
//! Bridges the suite5d InfoGenetics DNA visualization with
//! the phosphoros-kryptogenetik Infogenom network analysis.

use crate::suite5d::{InfoGenetics, DnaStrandSet};
use phosphoros_kryptogenetik::{Infogenom, GabrielCell, SpectralSignature, Point5D};
use thiserror::Error;
use std::collections::HashMap;

/// InfoGenetic integration error
#[derive(Debug, Error)]
pub enum InfoGeneticError {
    /// Analysis failed
    #[error("Analysis error: {0}")]
    AnalysisError(String),

    /// No data available
    #[error("No data available for analysis")]
    NoData,

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Result of InfoGenetic analysis
#[derive(Debug, Clone)]
pub struct InfoGeneticResult {
    /// DNA strand visualization data
    pub dna_strands: DnaStrandSet,

    /// Spectral signature (ψ, ρ, ω)
    pub signature: SpectralSignature,

    /// Total resonance score
    pub resonance: f64,

    /// 3D helix coordinates for visualization
    pub helix_coords: Vec<Vec<(f64, f64, f64)>>,

    /// Gabriel Cell network state
    pub cell_states: Vec<CellState>,
}

/// State of a single Gabriel Cell
#[derive(Debug, Clone)]
pub struct CellState {
    /// Cell identifier
    pub id: String,
    /// Psi component
    pub psi: f64,
    /// Rho component
    pub rho: f64,
    /// Omega component
    pub omega: f64,
    /// Current output
    pub output: f64,
}

/// InfoGenetic integration service
pub struct InfoGeneticIntegration {
    /// Core InfoGenetics for DNA analysis
    info_genetics: InfoGenetics,

    /// Infogenom network (coupled Gabriel Cells)
    infogenom: Option<Infogenom>,

    /// Analysis cache
    cache: HashMap<String, InfoGeneticResult>,
}

impl Default for InfoGeneticIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoGeneticIntegration {
    /// Create a new InfoGenetic integration instance
    pub fn new() -> Self {
        Self {
            info_genetics: InfoGenetics::new(),
            infogenom: None,
            cache: HashMap::new(),
        }
    }

    /// Initialize the Infogenom network with specified number of cells
    pub fn init_infogenom(&mut self, id: &str, num_cells: usize) {
        self.infogenom = Some(Infogenom::new(id.to_string(), num_cells));
    }

    /// Analyze a seed phrase and generate InfoGenetic data
    pub fn analyze_seed(&mut self, seed: &str) -> Result<InfoGeneticResult, InfoGeneticError> {
        if seed.is_empty() {
            return Err(InfoGeneticError::InvalidInput("Empty seed".to_string()));
        }

        // Check cache first
        if let Some(cached) = self.cache.get(seed) {
            return Ok(cached.clone());
        }

        // Analyze with InfoGenetics (creates DNA strands)
        self.info_genetics.analyze_seed(seed);

        let genome = self.info_genetics.genome();
        if genome.is_empty() {
            return Err(InfoGeneticError::NoData);
        }

        // Get the latest DNA strand set
        let dna_strands = genome.last().unwrap().clone();

        // Generate helix coordinates for visualization
        let helix_coords = dna_strands.helical_coordinates();

        // Create 5D embedding from DNA data
        let embedding = self.create_5d_embedding(&dna_strands);

        // Analyze with Infogenom if available
        let (signature, resonance, cell_states) = if let Some(ref mut infogenom) = self.infogenom {
            let scores = infogenom.evaluate(&embedding);
            let total_resonance = infogenom.total_resonance(&embedding);

            // Extract cell states (limited access through public interface)
            let cells: Vec<CellState> = scores.iter().enumerate().map(|(i, &score)| {
                CellState {
                    id: format!("cell_{}", i),
                    psi: score.max(0.0).min(1.0),
                    rho: (score * 0.9).max(0.0).min(1.0),
                    omega: (score * 0.8).max(0.0).min(1.0),
                    output: score,
                }
            }).collect();

            let sig = SpectralSignature::new(
                scores.get(0).copied().unwrap_or(0.5),
                scores.get(1).copied().unwrap_or(0.5),
                scores.get(2).copied().unwrap_or(0.5),
            );

            (sig, total_resonance, cells)
        } else {
            // Fallback: compute signature from DNA strands
            let avg_value: f64 = dna_strands.strands.iter()
                .flat_map(|s| s.iter())
                .sum::<f64>() / dna_strands.strands.iter().map(|s| s.len()).sum::<usize>().max(1) as f64;

            let sig = SpectralSignature::new(avg_value, avg_value * 0.9, avg_value * 0.8);
            let resonance = sig.resonance();

            (sig, resonance, vec![])
        };

        let result = InfoGeneticResult {
            dna_strands,
            signature,
            resonance,
            helix_coords,
            cell_states,
        };

        // Cache the result
        self.cache.insert(seed.to_string(), result.clone());

        Ok(result)
    }

    /// Analyze multiple seeds in batch
    pub fn batch_analyze(&mut self, seeds: &[&str]) -> Vec<Result<InfoGeneticResult, InfoGeneticError>> {
        seeds.iter().map(|seed| self.analyze_seed(seed)).collect()
    }

    /// Create a 5D embedding from DNA strand data
    fn create_5d_embedding(&self, dna_strands: &DnaStrandSet) -> Point5D {
        let strands = &dna_strands.strands;

        if strands.is_empty() {
            return Point5D::zero();
        }

        // Calculate 5D coordinates from strand statistics
        let total_values: usize = strands.iter().map(|s| s.len()).sum();
        if total_values == 0 {
            return Point5D::zero();
        }

        let all_values: Vec<f64> = strands.iter().flat_map(|s| s.iter().copied()).collect();

        // Dimension 1: Mean value
        let mean = all_values.iter().sum::<f64>() / all_values.len() as f64;

        // Dimension 2: Standard deviation
        let variance = all_values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / all_values.len() as f64;
        let std_dev = variance.sqrt();

        // Dimension 3: Skewness (asymmetry)
        let skewness = if std_dev > 1e-10 {
            all_values.iter()
                .map(|v| ((v - mean) / std_dev).powi(3))
                .sum::<f64>() / all_values.len() as f64
        } else {
            0.0
        };

        // Dimension 4: Kurtosis (tail weight)
        let kurtosis = if std_dev > 1e-10 {
            all_values.iter()
                .map(|v| ((v - mean) / std_dev).powi(4))
                .sum::<f64>() / all_values.len() as f64 - 3.0
        } else {
            0.0
        };

        // Dimension 5: Entropy approximation
        let entropy = self.calculate_entropy(&all_values);

        Point5D::new(
            mean.clamp(0.0, 1.0),
            std_dev.clamp(0.0, 1.0),
            (skewness / 3.0 + 0.5).clamp(0.0, 1.0),  // Normalize skewness
            (kurtosis / 10.0 + 0.5).clamp(0.0, 1.0), // Normalize kurtosis
            entropy.clamp(0.0, 1.0),
        )
    }

    /// Calculate Shannon entropy approximation
    fn calculate_entropy(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        // Bin values into 10 buckets
        let mut bins = [0usize; 10];
        for &v in values {
            let idx = ((v * 9.99).floor() as usize).min(9);
            bins[idx] += 1;
        }

        let total = values.len() as f64;
        let mut entropy = 0.0;

        for &count in &bins {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }

        // Normalize to [0, 1] (max entropy for 10 bins is log2(10) ≈ 3.32)
        entropy / 3.32
    }

    /// Get spectral signature for a given seed
    pub fn get_signature(&mut self, seed: &str) -> Result<SpectralSignature, InfoGeneticError> {
        let result = self.analyze_seed(seed)?;
        Ok(result.signature)
    }

    /// Get resonance score for a given seed
    pub fn get_resonance(&mut self, seed: &str) -> Result<f64, InfoGeneticError> {
        let result = self.analyze_seed(seed)?;
        Ok(result.resonance)
    }

    /// Search by spectral signature range
    pub fn search_by_signature(
        &self,
        min_psi: f64,
        max_psi: f64,
        min_rho: f64,
        max_rho: f64,
        min_omega: f64,
        max_omega: f64,
    ) -> Vec<(String, InfoGeneticResult)> {
        self.cache.iter()
            .filter(|(_, result)| {
                let sig = &result.signature;
                sig.psi >= min_psi && sig.psi <= max_psi &&
                sig.rho >= min_rho && sig.rho <= max_rho &&
                sig.omega >= min_omega && sig.omega <= max_omega
            })
            .map(|(seed, result)| (seed.clone(), result.clone()))
            .collect()
    }

    /// Search by minimum resonance threshold
    pub fn search_by_resonance(&self, min_resonance: f64) -> Vec<(String, InfoGeneticResult)> {
        self.cache.iter()
            .filter(|(_, result)| result.resonance >= min_resonance)
            .map(|(seed, result)| (seed.clone(), result.clone()))
            .collect()
    }

    /// Clear the analysis cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infogenetic_integration_new() {
        let integration = InfoGeneticIntegration::new();
        assert_eq!(integration.cache_size(), 0);
    }

    #[test]
    fn test_analyze_seed() {
        let mut integration = InfoGeneticIntegration::new();
        integration.init_infogenom("test", 4);

        let result = integration.analyze_seed("test_seed_phrase");
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(!result.dna_strands.strands.is_empty());
        assert!(!result.helix_coords.is_empty());
        assert!(result.resonance >= 0.0);
    }

    #[test]
    fn test_cache_behavior() {
        let mut integration = InfoGeneticIntegration::new();

        let _ = integration.analyze_seed("cached_seed");
        assert_eq!(integration.cache_size(), 1);

        // Second call should use cache
        let _ = integration.analyze_seed("cached_seed");
        assert_eq!(integration.cache_size(), 1);

        integration.clear_cache();
        assert_eq!(integration.cache_size(), 0);
    }

    #[test]
    fn test_empty_seed_error() {
        let mut integration = InfoGeneticIntegration::new();
        let result = integration.analyze_seed("");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_by_resonance() {
        let mut integration = InfoGeneticIntegration::new();

        let _ = integration.analyze_seed("seed1");
        let _ = integration.analyze_seed("seed2");
        let _ = integration.analyze_seed("seed3");

        let results = integration.search_by_resonance(0.0);
        assert!(!results.is_empty());
    }
}
