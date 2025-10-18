// ============================================================================
// PHOSPHOROS - Vollständiges Anwendungsbeispiel
// ============================================================================
// Demonstriert die 5D-Kryptogenetik-Analyse einer BIP39 Seed-Phrase
//
// Dieses Beispiel zeigt:
// 1. Seed-Phrase Embedding in 5D via Metatron
// 2. Spiral-Navigation mit Triton
// 3. Infogenom-basierte Analyse
// 4. TIC-Kristallisation
// 5. Semantische Mutation
// ============================================================================

use phosphoros_kryptogenetik::*;
use std::time::Instant;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  PHOSPHOROS Kryptogenetik - 5D Seed-Phrase Analysis         ║");
    println!("║  Prä-holographisches System zur Skalarprojektion            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // PHASE 1: Initialisierung
    // ========================================================================
    println!("🔧 Phase 1: System-Initialisierung");
    println!("─────────────────────────────────────────────────────────────────");
    
    let mut core = PhosphorosCore::new();
    println!("✓ PhosphorosCore initialisiert");
    println!("✓ Metatron-Geometrie (13 Nodes) aktiviert");
    println!();

    // ========================================================================
    // PHASE 2: Seed-Phrase 5D-Embedding
    // ========================================================================
    println!("🧬 Phase 2: Seed-Phrase Embedding");
    println!("─────────────────────────────────────────────────────────────────");
    
    // Beispiel BIP39 Seed-Phrase (erste 12 Wörter)
    let seed_phrase = vec![
        "abandon", "ability", "able", "about", 
        "above", "absent", "absorb", "abstract",
        "absurd", "abuse", "access", "accident"
    ];
    
    println!("Original Seed-Phrase:");
    println!("  {}", seed_phrase.join(" "));
    println!();

    let start = Instant::now();
    let embeddings = core.embed_seed_phrase(&seed_phrase);
    let embed_time = start.elapsed();
    
    println!("✓ 5D-Embedding abgeschlossen in {:?}", embed_time);
    println!("✓ {} Wörter → {} 5D-Vektoren", seed_phrase.len(), embeddings.len());
    println!();

    // Zeige erste 3 Embeddings
    println!("Beispiel-Embeddings:");
    for (i, emb) in embeddings.iter().take(3).enumerate() {
        println!("  Wort[{}]: [{:.3}, {:.3}, {:.3}, {:.3}, {:.3}] (norm: {:.3})",
                 i,
                 emb.coords[0], emb.coords[1], emb.coords[2], 
                 emb.coords[3], emb.coords[4],
                 emb.norm());
    }
    println!();

    // ========================================================================
    // PHASE 3: Infogenom-Konstruktion
    // ========================================================================
    println!("🧠 Phase 3: Infogenom-Netzwerk Konstruktion");
    println!("─────────────────────────────────────────────────────────────────");
    
    core.create_infogenom("seed_analyzer".to_string(), 8);
    println!("✓ Infogenom 'seed_analyzer' erstellt");
    println!("✓ 8 Gabriel Cells (Resonites) initialisiert");
    println!("✓ Coupling Strength: 0.5");
    println!();

    // ========================================================================
    // PHASE 4: QDASH Exploration
    // ========================================================================
    println!("🌀 Phase 4: QDASH Exploration (Triton Spiral)");
    println!("─────────────────────────────────────────────────────────────────");
    
    let exploration_steps = 500;
    let seed = 0x1337_C0DE_BEEF_FACE;
    
    println!("Parameter:");
    println!("  Max Steps: {}", exploration_steps);
    println!("  RNG Seed: 0x{:016X}", seed);
    println!();

    let start = Instant::now();
    let result = core.explore_keyspace("seed_analyzer", seed, exploration_steps)
        .expect("Exploration fehlgeschlagen");
    let explore_time = start.elapsed();
    
    println!("✓ Exploration abgeschlossen in {:?}", explore_time);
    println!();

    // ========================================================================
    // PHASE 5: Ergebnis-Analyse
    // ========================================================================
    println!("📊 Phase 5: Ergebnis-Analyse");
    println!("─────────────────────────────────────────────────────────────────");
    
    println!("Exploration-Metriken:");
    println!("  • Durchlaufene Schritte: {}", result.steps);
    println!("  • Beste Resonanz (D_max): {:.6}", result.best_resonance);
    println!("  • TIC-Kristalle gebildet: {}", result.num_crystals);
    println!();
    
    println!("Finale Position:");
    println!("  [{:.6}, {:.6}, {:.6}, {:.6}, {:.6}]",
             result.final_position.coords[0],
             result.final_position.coords[1],
             result.final_position.coords[2],
             result.final_position.coords[3],
             result.final_position.coords[4]);
    println!("  Norm: {:.6}", result.final_position.norm());
    println!();

    // ========================================================================
    // PHASE 6: Semantische Analyse der Original-Embeddings
    // ========================================================================
    println!("🔬 Phase 6: Semantische Kohärenz-Analyse");
    println!("─────────────────────────────────────────────────────────────────");
    
    analyze_embedding_coherence(&embeddings);
    println!();

    // ========================================================================
    // PHASE 7: Spiral-Konvergenz Simulation
    // ========================================================================
    println!("🔁 Phase 7: Ouroboros-Konvergenz Visualisierung");
    println!("─────────────────────────────────────────────────────────────────");
    
    visualize_spiral_convergence(seed);
    println!();

    // ========================================================================
    // FAZIT
    // ========================================================================
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  ANALYSE ABGESCHLOSSEN                                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Zusammenfassung:");
    println!("  ✓ 5D-Embedding: {} Vektoren generiert", embeddings.len());
    println!("  ✓ Metatron-Topologie: 13-Node Sacred Geometry");
    println!("  ✓ Gabriel Cells: 8 Resonites aktiv");
    println!("  ✓ Spiral-Navigation: {} Iterationen", result.steps);
    println!("  ✓ Beste Resonanz: {:.6}", result.best_resonance);
    println!("  ✓ TIC-Kristalle: {}", result.num_crystals);
    println!();
    println!("Das System ist bereit für PHOSPHOROS-Integration.");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn analyze_embedding_coherence(embeddings: &[Point5D]) {
    if embeddings.len() < 2 {
        println!("  Nicht genügend Embeddings für Kohärenz-Analyse");
        return;
    }

    // Berechne paarweise Distanzen
    let mut distances = Vec::new();
    let mut similarities = Vec::new();
    
    for i in 0..embeddings.len() {
        for j in (i+1)..embeddings.len() {
            let dist = embeddings[i].distance(&embeddings[j]);
            let sim = embeddings[i].dot(&embeddings[j]) / 
                     (embeddings[i].norm() * embeddings[j].norm() + 1e-12);
            distances.push(dist);
            similarities.push(sim);
        }
    }

    let mean_dist = distances.iter().sum::<f64>() / distances.len() as f64;
    let mean_sim = similarities.iter().sum::<f64>() / similarities.len() as f64;
    
    let var_dist: f64 = distances.iter()
        .map(|d| (d - mean_dist).powi(2))
        .sum::<f64>() / distances.len() as f64;
    let std_dist = var_dist.sqrt();

    println!("Kohärenz-Metriken:");
    println!("  • Mittlere Distanz: {:.6} ± {:.6}", mean_dist, std_dist);
    println!("  • Mittlere Ähnlichkeit: {:.6}", mean_sim);
    println!("  • Min Distanz: {:.6}", distances.iter().cloned().fold(f64::INFINITY, f64::min));
    println!("  • Max Distanz: {:.6}", distances.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    
    // Interpretiere Kohärenz
    let coherence_score = 1.0 / (1.0 + mean_dist);
    println!();
    println!("  → Kohärenz-Score: {:.4}", coherence_score);
    
    if coherence_score > 0.7 {
        println!("  → Bewertung: HOCH (stark strukturiert)");
    } else if coherence_score > 0.4 {
        println!("  → Bewertung: MITTEL (moderate Struktur)");
    } else {
        println!("  → Bewertung: NIEDRIG (schwach strukturiert)");
    }
}

fn visualize_spiral_convergence(seed: u64) {
    let mut spiral = TritonSpiralGenerator::new(seed);
    let mut evaluator = InformationAlchemyEvaluator::new(None);
    
    println!("Resonanz-Evolution über 20 Schritte:");
    println!();
    
    let steps = 20;
    let mut max_resonance = 0.0;
    
    for i in 0..steps {
        let point = spiral.generate_next();
        let signature = evaluator.evaluate(&point);
        let resonance = signature.resonance();
        
        if resonance > max_resonance {
            max_resonance = resonance;
        }
        
        spiral.update_ouroboros(signature);
        
        // ASCII-Visualisierung
        let bar_length = (resonance * 40.0) as usize;
        let bar: String = "█".repeat(bar_length);
        
        println!("  Step {:2}: {} {:.6}", i+1, bar, resonance);
    }
    
    println!();
    println!("  → Maximale Resonanz: {:.6}", max_resonance);
    println!("  → Ouroboros-Konvergenz: {}", 
             if max_resonance > 0.5 { "ERREICHT" } else { "IN PROGRESS" });
}

// ============================================================================
// ERWEITERTE ANALYSE-FUNKTIONEN
// ============================================================================

#[allow(dead_code)]
fn advanced_seed_mutation_analysis(core: &mut PhosphorosCore, seed_phrase: &[&str]) {
    println!("🧬 Erweiterte Mutations-Analyse");
    println!("─────────────────────────────────────────────────────────────────");
    
    // Original Embedding
    let original_embeddings = core.embed_seed_phrase(seed_phrase);
    let original_center = compute_centroid(&original_embeddings);
    
    println!("Original Phrase Zentroid:");
    println!("  [{:.4}, {:.4}, {:.4}, {:.4}, {:.4}]",
             original_center.coords[0],
             original_center.coords[1],
             original_center.coords[2],
             original_center.coords[3],
             original_center.coords[4]);
    println!();
    
    // Simuliere Mutationen
    let mutation_candidates = vec!["abstract", "absurd", "abuse"];
    
    println!("Mutations-Kandidaten:");
    for candidate in mutation_candidates {
        let mut mutated = seed_phrase.to_vec();
        mutated[mutated.len() - 1] = candidate;
        
        let mutated_embeddings = core.embed_seed_phrase(&mutated);
        let mutated_center = compute_centroid(&mutated_embeddings);
        
        let drift = original_center.distance(&mutated_center);
        let similarity = original_center.dot(&mutated_center) / 
                        (original_center.norm() * mutated_center.norm() + 1e-12);
        
        println!("  '{}' → Drift: {:.6}, Similarity: {:.6}", 
                 candidate, drift, similarity);
    }
}

fn compute_centroid(points: &[Point5D]) -> Point5D {
    let mut sum = Point5D::zero();
    for p in points {
        sum = sum.add(p);
    }
    sum.scale(1.0 / points.len() as f64)
}

// ============================================================================
// PERFORMANCE BENCHMARKING
// ============================================================================

#[allow(dead_code)]
fn benchmark_operations() {
    use std::time::Instant;
    
    println!("⚡ Performance Benchmarks");
    println!("─────────────────────────────────────────────────────────────────");
    
    let iterations = 10000;
    
    // Benchmark 1: Point5D Operations
    let p1 = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
    let p2 = Point5D::new(0.3, 0.7, 0.2, 0.8, 0.4);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = p1.dot(&p2);
        let _ = p1.distance(&p2);
        let _ = p1.add(&p2).normalize();
    }
    let elapsed = start.elapsed();
    println!("  Point5D Operations: {} ops in {:?}", 
             iterations * 3, elapsed);
    println!("    → {:.0} ops/sec", 
             (iterations * 3) as f64 / elapsed.as_secs_f64());
    
    // Benchmark 2: Metatron Embedding
    let metatron = MetatronGeometry::new();
    let start = Instant::now();
    for i in 0..iterations {
        let _ = metatron.embed_object(i as u64);
    }
    let elapsed = start.elapsed();
    println!("  Metatron Embeddings: {} ops in {:?}", iterations, elapsed);
    println!("    → {:.0} ops/sec", 
             iterations as f64 / elapsed.as_secs_f64());
    
    // Benchmark 3: Spiral Generation
    let mut spiral = TritonSpiralGenerator::new(42);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = spiral.generate_next();
    }
    let elapsed = start.elapsed();
    println!("  Spiral Generation: {} ops in {:?}", iterations, elapsed);
    println!("    → {:.0} ops/sec", 
             iterations as f64 / elapsed.as_secs_f64());
}
