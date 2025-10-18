//! PHOSPHOROS Satellite - Blockchain Forensic Analysis Subsystem
//!
//! This crate provides high-dimensional blockchain forensic analysis capabilities
//! integrated with the PHOSPHOROS ecosystem. It analyzes wallet activity, detects
//! anomalies, performs clustering, and generates topological summaries.
//!
//! # Features
//!
//! - **High-dimensional resonance analytics**: Vector geometry, clustering, and
//!   topological heuristics using `ndarray` and `petgraph`
//! - **Deterministic ingestion pipeline**: Normalizes entity vectors and maintains
//!   bounded archive of recent blockchain snapshots
//! - **Optional REST API**: Feature-gated Axum-based API for external integrations
//! - **Integration with PHOSPHOROS Core**: Seamless data flow with resonance engines
//!
//! # Example
//!
//! ```
//! use phosphoros_satellite::{SatelliteEngine, SatelliteConfig};
//! use phosphoros_satellite::models::{SnapshotIngest, EntityObservation};
//! use uuid::Uuid;
//! use std::collections::HashMap;
//!
//! let engine = SatelliteEngine::default();
//!
//! let ingest = SnapshotIngest {
//!     label: "demo".to_string(),
//!     observations: vec![
//!         EntityObservation {
//!             id: Uuid::new_v4(),
//!             address: "0xdeadbeef".to_string(),
//!             features: vec![0.1, 0.2, 0.3, 0.4, 0.5],
//!             connections: vec![],
//!             metadata: HashMap::new(),
//!         }
//!     ],
//!     context: HashMap::new(),
//! };
//!
//! let record = engine.ingest(ingest).unwrap();
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod analysis;
#[cfg(feature = "api")]
pub mod api;
pub mod config;
pub mod engine;
pub mod error;
pub mod models;
pub mod state;

// Re-export commonly used types
pub use analysis::AnalyticsPipeline;
#[cfg(feature = "api")]
pub use api::build_router;
pub use config::{AnalysisConfig, ApiConfig, SatelliteConfig};
pub use engine::SatelliteEngine;
pub use error::{Result, SatelliteError};
pub use models::{
    AnalysisReport, AnalysisRequest, AnomalyScore, EntityObservation, EntropySummary,
    ResonanceHotspot, SnapshotIngest, SnapshotRecord, TopologySummary,
};
pub use state::SatelliteState;
