# PHOSPHOROS

<div align="center">

**Geometric-Cybernetic Blockchain Forensics Platform**

*5D Information Space Analysis Engine with Quantum-Enhanced Capabilities*

[![Rust](https://img.shields.io/badge/Rust-1.75+-f74c00?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178c6?logo=typescript)](https://www.typescriptlang.org/)
[![React](https://img.shields.io/badge/React-18.3-61dafb?logo=react)](https://reactjs.org/)
[![Axum](https://img.shields.io/badge/Axum-0.7-orange)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)

[Features](#key-features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Documentation](#documentation) • [Web Interface](#web-interface)

</div>

---

## What is PHOSPHOROS?

PHOSPHOROS ist eine fortschrittliche **Blockchain-Forensik-Plattform**, die Transaktionen und Wallet-Zustände als Punkte in einem **fünfdimensionalen Informationsraum** modelliert. Das System nutzt einen einzigartigen geometrisch-kybernetischen Ansatz, bei dem strukturierte Trajektorien als spiralförmige Pfade erscheinen und neuartige analytische Fähigkeiten für Kryptowährungs-Untersuchungen ermöglichen.

Die Plattform kombiniert **heilige Geometrie** (Metatron-13-Knoten-Topologie), **Quantenalgorithmen** (Grover, QAOA, VQE) und **Multi-Chain-Unterstützung** (Bitcoin, Ethereum, Cosmos, Monero und mehr) zu einem leistungsstarken Werkzeug für Blockchain-Forensik, Anomalie-Erkennung und Entity-Clustering.

---

## Key Features

### 🔮 5D Geometric Analysis
- **Fünfdimensionaler Vektorraum**: Entities werden in R³ × R² eingebettet
- **Spektrale Signatur (ψ, ρ, ω)**: Kohärenz, Stabilität und Effizienz als fundamentales Triplett
- **Invariante Resonanz**: D = ψ · ρ · ω (mathematisch bewiesene Formel)
- **Metatron-Geometrie**: 13-Knoten heilige Geometrie-Einbettung
- **Holistische Matrix**: Multi-Stage-Evaluierung (Kosmokrator, Chronokrator, Mandorla, Monolith, Torus)

### ⚡ Quantum-Enhanced Search
- **Grover-Suche**: Quadratische Beschleunigung für Pattern Matching
- **QAOA**: Ring-Signatur-Analyse und kombinatorische Optimierung
- **VQE**: Eigenwert-Berechnung für Graph-Analyse
- **Quantum Walk**: Probabilistische Exploration
- **SCS Bridge**: Seraphic Calibration System für automatisches Parameter-Tuning

### 🔗 Multi-Chain Blockchain Forensics
- **Bitcoin**: P2PKH, P2SH, P2WPKH, P2WSH, P2TR (Taproot) • BIP32/44/84/86
- **Ethereum/EVM**: EIP-55 Checksummed • k256 ECDSA • Keccak-256
- **Cosmos SDK**: Bech32 (atom, osmo, juno) • Multi-Chain-Derivation
- **Monero**: Ed25519 Stealth Addresses • Ring-Signatur-Forensik
- **Weitere**: Substrate (SS58), Solana (Base58), Cardano (Bech32)

### 🕵️ Advanced Forensic Capabilities
- **Entity Clustering**: KNN, DBSCAN, Hierarchical Clustering
- **Anomalie-Erkennung**: Z-Score-basierte Outlier-Detection
- **Hotspot-Identifikation**: Hochdichte Cluster-Erkennung
- **Topologische Analyse**: Graph-Eigenschaften, Artikulationspunkte, Betti-Zahlen
- **Entropy-Analyse**: Feature-Verteilungs-Metriken
- **Ring-Signatur-Analyse**: Monero-spezifische Heuristiken

### 🌐 Modern Web Interface
- **React 18 Dashboard**: TypeScript • TailwindCSS • ShadcnUI
- **Echtzeit-Updates**: WebSocket-Integration mit 8 Event-Typen
- **Interaktive Visualisierungen**: D3.js Netzwerk-Graphen • ECharts Diagramme
- **Command Palette**: Schnellzugriff auf alle Funktionen (⌘+K)
- **Dark/Light Theme**: System-aware Theming
- **Responsive Design**: Desktop und Tablet optimiert

### 🔐 Stealth Networking
- **API-Mimikry**: OpenAI, Slack, Telegram, Discord, Generic REST
- **Steganographie**: Zero-Width Characters, Header-Payloads, Parameter-Embedding
- **Proxy-Management**: SOCKS5/HTTP Rotation • Request Randomization
- **Payload-Shaping**: Traffic-Normalisierung für autorisierte Operationen

---

## Architecture

PHOSPHOROS folgt einer **4-Layer-Architektur** mit klarer Separation of Concerns:

```
┌─────────────────────────────────────────────────────────────────────┐
│                         PHOSPHOROS PLATFORM                          │
├─────────────────────────────────────────────────────────────────────┤
│  🖥️  PRESENTATION LAYER                                             │
│  ├─ phosphoros-web        React 18 + TypeScript (PRIMARY)          │
│  │  ├─ Dashboard          Metrics Overview & Activity Feed         │
│  │  ├─ Wallet Page        BIP-39 Multi-Chain Derivation            │
│  │  ├─ Resonance Page     5D Spectral Analysis & Visualization     │
│  │  ├─ Cluster Page       KNN/DBSCAN/Hierarchical Clustering       │
│  │  ├─ Topology Page      D3.js Network Force Graph                │
│  │  ├─ Explorer           Entity Search & Investigation            │
│  │  ├─ Anomalies          Alert Management & Detection             │
│  │  ├─ Forensics          Investigation Workflows                  │
│  │  ├─ Infogenetik        Specialized Analysis                     │
│  │  └─ Settings           Configuration Management                 │
│  ├─ phosphoros-cli        TRITON Command-Line Interface            │
│  └─ phosphoros-dashboard  Desktop GUI (DEPRECATED)                 │
├─────────────────────────────────────────────────────────────────────┤
│  🌐  API LAYER                                                      │
│  └─ phosphoros-gateway    Axum REST + WebSocket + OpenAPI          │
│     ├─ Satellite API      Blockchain Forensics (11 endpoints)      │
│     ├─ Resonance API      5D Spectral Analysis (4 endpoints)       │
│     ├─ Wallet API         BIP-39 Derivation (4 endpoints)          │
│     ├─ Cluster API        Entity Clustering (4 endpoints)          │
│     ├─ WebSocket Server   Real-time Events (8 types)               │
│     ├─ /swagger-ui/       Interactive API Documentation            │
│     └─ /metrics           Prometheus Metrics Export                │
├─────────────────────────────────────────────────────────────────────┤
│  🔬  ANALYSIS LAYER                                                 │
│  ├─ phosphoros-satellite  Blockchain Forensic Analysis Engine      │
│  │  ├─ Feature Matrix Construction                                 │
│  │  ├─ Distance Computation (L2 Metric)                            │
│  │  ├─ KNN Graph Construction                                      │
│  │  ├─ Hotspot Detection                                           │
│  │  ├─ Anomaly Scoring                                             │
│  │  └─ Topological Analysis                                        │
│  ├─ phosphoros-quantum    Quantum Algorithm Implementations        │
│  │  ├─ Grover Search                                               │
│  │  ├─ QAOA                                                        │
│  │  ├─ VQE                                                         │
│  │  ├─ Quantum Walk                                                │
│  │  └─ SCS Bridge                                                  │
│  ├─ phosphoros-monero     Ring Signature & Stealth Analysis        │
│  │  ├─ Ring Member Analysis                                        │
│  │  ├─ Temporal Heuristics                                         │
│  │  ├─ Decoy Selection Analysis                                    │
│  │  └─ Key Image Tracking                                          │
│  └─ cryptogenetik-core    Topological Search Optimization          │
│     ├─ WT (Wormhole Funnel)                                        │
│     ├─ SW (Threshold Sweep)                                        │
│     ├─ DK (Double Kick)                                            │
│     └─ PI (Path Invariance)                                        │
├─────────────────────────────────────────────────────────────────────┤
│  ⚙️  CORE LAYER                                                     │
│  ├─ phosphoros-core       5D Geometry & Resonance Engine           │
│  │  ├─ Point5D            Complete 5D vector primitive             │
│  │  ├─ SpectralSignature  (ψ, ρ, ω) triplet                       │
│  │  ├─ MetatronGeometry   13-node sacred geometry                 │
│  │  └─ HolisticMatrix     Multi-stage resonance evaluation        │
│  ├─ phosphoros-bip39      Multi-Chain HD Wallet Derivation         │
│  │  ├─ BIP39 Mnemonic Support (12+ languages)                     │
│  │  ├─ BIP32/44/84/86 Paths                                       │
│  │  └─ 7 Blockchain Implementations                                │
│  ├─ phosphoros-stealthnet Privacy Protocol & Covert Networking     │
│  ├─ phosphoros-types      Shared Type Definitions                  │
│  └─ phosphoros-kryptogenetik Legacy 5D Scalar Projection          │
└─────────────────────────────────────────────────────────────────────┘
```

### Technology Stack

**Backend (Rust ~14,600 Lines)**

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Web Framework | Axum 0.7 | Async REST + WebSocket |
| Runtime | tokio 1.x | Multi-threaded async executor |
| Math | nalgebra, ndarray, petgraph | Linear algebra & graph theory |
| Cryptography | sha2, sha3, blake3, k256, curve25519-dalek | Multi-chain crypto primitives |
| HTTP Client | reqwest (SOCKS5) | External API calls |
| Serialization | serde, serde_json, serde_yaml | Data interchange |
| API Docs | utoipa | OpenAPI 3.0 auto-generation |
| Metrics | Prometheus | Observability |

**Frontend (TypeScript/React ~6,800 Lines)**

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Framework | React 18.3 + TypeScript 5.6 | Type-safe UI development |
| Build Tool | Vite 5.4 | Fast dev server & builds |
| Styling | TailwindCSS 3.4 + ShadcnUI | Utility-first CSS + components |
| State | Zustand 5.0 | Lightweight state management |
| Data Fetching | React Query 5.x | Server state & caching |
| Visualization | D3.js 7.9 + ECharts 5.5 | Network graphs & charts |
| Routing | React Router DOM 6.28 | Client-side routing |
| UI Components | Radix UI | Accessible primitives |

---

## Quick Start

### Prerequisites

- **Rust**: 1.75+ mit Cargo ([Installation](https://rustup.rs/))
- **Node.js**: 18+ für Web Dashboard ([Installation](https://nodejs.org/))
- **Docker**: Optional für Container-Deployment ([Installation](https://docs.docker.com/get-docker/))

### Installation & Build

```bash
# Clone Repository
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros

# Build Rust Workspace (Release Mode)
cargo build --workspace --release

# Run Tests
cargo test --workspace --all-features

# Generate Documentation
cargo doc --workspace --open
```

### Start Services

#### 1. Gateway API (Backend)

```bash
cargo run -p phosphoros-gateway --release
```

**Verfügbare Endpoints:**
- `http://localhost:8080/` - Service Information
- `http://localhost:8080/health` - Health Check
- `http://localhost:8080/swagger-ui/` - Interactive API Documentation
- `http://localhost:8080/api-docs/openapi.json` - OpenAPI Spec
- `http://localhost:8080/metrics` - Prometheus Metrics
- `ws://localhost:8080/ws` - WebSocket Real-time Events

#### 2. Web Dashboard (Frontend)

```bash
cd phosphoros-web
npm install
npm run dev
```

**Zugriff:** `http://localhost:5173`

#### 3. TRITON CLI (Command-Line)

```bash
cargo run -p phosphoros-cli --release -- --mode auto --cycles 40
```

**CLI Optionen:**
- `--name`: Instance Name (default: "TRITON")
- `--mode`: Operation Mode - auto/memory/hybrid (default: auto)
- `--cycles`: Auto Operation Cycles (default: 40)
- `--wordlist`: Wordlist File Path (default: data/wordlist.txt)

---

## Docker Deployment

### Using Docker Compose (Recommended)

```bash
# Start all services
docker-compose up -d

# Services:
# - Gateway API: http://localhost:8080
# - Web Dashboard: http://localhost:3000
# - Swagger UI: http://localhost:8080/swagger-ui/
# - WebSocket: ws://localhost:3000/ws

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Manual Docker Build

```bash
# Build image
docker build -t phosphoros:latest .

# Run gateway
docker run -p 8080:8080 phosphoros:latest

# Run web (production build included in main image)
docker run -p 3000:80 phosphoros:latest
```

**Siehe [DEPLOYMENT.md](DEPLOYMENT.md) für umfassende Deployment-Anweisungen, Troubleshooting und Production-Hardening.**

---

## Web Interface

Die **PHOSPHOROS Web-Oberfläche** ist die primäre Benutzeroberfläche für alle Forensik- und Analyse-Funktionen. Sie bietet 10 spezialisierte Seiten:

### 📊 Dashboard (Home)
- **Echtzeit-Metriken**: Seeds, Clusters, Entities, Anomalien
- **Activity Feed**: Live-Updates zu allen Systemereignissen
- **Service Status**: Scraper, Analyzer, Cluster Engine
- **Quick Actions**: Import Mnemonic, Start Analysis, View Clusters, Export Data
- **Export-Funktion**: JSON-Reports mit vollständigen Metriken

### 🔑 Wallet & Seeds
- **BIP-39 Mnemonic Import**: 12/24-Wort-Phrasen (12+ Sprachen)
- **Multi-Chain Derivation**: 7 Blockchains gleichzeitig
- **Address Range**: Konfigurierbare Start/End-Indizes
- **Blockchain-Auswahl**: Bitcoin, Ethereum, Polkadot, Kusama, Cosmos, Solana, Cardano
- **Wallet-Verwaltung**: Liste, Details, Löschen von importierten Wallets
- **WebSocket-Events**: Echtzeit-Benachrichtigungen bei erfolgreicher Derivation

### 🌈 Resonance Analysis
- **Spektrale Signatur**: Live-Visualisierung von ψ (Coherence), ρ (Stability), ω (Efficiency)
- **Resonanz-Gauge**: ECharts-basiertes Gauge für D = ψ·ρ·ω
- **Operator-Konfiguration**: WT, SW, DK, PI topologische Operatoren
- **Live-Analyse**: Start/Stop-Funktion mit Progress-Tracking
- **Time-Series-Chart**: Historische Resonanz-Daten
- **Gateway-Integration**: Compute Spectral & Evaluate Resonance API-Calls
- **Holistic Matrix**: Output/Gated-Status mit 5D Action Vectors

### 🔗 Cluster Analysis
- **Entity-Eingabe**: Address + Feature-Vector (CSV)
- **Algorithmus-Auswahl**: KNN, DBSCAN, Hierarchical
- **Parameter-Tuning**: k-Wert für KNN, Threshold für DBSCAN/Hierarchical
- **Snapshot-Management**: Cluster-Berechnungen mit IDs
- **Cluster-Explorer**: Expandierbare Cluster mit Member-Details
- **Cohesion-Scoring**: Cluster-Qualitäts-Metrik
- **WebSocket-Events**: Real-time Cluster Computed Notifications

### 🕸️ Topology Viewer
- **D3.js Force Graph**: Interaktive Netzwerk-Visualisierung
- **Node-Interaktion**: Drag & Drop, Zoom, Pan
- **Entity-Relationships**: Visuelle Darstellung von Verbindungen
- **Graph-Metriken**: Knoten, Kanten, Komponenten

### 🔍 Explorer (Investigation)
- **Entity-Suche**: Address-basierte Lookups
- **Transaction-Verfolgung**: Multi-Chain-Transaktions-Historie
- **Feature-Extraktion**: Automatische Feature-Vektoren

### ⚠️ Anomalies
- **Alert-Management**: Anomalie-Detection-Dashboard
- **Z-Score-basierte Detection**: Statistische Outlier-Erkennung
- **Hotspot-Identifikation**: High-Density Cluster Alerts

### 🧪 Forensics
- **Investigation Workflows**: Guided Forensic Analysis
- **Report-Generierung**: Comprehensive Forensic Reports
- **Evidence-Tracking**: Chain of Custody

### 🧬 Infogenetik
- **Specialized Analysis**: Infogenetic Pattern Recognition
- **Topological Operators**: WT, SW, DK, PI Visualization

### ⚙️ Settings
- **Theme-Toggle**: Dark/Light Mode
- **API-Konfiguration**: Backend-URL Management
- **System-Konfiguration**: Service-Parameter

**Siehe [GUI_DOCUMENTATION.md](GUI_DOCUMENTATION.md) für detaillierte Funktionsbeschreibungen.**

---

## Core Concepts

### 5D Information Space

Entities werden als Punkte in einem fünfdimensionalen Vektorraum `R³ × R²` dargestellt:

```rust
use phosphoros_core::Point5D;

let entity = Point5D::new(
    x,   // Spatial Coordinate 1
    y,   // Spatial Coordinate 2
    z,   // Spatial Coordinate 3
    u,   // Internal Coordinate 1 (Behavioral)
    v    // Internal Coordinate 2 (Temporal)
);

// Linear Algebra Operations
let distance = entity1.distance_to(&entity2);
let dot_product = entity1.dot(&entity2);
let normalized = entity1.normalize();
```

### Spectral Signature (ψ, ρ, ω)

Das fundamentale Triplett zur Charakterisierung von Entity-Resonanz:

| Component | Symbol | Beschreibung | Range |
|-----------|--------|--------------|-------|
| Coherence | ψ (Psi) | Phasensynchronisationsmaß | [0, 1] |
| Stability | ρ (Rho) | Strukturelle Dichte-Metrik | [0, 1] |
| Efficiency | ω (Omega) | Oszillationsraten-Indikator | [0, 1] |

**Invariante Resonanz-Formel:**

```
D = ψ · ρ · ω
```

```rust
use phosphoros_core::SpectralSignature;

let signature = SpectralSignature::new(0.85, 0.92, 0.78);
let resonance = signature.resonance(); // D = 0.85 * 0.92 * 0.78 = 0.610
```

### Metatron Sacred Geometry

Eine 13-Knoten kanonische Topologie zur Einbettung von Entities in den 5D-Raum:

```rust
use phosphoros_core::MetatronGeometry;

let geometry = MetatronGeometry::new();
let embedded = geometry.embed(&entity_features);
```

### Holistic Resonance Matrix

Multi-Stage Evaluation Engine mit Gating-Kriterien:

| Stage | Function | Description |
|-------|----------|-------------|
| **Kosmokrator** | Proof-of-Resonance Gating | Phase Coherence Filtering (ψ > threshold) |
| **Chronokrator** | Temporal Dynamics | Expansion Tracking mit t-Parameter |
| **Mandorla** | Perception-Intention | Intersection P⃗ · I⃗ (dot product) |
| **Monolith** | Action Singularity | Geometric Criterion Trigger |
| **Torus** | Phase Space | S¹ × S¹ Topology für State Tracking |

**Output Types:**
- `Output`: Erfolgreiche Evaluation → 5D Action Vector + Resonance Score
- `Gated`: Blockiert durch Kosmokrator → Reason String

---

## API Reference

### Gateway Endpoints (25 Total)

#### Health & Info (2)
```
GET  /                    # Gateway Information
GET  /health             # Health Check
```

#### Satellite Forensics API (11)
```
GET  /satellite/health                    # Health Check
GET  /satellite/v1/snapshots              # List Snapshots
POST /satellite/v1/snapshots              # Ingest New Snapshot
POST /satellite/v1/analyze/:id            # Run Analysis
GET  /satellite/v1/reports/latest         # Get Latest Report
GET  /satellite/v1/reports/:snapshot_id   # Get Specific Report
GET  /satellite/v1/entities/:snapshot_id  # List Entities
POST /satellite/v1/entities/:snapshot_id  # Add Entity
GET  /satellite/v1/metrics/:snapshot_id   # Get Metrics
DELETE /satellite/v1/snapshots/:id        # Delete Snapshot
POST /satellite/v1/reset                  # Reset All Data
```

#### Resonance API (4)
```
POST /resonance/analyze     # Analyze Resonance (Holistic Matrix)
POST /resonance/spectral    # Compute Spectral Signature
GET  /resonance/history     # Get Resonance History
DELETE /resonance/history   # Clear History
```

#### Wallet API (4)
```
POST /wallet/import        # Import BIP-39 Mnemonic
POST /wallet/derive        # Derive Addresses (Multi-Chain)
GET  /wallet/list          # List Stored Wallets
DELETE /wallet/:label      # Remove Wallet by Label
```

#### Cluster API (4)
```
POST /cluster/compute           # Compute Clusters (KNN/DBSCAN/Hierarchical)
GET  /cluster/all               # List All Clusters
GET  /cluster/:snapshot_id      # Get Clusters for Snapshot
GET  /cluster/:snapshot_id/:id  # Get Cluster Members
```

#### Other Endpoints
```
GET  /swagger-ui/          # Swagger UI (Interactive API Docs)
GET  /api-docs/openapi.json # OpenAPI 3.0 Specification
GET  /metrics              # Prometheus Metrics
WS   /ws                   # WebSocket Real-time Events
```

### WebSocket Events (8 Types)

```typescript
type WebSocketEvent =
  | { type: 'Log', level: string, message: string, timestamp: string }
  | { type: 'AnalysisProgress', snapshot_id: string, progress: number }
  | { type: 'AnalysisComplete', snapshot_id: string }
  | { type: 'ServiceStatus', service: string, status: string }
  | { type: 'ResonanceEvaluated', score: number, timestamp: string }
  | { type: 'WalletDerived', blockchain: string, count: number, timestamp: string }
  | { type: 'ClusterComputed', snapshot_id: string, num_clusters: number, timestamp: string }
  | { type: 'Notification', level: string, message: string }
```

---

## Quantum Algorithms

PHOSPHOROS integriert simulierte Quantenalgorithmen für enhanced Analysis:

| Algorithm | Application | Implementation | Status |
|-----------|-------------|----------------|--------|
| **Grover Search** | Pattern Matching mit quadratischer Beschleunigung | Oracle-based mit optimalen Iterationen | Production |
| **QAOA** | Ring Signature Analysis, Combinatorial Optimization | Variational Parameter Optimization | Production |
| **VQE** | Eigenvalue Computation für Graph Analysis | Ansatz-based Hybrid Approach | Production |
| **Quantum Walk** | Probabilistische Exploration Heuristics | Continuous/Discrete Time Variants | Production |
| **SCS Bridge** | Automatic Parameter Tuning | Seraphic Calibration System | Production |

```rust
use phosphoros_quantum::{GroverSearch, LocalSimulator};

let backend = LocalSimulator::new(num_qubits);
let grover = GroverSearch::new(search_space_size);
let result = grover.execute(&backend, oracle, optimal_iterations)?;
```

**Hinweis:** Alle Quantenalgorithmen laufen auf einem **klassischen Simulator**. Es ist keine echte Quanten-Hardware erforderlich.

---

## Stealth Networking

PHOSPHOROS enthält fortgeschrittene Stealth-Fähigkeiten für **autorisierte forensische Operationen**:

### API Mimicry

Traffic als legitime API-Calls tarnen:
- OpenAI API Format
- Slack API Format
- Telegram Bot API Format
- Discord API Format
- Generic REST Patterns

### Steganographic Encoding

- Zero-Width Character Encoding
- Header-based Invisible Payloads
- Parameter-embedded Data
- Body Steganography

### Proxy Management

- SOCKS5/HTTP Proxy Rotation
- Request Randomization (Temporal, Semantic, Structural)
- Payload Shaping für Traffic Normalization

**⚠️ Wichtiger Hinweis:** Diese Fähigkeiten sind **ausschließlich für autorisierte Nutzung** vorgesehen:
- Forensische Untersuchungen
- Security Research
- Compliance-approved Operations
- CTF Competitions
- Defensive Security

---

## Monero Forensics

Spezialisierte Analyse für Privacy-focused Cryptocurrencies:

### Ring Signature Analysis

```rust
use phosphoros_monero::{RingAnalyzer, TemporalHeuristic, DecoySelectionHeuristic};

let analyzer = RingAnalyzer::new(rpc_client);
let likely_real = analyzer.analyze_ring(&ring_members)?;
```

### Capabilities

- **Ring Member Analysis**: Heuristic-based Real Input Identification
- **Temporal Patterns**: Timing-based Ring Member Analysis
- **Decoy Selection**: Decoy Pattern Detection
- **Key Image Tracking**: Spent Output Detection
- **Transaction Graph**: Graph Construction mit Anomaly Reporting
- **Quantum-Enhanced**: QAOA-based Ring Analysis (optional)

---

## Development

### Testing

```bash
# All Tests
cargo test --workspace --all-features

# Specific Crate
cargo test -p phosphoros-quantum --all-features

# With Output
cargo test --workspace -- --nocapture

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --all-features --out Html
```

**Test Statistics:**
- 139+ Unit Tests
- Integration Tests für alle APIs
- WebSocket Event Tests
- React Component Tests (via Vitest)

### Benchmarks

```bash
# Core Benchmarks
cargo bench -p phosphoros-core

# Kryptogenetik Benchmarks
cargo bench -p phosphoros-kryptogenetik
```

### Code Quality

```bash
# Formatting
cargo fmt --all -- --check

# Linting
cargo clippy --workspace -- -D warnings

# Security Audit
cargo audit
```

### Web Development

```bash
cd phosphoros-web

# Development Server
npm run dev

# Production Build
npm run build

# Type Checking
npm run lint

# Preview Production Build
npm run preview
```

---

## Configuration

### Environment Variables

**Rust Services:**
```bash
RUST_LOG=info                        # Logging Level (trace, debug, info, warn, error)
PHOSPHOROS_HOST=0.0.0.0              # Bind Address
PHOSPHOROS_PORT=8080                 # Gateway Port
```

**Web Frontend:**
```bash
VITE_API_URL=http://localhost:8080   # Backend API URL
```

### Satellite Configuration

Beispiel `config.yaml`:

```yaml
analysis:
  knn_k: 10                          # K-Nearest Neighbors
  entropy_bins: 50                   # Entropy Histogram Bins
  resonance_threshold: 0.7           # Minimum Resonance Score
  max_snapshots: 1000                # Maximum Stored Snapshots

server:
  host: "0.0.0.0"
  port: 8080
  workers: 4

websocket:
  heartbeat_interval: 30             # Seconds
  client_timeout: 60                 # Seconds
```

---

## Project Structure

```
phosphoros/
├── crates/                           # Rust Workspace (12 Crates)
│   ├── cryptogenetik-core/           # Topological Search Optimization
│   ├── phosphoros-bip39/             # Multi-Chain HD Wallet Derivation
│   ├── phosphoros-cli/               # TRITON Command-Line Interface
│   ├── phosphoros-core/              # 5D Geometry & Resonance Engine
│   ├── phosphoros-dashboard/         # Desktop GUI (DEPRECATED)
│   ├── phosphoros-gateway/           # Axum REST API Gateway
│   ├── phosphoros-monero/            # Monero Ring Signature Forensics
│   ├── phosphoros-quantum/           # Quantum Algorithm Implementations
│   ├── phosphoros-satellite/         # Blockchain Forensic Analysis Engine
│   ├── phosphoros-stealthnet/        # Stealth Networking & Privacy
│   ├── phosphoros-types/             # Shared Type Definitions
│   └── (Individual README.md files per crate)
│
├── phosphoros-kryptogenetik/         # Legacy 5D Scalar Projection System
│
├── phosphoros-web/                   # React Web Dashboard (PRIMARY UI)
│   ├── src/
│   │   ├── components/
│   │   │   ├── ui/                   # ShadcnUI Primitives
│   │   │   ├── layout/               # AppShell, Sidebar, Header
│   │   │   ├── charts/               # ECharts Wrappers
│   │   │   ├── graphs/               # D3.js Force Graph
│   │   │   └── common/               # Shared Components
│   │   ├── features/                 # Page Components
│   │   │   ├── dashboard/            # Home Overview
│   │   │   ├── wallet/               # Seed Management
│   │   │   ├── resonance/            # Spectral Analysis
│   │   │   ├── cluster/              # Clustering
│   │   │   ├── topology/             # Network Graph
│   │   │   ├── investigation/        # Explorer, Anomalies, Forensics
│   │   │   ├── infogenetik/          # Infogenetic Analysis
│   │   │   └── settings/             # Configuration
│   │   ├── hooks/                    # Custom React Hooks
│   │   │   ├── useApi.ts             # 15 API Hooks
│   │   │   └── useWebSocket.ts       # WebSocket Integration
│   │   ├── stores/                   # Zustand State Management
│   │   ├── lib/                      # Utilities
│   │   ├── types/                    # TypeScript Types
│   │   ├── App.tsx                   # Route Definitions
│   │   └── main.tsx                  # React Entry Point
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   └── README.md
│
├── docs/                             # Comprehensive Documentation
│   ├── architecture.md               # System Design & Implementation
│   ├── api.md                        # REST API Reference
│   ├── quantum.md                    # Quantum Algorithms Documentation
│   ├── stealth-networking.md         # Stealth Capabilities
│   ├── law-enforcement.md            # Investigator Usage Guide
│   └── explorer-guide.md             # Interactive Explorer Documentation
│
├── Cargo.toml                        # Workspace Configuration
├── Cargo.lock                        # Dependency Lock File
├── docker-compose.yml                # Multi-Service Orchestration
├── Dockerfile                        # Multi-Stage Build (Rust + Node)
├── README.md                         # This File
├── GUI_DOCUMENTATION.md              # Detailed GUI Function Documentation
├── DEPLOYMENT.md                     # Production Deployment Guide
├── PHASE_2_STATUS.md                 # Consolidation Status
└── CONSOLIDATION_PLAN.md             # Architecture Consolidation Notes
```

---

## Documentation

Umfassende Dokumentation verfügbar in `/docs`:

| Dokument | Beschreibung |
|----------|--------------|
| **architecture.md** | Detaillierter System-Design & Implementation Blueprint |
| **api.md** | Vollständige REST API Referenz |
| **quantum.md** | Quantenalgorithmus-Dokumentation |
| **stealth-networking.md** | Stealth-Capabilities Übersicht |
| **law-enforcement.md** | Investigator Usage Guide |
| **explorer-guide.md** | Interactive Explorer Dokumentation |
| **GUI_DOCUMENTATION.md** | Detaillierte GUI-Funktionsbeschreibungen |
| **DEPLOYMENT.md** | Production Deployment & Troubleshooting |

---

## Security Notice

⚠️ **Wichtiger Sicherheitshinweis**

Dieses System ist ausschließlich für **forensische Analyse und autorisierte Forschung** konzipiert:

- ❌ **NICHT** für die Verwaltung echter Kryptowährungs-Holdings verwenden
- ❌ Generierte Keys sollten **NICHT** für Production-Wallets verwendet werden
- ℹ️ Quantenalgorithmen laufen auf **klassischer Simulation**
- ⚠️ Stealth-Capabilities erfordern **ordnungsgemäße Autorisierung**
- 📋 Einhaltung **geltender Gesetze und Vorschriften** ist obligatorisch

**Autorisierte Nutzungsszenarien:**
- ✅ Blockchain Forensic Investigations
- ✅ Security Research & Audits
- ✅ Compliance-approved Operations
- ✅ Educational & CTF Competitions
- ✅ Defensive Security Analysis

---

## Contributing

Beiträge sind willkommen! Bitte beachte:

1. **Code Quality**: `cargo fmt` und `cargo clippy` vor dem Commit
2. **Tests**: Neue Features benötigen Tests
3. **Documentation**: Dokumentiere öffentliche APIs
4. **Security**: Keine Einführung von Schwachstellen

---

## License

Dual-lizenziert unter [MIT](LICENSE-MIT) oder [Apache 2.0](LICENSE-APACHE).

Sie können zwischen den Lizenzen wählen:
- **MIT License**: Permissive, einfache Lizenz
- **Apache 2.0**: Permissive mit explizitem Patent-Grant

---

## Project Status

**Phase 2: Web Consolidation** ✅ **COMPLETED** (2026-01-01)

Die Plattform hat erfolgreich die Migration zu einer **unified web-only architecture** abgeschlossen:

- ✅ Desktop GUI (`phosphoros-dashboard`) → DEPRECATED
- ✅ Web GUI (`phosphoros-web`) → PRIMARY INTERFACE
- ✅ Gateway vollständig integriert (25 Endpoints)
- ✅ WebSocket Real-time Streaming (8 Event Types)
- ✅ Type-safe End-to-End (TypeScript + Rust)
- ✅ Production-Ready Deployment

**Aktuelle Metriken:**
- **14,600+ Zeilen** Rust-Code
- **6,800+ Zeilen** TypeScript/React-Code
- **139+ Unit Tests** ✅
- **25+ API Endpoints**
- **8 WebSocket Event Types**
- **10 Web Pages**
- **12 Rust Crates**

Siehe [PHASE_2_STATUS.md](PHASE_2_STATUS.md) und [DEPLOYMENT.md](DEPLOYMENT.md) für Details.

---

<div align="center">

**PHOSPHOROS**

*5D Geometric-Cybernetic Blockchain Forensics*

🔮 **Quantum-Enhanced** • 🔗 **Multi-Chain** • 🌐 **Modern Web Interface**

[Documentation](docs/) • [API Reference](http://localhost:8080/swagger-ui/) • [GUI Docs](GUI_DOCUMENTATION.md) • [Report Issue](https://github.com/LashSesh/phosphoros/issues)

---

Made with ⚡ by the PHOSPHOROS Team

</div>
