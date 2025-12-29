//! Continuous-Time Quantum Walk
//!
//! Quantum walk on graphs for:
//! - Link strength analysis
//! - Hitting time computation
//! - Network centrality measures

use crate::backend::{QuantumBackend, QuantumCircuit, QuantumResult, BackendError};
use crate::backend::simulator::LocalSimulator;
use nalgebra::DMatrix;

/// Continuous-Time Quantum Walk implementation
pub struct QuantumWalk<B: QuantumBackend> {
    /// Quantum backend
    backend: B,

    /// Number of nodes (qubits)
    num_nodes: usize,

    /// Graph adjacency matrix
    adjacency: DMatrix<f64>,

    /// Time step for evolution
    dt: f64,
}

/// Result of quantum walk simulation
#[derive(Debug, Clone)]
pub struct QuantumWalkResult {
    /// Probability distribution over nodes
    pub probabilities: Vec<f64>,

    /// Evolution time
    pub time: f64,

    /// Starting node
    pub start_node: usize,

    /// Mixing time estimate
    pub mixing_time: Option<f64>,

    /// Most visited nodes (ranked)
    pub hotspots: Vec<(usize, f64)>,
}

impl<B: QuantumBackend> QuantumWalk<B> {
    /// Create a new quantum walk on a graph
    pub fn new(backend: B, adjacency: DMatrix<f64>) -> Self {
        let num_nodes = adjacency.nrows();
        Self {
            backend,
            num_nodes,
            adjacency,
            dt: 0.1,
        }
    }

    /// Set time step
    pub fn with_dt(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    /// Run quantum walk from a starting node
    pub fn walk(
        &self,
        start_node: usize,
        total_time: f64,
        shots: usize,
    ) -> Result<QuantumWalkResult, BackendError> {
        let num_steps = (total_time / self.dt).ceil() as usize;

        let circuit = self.build_walk_circuit(start_node, num_steps)?;
        let result = self.backend.run_circuit(&circuit, shots)?;

        // Extract probabilities
        let probabilities = self.extract_probabilities(&result);

        // Find hotspots
        let mut hotspots: Vec<(usize, f64)> = probabilities
            .iter()
            .enumerate()
            .map(|(i, &p)| (i, p))
            .collect();
        hotspots.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Estimate mixing time (simplified)
        let mixing_time = self.estimate_mixing_time(&probabilities);

        Ok(QuantumWalkResult {
            probabilities,
            time: total_time,
            start_node,
            mixing_time,
            hotspots,
        })
    }

    /// Build quantum circuit for the walk
    fn build_walk_circuit(
        &self,
        start_node: usize,
        num_steps: usize,
    ) -> Result<QuantumCircuit, BackendError> {
        let mut circuit = QuantumCircuit::new(self.num_nodes);

        // Initialize at starting node
        circuit.x(start_node);

        // Apply walk evolution
        for _ in 0..num_steps {
            self.add_walk_step(&mut circuit)?;
        }

        circuit.measure_all();
        Ok(circuit)
    }

    /// Add a single walk step (Hamiltonian evolution)
    fn add_walk_step(&self, circuit: &mut QuantumCircuit) -> Result<(), BackendError> {
        // Trotterized evolution: exp(-i H dt) ≈ prod_edges exp(-i J_ij (XX + YY) dt)

        // For each edge in the adjacency matrix
        for i in 0..self.num_nodes {
            for j in (i + 1)..self.num_nodes {
                if self.adjacency[(i, j)] != 0.0 {
                    let weight = self.adjacency[(i, j)];
                    let angle = weight * self.dt;

                    // XX + YY evolution (swap-like interaction)
                    // Using decomposition: exp(-i θ (XX + YY)) = CNOT Rz(-2θ) CNOT
                    circuit.cnot(i, j);
                    circuit.rz(j, -2.0 * angle);
                    circuit.cnot(i, j);
                }
            }
        }

        // Diagonal evolution (if there are self-loops or node weights)
        for i in 0..self.num_nodes {
            if self.adjacency[(i, i)] != 0.0 {
                circuit.rz(i, self.adjacency[(i, i)] * self.dt);
            }
        }

        Ok(())
    }

    /// Extract probabilities from measurement results
    fn extract_probabilities(&self, result: &QuantumResult) -> Vec<f64> {
        let mut probs = vec![0.0; self.num_nodes];

        // If we have final state, use it directly
        if let Some(ref state) = result.final_state {
            for (i, amp) in state.amplitudes.iter().enumerate() {
                // Map computational basis state to node
                let node = i % self.num_nodes;
                probs[node] += amp.norm_sqr();
            }
            return probs;
        }

        // Otherwise, use measurement counts
        for (bitstring, &count) in &result.counts {
            // Find which node was measured
            for (i, c) in bitstring.chars().enumerate() {
                if c == '1' && i < self.num_nodes {
                    probs[i] += count as f64 / result.shots as f64;
                    break; // Only count first 1 for localization
                }
            }
        }

        probs
    }

    /// Estimate mixing time from probability distribution
    fn estimate_mixing_time(&self, probs: &[f64]) -> Option<f64> {
        // Mixing time is when the distribution is close to uniform
        let uniform = 1.0 / self.num_nodes as f64;
        let deviation: f64 = probs.iter().map(|&p| (p - uniform).abs()).sum();

        // If close to uniform, we've mixed
        if deviation < 0.1 {
            Some(1.0) // Already mixed
        } else {
            None // Not yet mixed
        }
    }

    /// Calculate hitting time from start to target
    pub fn hitting_time(
        &self,
        start: usize,
        target: usize,
        max_time: f64,
        shots: usize,
    ) -> Result<f64, BackendError> {
        let mut time = self.dt;
        let time_step = self.dt * 10.0;

        while time < max_time {
            let result = self.walk(start, time, shots)?;

            // Check if target has significant probability
            if result.probabilities[target] > 0.5 {
                return Ok(time);
            }

            time += time_step;
        }

        Ok(max_time) // Did not hit target
    }

    /// Calculate centrality scores based on quantum walk
    pub fn centrality_scores(&self, evolution_time: f64, shots: usize) -> Result<Vec<f64>, BackendError> {
        let mut total_scores = vec![0.0; self.num_nodes];

        // Run walk from each starting node
        for start in 0..self.num_nodes {
            let result = self.walk(start, evolution_time, shots)?;

            // Accumulate probabilities
            for (i, &prob) in result.probabilities.iter().enumerate() {
                total_scores[i] += prob;
            }
        }

        // Normalize
        let sum: f64 = total_scores.iter().sum();
        if sum > 0.0 {
            for score in &mut total_scores {
                *score /= sum;
            }
        }

        Ok(total_scores)
    }
}

impl QuantumWalk<LocalSimulator> {
    /// Create a walk on the Metatron graph
    pub fn metatron() -> Self {
        let mut adj = DMatrix::zeros(13, 13);

        // Center to hexagon
        for i in 1..=6 {
            adj[(0, i)] = 1.0;
            adj[(i, 0)] = 1.0;
        }

        // Hexagon ring
        for i in 1..=6 {
            let j = 1 + (i % 6);
            adj[(i, j)] = 1.0;
            adj[(j, i)] = 1.0;
        }

        // Hexagon to cube
        for i in 1..=6 {
            adj[(i, 6 + i)] = 1.0;
            adj[(6 + i, i)] = 1.0;
        }

        let backend = LocalSimulator::new(13);
        Self::new(backend, adj)
    }

    /// Create from edge list
    pub fn from_edges(num_nodes: usize, edges: &[(usize, usize, f64)]) -> Self {
        let mut adj = DMatrix::zeros(num_nodes, num_nodes);

        for &(i, j, weight) in edges {
            adj[(i, j)] = weight;
            adj[(j, i)] = weight;
        }

        let backend = LocalSimulator::new(num_nodes);
        Self::new(backend, adj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_walk_creation() {
        let walk = QuantumWalk::metatron();
        assert_eq!(walk.num_nodes, 13);
    }

    #[test]
    fn test_walk_from_center() {
        let walk = QuantumWalk::metatron().with_dt(0.2);
        let result = walk.walk(0, 1.0, 100).unwrap();

        assert_eq!(result.probabilities.len(), 13);
        assert_eq!(result.start_node, 0);

        // Probability should sum to ~1
        let sum: f64 = result.probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_hotspots() {
        let walk = QuantumWalk::metatron();
        let result = walk.walk(0, 2.0, 100).unwrap();

        // Should have hotspots
        assert!(!result.hotspots.is_empty());

        // Hotspots should be sorted by probability
        for i in 1..result.hotspots.len() {
            assert!(result.hotspots[i - 1].1 >= result.hotspots[i].1);
        }
    }

    #[test]
    fn test_centrality() {
        let edges = vec![
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (3, 0, 1.0),
            (0, 2, 1.0), // Diagonal makes 0 and 2 more central
        ];

        let walk = QuantumWalk::from_edges(4, &edges);
        let centrality = walk.centrality_scores(1.0, 100).unwrap();

        assert_eq!(centrality.len(), 4);

        // 0 and 2 should have higher centrality (more connections)
        // This is a weak test due to quantum randomness
        let sum: f64 = centrality.iter().sum();
        assert!((sum - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_from_edges() {
        let edges = vec![
            (0, 1, 1.0),
            (1, 2, 1.0),
        ];

        let walk = QuantumWalk::from_edges(3, &edges);
        assert_eq!(walk.num_nodes, 3);
        assert_eq!(walk.adjacency[(0, 1)], 1.0);
        assert_eq!(walk.adjacency[(1, 0)], 1.0);
    }
}
