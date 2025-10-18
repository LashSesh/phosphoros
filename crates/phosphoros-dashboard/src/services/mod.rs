//! Background services for autonomous operation

use parking_lot::RwLock;
use std::sync::Arc;

/// Service manager for autonomous background tasks
#[derive(Debug, Clone)]
pub struct ServiceManager {
    /// Scraper service state
    pub scraper: Arc<RwLock<ServiceState>>,
    /// Analyzer service state
    pub analyzer: Arc<RwLock<ServiceState>>,
    /// Cluster engine state
    pub cluster_engine: Arc<RwLock<ServiceState>>,
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self {
            scraper: Arc::new(RwLock::new(ServiceState::default())),
            analyzer: Arc::new(RwLock::new(ServiceState::default())),
            cluster_engine: Arc::new(RwLock::new(ServiceState::default())),
        }
    }
}

impl ServiceManager {
    /// Get active tasks count
    pub fn active_tasks(&self) -> usize {
        let mut count = 0;
        if self.scraper.read().running {
            count += 1;
        }
        if self.analyzer.read().running {
            count += 1;
        }
        if self.cluster_engine.read().running {
            count += 1;
        }
        count
    }

    /// Start all services
    pub fn start_all(&self) {
        self.scraper.write().running = true;
        self.analyzer.write().running = true;
        self.cluster_engine.write().running = true;
    }

    /// Pause all services
    pub fn pause_all(&self) {
        self.scraper.write().running = false;
        self.analyzer.write().running = false;
        self.cluster_engine.write().running = false;
    }
}

/// Service state
#[derive(Debug, Clone)]
pub struct ServiceState {
    /// Service running status
    pub running: bool,
    /// Items processed/analyzed
    pub processed: usize,
    /// Last update timestamp
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for ServiceState {
    fn default() -> Self {
        Self {
            running: false,
            processed: 0,
            last_update: None,
        }
    }
}
