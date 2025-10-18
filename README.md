# PHOSPHOROS - Prä-Holographisches System zur 5D-Skalarprojektion

> Enterprise-Ready Suite for High-Dimensional Blockchain Forensics

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/LashSesh/phosphoros)

**PHOSPHOROS** is a sophisticated enterprise system for topological mapping of cryptographic seed spaces using 5D-spectral analysis, holistic resonance engines, and autonomous blockchain forensics.

🎯 **NEW: [PHOSPHOROS Dashboard](#-phosphoros-dashboard)** - Fully integrated GUI with real-time autonomous operation!

📖 **[Read the Delta Blueprint](DELTA_BLUEPRINT.md)** for complete architecture documentation and AI agent integration guide.

## 🌟 Overview

PHOSPHOROS integrates advanced mathematical concepts with enterprise-grade blockchain forensics:

- **5D Spectral Geometry**: Metatron Sacred Geometry with 5D projection
- **Holistic Resonance**: Multi-layer resonance engine (Kosmokrator, Chronokrator, O.P.H.A.N.)
- **Topological Optimization**: Four operators (WT, SW, DK, PI) for search space reduction
- **Multichain Support**: BIP39 integration for Bitcoin, Ethereum, Substrate, and more
- **Satellite Forensics**: Blockchain analysis, anomaly detection, and cluster identification
- **Infogenetic Mapping**: Spectral signatures (ψ, ρ, ω) for cryptographic analysis
- **🆕 Living Lab Dashboard**: Real-time GUI with autonomous operation

## 🖥️ PHOSPHOROS Dashboard

The **PHOSPHOROS Dashboard** is a fully integrated, production-ready interface for blockchain forensics with autonomous background services.

### Key Features

✨ **6 Modular Panels:**
- 🏠 **Home**: Live overview with real-time service status and statistics
- 🔑 **Seed & Wallet**: BIP39 mnemonic import with multi-chain address generation
- 📊 **Resonance & Spectro**: Real-time 5D resonance analysis with spectral signatures
- 🔍 **Cluster Explorer**: Automatic cluster discovery and analysis
- 📝 **System Log**: Filterable logs with severity levels
- ⚙️ **Settings & Tasks**: Configuration and system reports

🤖 **3 Autonomous Services:**
- **Scraper**: Continuous entity discovery (5s interval)
- **Analyzer**: Real-time anomaly detection (3s interval)
- **Cluster Engine**: Automatic KNN clustering (10s interval)

🔄 **Real-Time Operation:**
- Thread-safe async architecture with tokio
- Live statistics and metrics
- Background task management
- Message-driven state updates

💾 **Data Export:**
- JSON export for programmatic access
- CSV export for spreadsheet analysis
- Markdown reports for documentation
- Comprehensive system reports

### Quick Start Dashboard

```bash
# Run the dashboard
cargo run -p phosphoros-dashboard --release

# With debug logging
RUST_LOG=debug cargo run -p phosphoros-dashboard
```

### Dashboard Screenshots

The dashboard features a professional dark mode interface with:
- Real-time service status indicators (🟢 Running / ⏸ Paused)
- Live entity, cluster, and anomaly counters
- Export buttons for all data types
- Filterable system logs
- Theme toggle (Dark/Light)

See [Dashboard Documentation](crates/phosphoros-dashboard/README.md) for detailed information.

## 📦 Workspace Structure

```
phosphoros/
├── crates/
│   ├── phosphoros-core/         # 5D geometry + holistic resonance
│   ├── cryptogenetik-core/      # Search + optimization
│   ├── phosphoros-bip39/        # Multichain wallet support
│   ├── phosphoros-satellite/    # Blockchain forensics & analysis
│   ├── phosphoros-dashboard/    # 🆕 GUI Dashboard (Living Lab)
│   ├── phosphoros-cli/          # Command-line tools
│   └── phosphoros-gateway/      # REST/WS API gateway
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

### Layer 4: Satellite Forensics (`phosphoros-satellite`)

Blockchain forensic analysis and anomaly detection:

- **Entity Tracking**: Wallet, validator, and contract observation
- **Anomaly Detection**: Z-score based unusual behavior identification
- **Cluster Analysis**: KNN graphs and resonance hotspot detection
- **Topological Analysis**: Connected components, articulation points, Betti numbers
- **Entropy Metrics**: Spectral entropy and distribution analysis
- **REST API**: Optional Axum-based API service (feature-gated)
- **Core Integration**: Seamless conversion to Point5D and SpectralSignature

**Features**: `api` (enables REST endpoints)

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
- **phosphoros-satellite**: 9 tests ✅
- **phosphoros-kryptogenetik**: 38 tests ✅
- **Total**: 139 tests passing ✅

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

- **Delta Blueprint**: [DELTA_BLUEPRINT.md](DELTA_BLUEPRINT.md) - Complete architecture guide for AI agents and developers
- **API Docs**: `cargo doc --workspace --all-features --open`
- **Examples**: See `examples/` directories in each crate
- **Architecture**: See individual crate READMEs
- **Implementation Summary**: [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)

## 🎯 Use Cases

1. **Cryptographic Research**: Analyze seed space topology with 5D spectral analysis
2. **Blockchain Forensics**: Detect Sybil attacks, money laundering, anomalous behavior
3. **Wallet Profiling**: Build behavioral profiles and detect patterns
4. **Forensic Analysis**: Map and explore cryptographic relationships
5. **Infogenetic Studies**: Build spectral databases of addresses
6. **Seed Recovery**: Reconstruct seeds via 5D-spectral convergence
7. **Multichain Analysis**: Cross-chain address generation and analysis
8. **Network Analysis**: Understand blockchain network topology and critical nodes
9. **🆕 Real-Time Monitoring**: Live dashboard for continuous blockchain surveillance
10. **🆕 Enterprise Reporting**: Automated report generation for compliance and analysis

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
- ✅ Extensive test coverage (139+ tests passing)
- ✅ Documentation for all public APIs
- ✅ `clippy` clean with `-D warnings`
- ✅ Thread-safe async architecture
- ✅ Real-time GUI dashboard
- ✅ Enterprise data export capabilities

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

**phosphoros-satellite**:
- `api`: Enables REST API endpoints (Axum, Tokio, Tower, Tracing)

**phosphoros-dashboard**:
- Default features include all integrations
- `tokio`, `canvas`, `advanced` enabled by default

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
