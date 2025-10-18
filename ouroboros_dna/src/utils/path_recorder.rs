use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

const DEFAULT_PATH: &str = "recorded_paths.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathEntry {
    pub cell: String,
    pub vector: Vec<f64>,
    pub seed: String,
    pub address: String,
    pub balance: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PathRecorder {
    path: PathBuf,
}

impl Default for PathRecorder {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

impl PathRecorder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn log_step(
        &self,
        cell_name: &str,
        vector: &[f64],
        seed: &str,
        address: &str,
        balance: Option<f64>,
    ) -> Result<()> {
        let mut entries = self.load_entries()?;
        entries.push(PathEntry {
            cell: cell_name.to_string(),
            vector: vector.to_vec(),
            seed: seed.to_string(),
            address: address.to_string(),
            balance,
        });
        self.write_entries(&entries)
    }

    pub fn load_all(&self) -> Result<Vec<PathEntry>> {
        self.load_entries()
    }

    pub fn get_path_by_cell(&self, cell_name: &str) -> Result<Vec<PathEntry>> {
        let entries = self.load_entries()?;
        Ok(entries
            .into_iter()
            .filter(|entry| entry.cell == cell_name)
            .collect())
    }

    fn load_entries(&self) -> Result<Vec<PathEntry>> {
        ensure_parent_exists(&self.path)?;
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.path)?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        let entries = serde_json::from_str(&content)?;
        Ok(entries)
    }

    fn write_entries(&self, entries: &[PathEntry]) -> Result<()> {
        ensure_parent_exists(&self.path)?;
        let serialized = serde_json::to_string_pretty(entries)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }
}

fn ensure_parent_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    } else {
        Ok(())
    }
}
