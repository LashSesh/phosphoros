//! Quantum-Enhanced Resonance Engines
//!
//! Provides quantum-accelerated implementations of the ResonanceEngine trait,
//! enabling hybrid classical-quantum resonance evaluation.

mod quantum_resonance;
mod hybrid_matrix;

pub use quantum_resonance::QuantumResonanceEngine;
pub use hybrid_matrix::HybridHolisticMatrix;

/// Configuration for quantum resonance engines
#[derive(Debug, Clone)]
pub struct QuantumEngineConfig {
    /// Number of shots for quantum measurements
    pub shots: usize,

    /// Threshold for switching to quantum evaluation
    pub quantum_threshold: f64,

    /// Maximum circuit depth
    pub max_depth: usize,

    /// Enable error mitigation
    pub error_mitigation: bool,

    /// Ansatz type for variational circuits
    pub ansatz_type: AnsatzType,
}

impl Default for QuantumEngineConfig {
    fn default() -> Self {
        Self {
            shots: 1000,
            quantum_threshold: 0.5,
            max_depth: 10,
            error_mitigation: false,
            ansatz_type: AnsatzType::HardwareEfficient,
        }
    }
}

/// Types of variational ansatzes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsatzType {
    /// Hardware-efficient ansatz with single-qubit rotations and CNOTs
    HardwareEfficient,

    /// Efficient SU(2) parametrization
    EfficientSU2,

    /// Metatron-geometry optimized ansatz
    MetatronOptimized,

    /// Custom user-defined ansatz
    Custom,
}
