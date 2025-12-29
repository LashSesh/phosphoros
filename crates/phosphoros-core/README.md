# phosphoros-core

Core 5D geometry and resonance engine for the PHOSPHOROS blockchain forensics platform.

## Overview

`phosphoros-core` provides the foundational primitives for working with 5-dimensional information spaces, spectral signatures, and holistic resonance evaluation. This crate is the mathematical heart of PHOSPHOROS, implementing the core algorithms for entity analysis and anomaly detection.

## Features

- **5D Geometry**: `Point5D` primitives with full linear algebra operations
- **Metatron Sacred Geometry**: 13-node topological structure for 5D embeddings
- **Spectral Signatures**: (ψ, ρ, ω) triplets with invariant resonance formula
- **Holistic Resonance Engine**: Complete multi-stage evaluation pipeline

## Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                      PHOSPHOROS Core                            │
├─────────────────────────────────────────────────────────────────┤
│  geometry/           │  resonance/                              │
│  ├── Point5D         │  ├── SpectralSignature (ψ, ρ, ω)        │
│  └── MetatronGeometry│  ├── ResonanceEngine (trait)            │
│      (13-node sacred │  └── holistic/                          │
│       geometry)      │      ├── HolisticMatrix (main engine)   │
│                      │      ├── Kosmokrator (PoR exclusion)    │
│                      │      ├── Chronokrator (temporal)        │
│                      │      ├── MandorlaField (P⃗ · I⃗)          │
│                      │      ├── Monolith (action trigger)      │
│                      │      └── TorusTopology (S¹ × S¹)        │
└─────────────────────────────────────────────────────────────────┘
```

## Core Concepts

### Spectral Signature σ = (ψ, ρ, ω)

The fundamental invariant of PHOSPHOROS. Every resonance calculation uses:

| Component | Symbol | Meaning |
|-----------|--------|---------|
| Coherence | ψ (psi) | Phase synchronization measure |
| Density | ρ (rho) | Information/structural density |
| Frequency | ω (omega) | Oscillation rate / phase |

The resonance formula **D = ψ·ρ·ω** is **INVARIANT** and central to all evaluations.

### 5D Information Space

Points in 5D space represent states in the information manifold:

| Coordinate | Dimension |
|------------|-----------|
| x, y, z | Spatial dimensions |
| w | Temporal dimension |
| v | Semantic/information dimension |

### Holistic Resonance Pipeline

The `HolisticMatrix` integrates multiple processing stages:

1. **Kosmokrator**: Proof-of-Resonance (PoR) gating - filters unstable states
2. **Chronokrator**: Temporal dynamics with adaptive thresholds
3. **Pfauenthron**: O.P.H.A.N. array + MandorlaField + Monolith
4. **Torus**: S¹ × S¹ phase space tracking

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
phosphoros-core = "1.0"
```

### Quick Start

```rust
use phosphoros_core::{HolisticMatrix, ResonanceEngine, Point5D};

// Create a holistic resonance engine
let mut engine = HolisticMatrix::default_config();

// Evaluate a state
let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
let intention = [0.6, 0.6, 0.6, 0.6, 0.6];
let gradient = [1.0, 0.0, 0.0, 0.0, 0.0];

let result = engine.evaluate(1.0, perception, intention, gradient, 0.5);
```

### Working with Spectral Signatures

```rust
use phosphoros_core::SpectralSignature;

// Create a spectral signature
let sig = SpectralSignature::new(0.8, 0.6, 0.5);

// Calculate resonance (invariant formula D = ψ·ρ·ω)
let resonance = sig.resonance();
assert!((resonance - 0.24).abs() < 1e-10);

// Calculate energy (L2 norm)
let energy = sig.energy();
```

### 5D Geometry Operations

```rust
use phosphoros_core::{Point5D, MetatronGeometry};

// Create 5D points
let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);

// Vector operations
let dot = p1.dot(&p2);       // Dot product
let dist = p1.distance(&p2); // Euclidean distance
let norm = p1.norm();        // L2 norm

// Embed objects via Metatron geometry
let metatron = MetatronGeometry::new();
let embedding = metatron.embed_object(0x1234_5678);
assert!((embedding.norm() - 1.0).abs() < 1e-10); // Normalized
```

### Handling Evaluation Results

```rust
use phosphoros_core::{HolisticMatrix, ResonanceEngine, Evaluation, GateReason};

let mut engine = HolisticMatrix::default_config();
let result = engine.evaluate(
    1.0,
    [0.5; 5],  // perception
    [0.6; 5],  // intention
    [1.0, 0.0, 0.0, 0.0, 0.0],  // gradient
    0.5        // theta
);

match result {
    Evaluation::Output { vector, score } => {
        println!("Action vector: {:?}, score: {}", vector, score);
    }
    Evaluation::Gated { reason } => {
        match reason {
            GateReason::LowCoherence => println!("Insufficient phase coherence"),
            GateReason::MonolithFailed => println!("Geometric criterion not met"),
            GateReason::InsufficientResonance => println!("Below resonance threshold"),
            GateReason::HighFluctuation => println!("Instability detected"),
            _ => println!("Gated: {:?}", reason),
        }
    }
}
```

## Cargo Features

| Feature | Description |
|---------|-------------|
| `default` | Core functionality with no external dependencies |
| `advanced-linalg` | Enables `nalgebra` and `ndarray` for advanced linear algebra |
| `parallel` | Enables `rayon` for parallel processing |
| `quantum` | Enables integration with `phosphoros-quantum` |

## Mathematical Background

### Proof-of-Resonance (PoR)

A state passes PoR if:
1. Coherence exceeds threshold: `κ(t) ≥ κ⋆`
2. Coherence is stable: `|dκ/dt| ≤ ε`

Where coherence is computed as:
```
κ(t) = |1/N Σⱼ e^(iθⱼ)|
```

### Chronokrator Dynamics

Total dynamics are computed as:
```
D_total(t) = (∏ D_i(t)) · Ω(t)
```

With adaptive threshold:
```
Θ(t) = mean + 0.5·√variance
```

### Mandorla Field

The Mandorla represents the intersection of perception and intention:
```
S_Mandorla = P⃗ · I⃗
```

Where P⃗ is the Gabriel Funnel (perception) and I⃗ is the Oriphiel Field (intention).

### Monolith Criterion

Action is triggered when:
```
M(t) = δ(∇(ψ,ρ,ω) · C⃗_Konus - Θ) > 0
```

## Testing

```bash
cargo test -p phosphoros-core
```

Run with all features:

```bash
cargo test -p phosphoros-core --all-features
```

## License

MIT OR Apache-2.0
