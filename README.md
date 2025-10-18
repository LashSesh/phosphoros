# PHOSPHOROS - Prä-Holographisches System zur 5D-Skalarprojektion

> Core Integration + GUI-Ready Gateway Architecture

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**PHOSPHOROS** is a sophisticated system for topological mapping of cryptographic seed spaces using 5D-spectral analysis and holistic resonance engines.

## 🌟 Overview

PHOSPHOROS integrates advanced mathematical concepts including:
- **5D Spectral Geometry**: Metatron Sacred Geometry with 5D projection
- **Holistic Resonance**: Multi-layer resonance engine (Kosmokrator, Chronokrator, O.P.H.A.N.)
- **Topological Optimization**: Four operators (WT, SW, DK, PI) for search space reduction
- **Multichain Support**: BIP39 integration for Bitcoin, Ethereum, Substrate, and more
- **Infogenetic Mapping**: Spectral signatures (ψ, ρ, ω) for cryptographic analysis

## 📦 Workspace Structure

```
phosphoros/
├── crates/
│   ├── phosphoros-core/         # 5D geometry + holistic resonance
│   ├── cryptogenetik-core/      # Search + optimization
│   ├── phosphoros-bip39/        # Multichain wallet support
│   ├── phosphoros-cli/          # Command-line tools (planned)
│   └── phosphoros-gateway/      # REST/WS API (planned)
├── ouroboros_dna/              # Visualization tools
└── phosphoros-kryptogenetik/   # Legacy implementation
```

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros
cargo build --workspace --release
```

### Basic Usage

```rust
use phosphoros_core::{HolisticMatrix, ResonanceEngine};
use cryptogenetik_core::{PhosphorosCore, OperatorSet, HookSet};

// Create holistic resonance engine
let engine = HolisticMatrix::default_config();

// Create search core with operators
let mut core = PhosphorosCore::default_config(engine);

// Explore keyspace
let seed_words = vec!["abandon".to_string(), "ability".to_string()];
let result = core.explore(seed_words, 1000, 12345)?;

println!("Best resonance: {}", result.best_resonance);
println!("Pruning factor: {:.2}%", result.pruning_factor * 100.0);
```

## 🏗️ Architecture

### Layer 1: Core (`phosphoros-core`)

The foundation providing 5D geometry and resonance engines:

- **Point5D**: 5-dimensional vector operations
- **MetatronGeometry**: 13-node sacred geometry → 5D embedding
- **SpectralSignature**: (ψ, ρ, ω) resonance invariant
- **HolisticMatrix**: Complete resonance engine with:
  - Kosmokrator (Proof-of-Resonance exclusion)
  - Chronokrator (temporal expansion)
  - Mandorla Field (perception-intention intersection)
  - Monolith (action singularity)
  - Torus Topology (S¹ × S¹ phase space)

**Features**: `advanced-linalg`, `parallel`

### Layer 2: Search & Optimization (`cryptogenetik-core`)

Search pipeline with topological operators:

- **Triton Pipeline**: Search loop with resonance evaluation
- **Operators**:
  - WT (Wormdorf-Trichter): Topological contraction
  - SW (Schwellenwert): Adaptive thresholding
  - DK (Divergenz-Kontrolle): Divergence locking
  - PI (Phase-Invarianz): Canonical form
- **Score Hooks**: Early filtering (checksum, partial words, patterns)

**Features**: `bip39-integration`, `parallel`

### Layer 3: Multichain (`phosphoros-bip39`)

BIP39 and multichain wallet support:

- **Mnemonic**: 12 languages, entropy generation, validation
- **Derivation**: BIP32/44/49/84/86 paths
- **Multichain**: Bitcoin, Ethereum, Substrate, Cosmos, Solana, Cardano, Monero
- **Resonance Analysis**: Optional spectral wordlist analysis

**Features**: `btc`, `evm`, `substrate`, `cosmos`, `solana`, `cardano`, `monero`, `resonance`

## 🔬 Key Concepts

### Spectral Signatures

The core invariant: σ = (ψ, ρ, ω)

```rust
pub struct SpectralSignature {
    pub psi: f64,    // Coherence/Semantics [0,1]
    pub rho: f64,    // Density/Structure [0,1]
    pub omega: f64,  // Frequency/Phase [0,1]
}

// Resonance score (INVARIANT - do not change)
fn resonance(&self) -> f64 {
    self.psi * self.rho * self.omega
}
```

### Resonance Engine Trait

Pluggable resonance engines:

```rust
pub trait ResonanceEngine {
    type State;
    
    fn reset(&mut self);
    fn evaluate(&mut self, t: f64, perception: [f64;5], 
                intention: [f64;5], gradient: [f64;5], theta: f64) -> Evaluation;
    fn state(&self) -> &Self::State;
}
```

### Evaluation Types

```rust
pub enum Evaluation {
    Gated { reason: GateReason },     // Blocked
    Output { vector: [f64;5], score: f64 }, // Passed
}
```

## 📊 Performance

### Test Coverage

- **phosphoros-core**: 46 tests ✅
- **phosphoros-bip39**: 12 tests ✅
- **cryptogenetik-core**: 13 tests ✅

### Benchmarks

```
5D Operations:     ~14M ops/sec
Metatron Embed:    ~1.2M ops/sec
Resonance Eval:    ~640K ops/sec
Search Step:       ~21K steps/sec
```

## 🛠️ Development

### Building

```bash
# Build all crates
cargo build --workspace --release

# Build specific crate
cargo build -p phosphoros-core --release

# Build with all features
cargo build --workspace --all-features --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests with all features
cargo test --workspace --all-features

# Run specific crate tests
cargo test -p cryptogenetik-core
```

### Linting

```bash
# Format check
cargo fmt --all -- --check

# Clippy (zero warnings)
cargo clippy --workspace -- -D warnings

# Documentation
cargo doc --workspace --all-features --no-deps
```

## 📖 Documentation

- **API Docs**: `cargo doc --workspace --all-features --open`
- **Examples**: See `examples/` directories in each crate
- **Architecture**: See individual crate READMEs

## 🎯 Use Cases

1. **Cryptographic Research**: Analyze seed space topology
2. **Forensic Analysis**: Map and explore cryptographic relationships
3. **Infogenetic Studies**: Build spectral databases of addresses
4. **Seed Recovery**: Reconstruct seeds via 5D-spectral convergence
5. **Multichain Analysis**: Cross-chain address generation and analysis

## ⚠️ Security Notice

**IMPORTANT**: This system is for research and forensic analysis only.

- ⚠️ Do NOT use for generating production private keys
- ⚠️ Do NOT use for managing real cryptocurrency wallets
- ⚠️ Always use official, audited wallet software for real funds

PHOSPHOROS is a research tool for understanding cryptographic seed spaces, not a production wallet.

## 🔐 Features

### Production-Ready

- ✅ No `unsafe` code
- ✅ No `unwrap()`/`expect()` in library code
- ✅ Comprehensive error handling with `thiserror`
- ✅ Full determinism (seeded RNG)
- ✅ Extensive test coverage
- ✅ Documentation for all public APIs
- ✅ `clippy` clean with `-D warnings`

### Feature Flags

**phosphoros-core**:
- `advanced-linalg`: nalgebra/ndarray support
- `parallel`: Rayon-based parallelization

**cryptogenetik-core**:
- `bip39-integration`: Link with phosphoros-bip39
- `parallel`: Parallel search

**phosphoros-bip39**:
- `btc`, `evm`, `substrate`, `cosmos`, `solana`, `cardano`, `monero`: Chain support
- `resonance`: Spectral wordlist analysis

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. Code passes `cargo fmt`
2. Code passes `cargo clippy -- -D warnings`
3. All tests pass: `cargo test --workspace --all-features`
4. New features include tests
5. Public APIs are documented

## 📄 License

Dual-licensed under MIT or Apache 2.0.

## 🙏 Acknowledgments

Based on research in:
- **Triton**: 5D-Spiral search with information alchemy
- **Metatron-IUL**: Sacred Geometry topology
- **Holistic Resonance**: Post-symbolic resonance collapse

---

**Version**: 1.0.0  
**Status**: Core functionality complete  
**Author**: PHOSPHOROS Project Team

*Built with ❤️ for the cryptographic research community*
