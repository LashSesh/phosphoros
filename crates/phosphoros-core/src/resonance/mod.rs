//! Resonance engine and spectral signatures

pub mod holistic;
pub mod spectral;
pub mod traits;

pub use holistic::{HolisticMatrix, MatrixState};
pub use spectral::SpectralSignature;
pub use traits::{Evaluation, GateReason, ResonanceEngine};
