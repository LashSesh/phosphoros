use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{OuroborosError, Result};

const DEFAULT_PATH: &str = "data/resonance_hits.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResonanceHit {
    timestamp: DateTime<Utc>,
    seed: String,
    address: String,
    balance: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ResonanceReturn {
    path: PathBuf,
}

impl Default for ResonanceReturn {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

impl ResonanceReturn {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn log_hit(&self, seed: &str, address: &str, balance: Option<f64>) -> Result<()> {
        let mut hits = self.load_hits()?;
        hits.push(ResonanceHit {
            timestamp: Utc::now(),
            seed: seed.to_string(),
            address: address.to_string(),
            balance,
        });
        self.write_hits(&hits)
    }

    fn load_hits(&self) -> Result<Vec<ResonanceHit>> {
        ensure_parent_exists(&self.path)?;
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let hits: Vec<ResonanceHit> = serde_json::from_reader(reader)?;
        Ok(hits)
    }

    fn write_hits(&self, hits: &[ResonanceHit]) -> Result<()> {
        ensure_parent_exists(&self.path)?;
        let serialized = serde_json::to_string_pretty(hits)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }
}

fn ensure_parent_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
        Ok(())
    } else {
        Err(OuroborosError::Config(format!(
            "Unable to determine parent directory for {:?}",
            path
        )))
    }
}
