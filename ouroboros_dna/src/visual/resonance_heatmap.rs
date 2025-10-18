use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{OuroborosError, Result};

const DEFAULT_PATH: &str = "data/heatmap.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeatEntry {
    pub count: u64,
    pub sum: f64,
}

#[derive(Debug, Clone)]
pub struct Heatmap {
    path: PathBuf,
}

impl Default for Heatmap {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

impl Heatmap {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn log(&self, address: &str, balance: Option<f64>) -> Result<()> {
        let mut data = self.load()?;
        let prefix = address.chars().take(6).collect::<String>();
        let entry = data.entry(prefix).or_default();
        entry.count += 1;
        entry.sum += balance.unwrap_or_default();
        self.store(&data)
    }

    pub fn load(&self) -> Result<HashMap<String, HeatEntry>> {
        ensure_parent_exists(&self.path)?;
        if !self.path.exists() {
            return Ok(HashMap::new());
        }
        let content = fs::read_to_string(&self.path)?;
        if content.trim().is_empty() {
            return Ok(HashMap::new());
        }
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }

    fn store(&self, data: &HashMap<String, HeatEntry>) -> Result<()> {
        ensure_parent_exists(&self.path)?;
        let serialized = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }
}

pub fn log_address_heat(address: &str, balance: Option<f64>) -> Result<()> {
    Heatmap::default().log(address, balance)
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
