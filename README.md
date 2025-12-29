# PHOSPHOROS

**Multichain Blockchain Forensics Platform with Quantum-Enhanced Analytics**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-green.svg)](#api-documentation)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](#docker-deployment)

PHOSPHOROS is an enterprise-grade blockchain forensics platform combining **multichain address analysis**, **5D spectral geometry**, and **quantum-enhanced algorithms** for comprehensive transaction tracing across Bitcoin, Ethereum, Cosmos, Monero, and more.

## Features

- **Multichain Support**: Bitcoin (P2PKH/P2SH/P2WPKH/P2TR), Ethereum (EIP-55), Cosmos SDK, Monero, Solana, Substrate
- **Quantum Algorithms**: QAOA, Grover search, VQE, Quantum Walk for enhanced forensic analysis
- **Enterprise Ready**: Docker deployment, OpenAPI 3.0 documentation, Prometheus metrics
- **BIP39/32/44**: Full hierarchical deterministic wallet derivation
- **Forensic Pipeline**: Anomaly detection, cluster analysis, topological pattern recognition

## Quick Start

### Installation

```bash
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros
cargo build --workspace --release
```

### Run Gateway API

```bash
cargo run -p phosphoros-gateway --release
```

The API will be available at:
- `http://localhost:8080/` - Service info
- `http://localhost:8080/swagger-ui/` - Interactive API docs
- `http://localhost:8080/metrics` - Prometheus metrics

### Docker Deployment

```bash
# Production build
docker build -t phosphoros .

# Run with docker-compose
docker-compose up -d
```

## Architecture

```
phosphoros/
├── crates/
│   ├── phosphoros-core/         # 5D geometry, spectral signatures, resonance engine
│   ├── phosphoros-quantum/      # Quantum algorithms (QAOA, Grover, VQE)
│   ├── phosphoros-bip39/        # Multichain wallet derivation
│   │   ├── bitcoin.rs           # P2PKH, P2SH-P2WPKH, P2WPKH, P2TR
│   │   ├── ethereum.rs          # EIP-55 checksummed addresses
│   │   ├── cosmos.rs            # Bech32 encoding for Cosmos SDK
│   │   └── monero.rs            # Ed25519 Monero addresses
│   ├── phosphoros-monero/       # Monero ring signature forensics
│   ├── phosphoros-satellite/    # Blockchain snapshot analysis
│   ├── phosphoros-gateway/      # REST API with OpenAPI + Prometheus
│   ├── phosphoros-stealthnet/   # Privacy protocol analysis
│   ├── phosphoros-dashboard/    # Visualization GUI
│   ├── phosphoros-cli/          # Command-line tools
│   └── cryptogenetik-core/      # Quantum-accelerated search
├── Dockerfile                   # Multi-stage production build
└── docker-compose.yml           # Service orchestration
```

## Multichain Address Generation

PHOSPHOROS provides cryptographically correct address generation for multiple blockchains:

### Bitcoin

```rust
use phosphoros_bip39::{Mnemonic, MasterKey, DerivationPath, CurveType};
use phosphoros_bip39::bitcoin::{generate_address, BitcoinAddressType, BitcoinNetwork};

let mnemonic = Mnemonic::generate(24, WordlistLanguage::English)?;
let seed = mnemonic.to_seed(None);
let master = MasterKey::from_seed(seed, CurveType::Secp256k1);
let key = master.derive(&DerivationPath::bitcoin(0, 0, 0))?;

// Generate all Bitcoin address types
let p2pkh = generate_address(&key, BitcoinAddressType::P2PKH, BitcoinNetwork::Mainnet)?;
let p2wpkh = generate_address(&key, BitcoinAddressType::P2WPKH, BitcoinNetwork::Mainnet)?;
let p2tr = generate_address(&key, BitcoinAddressType::P2TR, BitcoinNetwork::Mainnet)?;
```

### Ethereum

```rust
use phosphoros_bip39::ethereum::generate_address;

let eth_addr = generate_address(&key)?;
// Returns EIP-55 checksummed address: 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed
```

### Cosmos SDK

```rust
use phosphoros_bip39::cosmos::{generate_address, CosmosChain};

let cosmos = generate_address(&key, CosmosChain::cosmos())?;  // cosmos1...
let osmosis = generate_address(&key, CosmosChain::osmosis())?; // osmo1...
let juno = generate_address(&key, CosmosChain::juno())?;       // juno1...
```

## Quantum-Enhanced Forensics

### QAOA Ring Signature Analysis

Probabilistic identification of real inputs in Monero ring signatures:

```rust
use phosphoros_monero::QuantumRingAnalyzer;

let analyzer = QuantumRingAnalyzer::new(depth: 2, shots: 1000)
    .with_quantum_weight(0.5);

let result = analyzer.analyze(&ring_members, &classical_result)?;
println!("Combined confidence: {:.2}%", result.combined_confidence * 100.0);
```

### Grover-Accelerated Search

Quadratic speedup for transaction pattern matching:

```rust
use cryptogenetik_core::QuantumSearchEngine;

let engine = QuantumSearchEngine::new(config);
let result = engine.hybrid_search(search_space, quantum_oracle, classical_eval)?;
```

## API Documentation

The gateway exposes a RESTful API with automatic OpenAPI 3.0 documentation:

### Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Service information |
| `/health` | GET | Health check |
| `/metrics` | GET | Prometheus metrics |
| `/swagger-ui/` | GET | Interactive API docs |
| `/satellite/v1/snapshots` | GET/POST | Blockchain snapshots |
| `/satellite/v1/analyze/:id` | POST | Run forensic analysis |
| `/satellite/v1/reports/latest` | GET | Latest analysis report |

### Example: Ingest Snapshot

```bash
curl -X POST http://localhost:8080/satellite/v1/snapshots \
  -H "Content-Type: application/json" \
  -d '{
    "label": "btc-block-800000",
    "observations": [{
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
      "features": [0.1, 0.2, 0.3, 0.4, 0.5]
    }]
  }'
```

## Prometheus Metrics

PHOSPHOROS exports the following metrics for monitoring:

| Metric | Type | Description |
|--------|------|-------------|
| `phosphoros_http_requests_total` | Counter | HTTP requests by path/method/status |
| `phosphoros_http_request_duration_seconds` | Histogram | Request latency |
| `phosphoros_snapshots_ingested_total` | Counter | Snapshots ingested by chain |
| `phosphoros_analyses_completed_total` | Counter | Forensic analyses completed |
| `phosphoros_hotspots_detected` | Histogram | Resonance hotspots per analysis |

## Feature Flags

### phosphoros-bip39

```toml
[features]
btc = ["bitcoin", "bech32"]       # Bitcoin address generation
evm = ["k256", "sha3"]            # Ethereum/EVM addresses
cosmos = ["k256", "bech32"]       # Cosmos SDK chains
monero = ["curve25519-dalek"]     # Monero addresses
```

### phosphoros-quantum

```toml
[features]
grover = []      # Grover's search algorithm
qaoa = []        # Quantum Approximate Optimization
vqe = []         # Variational Quantum Eigensolver
full = []        # All algorithms
```

## Core Concepts

### Spectral Signature (psi, rho, omega)

The universal resonance triplet used across all PHOSPHOROS systems:

```rust
pub struct SpectralSignature {
    pub psi: f64,    // Coherence (0-1)
    pub rho: f64,    // Stability (0-1)
    pub omega: f64,  // Efficiency (0-1)
}

// Resonance invariant
fn resonance(&self) -> f64 {
    self.psi * self.rho * self.omega
}
```

### 5D Point Geometry

High-dimensional embedding for entity analysis:

```rust
pub struct Point5D {
    pub x: f64,  // Spatial
    pub y: f64,
    pub z: f64,
    pub w: f64,  // Temporal
    pub v: f64,  // Value/intensity
}
```

## Development

### Run Tests

```bash
# All tests
cargo test --workspace --all-features

# With specific chain features
cargo test -p phosphoros-bip39 --features btc,evm,cosmos
```

### Run Benchmarks

```bash
cargo bench -p phosphoros-core
```

### Clippy

```bash
cargo clippy --workspace --all-features -- -D warnings
```

## Security Notice

**IMPORTANT**: This system is designed for forensic analysis and research.

- Do NOT use for managing real cryptocurrency funds
- Do NOT use generated keys for production wallets
- Quantum simulation runs on classical hardware
- Always use audited wallet software for real assets

## Performance

```
5D Operations:       ~14M ops/sec
Metatron Embed:      ~1.2M ops/sec
Resonance Eval:      ~640K ops/sec
Bitcoin Address:     ~50K ops/sec
Ethereum Address:    ~45K ops/sec
QAOA Layer:          ~10K ops/sec
```

## License

Dual-licensed under MIT or Apache 2.0.

---

**Version**: 2.1.0
**Status**: Production-ready
**Maintainer**: PHOSPHOROS Project Team

*Enterprise blockchain forensics with quantum-enhanced analytics*
