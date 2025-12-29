//! # PHOSPHOROS Core
//!
//! Core 5D geometry and resonance engine for the PHOSPHOROS system.
//!
//! This crate provides the foundational primitives for working with 5-dimensional
//! information spaces, spectral signatures, and holistic resonance evaluation.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                      PHOSPHOROS Core                            │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  geometry/           │  resonance/                              │
//! │  ├── Point5D         │  ├── SpectralSignature (ψ, ρ, ω)        │
//! │  └── MetatronGeometry│  ├── ResonanceEngine (trait)            │
//! │      (13-node sacred │  └── holistic/                          │
//! │       geometry)      │      ├── HolisticMatrix (main engine)   │
//! │                      │      ├── Kosmokrator (PoR exclusion)    │
//! │                      │      ├── Chronokrator (temporal)        │
//! │                      │      ├── MandorlaField (P⃗ · I⃗)          │
//! │                      │      ├── Monolith (action trigger)      │
//! │                      │      └── TorusTopology (S¹ × S¹)        │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Core Concepts
//!
//! ### Spectral Signature σ = (ψ, ρ, ω)
//!
//! The fundamental invariant of PHOSPHOROS. Every resonance calculation uses:
//! - **ψ (psi)**: Coherence/Semantics - phase synchronization measure
//! - **ρ (rho)**: Density/Structure - information density
//! - **ω (omega)**: Frequency/Phase - oscillation rate
//!
//! The resonance formula `D = ψ·ρ·ω` is **INVARIANT** and central to all evaluations.
//!
//! ### 5D Information Space
//!
//! Points in 5D space represent states in the information manifold. The fifth
//! dimension extends traditional 3D+time with a semantic axis. [`Point5D`]
//! provides vector operations (dot product, norm, distance) in this space.
//!
//! ### Metatron Sacred Geometry
//!
//! A 13-node topological structure that provides canonical embedding points.
//! Objects are embedded into 5D space via weighted interpolation of these nodes,
//! creating a deterministic mapping from hash values to 5D positions.
//!
//! ### Holistic Resonance Engine
//!
//! The [`HolisticMatrix`] integrates multiple subsystems:
//!
//! 1. **Kosmokrator**: Proof-of-Resonance (PoR) gating - filters unstable states
//! 2. **Chronokrator**: Temporal dynamics with adaptive thresholds
//! 3. **Mandorla Field**: Perception-intention intersection (P⃗ · I⃗)
//! 4. **Monolith**: Action singularity trigger based on gradient alignment
//! 5. **Torus Topology**: S¹ × S¹ phase space for state tracking
//!
//! ## Features
//!
//! - **5D Geometry**: [`Point5D`] primitives and [`MetatronGeometry`] sacred topology
//! - **Spectral Signatures**: [`SpectralSignature`] with (ψ, ρ, ω) dynamics
//! - **Holistic Resonance**: Complete [`HolisticMatrix`] engine
//! - **Resonance Engine Trait**: Pluggable [`ResonanceEngine`] interface
//!
//! ## Cargo Features
//!
//! - `default`: Core functionality with no external dependencies
//! - `advanced-linalg`: Enables `nalgebra` and `ndarray` for advanced linear algebra
//! - `parallel`: Enables `rayon` for parallel processing
//! - `quantum`: Enables integration with `phosphoros-quantum`
//!
//! ## Quick Start
//!
//! ```rust
//! use phosphoros_core::{HolisticMatrix, ResonanceEngine, Point5D};
//!
//! // Create a holistic resonance engine
//! let mut engine = HolisticMatrix::default_config();
//!
//! // Evaluate a state
//! let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
//! let intention = [0.6, 0.6, 0.6, 0.6, 0.6];
//! let gradient = [1.0, 0.0, 0.0, 0.0, 0.0];
//!
//! let result = engine.evaluate(1.0, perception, intention, gradient, 0.5);
//! ```
//!
//! ## Example: Working with Spectral Signatures
//!
//! ```rust
//! use phosphoros_core::SpectralSignature;
//!
//! // Create a spectral signature
//! let sig = SpectralSignature::new(0.8, 0.6, 0.5);
//!
//! // Calculate resonance (invariant formula D = ψ·ρ·ω)
//! let resonance = sig.resonance();
//! assert!((resonance - 0.24).abs() < 1e-10);
//!
//! // Calculate energy (L2 norm)
//! let energy = sig.energy();
//! ```
//!
//! ## Example: 5D Geometry Operations
//!
//! ```rust
//! use phosphoros_core::{Point5D, MetatronGeometry};
//!
//! // Create 5D points
//! let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
//! let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);
//!
//! // Vector operations
//! let dot = p1.dot(&p2);      // Dot product
//! let dist = p1.distance(&p2); // Euclidean distance
//! let norm = p1.norm();        // L2 norm
//!
//! // Embed objects via Metatron geometry
//! let metatron = MetatronGeometry::new();
//! let embedding = metatron.embed_object(0x1234_5678);
//! assert!((embedding.norm() - 1.0).abs() < 1e-10); // Normalized
//! ```
//!
//! ## Example: Handling Evaluation Results
//!
//! ```rust
//! use phosphoros_core::{HolisticMatrix, ResonanceEngine, Evaluation, GateReason};
//!
//! let mut engine = HolisticMatrix::default_config();
//! let result = engine.evaluate(
//!     1.0,
//!     [0.5; 5],  // perception
//!     [0.6; 5],  // intention
//!     [1.0, 0.0, 0.0, 0.0, 0.0],  // gradient
//!     0.5        // theta
//! );
//!
//! match result {
//!     Evaluation::Output { vector, score } => {
//!         println!("Action vector: {:?}, score: {}", vector, score);
//!     }
//!     Evaluation::Gated { reason } => {
//!         match reason {
//!             GateReason::LowCoherence => println!("Insufficient phase coherence"),
//!             GateReason::MonolithFailed => println!("Geometric criterion not met"),
//!             _ => println!("Gated: {:?}", reason),
//!         }
//!     }
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod geometry;
pub mod resonance;

// Re-export core types
pub use geometry::{MetatronGeometry, Point5D};
pub use resonance::{
    Evaluation, GateReason, HolisticMatrix, MatrixState, ResonanceEngine, SpectralSignature,
};
