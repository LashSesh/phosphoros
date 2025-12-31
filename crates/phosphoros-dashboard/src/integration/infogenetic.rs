//! InfoGenetic integration module
//!
//! Bridges the suite5d InfoGenetics DNA visualization with
//! the phosphoros-kryptogenetik Infogenom network analysis.
//!
//! Now enhanced with:
//! - 5D Spiral topology from Klemm's axiomatic framework
//! - MEF Ledger persistent storage with hash-chained blocks
//! - Advanced query mode with spiral coordinate search
//! - Gabriel Cell network coupling with resonance invariant D = ψ·ρ·ω

use crate::suite5d::{InfoGenetics, DnaStrandSet};
use crate::integration::spiral_topology::{
    State5D, SpiralPath, SpiralParams, SpiralType,
    ResonanceFields, SpiralDynamics, SpiralCoordinates,
};
use crate::integration::mef_ledger::{
    InfogeneticLedger, CompactInfogeneticEntry, CompactSignature,
    CompactCellState, StrandStatistics, LedgerError,
};
use phosphoros_kryptogenetik::{Infogenom, GabrielCell, SpectralSignature, Point5D};
use thiserror::Error;
use std::collections::HashMap;
use std::path::Path;
use sha2::{Sha256, Digest};

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

    /// Ledger/storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Spiral topology error
    #[error("Spiral computation error: {0}")]
    SpiralError(String),

    /// Query error
    #[error("Query error: {0}")]
    QueryError(String),
}

impl From<LedgerError> for InfoGeneticError {
    fn from(e: LedgerError) -> Self {
        InfoGeneticError::StorageError(e.to_string())
    }
}

/// Advanced query type for infogenetic database
#[derive(Debug, Clone)]
pub enum AdvancedQuery {
    /// Search by 5D spiral coordinates within epsilon ball
    SpiralProximity {
        center: State5D,
        epsilon: f64,
    },
    /// Search by resonance distance (d_R metric)
    ResonanceDistance {
        reference: SpectralSignature,
        max_distance: f64,
        weights: (f64, f64, f64), // (w_ψ, w_ρ, w_ω)
    },
    /// Search along spiral path trajectory
    SpiralTrajectory {
        params: SpiralParams,
        t_min: f64,
        t_max: f64,
        tolerance: f64,
    },
    /// Combined multi-criteria search
    Composite {
        queries: Vec<AdvancedQuery>,
        operator: QueryOperator,
    },
    /// Search by cluster membership with resonance threshold
    ClusterResonance {
        cluster_id: Option<String>,
        min_resonance: f64,
        max_resonance: f64,
    },
}

/// Query combination operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOperator {
    And,
    Or,
}

/// Result of an advanced query
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Entry ID
    pub entry_id: String,
    /// Match score (0.0 to 1.0)
    pub score: f64,
    /// 5D state representation
    pub state: State5D,
    /// Spectral signature
    pub signature: SpectralSignature,
    /// Resonance value D = ψ·ρ·ω
    pub resonance: f64,
    /// Spiral position (parameter t)
    pub spiral_position: f64,
}

/// Result of InfoGenetic analysis
#[derive(Debug, Clone)]
pub struct InfoGeneticResult {
    /// DNA strand visualization data
    pub dna_strands: DnaStrandSet,

    /// Spectral signature (ψ, ρ, ω)
    pub signature: SpectralSignature,

    /// Total resonance score (D = ψ·ρ·ω invariant)
    pub resonance: f64,

    /// 3D helix coordinates for visualization
    pub helix_coords: Vec<Vec<(f64, f64, f64)>>,

    /// Gabriel Cell network state
    pub cell_states: Vec<CellState>,

    /// 5D state representation (from spiral topology)
    pub state_5d: State5D,

    /// Spiral position parameter t
    pub spiral_position: f64,

    /// Entry ID for persistent storage
    pub entry_id: Option<String>,
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
///
/// Provides unified access to:
/// - DNA strand analysis and helix visualization
/// - Gabriel Cell network with coupled resonators
/// - 5D spiral topology for state space navigation
/// - Persistent storage via MEF Ledger
/// - Advanced query capabilities
pub struct InfoGeneticIntegration {
    /// Core InfoGenetics for DNA analysis
    info_genetics: InfoGenetics,

    /// Infogenom network (coupled Gabriel Cells)
    infogenom: Option<Infogenom>,

    /// Analysis cache
    cache: HashMap<String, InfoGeneticResult>,

    /// Persistent ledger storage (optional)
    ledger: Option<InfogeneticLedger>,

    /// 5D spiral dynamics for state evolution
    spiral_dynamics: SpiralDynamics,

    /// Resonance fields Ψ = {ψ, ρ, ω}
    resonance_fields: ResonanceFields,

    /// Spiral path for coordinate mapping
    spiral_path: SpiralPath,

    /// Spiral coordinate system for indexing
    spiral_coords: SpiralCoordinates,

    /// Current spiral position counter
    spiral_position_counter: f64,
}

impl Default for InfoGeneticIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoGeneticIntegration {
    /// Create a new InfoGenetic integration instance
    pub fn new() -> Self {
        let spiral_params = SpiralParams::default();
        Self {
            info_genetics: InfoGenetics::new(),
            infogenom: None,
            cache: HashMap::new(),
            ledger: None,
            spiral_dynamics: SpiralDynamics::default(),
            resonance_fields: ResonanceFields::default(),
            spiral_path: SpiralPath::new(spiral_params),
            spiral_coords: SpiralCoordinates::new(spiral_params),
            spiral_position_counter: 0.0,
        }
    }

    /// Create with persistent storage enabled
    pub fn with_storage(storage_path: impl AsRef<Path>) -> Result<Self, InfoGeneticError> {
        let ledger = InfogeneticLedger::new(storage_path)?;
        let spiral_params = SpiralParams::default();
        Ok(Self {
            info_genetics: InfoGenetics::new(),
            infogenom: None,
            cache: HashMap::new(),
            ledger: Some(ledger),
            spiral_dynamics: SpiralDynamics::default(),
            resonance_fields: ResonanceFields::default(),
            spiral_path: SpiralPath::new(spiral_params),
            spiral_coords: SpiralCoordinates::new(spiral_params),
            spiral_position_counter: 0.0,
        })
    }

    /// Initialize the Infogenom network with specified number of cells
    pub fn init_infogenom(&mut self, id: &str, num_cells: usize) {
        self.infogenom = Some(Infogenom::new(id.to_string(), num_cells));
    }

    /// Enable persistent storage at the given path
    pub fn enable_storage(&mut self, storage_path: impl AsRef<Path>) -> Result<(), InfoGeneticError> {
        self.ledger = Some(InfogeneticLedger::new(storage_path)?);
        Ok(())
    }

    /// Configure spiral dynamics parameters
    pub fn configure_spiral(&mut self, params: SpiralParams, alpha: f64) {
        self.spiral_path = SpiralPath::new(params);
        self.spiral_coords = SpiralCoordinates::new(params);
        self.spiral_dynamics.alpha = alpha;
    }

    /// Configure resonance field weights
    pub fn configure_resonance_weights(&mut self, w_psi: f64, w_rho: f64, w_omega: f64) {
        self.resonance_fields = ResonanceFields::with_weights(w_psi, w_rho, w_omega);
    }

    /// Analyze a seed phrase and generate InfoGenetic data
    pub fn analyze_seed(&mut self, seed: &str) -> Result<InfoGeneticResult, InfoGeneticError> {
        self.analyze_seed_internal(seed, true)
    }

    /// Analyze without auto-persisting
    pub fn analyze_seed_transient(&mut self, seed: &str) -> Result<InfoGeneticResult, InfoGeneticError> {
        self.analyze_seed_internal(seed, false)
    }

    /// Internal analysis implementation
    fn analyze_seed_internal(&mut self, seed: &str, persist: bool) -> Result<InfoGeneticResult, InfoGeneticError> {
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

        // Create 5D state from embedding
        let state_5d = State5D::new(
            embedding.coords[0],
            embedding.coords[1],
            embedding.coords[2],
            embedding.coords[3],
            embedding.coords[4],
        );

        // Advance spiral position
        self.spiral_position_counter += 0.1;
        let spiral_position = self.spiral_position_counter;

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

        // Generate entry ID from seed hash
        let entry_id = self.generate_entry_id(seed);

        // Persist to ledger if enabled - create entry before borrowing ledger
        if persist && self.ledger.is_some() {
            let compact_entry = self.create_compact_entry(
                &entry_id, seed, &embedding, &signature, &dna_strands, &cell_states,
            );
            if let Some(ref mut ledger) = self.ledger {
                let _ = ledger.append_entry(compact_entry);
            }
        }

        let result = InfoGeneticResult {
            dna_strands,
            signature,
            resonance,
            helix_coords,
            cell_states,
            state_5d,
            spiral_position,
            entry_id: Some(entry_id),
        };

        // Cache the result
        self.cache.insert(seed.to_string(), result.clone());

        Ok(result)
    }

    /// Generate entry ID from seed (hashed for privacy)
    fn generate_entry_id(&self, seed: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        hasher.update(b"_infogenetic_");
        let hash = hasher.finalize();
        format!("ig_{:x}", &hash[..8].iter().fold(0u64, |acc, &b| acc * 256 + b as u64))
    }

    /// Create compact entry for ledger storage
    fn create_compact_entry(
        &self,
        entry_id: &str,
        seed: &str,
        embedding: &Point5D,
        signature: &SpectralSignature,
        dna_strands: &DnaStrandSet,
        cell_states: &[CellState],
    ) -> CompactInfogeneticEntry {
        // Hash seed for privacy
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let seed_hash = format!("{:x}", hasher.finalize());

        // Compute strand statistics
        let all_values: Vec<f64> = dna_strands.strands.iter()
            .flat_map(|s| s.iter().copied())
            .collect();
        let strand_stats = self.compute_strand_statistics(&all_values, dna_strands.strands.len());

        // Convert cell states
        let compact_cells: Vec<CompactCellState> = cell_states.iter().enumerate().map(|(i, cs)| {
            CompactCellState {
                idx: i,
                psi: cs.psi,
                rho: cs.rho,
                omega: cs.omega,
                output: cs.output,
            }
        }).collect();

        CompactInfogeneticEntry {
            entry_id: entry_id.to_string(),
            seed_hash,
            embedding: embedding.coords,
            signature: CompactSignature::new(signature.psi, signature.rho, signature.omega),
            strand_stats,
            cell_states: compact_cells,
            address_hashes: Vec::new(),
            cluster_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Compute strand statistics for storage
    fn compute_strand_statistics(&self, values: &[f64], num_strands: usize) -> StrandStatistics {
        if values.is_empty() {
            return StrandStatistics {
                num_strands,
                mean: 0.0,
                std_dev: 0.0,
                skewness: 0.0,
                kurtosis: 0.0,
                entropy: 0.0,
            };
        }

        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();

        let skewness = if std_dev > 1e-10 {
            values.iter().map(|v| ((v - mean) / std_dev).powi(3)).sum::<f64>() / n
        } else {
            0.0
        };

        let kurtosis = if std_dev > 1e-10 {
            values.iter().map(|v| ((v - mean) / std_dev).powi(4)).sum::<f64>() / n - 3.0
        } else {
            0.0
        };

        let entropy = self.calculate_entropy(values);

        StrandStatistics {
            num_strands,
            mean,
            std_dev,
            skewness,
            kurtosis,
            entropy,
        }
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

    // ==================== Advanced Query Mode ====================

    /// Execute an advanced query against the database
    pub fn advanced_query(&self, query: &AdvancedQuery) -> Vec<QueryResult> {
        match query {
            AdvancedQuery::SpiralProximity { center, epsilon } => {
                self.query_spiral_proximity(center, *epsilon)
            }
            AdvancedQuery::ResonanceDistance { reference, max_distance, weights } => {
                self.query_resonance_distance(reference, *max_distance, *weights)
            }
            AdvancedQuery::SpiralTrajectory { params, t_min, t_max, tolerance } => {
                self.query_spiral_trajectory(params, *t_min, *t_max, *tolerance)
            }
            AdvancedQuery::Composite { queries, operator } => {
                self.query_composite(queries, *operator)
            }
            AdvancedQuery::ClusterResonance { cluster_id, min_resonance, max_resonance } => {
                self.query_cluster_resonance(cluster_id.as_deref(), *min_resonance, *max_resonance)
            }
        }
    }

    /// Query by 5D spiral proximity (epsilon ball search)
    fn query_spiral_proximity(&self, center: &State5D, epsilon: f64) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for (seed, result) in &self.cache {
            let distance = center.distance(&result.state_5d);
            if distance < epsilon {
                let score = 1.0 - (distance / epsilon);
                results.push(QueryResult {
                    entry_id: result.entry_id.clone().unwrap_or_else(|| seed.clone()),
                    score,
                    state: result.state_5d,
                    signature: result.signature.clone(),
                    resonance: result.resonance,
                    spiral_position: result.spiral_position,
                });
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Query by resonance distance metric d_R
    fn query_resonance_distance(
        &self,
        reference: &SpectralSignature,
        max_distance: f64,
        weights: (f64, f64, f64),
    ) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for (seed, result) in &self.cache {
            let (w_psi, w_rho, w_omega) = weights;
            let d_r = w_psi * (reference.psi - result.signature.psi).abs()
                    + w_rho * (reference.rho - result.signature.rho).abs()
                    + w_omega * (reference.omega - result.signature.omega).abs();

            if d_r < max_distance {
                let score = 1.0 - (d_r / max_distance);
                results.push(QueryResult {
                    entry_id: result.entry_id.clone().unwrap_or_else(|| seed.clone()),
                    score,
                    state: result.state_5d,
                    signature: result.signature.clone(),
                    resonance: result.resonance,
                    spiral_position: result.spiral_position,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Query along a spiral trajectory
    fn query_spiral_trajectory(
        &self,
        params: &SpiralParams,
        t_min: f64,
        t_max: f64,
        tolerance: f64,
    ) -> Vec<QueryResult> {
        let mut results = Vec::new();
        let path = SpiralPath::new(*params);

        // Sample the trajectory
        let num_samples = 100;
        let dt = (t_max - t_min) / num_samples as f64;

        for (seed, result) in &self.cache {
            // Find closest point on trajectory
            let mut min_dist = f64::MAX;
            let mut best_t = t_min;

            for i in 0..num_samples {
                let t = t_min + dt * i as f64;
                let spiral_point = path.evaluate(t);
                let dist = spiral_point.distance(&result.state_5d);
                if dist < min_dist {
                    min_dist = dist;
                    best_t = t;
                }
            }

            if min_dist < tolerance {
                let score = 1.0 - (min_dist / tolerance);
                results.push(QueryResult {
                    entry_id: result.entry_id.clone().unwrap_or_else(|| seed.clone()),
                    score,
                    state: result.state_5d,
                    signature: result.signature.clone(),
                    resonance: result.resonance,
                    spiral_position: best_t,
                });
            }
        }

        results.sort_by(|a, b| a.spiral_position.partial_cmp(&b.spiral_position).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Query with composite criteria
    fn query_composite(&self, queries: &[AdvancedQuery], operator: QueryOperator) -> Vec<QueryResult> {
        if queries.is_empty() {
            return Vec::new();
        }

        let mut result_sets: Vec<Vec<QueryResult>> = queries.iter()
            .map(|q| self.advanced_query(q))
            .collect();

        match operator {
            QueryOperator::And => {
                // Intersection: keep only entries present in all result sets
                if result_sets.is_empty() {
                    return Vec::new();
                }

                let mut combined = result_sets.remove(0);
                for other_set in result_sets {
                    let other_ids: std::collections::HashSet<_> = other_set.iter()
                        .map(|r| r.entry_id.clone())
                        .collect();
                    combined.retain(|r| other_ids.contains(&r.entry_id));

                    // Update scores with minimum
                    for r in &mut combined {
                        if let Some(other) = other_set.iter().find(|o| o.entry_id == r.entry_id) {
                            r.score = r.score.min(other.score);
                        }
                    }
                }
                combined
            }
            QueryOperator::Or => {
                // Union: combine all results, keep highest score for duplicates
                let mut combined_map: HashMap<String, QueryResult> = HashMap::new();

                for set in result_sets {
                    for result in set {
                        combined_map.entry(result.entry_id.clone())
                            .and_modify(|existing| {
                                if result.score > existing.score {
                                    *existing = result.clone();
                                }
                            })
                            .or_insert(result);
                    }
                }

                let mut combined: Vec<_> = combined_map.into_values().collect();
                combined.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
                combined
            }
        }
    }

    /// Query by cluster membership and resonance range
    fn query_cluster_resonance(
        &self,
        _cluster_id: Option<&str>,
        min_resonance: f64,
        max_resonance: f64,
    ) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for (seed, result) in &self.cache {
            // Check resonance range
            if result.resonance < min_resonance || result.resonance > max_resonance {
                continue;
            }

            // If cluster_id is specified, would check against stored cluster
            // For now, include all matching resonance entries
            let score = (result.resonance - min_resonance) / (max_resonance - min_resonance).max(0.001);

            results.push(QueryResult {
                entry_id: result.entry_id.clone().unwrap_or_else(|| seed.clone()),
                score: score.clamp(0.0, 1.0),
                state: result.state_5d,
                signature: result.signature.clone(),
                resonance: result.resonance,
                spiral_position: result.spiral_position,
            });
        }

        results.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    // ==================== Spiral Dynamics ====================

    /// Evolve a state using spiral dynamics
    pub fn evolve_state(&self, initial: State5D, n_steps: usize) -> Vec<State5D> {
        self.spiral_dynamics.evolve(initial, n_steps)
    }

    /// Check for proof of resonance (convergence)
    pub fn check_convergence(&self, s_old: &State5D, s_new: &State5D, epsilon: f64) -> bool {
        self.spiral_dynamics.proof_of_resonance(s_old, s_new, epsilon)
    }

    /// Get spiral position on current path for a state
    pub fn compute_spiral_position(&self, state: &State5D) -> f64 {
        // Find closest point on spiral path
        let mut min_dist = f64::MAX;
        let mut best_t = 0.0;

        for i in 0..1000 {
            let t = i as f64 * 0.1;
            let spiral_point = self.spiral_path.evaluate(t);
            let dist = spiral_point.distance(state);
            if dist < min_dist {
                min_dist = dist;
                best_t = t;
            }
        }

        best_t
    }

    // ==================== Ledger Access ====================

    /// Get ledger statistics if storage is enabled
    pub fn ledger_statistics(&self) -> Option<crate::integration::mef_ledger::LedgerStatistics> {
        self.ledger.as_ref().map(|l| l.get_statistics())
    }

    /// Get entry count in persistent storage
    pub fn persistent_entry_count(&self) -> usize {
        self.ledger.as_ref().map(|l| l.entry_count()).unwrap_or(0)
    }

    /// Verify ledger chain integrity
    pub fn verify_ledger_integrity(&self) -> Result<bool, InfoGeneticError> {
        if let Some(ref ledger) = self.ledger {
            Ok(ledger.verify_chain_integrity(0)?)
        } else {
            Ok(true) // No ledger means no integrity issues
        }
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
