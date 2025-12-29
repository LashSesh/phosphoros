# PHOSPHOROS

<div align="center">

**Enterprise Blockchain Forensics Platform**

*Quantum-Enhanced Multi-Chain Analytics*

[![Rust](https://img.shields.io/badge/Rust-1.75+-f74c00?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178c6?logo=typescript)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-85ea2d?logo=swagger)](docs/api.md)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ed?logo=docker)](Dockerfile)

[Getting Started](#getting-started) | [Documentation](#documentation) | [API Reference](#api-reference) | [Contributing](#contributing)

</div>

---

## Overview

PHOSPHOROS is an enterprise-grade blockchain forensics platform that combines **5D spectral geometry**, **quantum-enhanced algorithms**, and **multi-chain analytics** for comprehensive transaction tracing and anomaly detection.

### Key Capabilities

| Feature | Description |
|---------|-------------|
| **Multi-Chain Support** | Bitcoin, Ethereum, Cosmos, Monero, and more |
| **Quantum Algorithms** | Grover, QAOA, VQE, Quantum Walk |
| **5D Resonance Engine** | Spectral signature analysis (ψ, ρ, ω) |
| **Forensic Pipeline** | Clustering, anomaly detection, pattern recognition |
| **Enterprise Ready** | Docker, OpenAPI 3.0, Prometheus metrics |

---

## Getting Started

### Prerequisites

- Rust 1.75+ with Cargo
- Node.js 18+ (for web dashboard)
- Docker & Docker Compose (optional)

### Quick Install

```bash
# Clone repository
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros

# Build all crates
cargo build --workspace --release

# Run tests
cargo test --workspace
```

### Start the API Gateway

```bash
cargo run -p phosphoros-gateway --release
```

**Available Endpoints:**
- `http://localhost:8080/` - Service info
- `http://localhost:8080/swagger-ui/` - Interactive API docs
- `http://localhost:8080/metrics` - Prometheus metrics
- `http://localhost:8080/health` - Health check

### Docker Deployment

```bash
# Build and run with Docker Compose
docker-compose up -d

# Services:
# - Gateway API: http://localhost:8080
# - Web Dashboard: http://localhost:3000
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        PHOSPHOROS                               │
├─────────────────────────────────────────────────────────────────┤
│  PRESENTATION                                                   │
│  ├── phosphoros-dashboard    Iced desktop GUI                  │
│  └── phosphoros-web          React 18 web dashboard            │
├─────────────────────────────────────────────────────────────────┤
│  API                                                            │
│  └── phosphoros-gateway      REST + OpenAPI + Prometheus       │
├─────────────────────────────────────────────────────────────────┤
│  ANALYSIS                                                       │
│  ├── phosphoros-satellite    Forensic analysis engine          │
│  ├── phosphoros-quantum      Quantum algorithms                │
│  ├── phosphoros-monero       Ring signature analysis           │
│  └── cryptogenetik-core      Quantum-accelerated search        │
├─────────────────────────────────────────────────────────────────┤
│  CORE                                                           │
│  ├── phosphoros-core         5D geometry & resonance           │
│  ├── phosphoros-bip39        Multi-chain HD wallets            │
│  └── phosphoros-stealthnet   Privacy protocol analysis         │
└─────────────────────────────────────────────────────────────────┘
```

### Crate Overview

| Crate | Purpose |
|-------|---------|
| `phosphoros-core` | 5D geometry, spectral signatures, resonance engine |
| `phosphoros-quantum` | QAOA, Grover, VQE, Quantum Walk algorithms |
| `phosphoros-bip39` | BIP39/32/44 multi-chain wallet derivation |
| `phosphoros-satellite` | Forensic analysis, clustering, anomaly detection |
| `phosphoros-gateway` | REST API with OpenAPI and Prometheus |
| `phosphoros-monero` | Monero ring signature forensics |
| `phosphoros-stealthnet` | Traffic mimicry, steganography |
| `phosphoros-dashboard` | Desktop GUI (Iced framework) |
| `phosphoros-web` | Web dashboard (React + TypeScript) |

---

## Supported Blockchains

| Chain | Address Types | Status |
|-------|---------------|--------|
| **Bitcoin** | P2PKH, P2SH-P2WPKH, P2WPKH, P2TR (Taproot) | Production |
| **Ethereum** | EIP-55 Checksummed | Production |
| **Cosmos SDK** | Bech32 (atom, osmo, juno, etc.) | Production |
| **Monero** | Ed25519 Stealth Addresses | Production |
| **Solana** | Base58 Public Keys | Planned |
| **Substrate** | SS58 Encoding | Planned |

---

## Core Concepts

### Spectral Signature (ψ, ρ, ω)

The fundamental resonance triplet used across all PHOSPHOROS systems:

```rust
use phosphoros_core::SpectralSignature;

let sig = SpectralSignature::new(0.85, 0.92, 0.78);
let resonance = sig.resonance();  // D = ψ · ρ · ω
```

| Component | Symbol | Meaning |
|-----------|--------|---------|
| Coherence | ψ (psi) | Phase synchronization |
| Stability | ρ (rho) | Structural density |
| Efficiency | ω (omega) | Oscillation rate |

### 5D Information Space

```rust
use phosphoros_core::{Point5D, MetatronGeometry};

let point = Point5D::new(x, y, z, w, v);
let metatron = MetatronGeometry::new();
let embedding = metatron.embed_object(entity_hash);
```

---

## API Reference

### REST Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/metrics` | GET | Prometheus metrics |
| `/swagger-ui/` | GET | Interactive API docs |
| `/satellite/v1/snapshots` | GET, POST | Manage snapshots |
| `/satellite/v1/analyze/:id` | POST | Run analysis |
| `/satellite/v1/reports/latest` | GET | Get latest report |

### Example: Ingest Blockchain Snapshot

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

---

## Development

### Run Tests

```bash
# All workspace tests
cargo test --workspace

# Specific crate with features
cargo test -p phosphoros-bip39 --features btc,evm,cosmos
```

### Run Benchmarks

```bash
cargo bench -p phosphoros-core
cargo bench -p phosphoros-kryptogenetik
```

### Code Quality

```bash
# Clippy lints
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all -- --check
```

### Web Dashboard Development

```bash
cd phosphoros-web
npm install
npm run dev  # http://localhost:5173
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [API Documentation](docs/api.md) | REST API reference |
| [Architecture](docs/architecture.md) | System design |
| [Law Enforcement Guide](docs/law-enforcement.md) | Usage for investigators |
| [Quantum Algorithms](docs/quantum.md) | Algorithm documentation |

---

## Performance

```
5D Vector Operations:    ~14M ops/sec
Metatron Embedding:      ~1.2M ops/sec
Resonance Evaluation:    ~640K ops/sec
Bitcoin Address Gen:     ~50K ops/sec
Ethereum Address Gen:    ~45K ops/sec
QAOA Circuit Layer:      ~10K ops/sec
```

---

## Security

**Important**: This platform is designed for forensic analysis and research purposes.

- Do NOT use for managing real cryptocurrency funds
- Do NOT use generated keys for production wallets
- Quantum algorithms run on classical simulation
- Use audited wallet software for real assets

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).

---

<div align="center">

**PHOSPHOROS v3.0**

Enterprise Blockchain Forensics with Quantum-Enhanced Analytics

[Report Issue](https://github.com/LashSesh/phosphoros/issues) | [Request Feature](https://github.com/LashSesh/phosphoros/issues)

</div>
