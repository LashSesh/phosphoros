# PHOSPHOROS Kryptogenetik Core

> **Prä-holographisches System zur 5D-Skalarprojektion**  
> Integration von Triton, Metatron-IUL und Gabriel Cells

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Production Ready](https://img.shields.io/badge/status-production--ready-green.svg)]()

---

## 🌟 Überblick

**PHOSPHOROS Kryptogenetik Core** ist das Herz des PHOSPHOROS Blockchain-Forensik-Systems. Es implementiert die mathematisch-formale 5D-Informationsgenetik zur topologischen Kartierung von BIP39-Seed-Spaces.

### Kernkonzepte

- **5D-Embedding**: Metatron Sacred Geometry (13-Node → 5D Projektion)
- **Gabriel Cells**: Resonite-Netzwerke mit (ψ, ρ, ω) Dynamik
- **Triton Spiral**: Golden Spiral Navigation mit Ouroboros-Feedback
- **QDASH Explorer**: Solve et Coagula mit Merkaba-Gate Logic
- **TIC-Kristallisation**: Temporal Invariant Crystal Formation

---

## 🚀 Quick Start

### Installation

```bash
# Add to your Cargo.toml
[dependencies]
phosphoros-kryptogenetik = { path = "../phosphoros-kryptogenetik" }
```

### Basis-Verwendung

```rust
use phosphoros_kryptogenetik::PhosphorosCore;

fn main() {
    // 1. Core initialisieren
    let mut core = PhosphorosCore::new();
    
    // 2. Seed-Phrase embedden
    let seed_words = vec!["abandon", "ability", "able", "about"];
    let embeddings = core.embed_seed_phrase(&seed_words);
    
    // 3. Infogenom erstellen
    core.create_infogenom("analyzer".to_string(), 8);
    
    // 4. QDASH Exploration
    let result = core.explore_keyspace("analyzer", 0x1337, 500)
        .expect("Exploration failed");
    
    println!("Best Resonance: {:.6}", result.best_resonance);
    println!("TIC Crystals: {}", result.num_crystals);
}
```

---

## 🏗️ Architektur

### Layer-Struktur

```
┌─────────────────────────────────────────────────────────┐
│  PHOSPHOROS Integration Layer                           │
│  ┌───────────────────────────────────────────────────┐  │
│  │ PhosphorosCore Bridge API                         │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  High-Level Components                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ QDASHExplorer│  │ Infogenom    │  │ Triton Spiral│  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
├─────────────────────────────────────────────────────────┤
│  Core Primitives                                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Gabriel Cell │  │ Metatron Geo │  │ Point5D      │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## 🧬 Komponenten-Details

### 1. Point5D - 5D-Geometrie

```rust
// L2-Norm
point.norm()
// Normalisierung
point.normalize()
// Dot-Product
point.dot(other)
// Euklidische Distanz
point.distance(other)
```

### 2. MetatronGeometry - Sacred Geometry

13 kanonische Metatron-Knoten:
- Center (0)
- Hexagon (1-6)
- Cube Vertices (7-12)

### 3. SpectralSignature - (ψ, ρ, ω)

```rust
let signature = SpectralSignature::new(psi, rho, omega);
let resonance = signature.resonance();  // D = ψ·ρ·ω
```

### 4. TritonSpiralGenerator - Golden Spiral

```rust
let mut spiral = TritonSpiralGenerator::new(seed);
let point = spiral.generate_next();
spiral.update_ouroboros(signature);
```

---

## 🔧 Build & Test

### Standard Build

```bash
cargo build --release
```

### Mit BIP39 Integration

```bash
cargo build --release --features bip39-integration
```

### Alle Features

```bash
cargo build --release --all-features
```

### Tests

```bash
cargo test
cargo test --all-features
```

### Benchmarks

```bash
cargo bench
```

### Dokumentation

```bash
cargo doc --open
```

---

## 📊 Features

- `bip39-integration`: BIP39 mnemonic parsing support
- `advanced-linalg`: Extended linear algebra with nalgebra/ndarray
- `parallel`: Parallel processing with rayon

---

## ✅ Quality Gates

All quality gates pass:

```bash
✓ cargo build --all-features --release
✓ cargo test --all-features
✓ cargo fmt -- --check
✓ cargo clippy -- -D warnings
✓ cargo bench (compiles)
✓ cargo doc
```

---

## 🧪 Examples

Run the included examples:

```bash
cargo run --example basic_exploration --release
cargo run --example seed_analysis
cargo run --example metatron_visualization
```

---

## 📄 Lizenz

Dual-lizenziert unter MIT oder Apache 2.0.

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

---

## 🔐 Sicherheitshinweis

**WICHTIG**: Dieses System ist für Forschung und Forensik gedacht. Es sollte NIEMALS verwendet werden, um:
- Private Keys zu generieren
- Seed-Phrases zu speichern
- Produktiv-Wallets zu verwalten

Verwende immer offizielle, geprüfte Wallet-Software für echte Kryptowährungen.

---

*Gebaut mit ❤️ für die Blockchain-Forensik-Community*
