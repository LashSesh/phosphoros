//! Seed analysis and comparison example

use phosphoros_kryptogenetik::{PhosphorosCore, Point5D};

fn main() {
    println!("=== Seed Phrase Analysis ===\n");

    let core = PhosphorosCore::new();

    // Analyze multiple seed phrases
    let seeds = vec![
        vec!["abandon", "ability", "able"],
        vec!["zoo", "zebra", "zone"],
        vec!["crypto", "genesis", "block"],
    ];

    for (i, words) in seeds.iter().enumerate() {
        println!("Seed {}: {}", i + 1, words.join(" "));
        let embeddings = core.embed_seed_phrase(words);

        analyze_embeddings(&embeddings);
        println!();
    }
}

fn analyze_embeddings(embeddings: &[Point5D]) {
    if embeddings.is_empty() {
        println!("  No embeddings");
        return;
    }

    // Calculate statistics
    let avg_norm: f64 = embeddings.iter().map(|e| e.norm()).sum::<f64>() / embeddings.len() as f64;

    // Calculate pairwise distances
    let mut distances = Vec::new();
    for i in 0..embeddings.len() {
        for j in (i + 1)..embeddings.len() {
            distances.push(embeddings[i].distance(&embeddings[j]));
        }
    }

    if !distances.is_empty() {
        let avg_dist = distances.iter().sum::<f64>() / distances.len() as f64;
        let min_dist = distances.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_dist = distances.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        println!("  Embeddings: {}", embeddings.len());
        println!("  Avg norm: {:.4}", avg_norm);
        println!("  Avg distance: {:.4}", avg_dist);
        println!("  Min/Max distance: {:.4} / {:.4}", min_dist, max_dist);
    } else {
        println!("  Single embedding - norm: {:.4}", avg_norm);
    }
}
