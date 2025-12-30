use sha2::{Digest, Sha256};

fn encode_seed(seed: &str) -> Vec<f64> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let digest = hasher.finalize();
    digest.iter().map(|byte| *byte as f64 / 255.0).collect()
}

#[derive(Debug, Default, Clone)]
pub struct SpectralChainWalker {
    reference_vectors: Vec<(String, Vec<f64>)>,
}

impl SpectralChainWalker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_reference_seeds<I, S>(&mut self, seeds: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.reference_vectors = seeds
            .into_iter()
            .map(|seed| {
                let seed = seed.as_ref().to_string();
                let vector = encode_seed(&seed);
                (seed, vector)
            })
            .collect();
    }

    pub fn reference_count(&self) -> usize {
        self.reference_vectors.len()
    }

    pub fn extrapolate_from_fragment(&self, fragment: &str, max_out: usize) -> Vec<String> {
        if fragment.is_empty() || max_out == 0 {
            return Vec::new();
        }
        let fragment_vector = encode_seed(fragment);
        let mut candidates: Vec<(String, f64)> = self
            .reference_vectors
            .iter()
            .map(|(seed, vector)| {
                let distance = l2_distance(vector, &fragment_vector);
                (seed.clone(), distance)
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates
            .into_iter()
            .take(max_out)
            .map(|(seed, _)| seed)
            .collect()
    }
}

fn l2_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(va, vb)| (va - vb).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chainwalker_returns_best_matches() {
        let mut walker = SpectralChainWalker::new();
        walker.load_reference_seeds(["alpha_seed", "beta_seed", "gamma_seed", "delta_seed"]);
        let result = walker.extrapolate_from_fragment("alpha_fragment", 2);
        assert_eq!(result.len(), 2);
        assert!(walker.reference_count() >= 4);
    }
}
