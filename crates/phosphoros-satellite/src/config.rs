//! Configuration for the Satellite subsystem.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::{Result, SatelliteError};

/// Root configuration for the Satellite forensic subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteConfig {
    /// API configuration (optional, only needed if API feature is enabled)
    #[serde(default)]
    pub api: ApiConfig,
    
    /// Analysis configuration
    #[serde(default)]
    pub analysis: AnalysisConfig,
}

impl Default for SatelliteConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            analysis: AnalysisConfig::default(),
        }
    }
}

impl SatelliteConfig {
    /// Loads configuration from a file path.
    ///
    /// Supports both JSON and YAML formats (detected by content).
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        let config: SatelliteConfig = if raw.trim_start().starts_with('{') {
            serde_json::from_str(&raw)?
        } else {
            serde_yaml::from_str(&raw)
                .map_err(|e| SatelliteError::YamlError(e.to_string()))?
        };
        Ok(config)
    }
}

/// API server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Host to bind to
    #[serde(default = "ApiConfig::default_host")]
    pub host: String,
    
    /// Port to bind to
    #[serde(default = "ApiConfig::default_port")]
    pub port: u16,
    
    /// Allowed CORS origins
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
            cors_allowed_origins: vec![],
        }
    }
}

impl ApiConfig {
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }

    fn default_port() -> u16 {
        8080
    }
}

/// Analysis pipeline configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Number of nearest neighbors for KNN graph
    #[serde(default = "AnalysisConfig::default_knn_k")]
    pub knn_k: usize,
    
    /// Number of bins for entropy calculation
    #[serde(default = "AnalysisConfig::default_entropy_bins")]
    pub entropy_bins: usize,
    
    /// Minimum resonance threshold for hotspot detection
    #[serde(default = "AnalysisConfig::default_resonance_threshold")]
    pub resonance_threshold: f64,
    
    /// Maximum number of snapshots to retain
    #[serde(default = "AnalysisConfig::default_max_snapshots")]
    pub max_snapshots: usize,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            knn_k: Self::default_knn_k(),
            entropy_bins: Self::default_entropy_bins(),
            resonance_threshold: Self::default_resonance_threshold(),
            max_snapshots: Self::default_max_snapshots(),
        }
    }
}

impl AnalysisConfig {
    const fn default_knn_k() -> usize {
        8
    }

    const fn default_entropy_bins() -> usize {
        16
    }

    const fn default_resonance_threshold() -> f64 {
        0.65
    }

    const fn default_max_snapshots() -> usize {
        32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SatelliteConfig::default();
        assert_eq!(config.analysis.knn_k, 8);
        assert_eq!(config.api.port, 8080);
    }
}
