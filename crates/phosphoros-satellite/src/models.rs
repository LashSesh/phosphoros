//! Data models for the Satellite forensic analysis subsystem.
//!
//! This module defines all data structures used for blockchain forensic analysis,
//! including entity observations, snapshots, analysis requests, and reports.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "api")]
use utoipa::ToSchema;

use crate::error::{Result, SatelliteError};

/// Represents a single entity (wallet, validator, smart-contract) captured during ingestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct EntityObservation {
    /// Unique identifier for this entity observation
    pub id: Uuid,
    /// Blockchain address or identifier
    pub address: String,
    /// Feature vector for analysis (normalized to unit vector)
    pub features: Vec<f64>,
    /// Connected entity IDs (for graph analysis)
    #[serde(default)]
    pub connections: Vec<Uuid>,
    /// Additional metadata about this entity
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl EntityObservation {
    /// Normalizes the feature vector to unit length.
    ///
    /// This ensures all entity observations are comparable in the feature space.
    pub fn normalize(&mut self) {
        let norm = self
            .features
            .iter()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt()
            .max(f64::EPSILON);
        for value in &mut self.features {
            *value /= norm;
        }
    }

    /// Validates the entity observation.
    pub fn validate(&self) -> Result<()> {
        if self.address.is_empty() {
            return Err(SatelliteError::InvalidInput(
                "Entity address cannot be empty".to_string(),
            ));
        }
        if self.features.is_empty() {
            return Err(SatelliteError::InvalidInput(
                "Entity must have at least one feature".to_string(),
            ));
        }
        Ok(())
    }
}

/// Raw ingestion payload accepted by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct SnapshotIngest {
    /// Human-readable label for this snapshot
    pub label: String,
    /// List of entity observations in this snapshot
    pub observations: Vec<EntityObservation>,
    /// Additional context metadata
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
}

impl SnapshotIngest {
    /// Validates the snapshot ingest payload.
    pub fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(SatelliteError::InvalidInput(
                "Snapshot label cannot be empty".to_string(),
            ));
        }
        if self.observations.is_empty() {
            return Err(SatelliteError::InvalidInput(
                "Snapshot must contain at least one observation".to_string(),
            ));
        }
        for obs in &self.observations {
            obs.validate()?;
        }
        Ok(())
    }
}

/// Stored snapshot enriched with timestamps and derived metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct SnapshotRecord {
    /// Unique identifier for this snapshot
    pub id: Uuid,
    /// Human-readable label
    pub label: String,
    /// Timestamp when this snapshot was captured
    pub captured_at: DateTime<Utc>,
    /// Entity observations (normalized)
    pub observations: Vec<EntityObservation>,
    /// Additional context metadata
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
}

impl SnapshotRecord {
    /// Creates a new snapshot record from an ingest payload.
    ///
    /// This normalizes all entity observations and assigns timestamps.
    pub fn new(mut ingest: SnapshotIngest) -> Result<Self> {
        ingest.validate()?;
        
        let id = Uuid::new_v4();
        let captured_at = Utc::now();
        
        // Normalize all observations
        for observation in &mut ingest.observations {
            observation.normalize();
        }
        
        Ok(SnapshotRecord {
            id,
            label: ingest.label,
            captured_at,
            observations: ingest.observations,
            context: ingest.context,
        })
    }
}

/// Request payload used to tune an analysis run.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct AnalysisRequest {
    /// Number of nearest neighbors for KNN graph construction
    #[serde(default)]
    pub knn_k: Option<usize>,
    /// Number of bins for entropy calculation
    #[serde(default)]
    pub entropy_bins: Option<usize>,
    /// Minimum resonance threshold for hotspot detection
    #[serde(default)]
    pub resonance_threshold: Option<f64>,
}

/// Result returned by the analytics pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct AnalysisReport {
    /// ID of the snapshot being analyzed
    pub snapshot_id: Uuid,
    /// Label of the snapshot
    pub label: String,
    /// When the snapshot was captured
    pub captured_at: DateTime<Utc>,
    /// Detected resonance hotspots
    pub resonance_hotspots: Vec<ResonanceHotspot>,
    /// Anomaly scores for entities
    pub anomaly_scores: Vec<AnomalyScore>,
    /// Topological analysis summary
    pub topology: TopologySummary,
    /// Entropy analysis summary
    pub entropy: EntropySummary,
}

/// A detected resonance hotspot in the entity graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct ResonanceHotspot {
    /// Origin entity ID
    pub origin: Uuid,
    /// Resonance magnitude (0.0 to 1.0)
    pub magnitude: f64,
    /// IDs of neighboring entities
    pub neighbours: Vec<Uuid>,
}

/// Anomaly score for a single entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct AnomalyScore {
    /// Entity ID
    pub entity: Uuid,
    /// Z-score indicating deviation from normal
    pub z_score: f64,
}

/// Summary of topological properties of the entity graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct TopologySummary {
    /// Number of connected components
    pub components: usize,
    /// Number of articulation points (critical nodes)
    pub articulation_points: usize,
    /// Estimated Betti numbers (topological invariants)
    pub betti_estimate: Vec<f64>,
}

/// Summary of entropy analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct EntropySummary {
    /// Spectral entropy value
    pub spectral: f64,
    /// Distribution of feature values across bins
    pub distribution: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_normalization() {
        let mut entity = EntityObservation {
            id: Uuid::new_v4(),
            address: "0xtest".to_string(),
            features: vec![3.0, 4.0],
            connections: vec![],
            metadata: HashMap::new(),
        };
        
        entity.normalize();
        
        let norm: f64 = entity.features.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_snapshot_validation() {
        let ingest = SnapshotIngest {
            label: "test".to_string(),
            observations: vec![],
            context: HashMap::new(),
        };
        
        assert!(ingest.validate().is_err());
    }
}
