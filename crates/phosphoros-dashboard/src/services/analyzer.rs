//! Analyzer service for pattern recognition and anomaly detection
//!
//! This service analyzes entities from the data pool and detects
//! anomalies using the phosphoros-satellite analytics pipeline.

use super::{ServiceState, DataPool, AnomalyData};
use parking_lot::RwLock;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

/// Analyzer service for pattern recognition
pub struct AnalyzerService {
    state: Arc<RwLock<ServiceState>>,
    data_pool: Arc<RwLock<DataPool>>,
}

impl AnalyzerService {
    /// Create a new analyzer service
    pub fn new(state: Arc<RwLock<ServiceState>>, data_pool: Arc<RwLock<DataPool>>) -> Self {
        Self {
            state,
            data_pool,
        }
    }

    /// Start the analyzer service
    pub fn start(&self) {
        self.state.write().running = true;
    }

    /// Pause the analyzer service
    pub fn pause(&self) {
        self.state.write().running = false;
    }

    /// Execute one analysis step
    pub fn tick(&self) {
        let mut state = self.state.write();
        if !state.running {
            return;
        }

        // Get entities from data pool
        let mut pool = self.data_pool.write();
        
        // Collect entity data first to avoid borrow issues
        let entities: Vec<_> = pool.entities.values().cloned().collect();
        
        // Analyze entities for anomalies
        for entity in entities {
            // Simple anomaly detection: check if any feature is extreme
            let has_anomaly = entity.features.iter().any(|&f| f > 0.95 || f < 0.05);
            
            if has_anomaly {
                let anomaly = AnomalyData {
                    id: Uuid::new_v4(),
                    entity_id: entity.id,
                    score: entity.features.iter().map(|f| f.abs()).sum::<f64>() / entity.features.len() as f64,
                    reason: "Extreme feature values detected".to_string(),
                    timestamp: Utc::now(),
                };
                
                pool.anomalies.push(anomaly);
            }
        }

        // Update stats
        state.processed += 1;
        state.last_update = Some(Utc::now());
        pool.last_activity = Some(Utc::now());
    }

    /// Get statistics
    pub fn stats(&self) -> AnalyzerStats {
        let state = self.state.read();
        let pool = self.data_pool.read();
        AnalyzerStats {
            running: state.running,
            analyzed: state.processed,
            anomalies_found: pool.anomalies.len(),
            last_update: state.last_update,
        }
    }
}

/// Analyzer statistics
#[derive(Debug, Clone)]
pub struct AnalyzerStats {
    /// Running status
    pub running: bool,
    /// Items analyzed
    pub analyzed: usize,
    /// Anomalies found
    pub anomalies_found: usize,
    /// Last update
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}
