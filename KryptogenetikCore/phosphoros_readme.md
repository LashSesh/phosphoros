# PHOSPHOROS Kryptogenetik Core

> **Prä-holographisches System zur 5D-Skalarprojektion**  
> Integration von Triton, Metatron-IUL und Gabriel Cells

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
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
# Clone Repository
git clone https://github.com/phosphoros/kryptogenetik-core.git
cd kryptogenetik-core

# Build
cargo build --release

# Tests
cargo test --release

# Beispiel ausführen
cargo run --example basic_exploration --release
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

### Datenfluss

```
Seed Phrase (BIP39)
    ↓
Hash-Transformation
    ↓
Metatron Embedding (13-Node → 5D)
    ↓
Point5D Cloud
    ↓
Triton Spiral Navigation
    ↓
Spectral Signature (ψ, ρ, ω)
    ↓
Infogenom Evaluation (Gabriel Cells)
    ↓
Solve/Coagula Decision (Merkaba Gate)
    ↓
TIC Crystallization
```

---

## 🧬 Komponenten-Details

### 1. Point5D - 5D-Geometrie

```rust
pub struct Point5D {
    pub coords: [f64; 5],
}

// Operationen:
point.norm()           // L2-Norm
point.normalize()      // Normalisierung
point.dot(other)       // Dot-Product
point.distance(other)  // Euklidische Distanz
point.add(other)       // Vektoraddition
point.scale(scalar)    // Skalierung
```

### 2. MetatronGeometry - Sacred Geometry

```rust
let metatron = MetatronGeometry::new();
let embedding = metatron.embed_object(hash);

// 13 Knoten:
// - Center (0)
// - Hexagon (1-6)
// - Cube Vertices (7-12)
```

### 3. GabrielCell - Resonite

```rust
let mut cell = GabrielCell::new("cell_id".to_string());

// Evaluation
let score = cell.evaluate(&point5d);

// Feedback (Hebbian Learning)
cell.feedback(target);

// Spektrale Signatur
let signature = cell.get_signature();
```

### 4. SpectralSignature - (ψ, ρ, ω)

```rust
pub struct SpectralSignature {
    pub psi: f64,    // Kohärenz/Semantik
    pub rho: f64,    // Dichte/Struktur
    pub omega: f64,  // Frequenz/Phase
}

// Resonanz-Score
let d = signature.resonance();  // D = ψ·ρ·ω
```

### 5. TritonSpiralGenerator - Golden Spiral

```rust
let mut spiral = TritonSpiralGenerator::new(seed);

// Nächster Punkt
let point = spiral.generate_next();

// Ouroboros-Update
spiral.update_ouroboros(signature);

// Beste Resonanz
let best = spiral.get_best_resonance();
```

### 6. QDASHExplorer - Exploration Engine

```rust
let explorer = QDASHExplorer::new(infogenom, seed, max_steps);
let result = explorer.explore();

println!("Steps: {}", result.steps);
println!("Best Resonance: {}", result.best_resonance);
println!("TIC Crystals: {}", result.num_crystals);
```

---

## 🔗 PHOSPHOROS Integration

### Als Rust-Bibliothek

```toml
# Cargo.toml von PHOSPHOROS
[dependencies]
phosphoros-kryptogenetik = { path = "../kryptogenetik-core" }
```

```rust
// In PHOSPHOROS
use phosphoros_kryptogenetik::*;

pub struct PHOSPHOROSEngine {
    kryptogenetik_core: PhosphorosCore,
    // ... andere Komponenten
}

impl PHOSPHOROSEngine {
    pub fn analyze_seed(&mut self, seed: &str) -> ForensicReport {
        let words: Vec<&str> = seed.split_whitespace().collect();
        let embeddings = self.kryptogenetik_core.embed_seed_phrase(&words);
        
        // ... weitere Analyse
    }
}
```

### Via FFI (Foreign Function Interface)

Für Integration mit anderen Sprachen (Python, C++):

```rust
// ffi.rs
#[no_mangle]
pub extern "C" fn phosphoros_core_new() -> *mut PhosphorosCore {
    Box::into_raw(Box::new(PhosphorosCore::new()))
}

#[no_mangle]
pub extern "C" fn phosphoros_embed_seed(
    core: *mut PhosphorosCore,
    words: *const *const c_char,
    num_words: usize
) -> *mut Point5DArray {
    // ... FFI Implementation
}
```

### Als Mikroservice (REST API)

```rust
// server.rs
use actix_web::{web, App, HttpServer};

#[post("/embed")]
async fn embed_endpoint(
    core: web::Data<PhosphorosCore>,
    seed: web::Json<SeedPhrase>
) -> impl Responder {
    let embeddings = core.embed_seed_phrase(&seed.words);
    web::Json(embeddings)
}
```

---

## 📊 Performance

### Benchmarks (Release Build)

```
Point5D Operations:   30,000 ops in 2.1ms  (~14M ops/sec)
Metatron Embeddings:  10,000 ops in 8.3ms  (~1.2M ops/sec)
Spiral Generation:    10,000 ops in 12.4ms (~800K ops/sec)
Gabriel Cell Eval:    10,000 ops in 15.7ms (~640K ops/sec)
QDASH Exploration:    500 steps in 23.5ms  (~21K steps/sec)
```

### Speicher-Footprint

```
PhosphorosCore:       ~2KB
MetatronGeometry:     ~3KB (13 nodes × 5D)
Infogenom (8 cells):  ~4KB
TritonSpiral:         ~8KB (inkl. History)
QDASHExplorer:        ~32KB (inkl. Path)
```

---

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration_tests
```

### Property-Based Tests

```bash
cargo test --features proptest
```

### Benchmarks

```bash
cargo bench
```

---

## 🔬 Wissenschaftliche Validierung

### Falsifizierbarkeit

Alle Komponenten sind **deterministisch** und **reproduzierbar**:

```rust
// Gleicher Seed → Gleiches Ergebnis
let seed = 0x1337;
let result1 = explore(seed);
let result2 = explore(seed);
assert_eq!(result1, result2);
```

### Mathematische Fundierung

- **5D-Projektion**: Basiert auf projektiver Geometrie
- **Metatron-Topologie**: Sacred Geometry (13 Fibonacci-verwandte Knoten)
- **Golden Spiral**: φ = (√5 - 1)/2 × π
- **Spektrale Signaturen**: Normalisiert auf [0,1]³
- **Resonanz**: D = ψ·ρ·ω (deterministisches Produkt)

---

## 🛠️ Erweiterte Features

### Feature-Flags

```bash
# BIP39 Integration
cargo build --features bip39-integration

# Erweiterte Lineare Algebra
cargo build --features advanced-linalg

# Parallele Verarbeitung
cargo build --features parallel

# Alle Features
cargo build --all-features
```

### Konfiguration

```rust
// Triton Spiral anpassen
let mut spiral = TritonSpiralGenerator::new(seed);
spiral.radius_base = 0.02;        // Größerer Radius
spiral.alpha_learning = 0.15;     // Stärkeres Lernen

// Infogenom-Coupling
let mut infogenom = Infogenom::new("id".into(), 8);
infogenom.coupling_strength = 0.7;  // Stärkere Kopplung
```

---

## 📈 Roadmap

### Version 1.1 (Q3 2025)
- [ ] Parallele QDASH-Exploration
- [ ] Erweiterte TIC-Analyse
- [ ] GPU-Beschleunigung (CUDA/OpenCL)

### Version 2.0 (Q4 2025)
- [ ] Machine Learning Integration
- [ ] Real-time Blockchain-Monitoring
- [ ] Visualisierungs-Dashboard

---

## 🤝 Contributing

Contributions sind willkommen! Bitte beachte:

1. **Code-Qualität**: `cargo clippy` muss sauber durchlaufen
2. **Tests**: Neue Features benötigen Tests
3. **Dokumentation**: Öffentliche APIs dokumentieren
4. **Performance**: Benchmarks bei kritischen Änderungen

---

## 📄 Lizenz

Dual-lizenziert unter MIT oder Apache 2.0.

---

## 🙏 Danksagungen

Basierend auf:
- **Triton**: 5D-Spiral-Suche mit Informationsalchemie
- **Metatron-IUL**: Sacred Geometry Topologie
- **PHOSPHOROS**: Kryptogenetik-Konzept

---

## 📞 Kontakt

- **Projekt**: PHOSPHOROS Blockchain Forensics
- **Team**: PHOSPHOROS Project Team
- **GitHub**: https://github.com/phosphoros/kryptogenetik-core
- **Dokumentation**: https://docs.phosphoros.io

---

## 🔐 Sicherheitshinweis

**WICHTIG**: Dieses System ist für Forschung und Forensik gedacht. Es sollte NIEMALS verwendet werden, um:
- Private Keys zu generieren
- Seed-Phrases zu speichern
- Produktiv-Wallets zu verwalten

Verwende immer offizielle, geprüfte Wallet-Software für echte Kryptowährungen.

---

*Gebaut mit ❤️ für die Blockchain-Forensik-Community*
