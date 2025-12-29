//! Quantum Resonance Engine
//!
//! A quantum-enhanced implementation of resonance evaluation that uses
//! variational quantum circuits to compute the (ψ, ρ, ω) resonance triplet.

use crate::backend::{QuantumBackend, QuantumCircuit, QuantumResult, BackendError};
use crate::backend::simulator::LocalSimulator;
use super::{QuantumEngineConfig, AnsatzType};
use std::f64::consts::PI;

/// Quantum-enhanced resonance engine using variational circuits
pub struct QuantumResonanceEngine<B: QuantumBackend> {
    /// Quantum backend for circuit execution
    backend: B,

    /// Engine configuration
    config: QuantumEngineConfig,

    /// Current variational parameters
    parameters: Vec<f64>,

    /// Cached evaluation results
    cache: Option<ResonanceCache>,

    /// Current time parameter
    time: f64,
}

/// Cached resonance evaluation data (for future cache invalidation)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ResonanceCache {
    perception: [f64; 5],
    intention: [f64; 5],
    result: QuantumResonanceResult,
}

/// Result of quantum resonance evaluation
#[derive(Debug, Clone)]
pub struct QuantumResonanceResult {
    /// The (ψ, ρ, ω) resonance triplet
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,

    /// Combined resonance score D = ψ·ρ·ω
    pub resonance: f64,

    /// Mandorla field intensity
    pub mandorla_intensity: f64,

    /// Raw quantum measurement result
    pub quantum_result: Option<QuantumResult>,

    /// Whether the evaluation passed the threshold
    pub passed_threshold: bool,
}

impl<B: QuantumBackend> QuantumResonanceEngine<B> {
    /// Create a new quantum resonance engine
    pub fn new(backend: B, config: QuantumEngineConfig) -> Self {
        // Initialize variational parameters based on ansatz
        let num_params = match config.ansatz_type {
            AnsatzType::HardwareEfficient => 3 * 13 * config.max_depth, // 3 rotations per qubit per layer
            AnsatzType::EfficientSU2 => 2 * 13 * config.max_depth,
            AnsatzType::MetatronOptimized => 13 * config.max_depth + 78, // 78 edges
            AnsatzType::Custom => 100,
        };

        let parameters = vec![0.0; num_params];

        Self {
            backend,
            config,
            parameters,
            cache: None,
            time: 0.0,
        }
    }

    /// Evaluate resonance for given perception and intention vectors
    pub fn evaluate(
        &mut self,
        t: f64,
        perception: [f64; 5],
        intention: [f64; 5],
        theta: f64,
    ) -> Result<QuantumResonanceResult, BackendError> {
        self.time = t;

        // Build quantum circuit for resonance evaluation
        let circuit = self.build_resonance_circuit(&perception, &intention)?;

        // Execute on quantum backend
        let quantum_result = self.backend.run_circuit(&circuit, self.config.shots)?;

        // Extract resonance triplet from measurement results
        let (psi, rho, omega) = self.extract_resonance_triplet(&quantum_result);

        // Calculate combined resonance
        let resonance = psi * rho * omega;

        // Calculate Mandorla field intensity (dot product of P and I encoded in quantum state)
        let mandorla_intensity = quantum_result
            .expectation("mandorla")
            .unwrap_or(resonance);

        let result = QuantumResonanceResult {
            psi,
            rho,
            omega,
            resonance,
            mandorla_intensity,
            quantum_result: Some(quantum_result),
            passed_threshold: resonance > theta,
        };

        // Cache the result
        self.cache = Some(ResonanceCache {
            perception,
            intention,
            result: result.clone(),
        });

        Ok(result)
    }

    /// Build the quantum circuit for resonance evaluation
    fn build_resonance_circuit(
        &self,
        perception: &[f64; 5],
        intention: &[f64; 5],
    ) -> Result<QuantumCircuit, BackendError> {
        let num_qubits = 13; // Metatron geometry
        let mut circuit = QuantumCircuit::new(num_qubits);

        // Step 1: Encode perception vector into first register (qubits 0-4)
        self.encode_5d_vector(&mut circuit, perception, 0)?;

        // Step 2: Encode intention vector into second register (qubits 5-9)
        self.encode_5d_vector(&mut circuit, intention, 5)?;

        // Step 3: Apply Mandorla entanglement (qubits 10-12 as ancilla)
        self.apply_mandorla_circuit(&mut circuit)?;

        // Step 4: Apply variational ansatz for resonance optimization
        self.apply_ansatz(&mut circuit)?;

        // Step 5: Measure all qubits
        circuit.measure_all();

        Ok(circuit)
    }

    /// Encode a 5D vector into 5 qubits using amplitude encoding
    fn encode_5d_vector(
        &self,
        circuit: &mut QuantumCircuit,
        vector: &[f64; 5],
        offset: usize,
    ) -> Result<(), BackendError> {
        // Normalize the vector
        let norm: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        let normalized: Vec<f64> = if norm > 1e-10 {
            vector.iter().map(|x| x / norm).collect()
        } else {
            vec![1.0, 0.0, 0.0, 0.0, 0.0]
        };

        // Use rotation gates to encode the vector
        for (i, &val) in normalized.iter().enumerate() {
            let qubit = offset + i;

            // Map value from [-1, 1] to rotation angle [0, π]
            let theta = (val.clamp(-1.0, 1.0) + 1.0) * PI / 2.0;

            // Apply Ry rotation for amplitude encoding
            circuit.ry(qubit, theta);

            // Apply Rz for phase encoding (using time parameter)
            circuit.rz(qubit, self.time * val);
        }

        Ok(())
    }

    /// Apply Mandorla entanglement circuit
    ///
    /// Creates entanglement between perception and intention registers
    /// to compute their "intersection" in quantum superposition
    fn apply_mandorla_circuit(&self, circuit: &mut QuantumCircuit) -> Result<(), BackendError> {
        // Hadamard on ancilla qubits
        circuit.h(10).h(11).h(12);

        // Controlled operations from perception to ancilla
        for i in 0..5 {
            circuit.cnot(i, 10 + (i % 3));
        }

        // Controlled operations from intention to ancilla
        for i in 0..5 {
            circuit.cnot(5 + i, 10 + ((i + 1) % 3));
        }

        // CZ gates between perception and intention (creates interference)
        for i in 0..5 {
            circuit.cz(i, 5 + i);
        }

        Ok(())
    }

    /// Apply the variational ansatz
    fn apply_ansatz(&self, circuit: &mut QuantumCircuit) -> Result<(), BackendError> {
        let depth = self.config.max_depth.min(3); // Limit depth for efficiency
        let mut param_idx = 0;

        for _layer in 0..depth {
            // Single-qubit rotations
            for qubit in 0..13 {
                if param_idx + 2 < self.parameters.len() {
                    circuit.rx(qubit, self.parameters[param_idx]);
                    circuit.ry(qubit, self.parameters[param_idx + 1]);
                    circuit.rz(qubit, self.parameters[param_idx + 2]);
                    param_idx += 3;
                }
            }

            // Entangling layer (Metatron geometry edges)
            // Inner hexagon
            for i in 0..6 {
                circuit.cnot(1 + i, 1 + ((i + 1) % 6));
            }

            // Outer cube connections
            for i in 0..6 {
                circuit.cz(1 + i, 7 + i);
            }

            // Center connections
            circuit.cnot(0, 1);
            circuit.cnot(0, 4);
            circuit.cnot(0, 7);
        }

        Ok(())
    }

    /// Extract resonance triplet from quantum measurement results
    fn extract_resonance_triplet(&self, result: &QuantumResult) -> (f64, f64, f64) {
        // If we have the final state, use it directly
        if let Some(ref state) = result.final_state {
            let amplitudes = &state.amplitudes;
            let n = amplitudes.len();

            // ψ (coherence): Measure off-diagonal coherence
            let psi: f64 = amplitudes.iter().map(|a| a.norm()).sum::<f64>() / (n as f64).sqrt();

            // ρ (purity/density): State purity
            let rho: f64 = amplitudes
                .iter()
                .map(|a| a.norm_sqr().powi(2))
                .sum::<f64>()
                .sqrt();

            // ω (frequency/phase): Phase coherence
            let phases: Vec<f64> = amplitudes
                .iter()
                .filter(|a| a.norm() > 1e-10)
                .map(|a| a.arg())
                .collect();

            let omega = if phases.is_empty() {
                0.5
            } else {
                let mean_phase: f64 = phases.iter().sum::<f64>() / phases.len() as f64;
                let phase_variance: f64 = phases
                    .iter()
                    .map(|p| (p - mean_phase).powi(2))
                    .sum::<f64>()
                    / phases.len() as f64;
                (-phase_variance).exp()
            };

            return (psi.clamp(0.0, 1.0), rho.clamp(0.0, 1.0), omega.clamp(0.0, 1.0));
        }

        // Fallback: estimate from measurement statistics
        let total_counts: usize = result.counts.values().sum();
        if total_counts == 0 {
            return (0.5, 0.5, 0.5);
        }

        // ψ: Entropy-based coherence measure
        let entropy: f64 = result
            .counts
            .values()
            .map(|&count| {
                let p = count as f64 / total_counts as f64;
                if p > 0.0 {
                    -p * p.ln()
                } else {
                    0.0
                }
            })
            .sum();

        let max_entropy = (result.counts.len() as f64).ln();
        let psi = if max_entropy > 0.0 {
            1.0 - (entropy / max_entropy)
        } else {
            1.0
        };

        // ρ: Concentration measure
        let max_count = result.counts.values().max().copied().unwrap_or(0);
        let rho = max_count as f64 / total_counts as f64;

        // ω: Use Z expectation if available
        let omega = result
            .expectation("Z")
            .map(|z| (z + 1.0) / 2.0) // Map from [-1, 1] to [0, 1]
            .unwrap_or(0.5);

        (psi.clamp(0.0, 1.0), rho.clamp(0.0, 1.0), omega.clamp(0.0, 1.0))
    }

    /// Update variational parameters (for optimization)
    pub fn update_parameters(&mut self, new_params: Vec<f64>) {
        self.parameters = new_params;
    }

    /// Get current parameters
    pub fn parameters(&self) -> &[f64] {
        &self.parameters
    }

    /// Reset the engine state
    pub fn reset(&mut self) {
        self.cache = None;
        self.time = 0.0;
        self.parameters = vec![0.0; self.parameters.len()];
    }

    /// Get the last cached result
    pub fn cached_result(&self) -> Option<&QuantumResonanceResult> {
        self.cache.as_ref().map(|c| &c.result)
    }
}

/// Create a quantum resonance engine with default local simulator
impl QuantumResonanceEngine<LocalSimulator> {
    /// Create with default settings
    pub fn default_local() -> Self {
        let backend = LocalSimulator::metatron();
        let config = QuantumEngineConfig::default();
        Self::new(backend, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_resonance_creation() {
        let engine = QuantumResonanceEngine::default_local();
        assert!(engine.parameters().len() > 0);
    }

    #[test]
    fn test_resonance_evaluation() {
        let mut engine = QuantumResonanceEngine::default_local();

        let perception = [0.5, 0.3, 0.7, 0.2, 0.8];
        let intention = [0.4, 0.6, 0.5, 0.3, 0.7];
        let theta = 0.1;

        let result = engine.evaluate(0.0, perception, intention, theta).unwrap();

        // Check that resonance values are in valid range
        assert!(result.psi >= 0.0 && result.psi <= 1.0);
        assert!(result.rho >= 0.0 && result.rho <= 1.0);
        assert!(result.omega >= 0.0 && result.omega <= 1.0);
        assert!(result.resonance >= 0.0 && result.resonance <= 1.0);
    }

    #[test]
    fn test_aligned_vectors_high_resonance() {
        let mut engine = QuantumResonanceEngine::default_local();

        // Identical perception and intention should have higher resonance
        let aligned = [0.5, 0.5, 0.5, 0.5, 0.5];

        let result = engine.evaluate(0.0, aligned, aligned, 0.0).unwrap();

        // Aligned vectors should produce meaningful resonance
        assert!(result.resonance > 0.0);
    }

    #[test]
    fn test_caching() {
        let mut engine = QuantumResonanceEngine::default_local();

        let p = [0.1, 0.2, 0.3, 0.4, 0.5];
        let i = [0.5, 0.4, 0.3, 0.2, 0.1];

        let _ = engine.evaluate(0.0, p, i, 0.1);

        assert!(engine.cached_result().is_some());
    }

    #[test]
    fn test_reset() {
        let mut engine = QuantumResonanceEngine::default_local();

        let p = [0.1, 0.2, 0.3, 0.4, 0.5];
        let i = [0.5, 0.4, 0.3, 0.2, 0.1];

        let _ = engine.evaluate(0.0, p, i, 0.1);
        engine.reset();

        assert!(engine.cached_result().is_none());
    }
}
