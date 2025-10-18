//! Panel definitions and state management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Panel identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelId {
    /// Live Overview (Home)
    Home,
    /// Seed & Wallet Management
    SeedManagement,
    /// Resonance & Spectrography
    Resonance,
    /// Cluster Explorer & Forensik
    ClusterExplorer,
    /// System Log / Notifications
    SystemLog,
    /// Settings & Tasks
    Settings,
}

impl PanelId {
    /// Get panel display name
    pub fn name(&self) -> &'static str {
        match self {
            PanelId::Home => "Live Overview",
            PanelId::SeedManagement => "Seed & Wallet",
            PanelId::Resonance => "Resonance & Spectro",
            PanelId::ClusterExplorer => "Cluster Explorer",
            PanelId::SystemLog => "System Log",
            PanelId::Settings => "Settings & Tasks",
        }
    }

    /// Get panel icon (using simple unicode symbols)
    pub fn icon(&self) -> &'static str {
        match self {
            PanelId::Home => "🏠",
            PanelId::SeedManagement => "🔑",
            PanelId::Resonance => "📊",
            PanelId::ClusterExplorer => "🔍",
            PanelId::SystemLog => "📝",
            PanelId::Settings => "⚙️",
        }
    }

    /// Get all panel IDs
    pub fn all() -> Vec<PanelId> {
        vec![
            PanelId::Home,
            PanelId::SeedManagement,
            PanelId::Resonance,
            PanelId::ClusterExplorer,
            PanelId::SystemLog,
            PanelId::Settings,
        ]
    }
}

/// Panel state container
#[derive(Debug, Default)]
pub struct PanelState {
    /// Home panel state
    pub home: HomeState,
    /// Seed management state
    pub seed_management: SeedManagementState,
    /// Resonance panel state
    pub resonance: ResonanceState,
    /// Cluster explorer state
    pub cluster: ClusterState,
    /// System log state
    pub log: LogState,
    /// Settings state
    pub settings: SettingsState,
}

/// Home panel state
#[derive(Debug, Default)]
pub struct HomeState {
    /// Active tasks count
    pub active_tasks: usize,
    /// Seeds collected
    pub seeds_count: usize,
    /// Clusters discovered
    pub clusters_count: usize,
    /// Coverage percentage
    pub coverage: f64,
    /// Activity history (last 100 points)
    pub activity_history: Vec<f64>,
}

/// Seed management state
#[derive(Debug, Default)]
pub struct SeedManagementState {
    /// Current input
    pub input: String,
    /// Imported seeds
    pub seeds: Vec<SeedInfo>,
    /// Currently selected seed index
    pub selected: Option<usize>,
}

/// Seed information
#[derive(Debug, Clone)]
pub struct SeedInfo {
    /// Seed ID
    pub id: String,
    /// Mnemonic phrase (masked)
    pub mnemonic_masked: String,
    /// Generated addresses
    pub addresses: Vec<AddressInfo>,
    /// Import timestamp
    pub imported_at: chrono::DateTime<chrono::Utc>,
}

/// Address information
#[derive(Debug, Clone)]
pub struct AddressInfo {
    /// Address string
    pub address: String,
    /// Chain type
    pub chain: String,
    /// Derivation path
    pub path: String,
}

/// Resonance panel state
#[derive(Debug, Default)]
pub struct ResonanceState {
    /// Analysis running
    pub running: bool,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Best resonance found
    pub best_resonance: f64,
    /// Operator configurations
    pub operators: HashMap<String, f64>,
    /// Resonance history for visualization
    pub resonance_history: Vec<(f64, f64, f64)>, // (psi, rho, omega)
}

/// Cluster explorer state
#[derive(Debug, Default)]
pub struct ClusterState {
    /// Search query
    pub search_query: String,
    /// Found clusters
    pub clusters: Vec<ClusterInfo>,
    /// Selected cluster
    pub selected: Option<usize>,
}

/// Cluster information
#[derive(Debug, Clone)]
pub struct ClusterInfo {
    /// Cluster ID
    pub id: String,
    /// Members count
    pub members: usize,
    /// Resonance magnitude
    pub resonance: f64,
    /// Discovery timestamp
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

/// System log state
#[derive(Debug, Default)]
pub struct LogState {
    /// Filter string
    pub filter: String,
    /// Auto-scroll enabled
    pub auto_scroll: bool,
}

/// Settings state
#[derive(Debug)]
pub struct SettingsState {
    /// Theme (dark/light)
    pub dark_mode: bool,
    /// Language
    pub language: String,
    /// Task priorities
    pub task_priorities: HashMap<String, usize>,
    /// Auto-start services
    pub auto_start: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            dark_mode: true,
            language: "English".to_string(),
            task_priorities: HashMap::new(),
            auto_start: true,
        }
    }
}
