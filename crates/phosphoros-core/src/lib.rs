//! # PHOSPHOROS Core
//!
//! Core 5D geometry and resonance engine for the PHOSPHOROS system.
//!
//! ## Features
//!
//! - **5D Geometry**: Point5D primitives and Metatron Sacred Geometry
//! - **Spectral Signatures**: (ψ, ρ, ω) dynamics with invariant resonance formula
//! - **Holistic Resonance**: Complete resonance engine with multi-layer architecture
//! - **Resonance Engine Trait**: Pluggable resonance engines
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

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod geometry;
pub mod resonance;

// Re-export core types
pub use geometry::{MetatronGeometry, Point5D};
pub use resonance::{
    Evaluation, GateReason, HolisticMatrix, MatrixState, ResonanceEngine, SpectralSignature,
};

