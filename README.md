# PHOSPHOROS

<div align="center">

**Geometric-Cybernetic Blockchain Forensics Platform**

*5D Information Space Analysis Engine with Quantum-Enhanced Capabilities*

[![Rust](https://img.shields.io/badge/Rust-1.75+-f74c00?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178c6?logo=typescript)](https://www.typescriptlang.org/)
[![React](https://img.shields.io/badge/React-18.3-61dafb?logo=react)](https://reactjs.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)

</div>

---

## Overview

PHOSPHOROS is an advanced blockchain forensics platform that models transactions and wallet states as points in a five-dimensional information space. The system employs a unique geometric-cybernetic approach where structured trajectories appear as spiral paths, enabling novel analytical capabilities for cryptocurrency investigation.

### Key Capabilities

- **5D Geometric Analysis**: Entity representation in five-dimensional vector space with Metatron 13-node sacred geometry embedding
- **Quantum-Enhanced Search**: Grover, QAOA, VQE algorithms for accelerated pattern matching and optimization
- **Multi-Chain Forensics**: Bitcoin, Ethereum, Cosmos SDK, and specialized Monero ring signature analysis
- **Stealth Networking**: API mimicry, steganographic payloads, and covert communication protocols
- **Unified Web Interface**: Modern React 18 dashboard with full Gateway API integration (desktop GUI deprecated)

> **📢 Consolidation Update (2026-01-01):** PHOSPHOROS has migrated to a **unified web-only architecture**. The desktop GUI (`phosphoros-dashboard`) is deprecated in favor of the comprehensive web interface. See [DEPLOYMENT.md](DEPLOYMENT.md) for details.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              PHOSPHOROS                                  │
├─────────────────────────────────────────────────────────────────────────┤
│  PRESENTATION LAYER                                                      │
│  ├── phosphoros-web           React 18 + TypeScript (PRIMARY)           │
│  │   ├── Resonance Page       5D Spectral Analysis                      │
│  │   ├── Wallet Page          BIP-39 Multichain Derivation              │
│  │   ├── Cluster Page         KNN/DBSCAN/Hierarchical Clustering        │
│  │   └── WebSocket Client     Real-time Gateway Events                  │
│  ├── phosphoros-cli           TRITON Command-Line Interface             │
│  └── phosphoros-dashboard     Desktop GUI (DEPRECATED)                  │
├─────────────────────────────────────────────────────────────────────────┤
│  API LAYER                                                               │
│  └── phosphoros-gateway       Axum REST + WebSocket + OpenAPI           │
│      ├── Satellite API        Blockchain Forensics (11 endpoints)       │
│      ├── Resonance API        5D Spectral Analysis (4 endpoints)        │
│      ├── Wallet API           BIP-39 Derivation (4 endpoints)           │
│      ├── Cluster API          Entity Clustering (4 endpoints)           │
│      └── WebSocket Server     Real-time Events (8 types)                │
├─────────────────────────────────────────────────────────────────────────┤
│  ANALYSIS LAYER                                                          │
│  ├── phosphoros-satellite     Blockchain Forensic Analysis Engine       │
│  ├── phosphoros-quantum       Quantum Algorithm Implementations         │
│  ├── phosphoros-monero        Ring Signature & Stealth Address Analysis │
│  └── cryptogenetik-core       Topological Search Optimization           │
├─────────────────────────────────────────────────────────────────────────┤
│  CORE LAYER                                                              │
│  ├── phosphoros-core          5D Geometry & Resonance Engine            │
│  ├── phosphoros-bip39         Multi-Chain HD Wallet Derivation          │
│  ├── phosphoros-stealthnet    Privacy Protocol & Covert Networking      │
│  ├── phosphoros-types         Shared Type Definitions                   │
│  └── phosphoros-kryptogenetik Legacy 5D Scalar Projection System        │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Concepts

### 5D Information Space

Entities are represented as points in a five-dimensional vector space `R³ × R²`:

```rust
use phosphoros_core::Point5D;

let entity = Point5D::new(
    x,   // Spatial coordinate 1
    y,   // Spatial coordinate 2
    z,   // Spatial coordinate 3
    u,   // Internal coordinate 1 (behavioral)
    v    // Internal coordinate 2 (temporal)
);
```

### Spectral Signature (ψ, ρ, ω)

The fundamental triplet characterizing entity resonance:

| Component | Symbol | Description |
|-----------|--------|-------------|
| Coherence | ψ (Psi) | Phase synchronization measure |
| Stability | ρ (Rho) | Structural density metric |
| Efficiency | ω (Omega) | Oscillation rate indicator |

**Invariant Resonance**: `D = ψ · ρ · ω`

```rust
use phosphoros_core::SpectralSignature;

let signature = SpectralSignature::new(0.85, 0.92, 0.78);
let resonance = signature.resonance(); // D = ψ·ρ·ω
```

### Metatron Sacred Geometry

A 13-node canonical topology for embedding entities in 5D space:

```rust
use phosphoros_core::MetatronGeometry;

let geometry = MetatronGeometry::new();
let embedded = geometry.embed(&entity_features);
```

### Holistic Resonance Matrix

Multi-stage evaluation engine with gating criteria:

| Stage | Function |
|-------|----------|
| **Kosmokrator** | Proof-of-Resonance gating (phase coherence filtering) |
| **Chronokrator** | Temporal dynamics with expansion tracking |
| **Mandorla** | Perception-intention intersection (P⃗ · I⃗) |
| **Monolith** | Action singularity trigger (geometric criterion) |
| **Torus** | S¹ × S¹ phase space for state tracking |

---

## Supported Blockchains

| Chain | Address Types | Features | Status |
|-------|---------------|----------|--------|
| **Bitcoin** | P2PKH, P2SH, P2WPKH, P2WSH, P2TR (Taproot) | Bech32 encoding, BIP32/44/84/86 | Production |
| **Ethereum/EVM** | EIP-55 Checksummed | k256 ECDSA, Keccak-256 | Production |
| **Cosmos SDK** | Bech32 (atom, osmo, juno, etc.) | k256 + bech32 | Production |
| **Monero** | Ed25519 Stealth Addresses | Ring signature forensics, Key image tracking | Production |
| **Substrate** | SS58 | Ed25519/Sr25519 | Placeholder |
| **Solana** | Base58 | Ed25519 | Placeholder |
| **Cardano** | Bech32 | Ed25519 | Placeholder |

---

## Quantum Algorithms

PHOSPHOROS integrates simulated quantum algorithms for enhanced analysis:

| Algorithm | Application | Implementation |
|-----------|-------------|----------------|
| **Grover Search** | Quadratic speedup for pattern matching | Oracle-based with optimal iterations |
| **QAOA** | Ring signature analysis, combinatorial optimization | Variational parameter optimization |
| **VQE** | Eigenvalue computation for graph analysis | Ansatz-based hybrid approach |
| **Quantum Walk** | Probabilistic exploration heuristics | Continuous/discrete time variants |
| **SCS Bridge** | Automatic parameter tuning | Seraphic Calibration System |

```rust
use phosphoros_quantum::{GroverSearch, LocalSimulator};

let backend = LocalSimulator::new(num_qubits);
let grover = GroverSearch::new(search_space_size);
let result = grover.execute(&backend, oracle, optimal_iterations)?;
```

---

## Installation

### Prerequisites

- **Rust**: 1.75+ with Cargo
- **Node.js**: 18+ (for web dashboard)
- **Docker**: Optional, for containerized deployment

### Build from Source

```bash
# Clone repository
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros

# Build entire workspace (release mode)
cargo build --workspace --release

# Run tests
cargo test --workspace --all-features

# Generate documentation
cargo doc --workspace --open
```

### Feature Flags

Enable specific functionality with Cargo features:

```bash
# BIP39 with Bitcoin and Ethereum support
cargo build -p phosphoros-bip39 --features btc,evm

# Quantum algorithms
cargo build -p phosphoros-quantum --features full

# Parallel processing
cargo build -p phosphoros-core --features parallel,advanced-linalg
```

---

## Running the Platform

### API Gateway

```bash
cargo run -p phosphoros-gateway --release
```

**Endpoints:**
| Endpoint | Description |
|----------|-------------|
| `http://localhost:8080/` | Service information |
| `http://localhost:8080/health` | Health check |
| `http://localhost:8080/metrics` | Prometheus metrics |
| `http://localhost:8080/swagger-ui/` | Interactive API documentation |
| `http://localhost:8080/api-docs/openapi.json` | OpenAPI specification |

### Desktop Dashboard (Living Lab)

```bash
cargo run -p phosphoros-dashboard --release
```

**12-Panel System:**
- Home (Live Overview)
- Seed & Wallet Management
- Resonance & Spectrography
- Cluster Explorer
- Search Space Explorer
- Network Topology
- Infogenetic Browser
- Anomaly Investigation
- Forensic Workflows
- Stealth/Privacy Controls
- System Log
- Settings & Tasks

### Web Dashboard

```bash
cd phosphoros-web
npm install
npm run dev
```

Access at `http://localhost:5173`

**Pages:**
- Dashboard: Metrics overview and activity feed
- Wallet: Seed/mnemonic management
- Resonance: Spectral analysis visualization
- Topology: Network force graph
- Investigation: Explorer, Anomalies, Forensics
- Infogenetik: Infogenetic analysis
- Settings: Configuration

### TRITON CLI

```bash
cargo run -p phosphoros-cli --release -- --mode auto --cycles 40
```

**Options:**
- `--name`: Instance name (default: "TRITON")
- `--mode`: Operation mode - auto/memory/hybrid (default: auto)
- `--cycles`: Auto operation cycles (default: 40)
- `--wordlist`: Wordlist file path (default: data/wordlist.txt)

---

## Docker Deployment

> **📖 Full deployment guide:** See [DEPLOYMENT.md](DEPLOYMENT.md) for comprehensive instructions, troubleshooting, and production hardening.

### Using Docker Compose

```bash
# Start all services
docker-compose up -d

# Services:
# - Gateway API: http://localhost:8080
# - Web Dashboard: http://localhost:3000
# - Swagger UI: http://localhost:8080/swagger-ui/
# - WebSocket: ws://localhost:3000/ws
```

### Manual Docker Build

```bash
# Build image
docker build -t phosphoros:latest .

# Run gateway
docker run -p 8080:8080 phosphoros:latest
```

---

## API Reference

### Satellite Forensics API

```
GET  /satellite/health              # Health check
GET  /satellite/v1/snapshots        # List ingested snapshots
POST /satellite/v1/snapshots        # Ingest new snapshot
POST /satellite/v1/analyze/:id      # Run analysis on snapshot
GET  /satellite/v1/reports/latest   # Get latest analysis report
```

### Analysis Pipeline

1. **Feature Matrix Construction**: Entity observations → feature vectors
2. **Distance Computation**: L2 distance matrix calculation
3. **KNN Graph**: K-nearest-neighbor graph construction
4. **Hotspot Detection**: High-density cluster identification
5. **Anomaly Scoring**: Z-score based outlier detection
6. **Topological Analysis**: Graph properties, articulation points, Betti numbers
7. **Entropy Analysis**: Feature distribution metrics

---

## Stealth Networking

PHOSPHOROS includes sophisticated stealth capabilities for authorized forensic operations:

### API Mimicry

Disguise traffic as legitimate API calls:
- OpenAI API format
- Slack API format
- Telegram Bot API format
- Discord API format
- Generic REST patterns

### Steganographic Encoding

- Zero-width character encoding
- Header-based invisible payloads
- Parameter-embedded data
- Body steganography

### Proxy Management

- SOCKS5/HTTP proxy rotation
- Request randomization (temporal, semantic, structural)
- Payload shaping for traffic normalization

**Note**: These capabilities are for authorized use only (forensics, research, security audits, compliance-approved operations).

---

## Monero Forensics

Specialized analysis for privacy-focused cryptocurrencies:

### Ring Signature Analysis

```rust
use phosphoros_monero::{RingAnalyzer, TemporalHeuristic, DecoySelectionHeuristic};

let analyzer = RingAnalyzer::new(rpc_client);
let likely_real = analyzer.analyze_ring(&ring_members)?;
```

### Capabilities

- **Ring Member Analysis**: Heuristic-based real input identification
- **Temporal Patterns**: Timing-based ring member analysis
- **Decoy Selection**: Decoy pattern detection
- **Key Image Tracking**: Spent output detection
- **Transaction Graph**: Graph construction with anomaly reporting
- **Quantum-Enhanced**: QAOA-based ring analysis (optional)

---

## Cryptogenetik Search

Topological optimization for search space exploration:

### Operators

| Code | Operator | Function |
|----------|------|----------|
| WT | Wormhole Funnel (Gabriel) | Search space contraction along curvature |
| SW | Threshold Sweep (Uriel) | Adaptive threshold gating |
| DK | Double Kick (Michael) | Lock/constraint mechanism |
| PI | Path Invariance (Raphael) | Canonical projection operator |

### TRITON Pipeline

```rust
use cryptogenetik_core::{TritonPipeline, OperatorSet};

let pipeline = TritonPipeline::new()
    .with_operators(OperatorSet::default())
    .with_score_hooks(vec![checksum_hook, partial_word_hook]);

let result = pipeline.run(search_space)?;
```

---

## Development

### Testing

```bash
# All tests
cargo test --workspace --all-features

# Specific crate
cargo test -p phosphoros-quantum --all-features

# With output
cargo test --workspace -- --nocapture
```

### Benchmarks

```bash
cargo bench -p phosphoros-core
cargo bench -p phosphoros-kryptogenetik
```

### Code Quality

```bash
# Formatting
cargo fmt --all -- --check

# Linting
cargo clippy --workspace -- -D warnings
```

### Web Development

```bash
cd phosphoros-web

# Development server
npm run dev

# Production build
npm run build

# Type checking
npm run lint
```

---

## Technology Stack

### Backend (Rust)

| Category | Technologies |
|----------|--------------|
| **Async Runtime** | tokio 1.x |
| **Web Framework** | Axum 0.7 |
| **GUI Framework** | iced 0.13 |
| **Serialization** | serde, serde_json, serde_yaml |
| **Cryptography** | sha2, sha3, blake3, k256, curve25519-dalek |
| **Mathematics** | nalgebra, ndarray, petgraph |
| **HTTP Client** | reqwest (with SOCKS5) |
| **Metrics** | Prometheus exporter |
| **API Docs** | utoipa (OpenAPI 3.0) |

### Frontend (TypeScript/React)

| Category | Technologies |
|----------|--------------|
| **Framework** | React 18.3 |
| **Build Tool** | Vite 5.4 |
| **State** | Zustand 5.0 |
| **Data Fetching** | React Query 5.x |
| **Styling** | TailwindCSS 3.4 |
| **UI Components** | Radix UI (ShadcnUI) |
| **Visualization** | D3.js 7.9, ECharts 5.5 |
| **Routing** | React Router DOM 6.28 |

---

## Configuration

### Environment Variables

```bash
# Rust Services
RUST_LOG=info                        # Logging level
PHOSPHOROS_HOST=0.0.0.0              # Bind address
PHOSPHOROS_PORT=8080                 # Gateway port

# Web Frontend
VITE_API_URL=http://localhost:8080   # Backend API URL
```

### Satellite Configuration

```yaml
analysis:
  knn_k: 10                          # K-nearest neighbors
  entropy_bins: 50                   # Entropy histogram bins
  resonance_threshold: 0.7           # Minimum resonance score
  max_snapshots: 1000                # Maximum stored snapshots
```

---

## Project Structure

```
phosphoros/
├── crates/
│   ├── cryptogenetik-core/          # Search optimization
│   ├── phosphoros-bip39/            # Multi-chain wallet
│   ├── phosphoros-cli/              # TRITON CLI
│   ├── phosphoros-core/             # 5D geometry engine
│   ├── phosphoros-dashboard/        # Desktop GUI
│   ├── phosphoros-gateway/          # REST API
│   ├── phosphoros-monero/           # Monero forensics
│   ├── phosphoros-quantum/          # Quantum algorithms
│   ├── phosphoros-satellite/        # Forensic analysis
│   ├── phosphoros-stealthnet/       # Stealth networking
│   └── phosphoros-types/            # Shared types
├── phosphoros-kryptogenetik/        # Legacy system
├── phosphoros-web/                  # React web app
├── docs/                            # Documentation
├── docker-compose.yml               # Container orchestration
├── Dockerfile                       # Multi-stage build
└── Cargo.toml                       # Workspace configuration
```

---

## Documentation

Additional documentation available in `/docs`:

- **architecture.md**: Detailed system design and implementation blueprint
- **api.md**: Complete REST API reference
- **quantum.md**: Quantum algorithm documentation
- **stealth-networking.md**: Stealth capabilities overview
- **law-enforcement.md**: Investigator usage guide
- **explorer-guide.md**: Interactive explorer documentation

---

## Security Notice

This system is designed for **forensic analysis and authorized research only**.

- Do not use for managing real cryptocurrency holdings
- Generated keys should not be used for production wallets
- Quantum algorithms run on classical simulation
- Stealth capabilities require proper authorization
- Comply with applicable laws and regulations

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).

---

<div align="center">

**PHOSPHOROS**

*5D Geometric-Cybernetic Blockchain Forensics*

[Documentation](docs/) · [API Reference](http://localhost:8080/swagger-ui/) · [Report Issue](https://github.com/LashSesh/phosphoros/issues)

</div>
