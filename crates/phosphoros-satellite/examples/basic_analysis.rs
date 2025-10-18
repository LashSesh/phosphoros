//! Example: Basic Satellite forensic analysis
//!
//! This example demonstrates:
//! - Creating a Satellite engine
//! - Ingesting blockchain entity snapshots
//! - Running forensic analysis
//! - Integrating with PHOSPHOROS core (Point5D, SpectralSignature)

use std::collections::HashMap;
use uuid::Uuid;

use phosphoros_satellite::{
    SatelliteEngine, SnapshotIngest, EntityObservation, AnalysisRequest,
    ToPoint5D, ToSpectralSignature,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PHOSPHOROS Satellite Forensic Analysis Example ===\n");

    // Create the Satellite engine
    let engine = SatelliteEngine::default();
    println!("✓ Satellite engine initialized\n");

    // Create sample blockchain entities (wallets)
    println!("Creating sample wallet observations...");
    let mut observations = Vec::new();
    
    // Normal wallets
    for i in 0..8 {
        observations.push(EntityObservation {
            id: Uuid::new_v4(),
            address: format!("0xnormal{:04x}", i),
            features: vec![
                0.3 + (i as f64) * 0.05,  // Feature 1
                0.4 + (i as f64) * 0.03,  // Feature 2
                0.5 + (i as f64) * 0.02,  // Feature 3
                0.6,                       // Feature 4
                0.7,                       // Feature 5
            ],
            connections: vec![],
            metadata: HashMap::new(),
        });
    }
    
    // Suspicious cluster (Sybil attack pattern)
    for i in 0..5 {
        observations.push(EntityObservation {
            id: Uuid::new_v4(),
            address: format!("0xsybil{:04x}", i),
            features: vec![
                0.85 + (i as f64) * 0.01,  // Very similar features (clustering)
                0.82 + (i as f64) * 0.01,
                0.88 + (i as f64) * 0.01,
                0.90,
                0.91,
            ],
            connections: vec![],
            metadata: {
                let mut map = HashMap::new();
                map.insert("suspicious".to_string(), serde_json::json!(true));
                map
            },
        });
    }
    
    // Anomalous wallet
    observations.push(EntityObservation {
        id: Uuid::new_v4(),
        address: "0xanomaly0001".to_string(),
        features: vec![0.05, 0.02, 0.98, 0.01, 0.03],  // Unusual pattern
        connections: vec![],
        metadata: HashMap::new(),
    });
    
    println!("✓ Created {} wallet observations\n", observations.len());

    // Ingest snapshot
    let snapshot = SnapshotIngest {
        label: "Example Blockchain Snapshot".to_string(),
        observations,
        context: {
            let mut ctx = HashMap::new();
            ctx.insert("network".to_string(), serde_json::json!("ethereum"));
            ctx.insert("block_height".to_string(), serde_json::json!(12345678));
            ctx
        },
    };
    
    println!("Ingesting snapshot...");
    let record = engine.ingest(snapshot)?;
    println!("✓ Snapshot ingested: {}\n", record.id);

    // Run analysis
    println!("Running forensic analysis...");
    let report = engine.analyze(record.id, AnalysisRequest {
        knn_k: Some(5),
        entropy_bins: Some(16),
        resonance_threshold: Some(0.7),
    })?;
    
    println!("✓ Analysis complete!\n");

    // Display results
    println!("=== ANALYSIS RESULTS ===\n");
    
    println!("📊 Topology Summary:");
    println!("  Connected Components: {}", report.topology.components);
    println!("  Articulation Points: {}", report.topology.articulation_points);
    println!("  Betti Numbers: {:?}\n", report.topology.betti_estimate);
    
    println!("🔥 Resonance Hotspots: {}", report.resonance_hotspots.len());
    for (i, hotspot) in report.resonance_hotspots.iter().take(3).enumerate() {
        println!("  Hotspot #{}: magnitude={:.3}, neighbors={}", 
                 i+1, hotspot.magnitude, hotspot.neighbours.len());
    }
    println!();
    
    println!("⚠️  Anomalies Detected: {}", report.anomaly_scores.len());
    let mut sorted_anomalies = report.anomaly_scores.clone();
    sorted_anomalies.sort_by(|a, b| b.z_score.abs().partial_cmp(&a.z_score.abs()).unwrap());
    for (i, anomaly) in sorted_anomalies.iter().take(5).enumerate() {
        println!("  #{}: z-score={:.3}", i+1, anomaly.z_score);
    }
    println!();
    
    println!("📈 Entropy Analysis:");
    println!("  Spectral Entropy: {:.3}", report.entropy.spectral);
    println!("  Distribution Bins: {}", report.entropy.distribution.len());
    println!();

    // Demonstrate PHOSPHOROS Core integration
    println!("=== PHOSPHOROS CORE INTEGRATION ===\n");
    
    println!("Converting entities to 5D space...");
    for (i, obs) in record.observations.iter().take(3).enumerate() {
        let point5d = obs.to_point5d()?;
        let spectral = obs.to_spectral_signature()?;
        
        println!("Entity #{} ({})", i+1, 
                 obs.address.chars().take(12).collect::<String>());
        println!("  Point5D: [{:.3}, {:.3}, {:.3}, {:.3}, {:.3}]",
                 point5d.coords[0], point5d.coords[1], point5d.coords[2],
                 point5d.coords[3], point5d.coords[4]);
        println!("  Spectral: ψ={:.3}, ρ={:.3}, ω={:.3}",
                 spectral.psi, spectral.rho, spectral.omega);
        println!("  Resonance: D={:.3}\n", spectral.psi * spectral.rho * spectral.omega);
    }

    println!("=== ANALYSIS COMPLETE ===");
    Ok(())
}
