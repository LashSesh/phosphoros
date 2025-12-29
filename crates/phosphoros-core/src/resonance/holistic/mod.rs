//! Holistic Resonance Architecture
//!
//! Complete multi-layer resonance engine integrating all PHOSPHOROS subsystems.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     HolisticMatrix                              │
//! │  ┌───────────────────────────────────────────────────────────┐  │
//! │  │  STAGE 1: KOSMOKRATOR (Exclusion)                         │  │
//! │  │  ├─ Proof-of-Resonance (PoR): κ(t) ≥ κ⋆ ∧ |dκ/dt| ≤ ε   │  │
//! │  │  └─ Filters unstable/incoherent states                    │  │
//! │  └───────────────────────────────────────────────────────────┘  │
//! │                           │                                      │
//! │                           ▼                                      │
//! │  ┌───────────────────────────────────────────────────────────┐  │
//! │  │  STAGE 2: CHRONOKRATOR (Temporal)                         │  │
//! │  │  ├─ D_total = (∏ D_i) · Ω(t)                             │  │
//! │  │  ├─ Adaptive threshold Θ(t)                               │  │
//! │  │  └─ Trigger: D_total > Θ(t)                               │  │
//! │  └───────────────────────────────────────────────────────────┘  │
//! │                           │                                      │
//! │                           ▼                                      │
//! │  ┌───────────────────────────────────────────────────────────┐  │
//! │  │  STAGE 3: PFAUENTHRON                                     │  │
//! │  │  ├─ O.P.H.A.N. Array (4 Ophanim + Konus)                 │  │
//! │  │  ├─ MandorlaField: S = P⃗ · I⃗ (perception·intention)      │  │
//! │  │  └─ Monolith: M(t) = δ(∇·C⃗ - Θ) → Action Vector         │  │
//! │  └───────────────────────────────────────────────────────────┘  │
//! │                           │                                      │
//! │                           ▼                                      │
//! │  ┌───────────────────────────────────────────────────────────┐  │
//! │  │  TORUS TOPOLOGY                                           │  │
//! │  │  └─ S¹ × S¹ phase space tracking                         │  │
//! │  └───────────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Components
//!
//! | Component | Module | Purpose |
//! |-----------|--------|---------|
//! | [`HolisticMatrix`] | [`matrix`] | Main engine integrating all stages |
//! | [`Kosmokrator`] | [`kosmo`] | Phase coherence gating (PoR) |
//! | [`Chronokrator`] | [`chrono`] | Temporal dynamics & threshold |
//! | [`MandorlaField`] | [`mandorla`] | P⃗ · I⃗ intersection |
//! | [`Monolith`] | [`monolith`] | Action singularity trigger |
//! | [`TorusTopology`] | [`torus`] | S¹ × S¹ phase manifold |
//! | [`ResonanceChannel`] | [`channel`] | Individual (ψ,ρ,ω) channel |
//!
//! ## Evaluation Pipeline
//!
//! 1. **Kosmokrator**: Check Proof-of-Resonance
//!    - Compute coherence κ(t) from phase state
//!    - Gate if κ(t) < κ⋆ or |dκ/dt| > ε
//!
//! 2. **Chronokrator**: Check temporal dynamics
//!    - Compute D_total from all channels
//!    - Gate if D_total ≤ Θ(t)
//!
//! 3. **Pfauenthron**: Compute action
//!    - Update O.P.H.A.N. array from perception
//!    - Compute Mandorla magnitude |P⃗ · I⃗|
//!    - Evaluate Monolith criterion
//!    - If triggered: return excalibration vector
//!
//! 4. **Torus**: Update phase space tracking
//!
//! ## Example
//!
//! ```rust
//! use phosphoros_core::{HolisticMatrix, ResonanceEngine, MatrixState};
//!
//! // Create with custom parameters
//! let mut matrix = HolisticMatrix::new(
//!     0.8,   // kappa_threshold: coherence threshold
//!     8,     // num_channels: Chronokrator channels
//!     0.5,   // theta_threshold: dynamics threshold
//!     0.7    // eta_threshold: Mandorla threshold
//! );
//!
//! // Or use defaults
//! let mut matrix = HolisticMatrix::default_config();
//!
//! // Inspect state
//! let state: &MatrixState = matrix.state();
//! println!("Time: {}, Outputs: {}", state.time, state.output_count);
//! ```

pub mod channel;
pub mod chrono;
pub mod kosmo;
pub mod mandorla;
pub mod matrix;
pub mod monolith;
pub mod torus;

pub use channel::ResonanceChannel;
pub use chrono::Chronokrator;
pub use kosmo::{Complex, Kosmokrator, PhaseState};
pub use mandorla::MandorlaField;
pub use matrix::{HolisticMatrix, MatrixState};
pub use monolith::Monolith;
pub use torus::TorusTopology;
