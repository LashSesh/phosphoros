use rand::prelude::*;
use rand_distr::Normal;
use sha2::{Digest, Sha256};

const RESONANCE_THRESHOLD: f64 = 1.2;
const DEFAULT_PERTURBATION: f64 = 0.03;
const MAX_VECTOR_LEN: usize = 32;

fn encode_seed(seed: &str) -> Vec<f64> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| *byte as f64 / 255.0)
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectralLink {
    pub seed_a: String,
    pub seed_b: String,
    pub distance: f64,
}

#[derive(Debug, Default, Clone)]
pub struct SpectralFabric {
    seed_vectors: Vec<(String, Vec<f64>)>,
}

impl SpectralFabric {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_seed(&mut self, seed: impl Into<String>) {
        let seed = seed.into();
        let vector = encode_seed(&seed);
        self.seed_vectors.push((seed, vector));
    }

    pub fn seed_vectors(&self) -> &[(String, Vec<f64>)] {
        &self.seed_vectors
    }

    pub fn seed_count(&self) -> usize {
        self.seed_vectors.len()
    }

    pub fn build_fabric(&self) -> Vec<SpectralLink> {
        let mut fabric = Vec::new();
        for i in 0..self.seed_vectors.len() {
            let (seed_a, vec_a) = &self.seed_vectors[i];
            for j in (i + 1)..self.seed_vectors.len() {
                let (seed_b, vec_b) = &self.seed_vectors[j];
                let distance = l2_distance(vec_a, vec_b);
                if distance < RESONANCE_THRESHOLD {
                    fabric.push(SpectralLink {
                        seed_a: seed_a.clone(),
                        seed_b: seed_b.clone(),
                        distance: (distance * 10_000.0).round() / 10_000.0,
                    });
                }
            }
        }
        fabric
    }

    pub fn get_projection_hint(&self, top_k: usize) -> Vec<String> {
        if self.seed_vectors.len() < 3 || top_k == 0 {
            return Vec::new();
        }

        let vector_len = self.seed_vectors[0].1.len().min(MAX_VECTOR_LEN);
        let mut mean_vector = vec![0.0; vector_len];
        for (_, vector) in &self.seed_vectors {
            for (idx, value) in vector.iter().take(vector_len).enumerate() {
                mean_vector[idx] += value;
            }
        }
        let scale = self.seed_vectors.len() as f64;
        for value in &mut mean_vector {
            *value /= scale;
        }

        let mut rng = thread_rng();
        let noise = Normal::new(0.0, DEFAULT_PERTURBATION)
            .unwrap_or_else(|_| Normal::new(0.0, 0.01).unwrap());

        (0..top_k)
            .map(|_| {
                let mut perturbed = mean_vector.clone();
                for value in &mut perturbed {
                    *value = (*value + noise.sample(&mut rng)).clamp(0.0, 1.0);
                }
                encode_hint(&perturbed)
            })
            .collect()
    }

    pub fn export_projected_seeds(&self, count: usize) -> Vec<String> {
        self.get_projection_hint(count)
            .into_iter()
            .map(|hint| format!("recon_{hint}"))
            .collect()
    }
}

fn encode_hint(vector: &[f64]) -> String {
    vector
        .iter()
        .take(MAX_VECTOR_LEN)
        .map(|value| {
            let value = (*value * 255.0).round() as i64;
            let clamped = value.rem_euclid(16) as u8;
            format!("{:x}", clamped)
        })
        .collect()
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
    fn fabric_builds_links() {
        let mut fabric = SpectralFabric::new();
        fabric.add_seed("alpha_seed");
        fabric.add_seed("alpha_seed");
        fabric.add_seed("gamma_seed");
        let links = fabric.build_fabric();
        assert!(links.iter().any(|link| link.distance.abs() < f64::EPSILON));
    }

    #[test]
    fn projections_are_generated() {
        let mut fabric = SpectralFabric::new();
        fabric.add_seed("alpha_seed");
        fabric.add_seed("beta_seed");
        fabric.add_seed("gamma_seed");
        fabric.add_seed("delta_seed");
        let hints = fabric.get_projection_hint(3);
        assert_eq!(hints.len(), 3);
        assert!(hints.iter().all(|hint| !hint.is_empty()));
    }
}
