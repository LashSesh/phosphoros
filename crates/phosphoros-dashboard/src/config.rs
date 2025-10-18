//! Application configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window configuration
    pub window: WindowConfig,
    /// Theme configuration
    pub theme: ThemeConfig,
    /// Service configuration
    pub services: ServiceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            services: ServiceConfig::default(),
        }
    }
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Minimum width
    pub min_width: f32,
    /// Minimum height
    pub min_height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1600.0,
            height: 900.0,
            min_width: 1280.0,
            min_height: 720.0,
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Dark mode enabled
    pub dark_mode: bool,
    /// Language
    pub language: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            language: "English".to_string(),
        }
    }
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Auto-start services
    pub auto_start: bool,
    /// Scraper enabled
    pub scraper_enabled: bool,
    /// Analyzer enabled
    pub analyzer_enabled: bool,
    /// Cluster engine enabled
    pub cluster_engine_enabled: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            scraper_enabled: true,
            analyzer_enabled: true,
            cluster_engine_enabled: true,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get default config path
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("phosphoros")
            .join("config.json")
    }
}
