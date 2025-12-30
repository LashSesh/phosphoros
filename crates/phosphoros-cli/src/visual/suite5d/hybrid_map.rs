use rand::prelude::*;
use sha2::{Digest, Sha256};

const EDGE_PROBABILITY: f64 = 0.15;
const SIGNATURE_LENGTH: usize = 20;

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
pub struct HybridNodeSpectrum {
    pub signature: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HybridNode {
    pub id: String,
    pub cell_type: String,
    pub position: (f64, f64),
    pub spectrum: Option<HybridNodeSpectrum>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HybridNodeSpec {
    pub cell_type: String,
    pub seed: Option<String>,
}

impl HybridNodeSpec {
    pub fn new(cell_type: impl Into<String>, seed: Option<String>) -> Self {
        Self {
            cell_type: cell_type.into(),
            seed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HybridMapSnapshot {
    pub nodes: Vec<HybridNode>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Default, Clone)]
pub struct HybridMap {
    nodes: Vec<HybridNode>,
    edges: Vec<(usize, usize)>,
}

impl HybridMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_with_rng<R: Rng + ?Sized>(&mut self, zellen: &[HybridNodeSpec], rng: &mut R) {
        self.nodes.clear();
        self.edges.clear();

        for (idx, spec) in zellen.iter().enumerate() {
            let id = format!("Z{idx}");
            let (position, spectrum) = if let Some(seed) = &spec.seed {
                let signature = encode_seed(seed);
                let x = signature.get(0).copied().unwrap_or_default() * 2.0 - 1.0;
                let y = signature.get(1).copied().unwrap_or_default() * 2.0 - 1.0;
                let spectrum = HybridNodeSpectrum {
                    signature: signature[..SIGNATURE_LENGTH.min(signature.len())].to_vec(),
                };
                ((x, y), Some(spectrum))
            } else {
                let angle = idx as f64 * 1.618_033_988_75; // goldener Winkel
                let x = angle.cos();
                let y = angle.sin();
                ((x, y), None)
            };

            self.nodes.push(HybridNode {
                id,
                cell_type: spec.cell_type.clone(),
                position,
                spectrum,
            });
        }

        let len = self.nodes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if rng.gen::<f64>() < EDGE_PROBABILITY {
                    self.edges.push((i, j));
                }
            }
        }
    }

    pub fn build_graph(&mut self, zellen: &[HybridNodeSpec]) {
        let mut rng = thread_rng();
        self.build_with_rng(zellen, &mut rng);
    }

    pub fn snapshot(&self) -> HybridMapSnapshot {
        let edges = self
            .edges
            .iter()
            .map(|(a, b)| (self.nodes[*a].id.clone(), self.nodes[*b].id.clone()))
            .collect();
        HybridMapSnapshot {
            nodes: self.nodes.clone(),
            edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn hybrid_map_produces_snapshot() {
        let mut map = HybridMap::new();
        let cells = vec![
            HybridNodeSpec::new("NavigatorCell", Some("seed_a".to_string())),
            HybridNodeSpec::new("MutatorCell", Some("seed_b".to_string())),
            HybridNodeSpec::new("WatcherCell", None),
        ];
        let mut rng = StdRng::seed_from_u64(7);
        map.build_with_rng(&cells, &mut rng);
        let snapshot = map.snapshot();
        assert_eq!(snapshot.nodes.len(), 3);
        assert!(snapshot.nodes.iter().any(|node| node.spectrum.is_some()));
    }
}
