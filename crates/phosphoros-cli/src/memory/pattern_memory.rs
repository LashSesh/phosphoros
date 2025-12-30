use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::error::{OuroborosError, Result};

const DEFAULT_PATH: &str = "data/pattern_memory.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryEntry {
    cell: String,
    vector: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct PatternMemory {
    path: PathBuf,
}

impl Default for PatternMemory {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

impl PatternMemory {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn save_path(&self, cell_id: &str, vector: &[f64]) -> Result<()> {
        let mut entries = self.load_entries()?;
        entries.push(MemoryEntry {
            cell: cell_id.to_string(),
            vector: vector.to_vec(),
        });
        self.write_entries(&entries)
    }

    pub fn repeat_path(&self, cell_id: &str, mutation: f64) -> Result<Option<Vec<Vec<f64>>>> {
        let entries = self.load_entries()?;
        let vectors: Vec<Vec<f64>> = entries
            .into_iter()
            .filter(|entry| entry.cell == cell_id)
            .map(|entry| self.mutate_vector(&entry.vector, mutation))
            .collect();
        if vectors.is_empty() {
            Ok(None)
        } else {
            Ok(Some(vectors))
        }
    }

    pub fn combine_paths(
        &self,
        cell_a: &str,
        cell_b: &str,
        mutation: f64,
    ) -> Result<Option<Vec<Vec<f64>>>> {
        let mut a = match self.repeat_path(cell_a, mutation)? {
            Some(v) if !v.is_empty() => v,
            _ => return Ok(None),
        };
        let mut b = match self.repeat_path(cell_b, mutation)? {
            Some(v) if !v.is_empty() => v,
            _ => return Ok(None),
        };

        let first_a = a.pop().unwrap();
        let first_b = b.pop().unwrap();

        if first_a.len() != first_b.len() {
            return Err(OuroborosError::Config(format!(
                "Vector length mismatch between {cell_a} and {cell_b}"
            )));
        }

        let combined: Vec<f64> = first_a
            .iter()
            .zip(first_b.iter())
            .map(|(x, y)| (x + y) / 2.0)
            .collect();
        Ok(Some(vec![self.mutate_vector(&combined, mutation)]))
    }

    fn load_entries(&self) -> Result<Vec<MemoryEntry>> {
        ensure_parent_exists(&self.path)?;
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let entries: Vec<MemoryEntry> = serde_json::from_reader(reader)?;
        Ok(entries)
    }

    fn write_entries(&self, entries: &[MemoryEntry]) -> Result<()> {
        ensure_parent_exists(&self.path)?;
        let serialized = serde_json::to_string_pretty(entries)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }

    fn mutate_vector(&self, vector: &[f64], mutation: f64) -> Vec<f64> {
        if mutation <= 0.0 {
            return vector.to_vec();
        }
        let mut rng = rand::thread_rng();
        vector
            .iter()
            .map(|value| {
                let delta = rng.gen_range(-mutation..=mutation);
                value + delta
            })
            .collect()
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
