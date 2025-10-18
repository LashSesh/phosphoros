//! Basic QDASH exploration example

use phosphoros_kryptogenetik::PhosphorosCore;

fn main() {
    println!("=== PHOSPHOROS Kryptogenetik - Basic Exploration ===\n");

    // Initialize core
    let mut core = PhosphorosCore::new();
    println!("✓ PhosphorosCore initialized");

    // Embed seed phrase
    let seed_words = vec!["abandon", "ability", "able", "about"];
    let embeddings = core.embed_seed_phrase(&seed_words);
    println!("✓ Embedded {} words into 5D space", embeddings.len());

    // Show first embedding
    if let Some(emb) = embeddings.first() {
        println!(
            "  First embedding: [{:.3}, {:.3}, {:.3}, {:.3}, {:.3}]",
            emb.coords[0], emb.coords[1], emb.coords[2], emb.coords[3], emb.coords[4]
        );
    }

    // Create Infogenom
    core.create_infogenom("analyzer".to_string(), 8);
    println!("✓ Created Infogenom with 8 Gabriel Cells");

    // Explore keyspace
    let result = core
        .explore_keyspace("analyzer", 0x1337, 500)
        .expect("Exploration failed");

    println!("\n=== Results ===");
    println!("Steps: {}", result.steps);
    println!("Best Resonance: {:.6}", result.best_resonance);
    println!("TIC Crystals: {}", result.num_crystals);
    println!(
        "Final Position: [{:.3}, {:.3}, {:.3}, {:.3}, {:.3}]",
        result.final_position.coords[0],
        result.final_position.coords[1],
        result.final_position.coords[2],
        result.final_position.coords[3],
        result.final_position.coords[4]
    );
}
