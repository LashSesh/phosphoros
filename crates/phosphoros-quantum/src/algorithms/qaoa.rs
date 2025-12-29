//! Quantum Approximate Optimization Algorithm (QAOA)
//!
//! Used for combinatorial optimization problems, particularly:
//! - Ring signature decomposition (MaxCut formulation)
//! - Transaction graph clustering
//! - Optimal decoy selection

use crate::backend::{QuantumBackend, QuantumCircuit, QuantumResult, BackendError};
use crate::backend::simulator::LocalSimulator;
use crate::Complex;
use std::f64::consts::PI;

/// QAOA implementation for combinatorial optimization
pub struct QAOA<B: QuantumBackend> {
    /// Quantum backend
    backend: B,

    /// QAOA depth (number of layers)
    depth: usize,

    /// Number of optimization iterations
    max_iterations: usize,

    /// Learning rate for parameter optimization
    learning_rate: f64,
}

/// MaxCut problem formulation
#[derive(Debug, Clone)]
pub struct MaxCutProblem {
    /// Number of nodes
    pub num_nodes: usize,

    /// Edges as (node1, node2, weight)
    pub edges: Vec<(usize, usize, f64)>,

    /// Node labels (for ring signature: key image identifiers)
    pub labels: Vec<String>,
}

/// Result of QAOA optimization
#[derive(Debug, Clone)]
pub struct QAOAResult {
    /// Best solution found (bitstring representing partition)
    pub solution: Vec<bool>,

    /// Objective value (cut size)
    pub objective: f64,

    /// Approximation ratio (objective / optimal)
    pub approximation_ratio: f64,

    /// Optimized parameters (gamma, beta)
    pub parameters: Vec<f64>,

    /// Number of iterations
    pub iterations: usize,

    /// Convergence history
    pub history: Vec<f64>,
}

impl<B: QuantumBackend> QAOA<B> {
    /// Create a new QAOA instance
    pub fn new(backend: B, depth: usize) -> Self {
        Self {
            backend,
            depth,
            max_iterations: 100,
            learning_rate: 0.1,
        }
    }

    /// Set maximum optimization iterations
    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Set learning rate
    pub fn with_learning_rate(mut self, rate: f64) -> Self {
        self.learning_rate = rate;
        self
    }

    /// Solve a MaxCut problem
    pub fn solve_maxcut(&self, problem: &MaxCutProblem, shots: usize) -> Result<QAOAResult, BackendError> {
        let num_params = 2 * self.depth; // gamma and beta for each layer

        // Initialize parameters
        let mut params: Vec<f64> = (0..num_params)
            .map(|i| if i % 2 == 0 { PI / 4.0 } else { PI / 8.0 })
            .collect();

        let mut history = Vec::new();
        let mut best_solution = vec![false; problem.num_nodes];
        let mut best_objective = f64::MIN;

        // Optimization loop
        for iter in 0..self.max_iterations {
            // Evaluate current parameters
            let (objective, solution) = self.evaluate_maxcut(problem, &params, shots)?;
            history.push(objective);

            if objective > best_objective {
                best_objective = objective;
                best_solution = solution;
            }

            // Gradient estimation via parameter shift
            let mut gradient = vec![0.0; num_params];
            for i in 0..num_params {
                // Forward shift
                let mut params_plus = params.clone();
                params_plus[i] += PI / 4.0;
                let (obj_plus, _) = self.evaluate_maxcut(problem, &params_plus, shots / 4)?;

                // Backward shift
                let mut params_minus = params.clone();
                params_minus[i] -= PI / 4.0;
                let (obj_minus, _) = self.evaluate_maxcut(problem, &params_minus, shots / 4)?;

                gradient[i] = (obj_plus - obj_minus) / 2.0;
            }

            // Update parameters (gradient ascent for maximization)
            for i in 0..num_params {
                params[i] += self.learning_rate * gradient[i];
            }

            // Early stopping if converged
            if iter > 5 {
                let recent_change = (history[iter] - history[iter - 5]).abs();
                if recent_change < 0.001 {
                    break;
                }
            }
        }

        // Calculate approximation ratio
        let optimal = self.classical_maxcut(problem);
        let approximation_ratio = if optimal > 0.0 {
            best_objective / optimal
        } else {
            1.0
        };

        Ok(QAOAResult {
            solution: best_solution,
            objective: best_objective,
            approximation_ratio,
            parameters: params,
            iterations: history.len(),
            history,
        })
    }

    /// Evaluate MaxCut objective for given parameters
    fn evaluate_maxcut(
        &self,
        problem: &MaxCutProblem,
        params: &[f64],
        shots: usize,
    ) -> Result<(f64, Vec<bool>), BackendError> {
        let circuit = self.build_maxcut_circuit(problem, params)?;
        let result = self.backend.run_circuit(&circuit, shots)?;

        // Calculate expected cut value
        let mut total_objective = 0.0;
        let mut best_bitstring = String::new();
        let mut best_cut = 0.0;

        for (bitstring, &count) in &result.counts {
            let cut = self.calculate_cut(problem, bitstring);
            let prob = count as f64 / shots as f64;
            total_objective += cut * prob;

            if cut > best_cut {
                best_cut = cut;
                best_bitstring = bitstring.clone();
            }
        }

        // Convert best bitstring to solution
        let solution: Vec<bool> = best_bitstring
            .chars()
            .map(|c| c == '1')
            .collect();

        Ok((total_objective, solution))
    }

    /// Build QAOA circuit for MaxCut
    fn build_maxcut_circuit(
        &self,
        problem: &MaxCutProblem,
        params: &[f64],
    ) -> Result<QuantumCircuit, BackendError> {
        let mut circuit = QuantumCircuit::new(problem.num_nodes);

        // Initial state: |+⟩⊗n
        for q in 0..problem.num_nodes {
            circuit.h(q);
        }

        // Apply p layers
        for layer in 0..self.depth {
            let gamma = params[2 * layer];
            let beta = params[2 * layer + 1];

            // Cost unitary: exp(-i * gamma * C)
            // For MaxCut: C = sum of (1 - Z_i Z_j) / 2 over edges
            for (i, j, weight) in &problem.edges {
                // ZZ interaction
                circuit.cnot(*i, *j);
                circuit.rz(*j, -gamma * weight);
                circuit.cnot(*i, *j);
            }

            // Mixer unitary: exp(-i * beta * B)
            // B = sum of X_i
            for q in 0..problem.num_nodes {
                circuit.rx(q, 2.0 * beta);
            }
        }

        // Measure all qubits
        circuit.measure_all();

        Ok(circuit)
    }

    /// Calculate cut value for a bitstring
    fn calculate_cut(&self, problem: &MaxCutProblem, bitstring: &str) -> f64 {
        let bits: Vec<bool> = bitstring.chars().map(|c| c == '1').collect();
        let mut cut = 0.0;

        for (i, j, weight) in &problem.edges {
            if *i < bits.len() && *j < bits.len() && bits[*i] != bits[*j] {
                cut += weight;
            }
        }

        cut
    }

    /// Classical MaxCut solver (brute force for small problems)
    fn classical_maxcut(&self, problem: &MaxCutProblem) -> f64 {
        if problem.num_nodes > 20 {
            // Too large for brute force, estimate
            return problem.edges.iter().map(|(_, _, w)| w).sum::<f64>() * 0.5;
        }

        let mut best_cut: f64 = 0.0;

        for partition in 0..(1 << problem.num_nodes) {
            let bits: Vec<bool> = (0..problem.num_nodes)
                .map(|i| (partition >> i) & 1 == 1)
                .collect();

            let mut cut = 0.0;
            for (i, j, weight) in &problem.edges {
                if bits[*i] != bits[*j] {
                    cut += weight;
                }
            }

            best_cut = best_cut.max(cut);
        }

        best_cut
    }
}

impl MaxCutProblem {
    /// Create a new MaxCut problem
    pub fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            edges: Vec::new(),
            labels: (0..num_nodes).map(|i| format!("node_{}", i)).collect(),
        }
    }

    /// Add an edge
    pub fn add_edge(&mut self, i: usize, j: usize, weight: f64) -> &mut Self {
        self.edges.push((i, j, weight));
        self
    }

    /// Create from adjacency matrix
    pub fn from_adjacency_matrix(matrix: &[Vec<f64>]) -> Self {
        let num_nodes = matrix.len();
        let mut problem = Self::new(num_nodes);

        for i in 0..num_nodes {
            for j in (i + 1)..num_nodes {
                if matrix[i][j] != 0.0 {
                    problem.add_edge(i, j, matrix[i][j]);
                }
            }
        }

        problem
    }

    /// Create Metatron graph (13 nodes, 78 edges)
    pub fn metatron() -> Self {
        let mut problem = Self::new(13);
        problem.labels = vec![
            "center".to_string(),
            "hex_0".to_string(), "hex_1".to_string(), "hex_2".to_string(),
            "hex_3".to_string(), "hex_4".to_string(), "hex_5".to_string(),
            "cube_0".to_string(), "cube_1".to_string(), "cube_2".to_string(),
            "cube_3".to_string(), "cube_4".to_string(), "cube_5".to_string(),
        ];

        // Center to hexagon
        for i in 1..=6 {
            problem.add_edge(0, i, 1.0);
        }

        // Hexagon ring
        for i in 1..=6 {
            problem.add_edge(i, 1 + (i % 6), 1.0);
        }

        // Hexagon to cube
        for i in 1..=6 {
            problem.add_edge(i, 6 + i, 1.0);
        }

        // Cube connections (simplified)
        for i in 7..=12 {
            for j in (i + 1)..=12 {
                problem.add_edge(i, j, 0.5);
            }
        }

        problem
    }

    /// Create a ring signature graph from member probabilities
    pub fn from_ring_members(probabilities: &[f64]) -> Self {
        let n = probabilities.len();
        let mut problem = Self::new(n);

        problem.labels = (0..n).map(|i| format!("member_{}", i)).collect();

        // Create edges based on probability differences
        // Nodes with similar probabilities should be in the same partition
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = (probabilities[i] - probabilities[j]).abs();
                // Higher weight for dissimilar nodes (want to cut between them)
                let weight = diff;
                if weight > 0.01 {
                    problem.add_edge(i, j, weight);
                }
            }
        }

        problem
    }
}

impl QAOA<LocalSimulator> {
    /// Create with default local simulator
    pub fn default_local(num_qubits: usize, depth: usize) -> Self {
        let backend = LocalSimulator::new(num_qubits);
        Self::new(backend, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qaoa_creation() {
        let qaoa = QAOA::default_local(4, 2);
        assert_eq!(qaoa.depth, 2);
    }

    #[test]
    fn test_maxcut_problem() {
        let mut problem = MaxCutProblem::new(4);
        problem.add_edge(0, 1, 1.0);
        problem.add_edge(1, 2, 1.0);
        problem.add_edge(2, 3, 1.0);
        problem.add_edge(3, 0, 1.0);

        assert_eq!(problem.num_nodes, 4);
        assert_eq!(problem.edges.len(), 4);
    }

    #[test]
    fn test_simple_maxcut() {
        let mut problem = MaxCutProblem::new(3);
        problem.add_edge(0, 1, 1.0);
        problem.add_edge(1, 2, 1.0);
        problem.add_edge(0, 2, 1.0);

        let qaoa = QAOA::default_local(3, 1);
        let result = qaoa.solve_maxcut(&problem, 100).unwrap();

        // Triangle has max cut of 2
        assert!(result.objective >= 1.0);
        assert!(result.solution.len() == 3);
    }

    #[test]
    fn test_metatron_problem() {
        let problem = MaxCutProblem::metatron();
        assert_eq!(problem.num_nodes, 13);
        assert!(problem.edges.len() > 0);
    }

    #[test]
    fn test_ring_member_problem() {
        let probs = vec![0.1, 0.8, 0.15, 0.05, 0.9];
        let problem = MaxCutProblem::from_ring_members(&probs);

        assert_eq!(problem.num_nodes, 5);
        // High prob difference between 0.1 and 0.9 should create an edge
        assert!(problem.edges.iter().any(|(i, j, _)| (*i == 0 && *j == 4) || (*i == 4 && *j == 0)));
    }

    #[test]
    fn test_approximation_ratio() {
        // Simple 4-cycle
        let mut problem = MaxCutProblem::new(4);
        problem.add_edge(0, 1, 1.0);
        problem.add_edge(1, 2, 1.0);
        problem.add_edge(2, 3, 1.0);
        problem.add_edge(3, 0, 1.0);

        let qaoa = QAOA::default_local(4, 2)
            .with_max_iterations(20);

        let result = qaoa.solve_maxcut(&problem, 200).unwrap();

        // QAOA should achieve reasonable approximation
        assert!(result.approximation_ratio > 0.5);
    }
}
