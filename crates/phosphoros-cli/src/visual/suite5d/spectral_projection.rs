use sha2::{Digest, Sha256};

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
pub struct SpectralSeries {
    pub label: String,
    pub samples: Vec<(f64, f64)>,
}

#[derive(Debug, Default, Clone)]
pub struct SpectralProjector {
    field_lines: Vec<Vec<f64>>,
}

impl SpectralProjector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_seed(&mut self, seed: &str) {
        let values = encode_seed(seed);
        self.field_lines.push(values);
    }

    pub fn field_count(&self) -> usize {
        self.field_lines.len()
    }

    pub fn projection(&self, count: usize) -> Vec<SpectralSeries> {
        if self.field_lines.is_empty() {
            return Vec::new();
        }
        let total = self.field_lines.len();
        let start = total.saturating_sub(count);
        self.field_lines
            .iter()
            .enumerate()
            .skip(start)
            .map(|(idx, field)| {
                let label = format!("Feld {}", idx + 1);
                let samples = field
                    .iter()
                    .enumerate()
                    .map(|(step, value)| {
                        let x = step as f64;
                        let phase = idx as f64 * std::f64::consts::PI / 6.0;
                        let y = (value * 2.0 * std::f64::consts::PI + phase).sin() * 0.5 + 0.5;
                        (x, y)
                    })
                    .collect();
                SpectralSeries { label, samples }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_returns_series() {
        let mut projector = SpectralProjector::new();
        projector.add_seed("alpha_seed");
        projector.add_seed("beta_seed");
        let series = projector.projection(2);
        assert_eq!(series.len(), 2);
        assert!(series[0].samples.len() > 10);
    }
}
