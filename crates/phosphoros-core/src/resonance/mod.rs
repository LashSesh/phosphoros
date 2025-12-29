//! Resonance engine and spectral signatures
//!
//! This module provides the core resonance evaluation framework for PHOSPHOROS.
//!
//! ## Module Structure
//!
//! - [`spectral`]: Spectral signatures σ = (ψ, ρ, ω) with invariant resonance formula
//! - [`traits`]: Core [`ResonanceEngine`] trait and [`Evaluation`] result type
//! - [`holistic`]: Complete multi-layer resonance engine implementation
//!
//! ## Core Invariant
//!
//! The resonance formula `D = ψ·ρ·ω` is **INVARIANT** across all PHOSPHOROS systems.
//! This formula combines:
//! - **ψ (psi)**: Coherence/Semantics
//! - **ρ (rho)**: Density/Structure
//! - **ω (omega)**: Frequency/Phase
//!
//! ## Evaluation Flow
//!
//! ```text
//! Input State → ResonanceEngine::evaluate() → Evaluation
//!                      │                          │
//!                      ▼                          ▼
//!              ┌──────────────┐          ┌──────────────────┐
//!              │ Multi-stage  │          │ Output { vector, │
//!              │ gating &     │    OR    │   score }        │
//!              │ dynamics     │          │                  │
//!              └──────────────┘          │ Gated { reason } │
//!                                        └──────────────────┘
//! ```
//!
//! ## Example: Using the ResonanceEngine Trait
//!
//! ```rust
//! use phosphoros_core::{HolisticMatrix, ResonanceEngine, Evaluation};
//!
//! // Create an engine
//! let mut engine = HolisticMatrix::default_config();
//!
//! // Evaluate
//! let result = engine.evaluate(
//!     1.0,                              // time
//!     [0.5; 5],                         // perception
//!     [0.6; 5],                         // intention
//!     [1.0, 0.0, 0.0, 0.0, 0.0],       // gradient
//!     0.5                               // theta
//! );
//!
//! // Handle result
//! if let Evaluation::Output { vector, score } = result {
//!     println!("Action: {:?}, Score: {}", vector, score);
//! }
//!
//! // Reset for new session
//! engine.reset();
//! ```
//!
//! ## Gate Reasons
//!
//! When evaluation fails, a [`GateReason`] explains why:
//!
//! | Reason | Meaning |
//! |--------|---------|
//! | `LowCoherence` | Phase synchronization below κ⋆ threshold |
//! | `HighFluctuation` | Instability detected (|dκ/dt| > ε) |
//! | `MonolithFailed` | Geometric criterion ∇·C⃗ < Θ |
//! | `InsufficientResonance` | D_total below dynamic threshold |

pub mod holistic;
pub mod spectral;
pub mod traits;

pub use holistic::{HolisticMatrix, MatrixState};
pub use spectral::SpectralSignature;
pub use traits::{Evaluation, GateReason, ResonanceEngine};
