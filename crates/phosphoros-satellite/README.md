# phosphoros-satellite

High-dimensional blockchain forensic analysis subsystem for PHOSPHOROS.

## Overview

`phosphoros-satellite` provides advanced blockchain forensic capabilities integrated
with the PHOSPHOROS resonance engine ecosystem. It analyzes wallet activity patterns,
detects anomalies, performs clustering, and generates topological summaries.

## Features

- **High-dimensional resonance analytics**: Vector geometry, clustering, and topological
  heuristics using `ndarray` and `petgraph`
- **Deterministic ingestion pipeline**: Normalizes entity vectors and maintains bounded
  archive of recent blockchain snapshots
- **Anomaly detection**: Identifies unusual wallet behavior using z-score analysis
- **Resonance hotspot detection**: Finds clusters of highly connected entities
- **Topological analysis**: Computes graph properties, articulation points, and Betti numbers
- **Entropy analysis**: Measures distribution of feature values
- **Optional REST API**: Feature-gated Axum-based API service (enable `api` feature)
- **Integration with PHOSPHOROS Core**: Seamless data flow with core resonance engines

## Usage

### Basic Analysis

```rust
use phosphoros_satellite::{SatelliteEngine, SatelliteConfig};
use phosphoros_satellite::models::{SnapshotIngest, EntityObservation, AnalysisRequest};
use uuid::Uuid;
use std::collections::HashMap;

// Create engine with default configuration
let engine = SatelliteEngine::default();

// Ingest a snapshot of blockchain entities
let ingest = SnapshotIngest {
    label: "eth_analysis_2024".to_string(),
    observations: vec![
        EntityObservation {
            id: Uuid::new_v4(),
            address: "0xdeadbeef...".to_string(),
            features: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            connections: vec![],
            metadata: HashMap::new(),
        }
    ],
    context: HashMap::new(),
};

let record = engine.ingest(ingest)?;

// Analyze the snapshot
let report = engine.analyze(record.id, AnalysisRequest::default())?;

println!("Resonance hotspots: {}", report.resonance_hotspots.len());
println!("Anomalies detected: {}", report.anomaly_scores.len());
println!("Components: {}", report.topology.components);
```

### With REST API

Enable the `api` feature to expose HTTP endpoints:

```toml
[dependencies]
phosphoros-satellite = { version = "0.1", features = ["api"] }
```

```rust
use std::sync::Arc;
use phosphoros_satellite::{SatelliteEngine, build_router};

#[tokio::main]
async fn main() {
    let engine = Arc::new(SatelliteEngine::default());
    let router = build_router(engine);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```

## Configuration

Configuration can be loaded from YAML or JSON:

```yaml
api:
  host: "0.0.0.0"
  port: 8080

analysis:
  knn_k: 8
  entropy_bins: 16
  resonance_threshold: 0.65
  max_snapshots: 32
```

```rust
use phosphoros_satellite::SatelliteConfig;

let config = SatelliteConfig::load_from_path("config.yaml")?;
let engine = SatelliteEngine::new(config);
```

## API Endpoints

When the `api` feature is enabled:

- `GET /health` - Health check
- `GET /v1/snapshots` - List all snapshots
- `POST /v1/snapshots` - Ingest a new snapshot
- `POST /v1/analyze/:id` - Run analysis on a snapshot
- `GET /v1/reports/latest` - Get the most recent analysis report

## Integration with PHOSPHOROS Core

The Satellite subsystem integrates seamlessly with PHOSPHOROS core components:

- Uses `phosphoros-core` for shared data structures
- Compatible with resonance engine trait system
- Can feed analysis results to other PHOSPHOROS modules
- Prepared for dashboard/UI integration

## Analysis Pipeline

The analytics pipeline performs:

1. **Feature Matrix Construction**: Builds normalized feature vectors
2. **Distance Matrix Computation**: Calculates L2 distances between entities
3. **KNN Graph Construction**: Builds k-nearest-neighbor graph
4. **Resonance Hotspot Detection**: Identifies high-resonance clusters
5. **Anomaly Scoring**: Computes z-scores for unusual behavior
6. **Topological Analysis**: Analyzes graph structure and connectivity
7. **Entropy Calculation**: Measures feature distribution

## Features

- `default`: Core functionality only
- `api`: Enables REST API endpoints (requires `axum`, `tokio`, `tower`, `tracing`)

## License

Dual-licensed under MIT or Apache 2.0.
