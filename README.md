# PHOSPHOROS

**Quantum-Enhanced Blockchain Forensics Engine**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Quantum](https://img.shields.io/badge/quantum-enabled-blueviolet.svg)](#quantum-algorithms)

PHOSPHOROS is an advanced forensic analysis system combining **5D spectral geometry**, **holistic resonance engines**, and **quantum algorithms** for blockchain investigation. Features the world's first **quantum-based Monero ring signature analysis** using QAOA.

## Quantum Computing Integration

PHOSPHOROS integrates a full quantum simulation framework for enhanced forensic capabilities:

### QAOA Ring Signature Analysis

The **Quantum Approximate Optimization Algorithm** provides probabilistic identification of real transaction inputs in Monero ring signatures:

```rust
use phosphoros_monero::{QuantumRingAnalyzer, RingAnalysisResult};

// Quantum-classical hybrid analysis
let analyzer = QuantumRingAnalyzer::new(depth: 2, shots: 1000)
    .with_quantum_weight(0.5);

let result = analyzer.analyze(&ring_members, &classical_result)?;

println!("Classical prediction: {:?}", result.classical_index);
println!("Quantum prediction: {:?}", result.quantum_index);
println!("Combined confidence: {:.2}%", result.combined_confidence * 100.0);
println!("Quantum improvement: {:.2}%", result.improvement_over_classical * 100.0);
```

### Grover-Accelerated Search

√N speedup for unstructured search in the Triton pipeline:

```rust
use cryptogenetik_core::{QuantumSearchEngine, QuantumSearchConfig};

let engine = QuantumSearchEngine::new(
    QuantumSearchConfig::new(8).with_threshold(0.7)
);

// Hybrid quantum-classical search
let result = engine.hybrid_search(
    search_space,
    |idx| quantum_score_fn(idx),  // Quantum pruning
    |idx| classical_eval(idx),     // Classical evaluation
)?;

println!("Quantum pruning factor: {:.2}%", result.quantum_pruning_factor * 100.0);
println!("Effective speedup: {:.1}x", result.effective_speedup(search_space));
```

### Quantum Algorithms

| Algorithm | Use Case | Speedup |
|-----------|----------|---------|
| **QAOA** | MaxCut optimization, ring decomposition | Problem-dependent |
| **Grover** | Unstructured search | √N |
| **VQE** | Ground state estimation | Exponential (quantum advantage) |
| **Quantum Walk** | Graph centrality, link analysis | Polynomial |

### SCS Calibration System

Seraphic Calibration System with **Double-Kick operator** for automatic hyperparameter tuning:

```
T = Φ_V ∘ Φ_U

Φ_U: velocity update (ψ component)
Φ_V: config update (ρ component)
```

Supports four regimes: `Standard`, `Aggressive`, `Exploring`, `Homeostasis`

## Architecture

```
phosphoros/
├── crates/
│   ├── phosphoros-quantum/      # Quantum computing framework
│   │   ├── backend/             # QuantumBackend trait, LocalSimulator
│   │   ├── algorithms/          # Grover, QAOA, VQE, Quantum Walk
│   │   ├── calibration/         # SCS Bridge, Double-Kick operator
│   │   ├── engines/             # QuantumResonanceEngine, HybridMatrix
│   │   └── forensics/           # RingSignatureQAOA
│   │
│   ├── phosphoros-core/         # 5D geometry + holistic resonance
│   ├── phosphoros-monero/       # Monero forensics + quantum integration
│   ├── cryptogenetik-core/      # Search + Grover acceleration
│   ├── phosphoros-bip39/        # Multichain wallet support
│   ├── phosphoros-satellite/    # Blockchain forensics
│   ├── phosphoros-dashboard/    # GUI Dashboard
│   ├── phosphoros-gateway/      # REST/WS API
│   └── phosphoros-cli/          # Command-line tools
│
├── ouroboros_dna/               # Visualization
└── phosphoros-kryptogenetik/    # Legacy implementation
```

## Core Concepts

### Performance Triplet (ψ, ρ, ω)

The universal resonance invariant used across all PHOSPHOROS systems:

```rust
pub struct SpectralSignature {
    pub psi: f64,    // Coherence (0-1)
    pub rho: f64,    // Stability (0-1)
    pub omega: f64,  // Efficiency (0-1)
}

// Resonance score (INVARIANT)
fn resonance(&self) -> f64 {
    self.psi * self.rho * self.omega
}
```

### 13-Node Metatron Geometry

Sacred geometry topology embedded in 5D space:

- 12 peripheral nodes + 1 central node
- Encodes high-dimensional quantum states
- Used for state space discretization

### Holistic Matrix Engine

Multi-layer resonance evaluation:

1. **Kosmokrator**: Proof-of-Resonance exclusion gate
2. **Chronokrator**: Temporal expansion control
3. **Mandorla Field**: Perception-intention intersection
4. **Monolith**: Action singularity trigger
5. **Torus Topology**: S¹ × S¹ phase space navigation

## Quick Start

### Installation

```bash
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros
cargo build --workspace --all-features --release
```

### Run Dashboard

```bash
cargo run -p phosphoros-dashboard --release
```

### Run Tests

```bash
cargo test --workspace --all-features
```

## Feature Flags

### phosphoros-quantum
- `simulator` (default): Local quantum simulator
- `grover`: Grover's search algorithm
- `qaoa`: Quantum Approximate Optimization
- `vqe`: Variational Quantum Eigensolver
- `quantum-walk`: Continuous-time quantum walks
- `scs`: Seraphic Calibration System
- `full`: All algorithms

### phosphoros-monero
- `quantum`: QAOA ring signature analysis

### cryptogenetik-core
- `quantum`: Grover-accelerated search

### phosphoros-core
- `quantum`: Quantum resonance engine
- `advanced-linalg`: nalgebra/ndarray
- `parallel`: Rayon parallelization

## Monero Ring Signature Analysis

PHOSPHOROS provides the first quantum-enhanced approach to Monero forensics:

### Classical Heuristics
- **Temporal Analysis**: Gamma distribution deviation detection
- **Decoy Selection**: Pattern anomaly identification
- **Output Reuse**: Frequency-based filtering

### Quantum Enhancement
- **QAOA MaxCut**: Partitions ring into real/decoy groups
- **Quantum Walk**: Centrality-based ranking
- **Hybrid Combination**: Weighted classical-quantum fusion

### Analysis Pipeline

```
Ring Signature
      │
      ▼
┌─────────────────────────────────────┐
│     Classical Heuristics            │
│  (Temporal, Decoy, Reuse analysis)  │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│      Quantum QAOA Analysis          │
│  (MaxCut partitioning of ring)      │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│     Hybrid Combination              │
│  (Weighted probability fusion)      │
└───────────────┬─────────────────────┘
                │
                ▼
        Predicted Real Input
```

## Performance

### Test Coverage
- phosphoros-quantum: 20+ tests
- phosphoros-core: 46 tests
- phosphoros-monero: 15 tests
- cryptogenetik-core: 13 tests
- Total: 150+ tests passing

### Benchmarks
```
5D Operations:       ~14M ops/sec
Metatron Embed:      ~1.2M ops/sec
Resonance Eval:      ~640K ops/sec
Quantum Gate (1Q):   ~500K ops/sec
QAOA Layer:          ~10K ops/sec
Grover Iteration:    ~20K ops/sec
```

## Security Notice

**IMPORTANT**: This system is for research and forensic analysis only.

- Do NOT use for generating production private keys
- Do NOT use for managing real cryptocurrency wallets
- Always use official, audited wallet software for real funds
- Quantum simulation runs on classical hardware

## Production Quality

- No `unsafe` code
- Comprehensive error handling with `thiserror`
- Full determinism (seeded RNG)
- Thread-safe async architecture
- Documentation for all public APIs
- `clippy` clean with `-D warnings`

## License

Dual-licensed under MIT or Apache 2.0.

---

**Version**: 2.0.0 (Quantum Edition)
**Status**: Production-ready with quantum integration
**Author**: PHOSPHOROS Project Team

*The first quantum-enhanced blockchain forensics system*
