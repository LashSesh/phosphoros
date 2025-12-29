//! Quantum Algorithms for PHOSPHOROS
//!
//! This module provides quantum algorithms optimized for forensic analysis:
//!
//! - **Grover Search**: Quadratic speedup for Triton search
//! - **QAOA**: Quantum approximate optimization for graph problems (ring signatures)
//! - **VQE**: Variational quantum eigensolver for ground state problems
//! - **Quantum Walk**: Continuous-time quantum walk for link analysis

pub mod grover;
pub mod qaoa;
pub mod vqe;
pub mod quantum_walk;

pub use grover::{GroverSearch, GroverOracle};
pub use qaoa::{QAOA, MaxCutProblem};
pub use vqe::VQE;
pub use quantum_walk::QuantumWalk;
