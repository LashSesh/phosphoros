# PHOSPHOROS

<div align="center">

**Infogenetisches Blockchain-Forensik-System**

*Basierend auf der Axiomatischen 5D-Spiral-Topologie von Sebastian Klemm*

[![Rust](https://img.shields.io/badge/Rust-1.75+-f74c00?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178c6?logo=typescript)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)

</div>

---

## Konzept

PHOSPHOROS implementiert ein **geometrisch-kybernetisches Framework** zur Blockchain-Analyse. Das System modelliert Transaktionen und Wallet-Zustände als Punkte in einem fünfdimensionalen Zustandsraum, wobei strukturierte Trajektorien in Form von Spiralpfaden erscheinen.

### Theoretische Grundlage

Das Framework basiert auf der **Axiomatischen 5D-Spiral-Topologie** (Klemm, 2025):

```
S = (X, Γ, π_spatial, π_internal, Ψ, W, Φ)
```

| Symbol | Beschreibung |
|--------|--------------|
| **X** | 5D-Zustandsraum (R³ × R²) |
| **Γ** | Menge der Spiralpfade γ : I → X |
| **π_spatial** | Projektion auf räumliche Koordinaten (x, y, z) |
| **π_internal** | Projektion auf interne Koordinaten (x₄, x₅) |
| **Ψ** | Resonanzfelder {ψ, ρ, ω} |
| **W** | Aktionsfunktional auf Sequenzen |
| **Φ** | Dynamikoperator (kybernetisches Feedback) |

### Resonanz-Triplett (ψ, ρ, ω)

Die fundamentale Signatur für alle Analysen:

```rust
use phosphoros_core::SpectralSignature;

let sig = SpectralSignature::new(0.85, 0.92, 0.78);
let resonance = sig.resonance();  // D = ψ · ρ · ω
```

| Komponente | Symbol | Bedeutung |
|------------|--------|-----------|
| Kohärenz | ψ (Psi) | Phasensynchronisation |
| Stabilität | ρ (Rho) | Strukturelle Dichte |
| Effizienz | ω (Omega) | Oszillationsrate |

---

## Architektur

```
┌─────────────────────────────────────────────────────────────────┐
│                         PHOSPHOROS                              │
├─────────────────────────────────────────────────────────────────┤
│  PRÄSENTATION                                                   │
│  ├── phosphoros-dashboard    Iced Desktop GUI (Living Lab)     │
│  ├── phosphoros-web          React 18 Web Dashboard            │
│  └── phosphoros-cli          Terminal Interface + Visualizer   │
├─────────────────────────────────────────────────────────────────┤
│  API                                                            │
│  └── phosphoros-gateway      REST + OpenAPI + Prometheus       │
├─────────────────────────────────────────────────────────────────┤
│  ANALYSE                                                        │
│  ├── phosphoros-satellite    Forensische Analyse-Engine        │
│  ├── phosphoros-quantum      Quanten-Algorithmen (QAOA, VQE)   │
│  ├── phosphoros-monero       Ring-Signatur-Analyse             │
│  └── cryptogenetik-core      Quanten-beschleunigte Suche       │
├─────────────────────────────────────────────────────────────────┤
│  KERN                                                           │
│  ├── phosphoros-core         5D-Geometrie & Resonanz-Engine    │
│  ├── phosphoros-bip39        Multi-Chain HD-Wallet-Derivation  │
│  ├── phosphoros-stealthnet   Privacy-Protokoll-Analyse         │
│  ├── phosphoros-types        Gemeinsame Typdefinitionen        │
│  └── phosphoros-kryptogenetik InfoGenetik-Integration          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Unterstützte Blockchains

| Chain | Adresstypen | Status |
|-------|-------------|--------|
| **Bitcoin** | P2PKH, P2SH-P2WPKH, P2WPKH, P2TR (Taproot) | Produktiv |
| **Ethereum** | EIP-55 Checksummed | Produktiv |
| **Cosmos SDK** | Bech32 (atom, osmo, juno, etc.) | Produktiv |
| **Monero** | Ed25519 Stealth Addresses | Produktiv |

---

## Installation

### Voraussetzungen

- Rust 1.75+ mit Cargo
- Node.js 18+ (für Web-Dashboard)
- Docker (optional)

### Build

```bash
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros

# Workspace kompilieren
cargo build --workspace --release

# Tests ausführen
cargo test --workspace
```

### API-Gateway starten

```bash
cargo run -p phosphoros-gateway --release
```

**Endpunkte:**
- `http://localhost:8080/` - Service-Info
- `http://localhost:8080/swagger-ui/` - API-Dokumentation
- `http://localhost:8080/metrics` - Prometheus Metriken
- `http://localhost:8080/health` - Health Check

### Desktop-Dashboard (Living Lab)

```bash
cargo run -p phosphoros-dashboard --release
```

### Web-Dashboard

```bash
cd phosphoros-web
npm install
npm run dev  # http://localhost:5173
```

---

## Quanten-Algorithmen

PHOSPHOROS integriert simulierte Quanten-Algorithmen für erweiterte Analyse:

| Algorithmus | Anwendung |
|-------------|-----------|
| **Grover** | Beschleunigte Suche in Adressräumen |
| **QAOA** | Kombinatorische Optimierung |
| **VQE** | Variational Quantum Eigensolver |
| **Quantum Walk** | Graph-Traversierung |
| **SCS** | Spectral Clustering Search |

```rust
use phosphoros_quantum::GroverSearch;

let grover = GroverSearch::new(search_space);
let result = grover.execute(oracle, iterations)?;
```

---

## 5D-Spiralpfade

Das System unterscheidet verschiedene Klassen von Spiralpfaden:

### Archimedische Spirale

```
r(t) = a + bt       (konstanter radialer Abstand)
φ(t) = t
h(t) = ct
```

### Logarithmische Spirale

```
r(t) = a·e^(bt)     (exponentielles Wachstum)
φ(t) = t
h(t) = ct
```

### Verallgemeinerte 5D-Spirale

```rust
use phosphoros_core::{Point5D, SpiralPath};

let spiral = SpiralPath::generalized(|t| {
    Point5D::new(
        R(t) * t.cos(),
        R(t) * t.sin(),
        H(t),
        U(t),
        V(t)
    )
});
```

---

## Kryptogenetik-Module

### DK-Lock Optimizer

Deterministisches Schlüssel-Locking für Seed-Rekonstruktion.

### Wormdorf-Trichter

Konvergenz-basierte Suchoptimierung im 5D-Raum.

### Pi-Canonical

Kanonische Darstellung für π-basierte Berechnungen.

### Triton Pipeline

GPU-beschleunigte Verarbeitung für Batch-Operationen.

---

## Entwicklung

### Tests

```bash
# Alle Tests
cargo test --workspace

# Spezifisches Crate mit Features
cargo test -p phosphoros-bip39 --features btc,evm,cosmos
```

### Benchmarks

```bash
cargo bench -p phosphoros-core
cargo bench -p phosphoros-kryptogenetik
```

### Code-Qualität

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

---

## Referenz

**Theoretische Grundlage:**

> Klemm, Sebastian. *Axiomatic 5D Spiral Topology: A Geometric-Cybernetic Framework.* November 2025.

---

## Sicherheitshinweis

Dieses System dient ausschließlich der forensischen Analyse und Forschung.

- Nicht für die Verwaltung echter Kryptowährungen verwenden
- Generierte Schlüssel nicht für produktive Wallets nutzen
- Quanten-Algorithmen laufen auf klassischer Simulation

---

## Lizenz

Dual-lizenziert unter [MIT](LICENSE-MIT) oder [Apache 2.0](LICENSE-APACHE).

---

<div align="center">

**PHOSPHOROS**

*Geometrisch-Kybernetische Blockchain-Forensik*

</div>
