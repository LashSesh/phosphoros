use sha2::{Digest, Sha256};

use super::SpectralLink;

fn project_seed_to_xyz(seed: &str) -> [f64; 3] {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let digest = hasher.finalize();
    [
        digest[0] as f64 / 255.0,
        digest[1] as f64 / 255.0,
        digest[2] as f64 / 255.0,
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub struct CotopSegment {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub distance: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CotopTopology {
    pub segments: Vec<CotopSegment>,
}

impl CotopTopology {
    pub fn from_links(links: &[SpectralLink]) -> Self {
        let segments = links
            .iter()
            .map(|link| CotopSegment {
                start: project_seed_to_xyz(&link.seed_a),
                end: project_seed_to_xyz(&link.seed_b),
                distance: link.distance,
            })
            .collect();
        Self { segments }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cotop_generates_segments() {
        let links = vec![SpectralLink {
            seed_a: "alpha_seed".into(),
            seed_b: "beta_seed".into(),
            distance: 0.4,
        }];
        let topology = CotopTopology::from_links(&links);
        assert_eq!(topology.segments.len(), 1);
        assert!(topology.segments[0].start[0] >= 0.0);
    }
}
