//! PhosphorosCore - High-level bridge API for PHOSPHOROS integration

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::explorer::{ExplorationResult, QDASHExplorer};
use crate::geometry::{MetatronGeometry, Point5D};
use crate::resonance::Infogenom;
use crate::{Error, Result};

/// Central bridge API for PHOSPHOROS integration
pub struct PhosphorosCore {
    metatron: MetatronGeometry,
    infogenoms: HashMap<String, Infogenom>,
}

impl PhosphorosCore {
    /// Create a new PhosphorosCore instance
    pub fn new() -> Self {
        Self {
            metatron: MetatronGeometry::new(),
            infogenoms: HashMap::new(),
        }
    }

    /// Embed BIP39 seed phrase into 5D space
    pub fn embed_seed_phrase(&self, words: &[&str]) -> Vec<Point5D> {
        words
            .iter()
            .map(|word| {
                let hash = self.hash_word(word);
                self.metatron.embed_object(hash)
            })
            .collect()
    }

    /// Create a new Infogenom with specified number of cells
    pub fn create_infogenom(&mut self, id: String, num_cells: usize) {
        self.infogenoms
            .insert(id.clone(), Infogenom::new(id, num_cells));
    }

    /// Explore keyspace using QDASH with specified Infogenom
    pub fn explore_keyspace(
        &mut self,
        infogenom_id: &str,
        seed: u64,
        max_steps: usize,
    ) -> Result<ExplorationResult> {
        let infogenom = self
            .infogenoms
            .remove(infogenom_id)
            .ok_or_else(|| Error::InfogenomNotFound(infogenom_id.to_string()))?;

        let mut explorer = QDASHExplorer::new(infogenom, seed, max_steps);
        let result = explorer.explore();

        // Return infogenom to pool
        self.infogenoms
            .insert(infogenom_id.to_string(), explorer.infogenom);

        Ok(result)
    }

    fn hash_word(&self, word: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for PhosphorosCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phosphoros_core_creation() {
        let core = PhosphorosCore::new();
        assert_eq!(core.infogenoms.len(), 0);
    }

    #[test]
    fn test_embed_seed_phrase() {
        let core = PhosphorosCore::new();
        let words = vec!["abandon", "ability", "able"];
        let embeddings = core.embed_seed_phrase(&words);
        assert_eq!(embeddings.len(), 3);

        // Test determinism
        let embeddings2 = core.embed_seed_phrase(&words);
        assert_eq!(embeddings, embeddings2);
    }

    #[test]
    fn test_create_infogenom() {
        let mut core = PhosphorosCore::new();
        core.create_infogenom("test".to_string(), 8);
        assert_eq!(core.infogenoms.len(), 1);
    }

    #[test]
    fn test_explore_keyspace() {
        let mut core = PhosphorosCore::new();
        core.create_infogenom("test_infogenom".to_string(), 4);

        let result = core.explore_keyspace("test_infogenom", 42, 100);
        assert!(result.is_ok());

        let res = result.unwrap();
        assert!(res.steps > 0);
        assert!(res.best_resonance >= 0.0);
    }

    #[test]
    fn test_explore_missing_infogenom() {
        let mut core = PhosphorosCore::new();
        let result = core.explore_keyspace("nonexistent", 42, 100);
        assert!(result.is_err());
    }
}
