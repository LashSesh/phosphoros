//! Integration tests for phosphoros-kryptogenetik

use phosphoros_kryptogenetik::*;

#[test]
fn test_complete_pipeline() {
    let mut core = PhosphorosCore::new();

    // Embed seed phrase
    let words = vec!["abandon", "ability", "able"];
    let embeddings = core.embed_seed_phrase(&words);
    assert_eq!(embeddings.len(), 3);

    // All embeddings should be normalized
    for emb in &embeddings {
        assert!((emb.norm() - 1.0).abs() < 1e-10);
    }

    // Create infogenom
    core.create_infogenom("test".to_string(), 8);

    // Explore
    let result = core.explore_keyspace("test", 42, 100).unwrap();
    assert_eq!(result.steps, 100);
    assert!(result.best_resonance >= 0.0);
    assert!(result.best_resonance <= 1.0);
}

#[test]
fn test_determinism() {
    let mut core1 = PhosphorosCore::new();
    let mut core2 = PhosphorosCore::new();

    let words = vec!["test", "seed", "phrase"];
    let emb1 = core1.embed_seed_phrase(&words);
    let emb2 = core2.embed_seed_phrase(&words);

    assert_eq!(emb1, emb2);

    core1.create_infogenom("test1".to_string(), 4);
    core2.create_infogenom("test2".to_string(), 4);

    let result1 = core1.explore_keyspace("test1", 42, 50).unwrap();
    let result2 = core2.explore_keyspace("test2", 42, 50).unwrap();

    assert_eq!(result1.steps, result2.steps);
    assert_eq!(result1.best_resonance, result2.best_resonance);
}

#[test]
fn test_point5d_operations() {
    let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
    let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);

    // Norm
    assert!((p1.norm() - 1.0).abs() < 1e-10);

    // Dot product (orthogonal)
    assert!(p1.dot(&p2).abs() < 1e-10);

    // Distance
    let dist = p1.distance(&p2);
    assert!((dist - 2.0_f64.sqrt()).abs() < 1e-10);

    // Addition
    let p3 = p1.add(&p2);
    assert!((p3.coords[0] - 1.0).abs() < 1e-10);
    assert!((p3.coords[1] - 1.0).abs() < 1e-10);

    // Scaling
    let p4 = p1.scale(2.0);
    assert!((p4.coords[0] - 2.0).abs() < 1e-10);
}

#[test]
fn test_spectral_signature() {
    let sig = SpectralSignature::new(0.8, 0.9, 0.7);

    // Check clamping
    assert_eq!(sig.psi, 0.8);
    assert_eq!(sig.rho, 0.9);
    assert_eq!(sig.omega, 0.7);

    // Check resonance (INVARIANT)
    let resonance = sig.resonance();
    assert!((resonance - 0.504).abs() < 1e-10);

    // Test clamping with out-of-range values
    let sig2 = SpectralSignature::new(1.5, -0.5, 0.5);
    assert_eq!(sig2.psi, 1.0);
    assert_eq!(sig2.rho, 0.0);
    assert_eq!(sig2.omega, 0.5);
}

#[test]
fn test_metatron_geometry() {
    let metatron = MetatronGeometry::new();

    // Check non-center nodes are normalized (skip node 0 which is at origin)
    for i in 1..13 {
        let node = metatron.get_node_5d(i);
        assert!((node.norm() - 1.0).abs() < 1e-10);
    }

    // Test embedding determinism
    let emb1 = metatron.embed_object(12345);
    let emb2 = metatron.embed_object(12345);
    assert_eq!(emb1, emb2);

    // Test embedding is normalized
    assert!((emb1.norm() - 1.0).abs() < 1e-10);
}

#[test]
fn test_gabriel_cell() {
    let mut cell = GabrielCell::new("test".to_string());
    let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

    let output = cell.evaluate(&point);
    assert!(output >= 0.0 && output <= 1.0);

    let sig = cell.get_signature();
    assert!(sig.psi >= 0.0 && sig.psi <= 1.0);
    assert!(sig.rho >= 0.0 && sig.rho <= 1.0);
    assert!(sig.omega >= 0.0 && sig.omega <= 1.0);
}

#[test]
fn test_triton_spiral() {
    let mut spiral = TritonSpiralGenerator::new(42);

    let p1 = spiral.generate_next();
    let p2 = spiral.generate_next();

    // Points should be different
    assert!(p1.distance(&p2) > 0.0);

    // Both should be normalized
    assert!((p1.norm() - 1.0).abs() < 1e-10);
    assert!((p2.norm() - 1.0).abs() < 1e-10);
}

#[test]
fn test_infogenom() {
    let mut infogenom = Infogenom::new("test".to_string(), 8);
    let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

    let scores = infogenom.evaluate(&point);
    assert_eq!(scores.len(), 8);

    for score in &scores {
        assert!(*score >= 0.0 && *score <= 1.0);
    }
}

#[test]
fn test_qdash_explorer() {
    let infogenom = Infogenom::new("test".to_string(), 4);
    let mut explorer = QDASHExplorer::new(infogenom, 42, 100);

    let result = explorer.explore();

    assert_eq!(result.steps, 100);
    assert!(result.best_resonance >= 0.0);
    assert!(result.num_crystals >= 0);
}

#[test]
fn test_error_handling() {
    let mut core = PhosphorosCore::new();

    // Try to explore with non-existent infogenom
    let result = core.explore_keyspace("nonexistent", 42, 100);
    assert!(result.is_err());
}
