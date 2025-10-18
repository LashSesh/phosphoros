//! Forensic engine for the Satellite subsystem.
//!
//! This module provides the central orchestration engine that coordinates
//! ingestion, analysis, and data retention.

use std::sync::Arc;

use parking_lot::RwLock;
use uuid::Uuid;

use crate::analysis::AnalyticsPipeline;
use crate::config::SatelliteConfig;
use crate::error::{Result, SatelliteError};
use crate::models::{AnalysisReport, AnalysisRequest, SnapshotIngest, SnapshotRecord};
use crate::state::SatelliteState;

/// Central engine that orchestrates ingestion, analytics, and data retention.
#[derive(Debug)]
pub struct SatelliteEngine {
    config: SatelliteConfig,
    pipeline: AnalyticsPipeline,
    state: Arc<SatelliteState>,
    last_report: RwLock<Option<AnalysisReport>>,
}

impl SatelliteEngine {
    /// Creates a new Satellite engine with the given configuration.
    pub fn new(config: SatelliteConfig) -> Self {
        let analysis_config = config.analysis.clone();
        let pipeline = AnalyticsPipeline::new(analysis_config.clone());
        let state = Arc::new(SatelliteState::new(&analysis_config));
        Self {
            config,
            pipeline,
            state,
            last_report: RwLock::new(None),
        }
    }

    /// Returns a reference to the configuration.
    pub fn config(&self) -> &SatelliteConfig {
        &self.config
    }

    /// Ingests a new snapshot.
    ///
    /// The snapshot is validated, normalized, and stored. Returns the stored record.
    pub fn ingest(&self, ingest: SnapshotIngest) -> Result<SnapshotRecord> {
        let record = SnapshotRecord::new(ingest)?;
        #[cfg(feature = "api")]
        {
            let id = record.id;
            tracing::info!(snapshot_id = %id, "ingested snapshot");
        }
        self.state.insert(record.clone());
        Ok(record)
    }

    /// Lists all stored snapshots.
    pub fn list_snapshots(&self) -> Vec<SnapshotRecord> {
        self.state.list()
    }

    /// Analyzes a snapshot by ID.
    ///
    /// Returns an error if the snapshot is not found.
    pub fn analyze(&self, id: Uuid, request: AnalysisRequest) -> Result<AnalysisReport> {
        let snapshot = self
            .state
            .get(&id)
            .ok_or_else(|| SatelliteError::SnapshotNotFound(id.to_string()))?;
        let report = self.pipeline.run(&snapshot, request);
        *self.last_report.write() = Some(report.clone());
        Ok(report)
    }

    /// Returns the most recent analysis report, if any.
    pub fn last_report(&self) -> Option<AnalysisReport> {
        self.last_report.read().clone()
    }

    /// Returns a reference to the state.
    pub fn state(&self) -> Arc<SatelliteState> {
        Arc::clone(&self.state)
    }
}

impl Default for SatelliteEngine {
    fn default() -> Self {
        Self::new(SatelliteConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EntityObservation;
    use std::collections::HashMap;

    #[test]
    fn test_engine_ingestion() {
        let engine = SatelliteEngine::default();
        
        let ingest = SnapshotIngest {
            label: "test".to_string(),
            observations: vec![EntityObservation {
                id: Uuid::new_v4(),
                address: "0xtest".to_string(),
                features: vec![1.0, 2.0, 3.0],
                connections: vec![],
                metadata: HashMap::new(),
            }],
            context: HashMap::new(),
        };
        
        let record = engine.ingest(ingest).unwrap();
        assert_eq!(engine.list_snapshots().len(), 1);
        
        // Should be able to analyze it
        let report = engine.analyze(record.id, AnalysisRequest::default()).unwrap();
        assert_eq!(report.snapshot_id, record.id);
    }
}
