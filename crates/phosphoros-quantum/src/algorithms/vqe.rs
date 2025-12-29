//! Variational Quantum Eigensolver (VQE)
//!
//! Used to find ground states of Hamiltonians, applicable to:
//! - Resonance field optimization
//! - Energy landscape analysis

use crate::backend::{QuantumBackend, QuantumCircuit, BackendError};
use crate::backend::simulator::LocalSimulator;
use std::f64::consts::PI;

/// Variational Quantum Eigensolver
pub struct VQE<B: QuantumBackend> {
    /// Quantum backend
    backend: B,

    /// Number of qubits
    num_qubits: usize,

    /// Ansatz depth
    depth: usize,

    /// Maximum optimization iterations
    max_iterations: usize,

    /// Convergence tolerance
    tolerance: f64,
}

/// Result of VQE optimization
#[derive(Debug, Clone)]
pub struct VQEResult {
    /// Ground state energy estimate
    pub energy: f64,

    /// Optimized variational parameters
    pub parameters: Vec<f64>,

    /// Convergence achieved
    pub converged: bool,

    /// Number of iterations
    pub iterations: usize,

    /// Energy history
    pub history: Vec<f64>,
}

/// Hamiltonian representation for VQE
#[derive(Debug, Clone)]
pub struct Hamiltonian {
    /// Pauli terms: (coefficient, pauli_string)
    /// Pauli string: "XYZII" means X⊗Y⊗Z⊗I⊗I
    pub terms: Vec<(f64, String)>,

    /// Number of qubits
    pub num_qubits: usize,
}

impl<B: QuantumBackend> VQE<B> {
    /// Create a new VQE instance
    pub fn new(backend: B, num_qubits: usize, depth: usize) -> Self {
        Self {
            backend,
            num_qubits,
            depth,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Find the ground state energy
    pub fn find_ground_state(&self, hamiltonian: &Hamiltonian, shots: usize) -> Result<VQEResult, BackendError> {
        let num_params = 3 * self.num_qubits * self.depth;
        let mut params: Vec<f64> = vec![0.1; num_params];

        let mut history = Vec::new();
        let mut best_energy = f64::MAX;
        let mut best_params = params.clone();

        for iter in 0..self.max_iterations {
            // Evaluate energy
            let energy = self.evaluate_energy(hamiltonian, &params, shots)?;
            history.push(energy);

            if energy < best_energy {
                best_energy = energy;
                best_params = params.clone();
            }

            // Check convergence
            if iter > 0 && (history[iter - 1] - energy).abs() < self.tolerance {
                return Ok(VQEResult {
                    energy: best_energy,
                    parameters: best_params,
                    converged: true,
                    iterations: iter + 1,
                    history,
                });
            }

            // Gradient-free optimization (COBYLA-like)
            params = self.optimize_step(&params, hamiltonian, shots)?;
        }

        Ok(VQEResult {
            energy: best_energy,
            parameters: best_params,
            converged: false,
            iterations: self.max_iterations,
            history,
        })
    }

    /// Evaluate energy expectation value
    fn evaluate_energy(
        &self,
        hamiltonian: &Hamiltonian,
        params: &[f64],
        shots: usize,
    ) -> Result<f64, BackendError> {
        let mut total_energy = 0.0;

        for (coeff, pauli_string) in &hamiltonian.terms {
            let expectation = self.measure_pauli_expectation(pauli_string, params, shots)?;
            total_energy += coeff * expectation;
        }

        Ok(total_energy)
    }

    /// Measure expectation value of a Pauli string
    fn measure_pauli_expectation(
        &self,
        pauli_string: &str,
        params: &[f64],
        shots: usize,
    ) -> Result<f64, BackendError> {
        let mut circuit = self.build_ansatz_circuit(params)?;

        // Add basis rotation for measurement
        for (q, pauli) in pauli_string.chars().enumerate() {
            match pauli {
                'X' => { circuit.h(q); }
                'Y' => {
                    circuit.rz(q, -PI / 2.0);
                    circuit.h(q);
                }
                'Z' | 'I' => {} // No rotation needed for Z or I
                _ => {}
            }
        }

        circuit.measure_all();

        let result = self.backend.run_circuit(&circuit, shots)?;

        // Calculate expectation from measurements
        let mut expectation = 0.0;
        for (bitstring, &count) in &result.counts {
            // Calculate parity for non-identity Paulis
            let mut parity = 0;
            for (q, pauli) in pauli_string.chars().enumerate() {
                if pauli != 'I' && q < bitstring.len() {
                    let bit = bitstring.chars().nth(q).map(|c| c == '1').unwrap_or(false);
                    if bit {
                        parity ^= 1;
                    }
                }
            }

            let eigenvalue = if parity == 0 { 1.0 } else { -1.0 };
            expectation += eigenvalue * count as f64 / shots as f64;
        }

        Ok(expectation)
    }

    /// Build the variational ansatz circuit
    fn build_ansatz_circuit(&self, params: &[f64]) -> Result<QuantumCircuit, BackendError> {
        let mut circuit = QuantumCircuit::new(self.num_qubits);
        let mut param_idx = 0;

        for _layer in 0..self.depth {
            // Single-qubit rotations
            for q in 0..self.num_qubits {
                if param_idx + 2 < params.len() {
                    circuit.rx(q, params[param_idx]);
                    circuit.ry(q, params[param_idx + 1]);
                    circuit.rz(q, params[param_idx + 2]);
                    param_idx += 3;
                }
            }

            // Entangling layer
            for q in 0..(self.num_qubits - 1) {
                circuit.cnot(q, q + 1);
            }
            // Close the loop
            if self.num_qubits > 2 {
                circuit.cnot(self.num_qubits - 1, 0);
            }
        }

        Ok(circuit)
    }

    /// Simple optimization step (coordinate descent)
    fn optimize_step(
        &self,
        params: &[f64],
        hamiltonian: &Hamiltonian,
        shots: usize,
    ) -> Result<Vec<f64>, BackendError> {
        let mut new_params = params.to_vec();
        let step_size = 0.1;

        // Random coordinate descent
        let coord = rand::random::<usize>() % params.len();

        let mut params_plus = new_params.clone();
        params_plus[coord] += step_size;
        let energy_plus = self.evaluate_energy(hamiltonian, &params_plus, shots / 2)?;

        let mut params_minus = new_params.clone();
        params_minus[coord] -= step_size;
        let energy_minus = self.evaluate_energy(hamiltonian, &params_minus, shots / 2)?;

        let current_energy = self.evaluate_energy(hamiltonian, &new_params, shots / 2)?;

        if energy_plus < current_energy && energy_plus < energy_minus {
            new_params[coord] += step_size;
        } else if energy_minus < current_energy {
            new_params[coord] -= step_size;
        }

        Ok(new_params)
    }
}

impl Hamiltonian {
    /// Create a new empty Hamiltonian
    pub fn new(num_qubits: usize) -> Self {
        Self {
            terms: Vec::new(),
            num_qubits,
        }
    }

    /// Add a Pauli term
    pub fn add_term(&mut self, coefficient: f64, pauli_string: &str) -> &mut Self {
        self.terms.push((coefficient, pauli_string.to_string()));
        self
    }

    /// Create a simple Ising model Hamiltonian
    pub fn ising(num_qubits: usize, j_coupling: f64, h_field: f64) -> Self {
        let mut h = Self::new(num_qubits);

        // ZZ interactions
        for i in 0..num_qubits {
            let j = (i + 1) % num_qubits;
            let mut pauli = vec!['I'; num_qubits];
            pauli[i] = 'Z';
            pauli[j] = 'Z';
            h.add_term(-j_coupling, &pauli.iter().collect::<String>());
        }

        // X field
        for i in 0..num_qubits {
            let mut pauli = vec!['I'; num_qubits];
            pauli[i] = 'X';
            h.add_term(-h_field, &pauli.iter().collect::<String>());
        }

        h
    }

    /// Create a Hamiltonian for Metatron graph Laplacian
    pub fn metatron_laplacian() -> Self {
        let mut h = Self::new(13);

        // Graph Laplacian: L = D - A
        // For quantum: H = sum_edges (I - Z_i Z_j)
        // Metatron edges (simplified)
        let edges = vec![
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), // center to hexagon
            (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 1), // hexagon ring
            (1, 7), (2, 8), (3, 9), (4, 10), (5, 11), (6, 12), // hex to cube
        ];

        for (i, j) in edges {
            let mut pauli = vec!['I'; 13];
            pauli[i] = 'Z';
            pauli[j] = 'Z';
            h.add_term(0.5, &pauli.iter().collect::<String>());
        }

        h
    }
}

impl VQE<LocalSimulator> {
    /// Create with default local simulator
    pub fn default_local(num_qubits: usize, depth: usize) -> Self {
        let backend = LocalSimulator::new(num_qubits);
        Self::new(backend, num_qubits, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vqe_creation() {
        let vqe = VQE::default_local(4, 2);
        assert_eq!(vqe.num_qubits, 4);
        assert_eq!(vqe.depth, 2);
    }

    #[test]
    fn test_hamiltonian() {
        let mut h = Hamiltonian::new(2);
        h.add_term(1.0, "ZZ");
        h.add_term(0.5, "XI");

        assert_eq!(h.terms.len(), 2);
    }

    #[test]
    fn test_ising_hamiltonian() {
        let h = Hamiltonian::ising(4, 1.0, 0.5);
        assert!(h.terms.len() > 0);
    }

    #[test]
    fn test_simple_vqe() {
        let vqe = VQE::default_local(2, 1)
            .with_max_iterations(10);

        let h = Hamiltonian::ising(2, 1.0, 0.5);
        let result = vqe.find_ground_state(&h, 100).unwrap();

        // Energy should be finite
        assert!(result.energy.is_finite());
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_metatron_hamiltonian() {
        let h = Hamiltonian::metatron_laplacian();
        assert_eq!(h.num_qubits, 13);
        assert!(h.terms.len() > 0);
    }
}
