//! # PHOSPHOROS Quantum Module
//!
//! Quantum computing integration for PHOSPHOROS forensic analysis.
//!
//! ## Features
//!
//! - **Quantum Backend**: Pluggable quantum computing backends (simulator, IBM, etc.)
//! - **QuantumResonanceEngine**: Quantum-enhanced resonance evaluation
//! - **Grover Search**: Quadratic speedup for Triton search
//! - **QAOA**: Quantum approximate optimization for graph problems
//! - **SCS Integration**: Seraphic Calibration System for auto-tuning
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │              phosphoros-quantum                  │
//! ├─────────────────────────────────────────────────┤
//! │  backend/     - Quantum backend abstraction     │
//! │  engines/     - QuantumResonanceEngine          │
//! │  algorithms/  - Grover, QAOA, VQE, Quantum Walk │
//! │  calibration/ - SCS bridge for auto-tuning     │
//! │  forensics/   - Ring signature QAOA analysis   │
//! └─────────────────────────────────────────────────┘
//! ```

pub mod backend;
pub mod engines;
pub mod algorithms;
pub mod calibration;
pub mod forensics;

// Re-exports for convenience
pub use backend::{QuantumBackend, QuantumCircuit, QuantumResult, QuantumGate};
pub use backend::simulator::LocalSimulator;
pub use engines::{QuantumResonanceEngine, HybridHolisticMatrix};
pub use algorithms::grover::GroverSearch;
pub use algorithms::qaoa::QAOA;
pub use calibration::SCSBridge;

/// Quantum state representation using complex amplitudes
pub type Complex = num_complex::Complex64;

/// Prelude for common imports
pub mod prelude {
    pub use super::backend::{QuantumBackend, QuantumCircuit, QuantumResult};
    pub use super::engines::{QuantumResonanceEngine, HybridHolisticMatrix};
    pub use super::algorithms::grover::GroverSearch;
    pub use super::algorithms::qaoa::QAOA;
    pub use super::Complex;
}
