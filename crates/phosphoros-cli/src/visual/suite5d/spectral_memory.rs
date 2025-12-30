use chrono::{DateTime, Local, Utc};
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
pub struct SpectralSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cell_type: String,
    pub spectrum: Vec<f64>,
}

impl SpectralSnapshot {
    pub fn formatted_timestamp(&self) -> String {
        let local: DateTime<Local> = DateTime::from(self.timestamp);
        local.format("%H:%M:%S").to_string()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SpectralMemory {
    snapshots: Vec<SpectralSnapshot>,
}

impl SpectralMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store_snapshot(&mut self, seed: &str, cell_type: impl Into<String>) {
        let timestamp = Utc::now();
        let spectrum = encode_seed(seed);
        self.snapshots.push(SpectralSnapshot {
            timestamp,
            cell_type: cell_type.into(),
            spectrum,
        });
    }

    pub fn latest(&self, count: usize) -> Vec<SpectralSnapshot> {
        let start = self.snapshots.len().saturating_sub(count);
        self.snapshots[start..].to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_stores_snapshots() {
        let mut memory = SpectralMemory::new();
        memory.store_snapshot("seed_a", "NavigatorCell");
        let latest = memory.latest(1);
        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0].cell_type, "NavigatorCell");
    }
}
