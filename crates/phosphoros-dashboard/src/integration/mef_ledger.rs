//! MEF Ledger Integration Module
//!
//! Provides persistent storage for Infogenetic entries using a hash-chained
//! block structure inspired by the MEF (Multi-dimensional Embedding Framework)
//! ledger system.
//!
//! Features:
//! - Hash-chained blocks for data integrity
//! - Compact storage with SHA-256 verification
//! - JSON-based persistence with canonical serialization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// MEF Ledger errors
#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Chain integrity violated at block {0}")]
    ChainIntegrityError(i32),

    #[error("Block not found: {0}")]
    BlockNotFound(i32),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Compact 5D spectral signature for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactSignature {
    /// Psi component (ψ)
    pub psi: f64,
    /// Rho component (ρ)
    pub rho: f64,
    /// Omega component (ω)
    pub omega: f64,
    /// Resonance value D = ψ·ρ·ω
    pub resonance: f64,
}

impl CompactSignature {
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            psi,
            rho,
            omega,
            resonance: psi * rho * omega,
        }
    }
}

/// Compact Infogenetic entry for ledger storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactInfogeneticEntry {
    /// Entry ID (derived from seed hash)
    pub entry_id: String,
    /// Seed phrase hash (not the actual seed for security)
    pub seed_hash: String,
    /// 5D embedding coordinates
    pub embedding: [f64; 5],
    /// Spectral signature
    pub signature: CompactSignature,
    /// DNA strand statistics
    pub strand_stats: StrandStatistics,
    /// Gabriel Cell states (compressed)
    pub cell_states: Vec<CompactCellState>,
    /// Associated addresses (hashed)
    pub address_hashes: Vec<String>,
    /// Cluster ID if assigned
    pub cluster_id: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// DNA strand statistics for compact storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrandStatistics {
    /// Number of strands
    pub num_strands: usize,
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Skewness
    pub skewness: f64,
    /// Kurtosis
    pub kurtosis: f64,
    /// Entropy
    pub entropy: f64,
}

/// Compact Gabriel Cell state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactCellState {
    /// Cell index
    pub idx: usize,
    /// Psi value
    pub psi: f64,
    /// Rho value
    pub rho: f64,
    /// Omega value
    pub omega: f64,
    /// Output value
    pub output: f64,
}

/// Block summary for index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub index: i32,
    pub hash: String,
    pub entry_id: String,
    pub timestamp: String,
    pub file: String,
}

/// Ledger metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerMetadata {
    pub created: String,
    pub last_updated: String,
    pub version: String,
    pub entry_count: usize,
}

impl Default for LedgerMetadata {
    fn default() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        Self {
            created: now.clone(),
            last_updated: now,
            version: "1.0.0".to_string(),
            entry_count: 0,
        }
    }
}

/// Ledger index structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerIndex {
    pub blocks: Vec<BlockSummary>,
    pub current_index: i32,
    pub metadata: LedgerMetadata,
}

impl Default for LedgerIndex {
    fn default() -> Self {
        Self {
            blocks: Vec::new(),
            current_index: -1,
            metadata: LedgerMetadata::default(),
        }
    }
}

/// Infogenetic block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfogeneticBlock {
    /// Block index
    pub index: i32,
    /// Previous block hash
    pub previous_hash: String,
    /// Block timestamp
    pub timestamp: String,
    /// Entry ID
    pub entry_id: String,
    /// Compact entry data
    pub data: CompactInfogeneticEntry,
    /// Proof of resonance (D = ψ·ρ·ω)
    pub proof: ProofOfResonance,
    /// Block hash
    pub hash: String,
}

/// Proof of resonance for block validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfResonance {
    /// Computed resonance value
    pub resonance: f64,
    /// Merkle root of entry data
    pub merkle_root: String,
    /// Spiral position (parameter t)
    pub spiral_position: f64,
    /// Convergence achieved
    pub converged: bool,
}

/// Infogenetic Ledger for persistent storage
pub struct InfogeneticLedger {
    /// Ledger storage path
    ledger_path: PathBuf,
    /// Index of all blocks
    index: LedgerIndex,
    /// Genesis hash
    genesis_hash: String,
    /// In-memory cache of recent blocks
    cache: HashMap<i32, InfogeneticBlock>,
    /// Maximum cache size
    max_cache_size: usize,
}

impl InfogeneticLedger {
    /// Create or open a ledger at the given path
    pub fn new(ledger_path: impl AsRef<Path>) -> Result<Self, LedgerError> {
        let ledger_path = ledger_path.as_ref().to_path_buf();
        fs::create_dir_all(&ledger_path)?;

        let index_file = ledger_path.join("infogenetic_index.json");
        let index = Self::load_index(&index_file)?;
        let genesis_hash = "0".repeat(64);

        Ok(Self {
            ledger_path,
            index,
            genesis_hash,
            cache: HashMap::new(),
            max_cache_size: 100,
        })
    }

    /// Load ledger index from disk
    fn load_index(index_file: &Path) -> Result<LedgerIndex, LedgerError> {
        if index_file.exists() {
            let contents = fs::read_to_string(index_file)?;
            let index: LedgerIndex = serde_json::from_str(&contents)?;
            Ok(index)
        } else {
            Ok(LedgerIndex::default())
        }
    }

    /// Save ledger index to disk
    fn save_index(&mut self) -> Result<(), LedgerError> {
        self.index.metadata.last_updated =
            Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        self.index.metadata.entry_count = (self.index.current_index + 1) as usize;

        let index_file = self.ledger_path.join("infogenetic_index.json");
        let json = serde_json::to_string_pretty(&self.index)?;
        fs::write(&index_file, json)?;

        Ok(())
    }

    /// Canonicalize JSON for deterministic hashing
    fn canonicalize_json(value: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let mut sorted_map = serde_json::Map::new();
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();
                for key in keys {
                    sorted_map.insert(key.clone(), Self::canonicalize_json(&map[key]));
                }
                serde_json::Value::Object(sorted_map)
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(Self::canonicalize_json).collect())
            }
            _ => value.clone(),
        }
    }

    /// Compute SHA256 hash of block data
    pub fn compute_block_hash(block: &serde_json::Value) -> String {
        let mut block_data = block.clone();
        if let Some(obj) = block_data.as_object_mut() {
            obj.remove("hash");
        }

        let canonical_block = Self::canonicalize_json(&block_data);
        let block_str = serde_json::to_string(&canonical_block).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(block_str.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Compute Merkle root from entry data
    fn compute_merkle_root(entry: &CompactInfogeneticEntry) -> String {
        let leaves: Vec<String> = vec![
            entry.entry_id.clone(),
            entry.seed_hash.clone(),
            format!("{:?}", entry.embedding),
            format!("{:.15}", entry.signature.resonance),
        ];

        // Hash all leaves
        let hashed_leaves: Vec<String> = leaves
            .iter()
            .map(|leaf| {
                let mut hasher = Sha256::new();
                hasher.update(leaf.as_bytes());
                format!("{:x}", hasher.finalize())
            })
            .collect();

        // Simple Merkle root (combine pairwise)
        let mut current = hashed_leaves;
        while current.len() > 1 {
            let mut next = Vec::new();
            for chunk in current.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0])
                };
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                next.push(format!("{:x}", hasher.finalize()));
            }
            current = next;
        }

        current.into_iter().next().unwrap_or_else(|| "0".repeat(64))
    }

    /// Get the most recent block hash
    pub fn get_last_hash(&self) -> Result<String, LedgerError> {
        if let Some(block) = self.get_block(self.index.current_index)? {
            Ok(block.hash)
        } else {
            Ok(self.genesis_hash.clone())
        }
    }

    /// Get a block by index
    pub fn get_block(&self, index: i32) -> Result<Option<InfogeneticBlock>, LedgerError> {
        if index < 0 || index > self.index.current_index {
            return Ok(None);
        }

        // Check cache first
        if let Some(block) = self.cache.get(&index) {
            return Ok(Some(block.clone()));
        }

        // Load from disk
        let block_file = self.ledger_path.join(format!("infogenetic_{:06}.block", index));
        if !block_file.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&block_file)?;
        let block: InfogeneticBlock = serde_json::from_str(&contents)?;

        Ok(Some(block))
    }

    /// Append a new entry to the ledger
    pub fn append_entry(&mut self, entry: CompactInfogeneticEntry) -> Result<InfogeneticBlock, LedgerError> {
        let next_index = self.index.current_index + 1;
        let previous_hash = self.get_last_hash()?;
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();

        // Compute proof of resonance
        let merkle_root = Self::compute_merkle_root(&entry);
        let proof = ProofOfResonance {
            resonance: entry.signature.resonance,
            merkle_root,
            spiral_position: next_index as f64 * 0.1,
            converged: entry.signature.resonance > 0.0,
        };

        // Create block (without hash initially)
        let mut block_json = serde_json::json!({
            "index": next_index,
            "previous_hash": previous_hash,
            "timestamp": timestamp,
            "entry_id": entry.entry_id.clone(),
            "data": entry,
            "proof": proof,
        });

        // Compute and add hash
        let hash = Self::compute_block_hash(&block_json);
        block_json["hash"] = serde_json::json!(hash);

        // Deserialize to block
        let block: InfogeneticBlock = serde_json::from_value(block_json)?;

        // Save to disk
        let block_file = self.ledger_path.join(format!("infogenetic_{:06}.block", next_index));
        let json = serde_json::to_string_pretty(&block)?;
        fs::write(&block_file, json)?;

        // Update index
        self.index.blocks.push(BlockSummary {
            index: block.index,
            hash: block.hash.clone(),
            entry_id: block.entry_id.clone(),
            timestamp: block.timestamp.clone(),
            file: block_file.file_name().unwrap().to_string_lossy().to_string(),
        });
        self.index.current_index = next_index;
        self.save_index()?;

        // Add to cache
        if self.cache.len() >= self.max_cache_size {
            // Remove oldest entry
            let oldest = self.cache.keys().min().copied();
            if let Some(key) = oldest {
                self.cache.remove(&key);
            }
        }
        self.cache.insert(block.index, block.clone());

        Ok(block)
    }

    /// Verify block hash
    pub fn verify_block_hash(&self, block: &InfogeneticBlock) -> bool {
        let block_json = serde_json::to_value(block).unwrap();
        let computed_hash = Self::compute_block_hash(&block_json);
        block.hash == computed_hash
    }

    /// Verify chain integrity from start_index
    pub fn verify_chain_integrity(&self, start_index: i32) -> Result<bool, LedgerError> {
        if self.index.current_index < 0 {
            return Ok(true);
        }

        let mut prev_hash = if start_index == 0 {
            Some(self.genesis_hash.clone())
        } else {
            None
        };

        for i in start_index..=self.index.current_index {
            let block = self.get_block(i)?
                .ok_or_else(|| LedgerError::BlockNotFound(i))?;

            // Verify block hash
            if !self.verify_block_hash(&block) {
                return Err(LedgerError::ChainIntegrityError(i));
            }

            // Verify chain linkage
            if let Some(ref expected_prev) = prev_hash {
                if block.previous_hash != *expected_prev {
                    return Err(LedgerError::ChainIntegrityError(i));
                }
            }

            prev_hash = Some(block.hash.clone());
        }

        Ok(true)
    }

    /// Search entries by resonance range
    pub fn search_by_resonance(&self, min_resonance: f64, max_resonance: f64) -> Result<Vec<InfogeneticBlock>, LedgerError> {
        let mut results = Vec::new();

        for i in 0..=self.index.current_index {
            if let Some(block) = self.get_block(i)? {
                let resonance = block.data.signature.resonance;
                if resonance >= min_resonance && resonance <= max_resonance {
                    results.push(block);
                }
            }
        }

        Ok(results)
    }

    /// Search entries by cluster ID
    pub fn search_by_cluster(&self, cluster_id: &str) -> Result<Vec<InfogeneticBlock>, LedgerError> {
        let mut results = Vec::new();

        for i in 0..=self.index.current_index {
            if let Some(block) = self.get_block(i)? {
                if block.data.cluster_id.as_deref() == Some(cluster_id) {
                    results.push(block);
                }
            }
        }

        Ok(results)
    }

    /// Get ledger statistics
    pub fn get_statistics(&self) -> LedgerStatistics {
        // Statistics computed from index without loading full blocks
        let _total_resonance = 0.0;
        let _min_resonance = f64::MAX;
        let _max_resonance = f64::MIN;

        for _summary in &self.index.blocks {
            // We'd need to load blocks for full stats, but for now use index
        }

        LedgerStatistics {
            total_entries: (self.index.current_index + 1).max(0) as usize,
            first_entry: self.index.blocks.first().map(|b| b.timestamp.clone()),
            last_entry: self.index.blocks.last().map(|b| b.timestamp.clone()),
            chain_valid: self.verify_chain_integrity(0).unwrap_or(false),
            metadata: self.index.metadata.clone(),
        }
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        (self.index.current_index + 1).max(0) as usize
    }
}

/// Ledger statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStatistics {
    pub total_entries: usize,
    pub first_entry: Option<String>,
    pub last_entry: Option<String>,
    pub chain_valid: bool,
    pub metadata: LedgerMetadata,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_entry() -> CompactInfogeneticEntry {
        CompactInfogeneticEntry {
            entry_id: "test-entry-001".to_string(),
            seed_hash: "abc123def456".to_string(),
            embedding: [0.5, 0.3, 0.2, 0.1, 0.4],
            signature: CompactSignature::new(0.7, 0.8, 0.6),
            strand_stats: StrandStatistics {
                num_strands: 4,
                mean: 0.5,
                std_dev: 0.15,
                skewness: 0.1,
                kurtosis: -0.2,
                entropy: 0.8,
            },
            cell_states: vec![
                CompactCellState { idx: 0, psi: 0.7, rho: 0.8, omega: 0.6, output: 0.336 },
            ],
            address_hashes: vec!["addr_hash_1".to_string()],
            cluster_id: None,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_ledger_creation() {
        let temp_dir = TempDir::new().unwrap();
        let ledger = InfogeneticLedger::new(temp_dir.path()).unwrap();

        assert_eq!(ledger.entry_count(), 0);
    }

    #[test]
    fn test_append_and_retrieve() {
        let temp_dir = TempDir::new().unwrap();
        let mut ledger = InfogeneticLedger::new(temp_dir.path()).unwrap();

        let entry = create_test_entry();
        let block = ledger.append_entry(entry.clone()).unwrap();

        assert_eq!(block.index, 0);
        assert_eq!(block.entry_id, "test-entry-001");
        assert_eq!(ledger.entry_count(), 1);

        // Retrieve
        let retrieved = ledger.get_block(0).unwrap().unwrap();
        assert_eq!(retrieved.entry_id, "test-entry-001");
    }

    #[test]
    fn test_chain_integrity() {
        let temp_dir = TempDir::new().unwrap();
        let mut ledger = InfogeneticLedger::new(temp_dir.path()).unwrap();

        // Add multiple entries
        for i in 0..5 {
            let mut entry = create_test_entry();
            entry.entry_id = format!("entry-{}", i);
            ledger.append_entry(entry).unwrap();
        }

        // Verify chain
        assert!(ledger.verify_chain_integrity(0).unwrap());
    }

    #[test]
    fn test_search_by_resonance() {
        let temp_dir = TempDir::new().unwrap();
        let mut ledger = InfogeneticLedger::new(temp_dir.path()).unwrap();

        // Add entries with different resonance values
        for i in 0..5 {
            let mut entry = create_test_entry();
            entry.entry_id = format!("entry-{}", i);
            entry.signature = CompactSignature::new(0.2 * (i + 1) as f64, 0.8, 0.6);
            ledger.append_entry(entry).unwrap();
        }

        // Search for high resonance
        let results = ledger.search_by_resonance(0.3, 1.0).unwrap();
        assert!(!results.is_empty());
    }
}
