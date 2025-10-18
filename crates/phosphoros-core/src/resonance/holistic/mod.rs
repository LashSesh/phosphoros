//! Holistic Resonance Architecture
//!
//! Complete integration of:
//! - Kosmokrator (exclusion via Proof-of-Resonance)
//! - Chronokrator (temporal expansion)
//! - O.P.H.A.N. Array (orbital projection)
//! - Mandorla Field (perception-intention intersection)
//! - Monolith (action singularity)
//! - Torus Topology (S¹ × S¹ phase space)

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
