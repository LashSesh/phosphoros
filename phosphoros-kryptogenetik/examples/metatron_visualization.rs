//! Metatron geometry visualization example

use phosphoros_kryptogenetik::{MetatronGeometry, Point5D};

fn main() {
    println!("=== Metatron Sacred Geometry Visualization ===\n");

    let metatron = MetatronGeometry::new();

    println!("13 Canonical Nodes in 5D:");
    println!("─────────────────────────────────────────");

    for i in 0..13 {
        let node = metatron.get_node_5d(i);
        print_point5d(i, &node);
    }

    println!("\n=== Object Embeddings ===\n");

    // Test deterministic embedding
    let hash1 = 0x1234_5678_9abc_def0;
    let emb1 = metatron.embed_object(hash1);
    let emb2 = metatron.embed_object(hash1);

    println!("Hash: 0x{:016X}", hash1);
    print_point5d(999, &emb1);
    println!("Deterministic: {}", emb1 == emb2);

    println!("\n=== Different Hash Values ===\n");
    for i in 0..5 {
        let hash = (i * 1000) as u64;
        let emb = metatron.embed_object(hash);
        println!("Hash {:5}: norm={:.6}", hash, emb.norm());
    }
}

fn print_point5d(index: usize, point: &Point5D) {
    println!(
        "  Node {:2}: [{:7.4}, {:7.4}, {:7.4}, {:7.4}, {:7.4}] norm={:.4}",
        index,
        point.coords[0],
        point.coords[1],
        point.coords[2],
        point.coords[3],
        point.coords[4],
        point.norm()
    );
}
