use sha2::{Digest, Sha256};

fn encode_seed(seed: &str) -> Vec<f64> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher
        .finalize()
        .iter()
        .take(50)
        .map(|byte| *byte as f64 / 255.0)
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct DnaStrandSet {
    pub strands: Vec<Vec<f64>>,
}

impl DnaStrandSet {
    pub fn helical_coordinates(&self) -> Vec<Vec<(f64, f64, f64)>> {
        self.strands
            .iter()
            .enumerate()
            .map(|(index, strand)| {
                let theta_step = std::f64::consts::TAU * 2.0 / strand.len().max(1) as f64;
                strand
                    .iter()
                    .enumerate()
                    .map(|(i, value)| {
                        let theta = i as f64 * theta_step;
                        let radius = *value * 0.2 + 0.1;
                        let x = radius * (theta + index as f64).cos();
                        let y = radius * (theta + index as f64).sin();
                        let z = i as f64 / strand.len().max(1) as f64 + index as f64 * 0.25;
                        (x, y, z)
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct InfoGenetics {
    genome: Vec<DnaStrandSet>,
}

impl InfoGenetics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze_seed(&mut self, seed: &str) {
        let chromosomen = encode_seed(seed);
        let strands = chromosomen
            .chunks(10)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        self.genome.push(DnaStrandSet { strands });
    }

    pub fn genome(&self) -> &[DnaStrandSet] {
        &self.genome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_genetics_produces_helices() {
        let mut info = InfoGenetics::new();
        info.analyze_seed("alpha_seed");
        let genome = info.genome();
        assert_eq!(genome.len(), 1);
        let coords = genome[0].helical_coordinates();
        assert_eq!(coords.len(), genome[0].strands.len());
        assert!(!coords[0].is_empty());
    }
}
