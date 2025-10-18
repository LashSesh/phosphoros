use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{OuroborosError, Result};

const DEFAULT_PATH: &str = "data/collective_field.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldPoint {
    seed: String,
    vector: Vec<f64>,
    address: String,
    balance: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CollectiveField {
    path: PathBuf,
}

impl Default for CollectiveField {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_PATH),
        }
    }
}

impl CollectiveField {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn log_point(
        &self,
        seed: &str,
        vector: &[f64],
        address: &str,
        balance: Option<f64>,
    ) -> Result<()> {
        let mut points = self.load_points()?;
        points.push(FieldPoint {
            seed: seed.to_string(),
            vector: vector.to_vec(),
            address: address.to_string(),
            balance,
        });
        self.write_points(&points)
    }

    pub fn load_vectors(&self) -> Result<Vec<Vec<f64>>> {
        Ok(self.load_points()?.into_iter().map(|p| p.vector).collect())
    }

    fn load_points(&self) -> Result<Vec<FieldPoint>> {
        ensure_parent_exists(&self.path)?;
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let points: Vec<FieldPoint> = serde_json::from_reader(reader)?;
        Ok(points)
    }

    fn write_points(&self, points: &[FieldPoint]) -> Result<()> {
        ensure_parent_exists(&self.path)?;
        let serialized = serde_json::to_string_pretty(points)?;
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
