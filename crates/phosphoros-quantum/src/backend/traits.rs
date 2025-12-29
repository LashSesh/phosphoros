//! Core traits for quantum backend abstraction

use super::{BackendError, QuantumFeature};
use crate::Complex;
use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;

/// Core trait for quantum computing backends
pub trait QuantumBackend: Send + Sync {
    /// Get the backend name
    fn name(&self) -> &str;

    /// Get the number of available qubits
    fn qubit_count(&self) -> usize;

    /// Check if a feature is supported
    fn supports_feature(&self, feature: QuantumFeature) -> bool;

    /// Execute a quantum circuit
    fn run_circuit(&self, circuit: &QuantumCircuit, shots: usize) -> Result<QuantumResult, BackendError>;

    /// Get the current quantum state (for simulators)
    fn get_state(&self) -> Option<QuantumState> {
        None
    }

    /// Reset the backend to initial state
    fn reset(&mut self);
}

/// Represents a quantum circuit as a sequence of gates
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<QuantumGate>,
    pub measurements: Vec<usize>,
    pub parameters: HashMap<String, f64>,
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
            measurements: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    /// Add a gate to the circuit
    pub fn add_gate(&mut self, gate: QuantumGate) -> &mut Self {
        self.gates.push(gate);
        self
    }

    /// Add Hadamard gate
    pub fn h(&mut self, qubit: usize) -> &mut Self {
        self.add_gate(QuantumGate::H(qubit))
    }

    /// Add Pauli-X gate
    pub fn x(&mut self, qubit: usize) -> &mut Self {
        self.add_gate(QuantumGate::X(qubit))
    }

    /// Add Pauli-Y gate
    pub fn y(&mut self, qubit: usize) -> &mut Self {
        self.add_gate(QuantumGate::Y(qubit))
    }

    /// Add Pauli-Z gate
    pub fn z(&mut self, qubit: usize) -> &mut Self {
        self.add_gate(QuantumGate::Z(qubit))
    }

    /// Add rotation around X axis
    pub fn rx(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.add_gate(QuantumGate::Rx(qubit, theta))
    }

    /// Add rotation around Y axis
    pub fn ry(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.add_gate(QuantumGate::Ry(qubit, theta))
    }

    /// Add rotation around Z axis
    pub fn rz(&mut self, qubit: usize, theta: f64) -> &mut Self {
        self.add_gate(QuantumGate::Rz(qubit, theta))
    }

    /// Add CNOT gate
    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.add_gate(QuantumGate::CNOT(control, target))
    }

    /// Add CZ gate
    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        self.add_gate(QuantumGate::CZ(control, target))
    }

    /// Add measurement
    pub fn measure(&mut self, qubit: usize) -> &mut Self {
        self.measurements.push(qubit);
        self
    }

    /// Add measurements for all qubits
    pub fn measure_all(&mut self) -> &mut Self {
        for q in 0..self.num_qubits {
            self.measurements.push(q);
        }
        self
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, name: &str, value: f64) -> &mut Self {
        self.parameters.insert(name.to_string(), value);
        self
    }

    /// Get circuit depth (number of gate layers)
    pub fn depth(&self) -> usize {
        // Simplified depth calculation
        self.gates.len()
    }
}

/// Quantum gates supported by the framework
#[derive(Debug, Clone)]
pub enum QuantumGate {
    // Single-qubit gates
    H(usize),           // Hadamard
    X(usize),           // Pauli-X
    Y(usize),           // Pauli-Y
    Z(usize),           // Pauli-Z
    S(usize),           // Phase gate (√Z)
    T(usize),           // T gate (√S)
    Rx(usize, f64),     // Rotation around X
    Ry(usize, f64),     // Rotation around Y
    Rz(usize, f64),     // Rotation around Z

    // Two-qubit gates
    CNOT(usize, usize), // Controlled-NOT
    CZ(usize, usize),   // Controlled-Z
    SWAP(usize, usize), // SWAP gate

    // Multi-qubit gates
    Toffoli(usize, usize, usize), // CCX

    // Parameterized gates (for variational algorithms)
    RXX(usize, usize, f64), // XX rotation
    RYY(usize, usize, f64), // YY rotation
    RZZ(usize, usize, f64), // ZZ rotation

    // Custom unitary
    Custom {
        qubits: Vec<usize>,
        matrix: DMatrix<Complex>,
        name: String,
    },

    // Oracle for Grover's algorithm
    Oracle {
        qubits: Vec<usize>,
        marked_states: Vec<usize>,
    },
}

impl QuantumGate {
    /// Get the matrix representation of this gate
    pub fn to_matrix(&self) -> DMatrix<Complex> {
        use std::f64::consts::{FRAC_1_SQRT_2, PI};
        let sqrt2_inv = Complex::new(FRAC_1_SQRT_2, 0.0);
        let zero = Complex::new(0.0, 0.0);
        let one = Complex::new(1.0, 0.0);
        let neg_one = Complex::new(-1.0, 0.0);
        let i = Complex::new(0.0, 1.0);
        let neg_i = Complex::new(0.0, -1.0);

        match self {
            QuantumGate::H(_) => DMatrix::from_row_slice(2, 2, &[
                sqrt2_inv, sqrt2_inv,
                sqrt2_inv, -sqrt2_inv,
            ]),

            QuantumGate::X(_) => DMatrix::from_row_slice(2, 2, &[
                zero, one,
                one, zero,
            ]),

            QuantumGate::Y(_) => DMatrix::from_row_slice(2, 2, &[
                zero, neg_i,
                i, zero,
            ]),

            QuantumGate::Z(_) => DMatrix::from_row_slice(2, 2, &[
                one, zero,
                zero, neg_one,
            ]),

            QuantumGate::S(_) => DMatrix::from_row_slice(2, 2, &[
                one, zero,
                zero, i,
            ]),

            QuantumGate::T(_) => {
                let t_phase = Complex::from_polar(1.0, PI / 4.0);
                DMatrix::from_row_slice(2, 2, &[
                    one, zero,
                    zero, t_phase,
                ])
            }

            QuantumGate::Rx(_, theta) => {
                let c = Complex::new((theta / 2.0).cos(), 0.0);
                let s = Complex::new(0.0, -(theta / 2.0).sin());
                DMatrix::from_row_slice(2, 2, &[c, s, s, c])
            }

            QuantumGate::Ry(_, theta) => {
                let c = Complex::new((theta / 2.0).cos(), 0.0);
                let s = Complex::new((theta / 2.0).sin(), 0.0);
                DMatrix::from_row_slice(2, 2, &[c, -s, s, c])
            }

            QuantumGate::Rz(_, theta) => {
                let exp_neg = Complex::from_polar(1.0, -theta / 2.0);
                let exp_pos = Complex::from_polar(1.0, theta / 2.0);
                DMatrix::from_row_slice(2, 2, &[exp_neg, zero, zero, exp_pos])
            }

            QuantumGate::CNOT(_, _) => DMatrix::from_row_slice(4, 4, &[
                one, zero, zero, zero,
                zero, one, zero, zero,
                zero, zero, zero, one,
                zero, zero, one, zero,
            ]),

            QuantumGate::CZ(_, _) => DMatrix::from_row_slice(4, 4, &[
                one, zero, zero, zero,
                zero, one, zero, zero,
                zero, zero, one, zero,
                zero, zero, zero, neg_one,
            ]),

            QuantumGate::SWAP(_, _) => DMatrix::from_row_slice(4, 4, &[
                one, zero, zero, zero,
                zero, zero, one, zero,
                zero, one, zero, zero,
                zero, zero, zero, one,
            ]),

            QuantumGate::Custom { matrix, .. } => matrix.clone(),

            _ => {
                // Default identity for unimplemented gates
                DMatrix::identity(2, 2)
            }
        }
    }

    /// Get the qubits this gate acts on
    pub fn qubits(&self) -> Vec<usize> {
        match self {
            QuantumGate::H(q) | QuantumGate::X(q) | QuantumGate::Y(q) |
            QuantumGate::Z(q) | QuantumGate::S(q) | QuantumGate::T(q) |
            QuantumGate::Rx(q, _) | QuantumGate::Ry(q, _) | QuantumGate::Rz(q, _) => vec![*q],

            QuantumGate::CNOT(c, t) | QuantumGate::CZ(c, t) |
            QuantumGate::SWAP(c, t) | QuantumGate::RXX(c, t, _) |
            QuantumGate::RYY(c, t, _) | QuantumGate::RZZ(c, t, _) => vec![*c, *t],

            QuantumGate::Toffoli(a, b, c) => vec![*a, *b, *c],

            QuantumGate::Custom { qubits, .. } | QuantumGate::Oracle { qubits, .. } => qubits.clone(),
        }
    }
}

/// Quantum state as a vector of complex amplitudes
#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitudes: DVector<Complex>,
    pub num_qubits: usize,
}

impl QuantumState {
    /// Create a new quantum state initialized to |0...0⟩
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut amplitudes = DVector::zeros(dim);
        amplitudes[0] = Complex::new(1.0, 0.0);

        Self { amplitudes, num_qubits }
    }

    /// Create a state from amplitudes
    pub fn from_amplitudes(amplitudes: DVector<Complex>) -> Self {
        let dim = amplitudes.len();
        let num_qubits = (dim as f64).log2() as usize;
        Self { amplitudes, num_qubits }
    }

    /// Get probability of measuring a specific basis state
    pub fn probability(&self, state_index: usize) -> f64 {
        self.amplitudes[state_index].norm_sqr()
    }

    /// Get all probabilities
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a.norm_sqr()).collect()
    }

    /// Normalize the state
    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt();
        if norm > 1e-10 {
            self.amplitudes /= Complex::new(norm, 0.0);
        }
    }

    /// Apply a phase flip to marked states (for Grover's oracle)
    pub fn phase_flip(&mut self, marked_states: &[usize]) {
        for &state in marked_states {
            if state < self.amplitudes.len() {
                self.amplitudes[state] *= Complex::new(-1.0, 0.0);
            }
        }
    }

    /// Calculate inner product with another state
    pub fn inner_product(&self, other: &QuantumState) -> Complex {
        self.amplitudes.iter()
            .zip(other.amplitudes.iter())
            .map(|(a, b)| a.conj() * b)
            .sum()
    }

    /// Calculate fidelity with another state
    pub fn fidelity(&self, other: &QuantumState) -> f64 {
        self.inner_product(other).norm_sqr()
    }
}

/// Result of executing a quantum circuit
#[derive(Debug, Clone)]
pub struct QuantumResult {
    /// Measurement counts for each basis state
    pub counts: HashMap<String, usize>,

    /// Total number of shots
    pub shots: usize,

    /// Expectation values for observables
    pub expectation_values: HashMap<String, f64>,

    /// Final quantum state (if available from simulator)
    pub final_state: Option<QuantumState>,
}

impl QuantumResult {
    /// Create empty result
    pub fn new(shots: usize) -> Self {
        Self {
            counts: HashMap::new(),
            shots,
            expectation_values: HashMap::new(),
            final_state: None,
        }
    }

    /// Get probability of a specific measurement outcome
    pub fn probability(&self, outcome: &str) -> f64 {
        self.counts.get(outcome).copied().unwrap_or(0) as f64 / self.shots as f64
    }

    /// Get the most likely measurement outcome
    pub fn most_likely(&self) -> Option<(&String, &usize)> {
        self.counts.iter().max_by_key(|(_, &count)| count)
    }

    /// Get expectation value for an observable
    pub fn expectation(&self, observable: &str) -> Option<f64> {
        self.expectation_values.get(observable).copied()
    }

    /// Calculate the average value from counts (treating bitstrings as integers)
    pub fn average_value(&self) -> f64 {
        let mut sum = 0.0;
        for (bitstring, &count) in &self.counts {
            let value = usize::from_str_radix(bitstring, 2).unwrap_or(0);
            sum += value as f64 * count as f64;
        }
        sum / self.shots as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_creation() {
        let state = QuantumState::new(3);
        assert_eq!(state.num_qubits, 3);
        assert_eq!(state.amplitudes.len(), 8);
        assert!((state.probability(0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_circuit_builder() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.h(0).cnot(0, 1).measure_all();

        assert_eq!(circuit.num_qubits, 2);
        assert_eq!(circuit.gates.len(), 2);
        assert_eq!(circuit.measurements.len(), 2);
    }

    #[test]
    fn test_hadamard_matrix() {
        let h = QuantumGate::H(0);
        let matrix = h.to_matrix();

        // H² = I
        let h_squared = &matrix * &matrix;
        let identity = DMatrix::<Complex>::identity(2, 2);

        for i in 0..2 {
            for j in 0..2 {
                assert!((h_squared[(i, j)] - identity[(i, j)]).norm() < 1e-10);
            }
        }
    }
}
