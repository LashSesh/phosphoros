//! Quantum-Enhanced Ring Signature Analysis
//!
//! This module provides quantum computing acceleration for ring signature analysis
//! using QAOA (Quantum Approximate Optimization Algorithm) for optimal decoy identification.
//!
//! # Features
//!
//! Enable with the `quantum` feature flag:
//! ```toml
//! [dependencies]
//! phosphoros-monero = { path = "...", features = ["quantum"] }
//! ```

#[cfg(feature = "quantum")]
use phosphoros_quantum::forensics::{RingSignatureQAOA, RingMember, QuantumRingAnalysis};
#[cfg(feature = "quantum")]
use phosphoros_quantum::backend::simulator::LocalSimulator;

use crate::ring_analysis::{RingAnalysisResult, RingMemberInfo};
use crate::error::Error;

/// Quantum-enhanced ring signature analyzer
///
/// Combines classical heuristics with quantum QAOA for improved accuracy.
#[cfg(feature = "quantum")]
pub struct QuantumRingAnalyzer {
    /// QAOA depth (layers)
    depth: usize,
    /// Number of measurement shots
    shots: usize,
    /// Weight for quantum results (0.0 = classical only, 1.0 = quantum only)
    quantum_weight: f64,
}

#[cfg(feature = "quantum")]
impl QuantumRingAnalyzer {
    /// Create a new quantum ring analyzer
    pub fn new(depth: usize, shots: usize) -> Self {
        Self {
            depth,
            shots,
            quantum_weight: 0.5, // Balanced by default
        }
    }

    /// Set the weight for quantum results
    pub fn with_quantum_weight(mut self, weight: f64) -> Self {
        self.quantum_weight = weight.clamp(0.0, 1.0);
        self
    }

    /// Analyze a ring signature using quantum-classical hybrid approach
    pub fn analyze(
        &self,
        ring_members: &[RingMemberInfo],
        classical_result: &RingAnalysisResult,
    ) -> Result<QuantumEnhancedResult, Error> {
        // Convert to quantum format
        let quantum_members: Vec<RingMember> = ring_members
            .iter()
            .enumerate()
            .map(|(i, m)| RingMember {
                index: i,
                global_index: m.global_index,
                block_height: m.block_height,
                age_blocks: m.age_blocks,
                prior_probability: m.probability,
                public_key: m.public_key.clone(),
            })
            .collect();

        // Run QAOA analysis
        let backend = LocalSimulator::new(ring_members.len().min(13));
        let qaoa = RingSignatureQAOA::new(backend, self.depth);

        let quantum_result = qaoa
            .analyze(&quantum_members)
            .map_err(|e| Error::AnalysisFailed(format!("Quantum analysis failed: {:?}", e)))?;

        // Combine classical and quantum results
        let combined_probabilities = self.combine_results(
            ring_members,
            classical_result,
            &quantum_result,
        );

        // Find best candidate
        let (best_index, best_confidence) = combined_probabilities
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, &p)| (i, p))
            .unwrap_or((0, 0.0));

        Ok(QuantumEnhancedResult {
            classical_index: classical_result.combined_result.predicted_index,
            quantum_index: Some(quantum_result.predicted_real_index),
            combined_index: best_index,
            classical_confidence: classical_result.confidence,
            quantum_confidence: quantum_result.confidence,
            combined_confidence: best_confidence,
            probabilities: combined_probabilities,
            quantum_details: Some(quantum_result),
            improvement_over_classical: best_confidence - classical_result.confidence,
        })
    }

    /// Combine classical and quantum probability distributions
    fn combine_results(
        &self,
        ring_members: &[RingMemberInfo],
        classical: &RingAnalysisResult,
        quantum: &QuantumRingAnalysis,
    ) -> Vec<f64> {
        let n = ring_members.len();
        let mut combined = vec![0.0; n];

        // Get classical probabilities
        let classical_probs: Vec<f64> = ring_members.iter().map(|m| m.probability).collect();

        // Combine with quantum probabilities
        for i in 0..n {
            let classical_p = classical_probs.get(i).copied().unwrap_or(0.0);
            let quantum_p = quantum.probabilities.get(i).copied().unwrap_or(0.0);

            combined[i] = (1.0 - self.quantum_weight) * classical_p
                        + self.quantum_weight * quantum_p;
        }

        // Normalize
        let sum: f64 = combined.iter().sum();
        if sum > 0.0 {
            for p in &mut combined {
                *p /= sum;
            }
        }

        combined
    }
}

/// Result of quantum-enhanced ring analysis
#[derive(Debug, Clone)]
pub struct QuantumEnhancedResult {
    /// Classical analysis prediction
    pub classical_index: Option<usize>,
    /// Quantum analysis prediction
    pub quantum_index: Option<usize>,
    /// Combined prediction
    pub combined_index: usize,
    /// Classical confidence
    pub classical_confidence: f64,
    /// Quantum confidence
    pub quantum_confidence: f64,
    /// Combined confidence
    pub combined_confidence: f64,
    /// Per-member probability distribution
    pub probabilities: Vec<f64>,
    /// Full quantum analysis details
    #[cfg(feature = "quantum")]
    pub quantum_details: Option<QuantumRingAnalysis>,
    #[cfg(not(feature = "quantum"))]
    pub quantum_details: Option<()>,
    /// Improvement over classical analysis
    pub improvement_over_classical: f64,
}

impl QuantumEnhancedResult {
    /// Check if quantum analysis improved over classical
    pub fn quantum_improved(&self) -> bool {
        self.improvement_over_classical > 0.0
    }

    /// Get the agreement level between classical and quantum
    pub fn classical_quantum_agreement(&self) -> bool {
        self.classical_index == self.quantum_index
    }
}

/// Placeholder analyzer when quantum feature is disabled
#[cfg(not(feature = "quantum"))]
pub struct QuantumRingAnalyzer;

#[cfg(not(feature = "quantum"))]
impl QuantumRingAnalyzer {
    /// Create analyzer (quantum disabled)
    pub fn new(_depth: usize, _shots: usize) -> Self {
        Self
    }

    /// Analyze returns error when quantum is disabled
    pub fn analyze(
        &self,
        _ring_members: &[RingMemberInfo],
        _classical_result: &RingAnalysisResult,
    ) -> Result<QuantumEnhancedResult, Error> {
        Err(Error::AnalysisFailed(
            "Quantum feature not enabled. Compile with --features quantum".to_string()
        ))
    }
}

#[cfg(all(test, feature = "quantum"))]
mod tests {
    use super::*;
    use crate::heuristics::HeuristicResult;

    fn create_test_members() -> Vec<RingMemberInfo> {
        vec![
            RingMemberInfo {
                global_index: 100,
                block_height: 1000,
                public_key: "pk_100".to_string(),
                age_blocks: 1000,
                source_tx_hash: None,
                probability: 0.1,
            },
            RingMemberInfo {
                global_index: 200,
                block_height: 1800,
                public_key: "pk_200".to_string(),
                age_blocks: 200,
                source_tx_hash: None,
                probability: 0.7,
            },
            RingMemberInfo {
                global_index: 300,
                block_height: 1100,
                public_key: "pk_300".to_string(),
                age_blocks: 900,
                source_tx_hash: None,
                probability: 0.1,
            },
            RingMemberInfo {
                global_index: 400,
                block_height: 1050,
                public_key: "pk_400".to_string(),
                age_blocks: 950,
                source_tx_hash: None,
                probability: 0.1,
            },
        ]
    }

    fn create_test_classical_result() -> RingAnalysisResult {
        RingAnalysisResult {
            tx_hash: "test_tx".to_string(),
            input_index: 0,
            key_image: "key_image".to_string(),
            ring_size: 4,
            heuristic_results: vec![],
            combined_result: HeuristicResult {
                heuristic_name: "combined".to_string(),
                predicted_index: Some(1),
                confidence: 0.7,
                member_scores: vec![0.1, 0.7, 0.1, 0.1],
                explanation: "Test combined result".to_string(),
            },
            ring_members: create_test_members(),
            confidence: 0.7,
            reliable: true,
        }
    }

    #[test]
    fn test_quantum_analyzer_creation() {
        let analyzer = QuantumRingAnalyzer::new(2, 100);
        assert_eq!(analyzer.depth, 2);
    }

    #[test]
    fn test_quantum_analysis() {
        let analyzer = QuantumRingAnalyzer::new(1, 50)
            .with_quantum_weight(0.3);

        let members = create_test_members();
        let classical = create_test_classical_result();

        let result = analyzer.analyze(&members, &classical);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.combined_confidence >= 0.0);
        assert!(result.probabilities.len() == 4);
    }
}
