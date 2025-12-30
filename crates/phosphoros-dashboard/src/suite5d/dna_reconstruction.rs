use std::collections::HashMap;

use sha2::{Digest, Sha256};

fn extract_dna(seed: &str) -> Vec<f64> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher
        .finalize()
        .iter()
        .take(50)
        .map(|byte| *byte as f64 / 255.0)
        .collect()
}

#[derive(Debug, Default, Clone)]
pub struct DNAReconstructor {
    genbank: HashMap<String, Vec<f64>>,
}

impl DNAReconstructor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, zelle_id: impl Into<String>, seed: &str) {
        let dna = extract_dna(seed);
        self.genbank.insert(zelle_id.into(), dna);
    }

    pub fn compare(&self, id1: &str, id2: &str) -> f64 {
        let d1 = match self.genbank.get(id1) {
            Some(dna) => dna,
            None => return 0.0,
        };
        let d2 = match self.genbank.get(id2) {
            Some(dna) => dna,
            None => return 0.0,
        };
        let len = d1.len().min(d2.len());
        if len == 0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for i in 0..len {
            let diff = d1[i] - d2[i];
            sum += diff * diff;
        }
        let norm = (sum / len as f64).sqrt();
        (1.0 - norm).clamp(0.0, 1.0)
    }

    pub fn closest_match(&self, reference_id: &str) -> Option<(String, f64)> {
        if !self.genbank.contains_key(reference_id) {
            return None;
        }
        let mut best: Option<(String, f64)> = None;
        for (other_id, _) in self
            .genbank
            .iter()
            .filter(|(id, _)| id.as_str() != reference_id)
        {
            let score = self.compare(reference_id, other_id);
            if best.as_ref().map(|(_, s)| score > *s).unwrap_or(true) {
                best = Some((other_id.clone(), score));
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_comparison_returns_score() {
        let mut recon = DNAReconstructor::new();
        recon.store("cell_a", "alpha_seed");
        recon.store("cell_b", "beta_seed");
        recon.store("cell_c", "alpha_seed");
        let score = recon.compare("cell_a", "cell_c");
        assert!(score > 0.9);
        let best = recon.closest_match("cell_a").unwrap();
        assert_eq!(best.0, "cell_c");
    }
}
