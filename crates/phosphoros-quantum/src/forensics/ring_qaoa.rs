//! Ring Signature Analysis using QAOA
//!
//! Applies Quantum Approximate Optimization to identify
//! the true spend in Monero ring signatures.

use crate::algorithms::qaoa::{QAOA, MaxCutProblem, QAOAResult};
use crate::algorithms::quantum_walk::QuantumWalk;
use crate::backend::QuantumBackend;
use crate::backend::simulator::LocalSimulator;
use crate::backend::BackendError;
use nalgebra::DMatrix;

/// Quantum-enhanced ring signature analyzer
pub struct RingSignatureQAOA<B: QuantumBackend> {
    /// QAOA solver
    qaoa: QAOA<B>,

    /// QAOA depth parameter
    depth: usize,

    /// Number of measurement shots
    shots: usize,
}

/// Result of quantum ring signature analysis
#[derive(Debug, Clone)]
pub struct QuantumRingAnalysis {
    /// Most likely real spend index
    pub predicted_real_index: usize,

    /// Confidence in the prediction
    pub confidence: f64,

    /// Probability distribution over ring members
    pub probabilities: Vec<f64>,

    /// QAOA result details
    pub qaoa_result: Option<QAOAResult>,

    /// Quantum walk centrality scores
    pub centrality_scores: Option<Vec<f64>>,

    /// Combined analysis score
    pub combined_score: f64,
}

/// Information about a ring member
#[derive(Debug, Clone)]
pub struct RingMember {
    /// Index in the ring
    pub index: usize,

    /// Global output index
    pub global_index: u64,

    /// Block height of the output
    pub block_height: u64,

    /// Age in blocks
    pub age_blocks: u64,

    /// Prior probability (from classical heuristics)
    pub prior_probability: f64,

    /// Public key (hex string)
    pub public_key: String,
}

impl<B: QuantumBackend> RingSignatureQAOA<B> {
    /// Create a new quantum ring analyzer
    pub fn new(backend: B, depth: usize) -> Self {
        let qaoa = QAOA::new(backend, depth);
        Self {
            qaoa,
            depth,
            shots: 1000,
        }
    }

    /// Set number of shots
    pub fn with_shots(mut self, shots: usize) -> Self {
        self.shots = shots;
        self
    }

    /// Analyze a ring signature
    pub fn analyze(&self, members: &[RingMember]) -> Result<QuantumRingAnalysis, BackendError> {
        if members.is_empty() {
            return Err(BackendError::InvalidCircuit("Empty ring".to_string()));
        }

        let n = members.len();

        // Step 1: Build MaxCut problem from ring members
        let problem = self.build_ring_problem(members);

        // Step 2: Solve with QAOA
        let qaoa_result = self.qaoa.solve_maxcut(&problem, self.shots)?;

        // Step 3: Interpret results
        let probabilities = self.interpret_partition(&qaoa_result, members);

        // Step 4: Find most likely real spend
        let (predicted_index, confidence) = self.find_most_likely(&probabilities);

        Ok(QuantumRingAnalysis {
            predicted_real_index: predicted_index,
            confidence,
            probabilities,
            qaoa_result: Some(qaoa_result),
            centrality_scores: None,
            combined_score: confidence,
        })
    }

    /// Analyze with additional quantum walk
    pub fn analyze_with_walk(&self, members: &[RingMember]) -> Result<QuantumRingAnalysis, BackendError> {
        // First do QAOA analysis
        let mut result = self.analyze(members)?;

        // Then add quantum walk centrality
        let walk = self.build_quantum_walk(members);
        let walk_result = walk.centrality_scores(2.0, self.shots / 2)?;

        // Combine scores
        let combined_probs: Vec<f64> = result.probabilities
            .iter()
            .zip(walk_result.iter())
            .map(|(p, c)| p * 0.7 + c * 0.3) // Weighted combination
            .collect();

        // Normalize
        let sum: f64 = combined_probs.iter().sum();
        let normalized: Vec<f64> = combined_probs.iter().map(|p| p / sum).collect();

        // Recalculate prediction
        let (predicted_index, confidence) = self.find_most_likely(&normalized);

        result.centrality_scores = Some(walk_result);
        result.probabilities = normalized;
        result.predicted_real_index = predicted_index;
        result.confidence = confidence;
        result.combined_score = confidence;

        Ok(result)
    }

    /// Build MaxCut problem from ring members
    fn build_ring_problem(&self, members: &[RingMember]) -> MaxCutProblem {
        let n = members.len();
        let mut problem = MaxCutProblem::new(n);

        problem.labels = members.iter()
            .map(|m| format!("member_{}", m.index))
            .collect();

        // Create edges based on:
        // 1. Age difference (similar ages suggest same source)
        // 2. Prior probability difference (different priors suggest different categories)

        for i in 0..n {
            for j in (i + 1)..n {
                let age_i = members[i].age_blocks as f64;
                let age_j = members[j].age_blocks as f64;
                let max_age = age_i.max(age_j).max(1.0);

                // Normalized age difference
                let age_diff = (age_i - age_j).abs() / max_age;

                // Prior probability difference
                let prior_diff = (members[i].prior_probability - members[j].prior_probability).abs();

                // Combined weight: higher for dissimilar members
                // (want to cut between real and decoys)
                let weight = age_diff * 0.5 + prior_diff * 0.5;

                if weight > 0.01 {
                    problem.add_edge(i, j, weight);
                }
            }
        }

        problem
    }

    /// Build quantum walk graph from ring members
    fn build_quantum_walk(&self, members: &[RingMember]) -> QuantumWalk<LocalSimulator> {
        let n = members.len();
        let mut adj = DMatrix::zeros(n, n);

        // Build adjacency based on temporal proximity
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let age_i = members[i].age_blocks as f64;
                    let age_j = members[j].age_blocks as f64;
                    let max_age = age_i.max(age_j).max(1.0);

                    // Inverse age difference as connection strength
                    let age_diff = (age_i - age_j).abs() / max_age;
                    let weight = 1.0 - age_diff;

                    if weight > 0.1 {
                        adj[(i, j)] = weight;
                    }
                }
            }
        }

        let backend = LocalSimulator::new(n);
        QuantumWalk::new(backend, adj).with_dt(0.2)
    }

    /// Interpret QAOA partition result
    fn interpret_partition(&self, result: &QAOAResult, members: &[RingMember]) -> Vec<f64> {
        let n = members.len();
        let partition = &result.solution;

        // The partition divides members into two groups
        // The smaller group (fewer members) is more likely to contain the real spend

        let group_0: Vec<usize> = partition.iter().enumerate()
            .filter(|(_, &b)| !b)
            .map(|(i, _)| i)
            .collect();

        let group_1: Vec<usize> = partition.iter().enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i)
            .collect();

        // Identify smaller group as likely containing real spend
        let real_group = if group_0.len() <= group_1.len() { &group_0 } else { &group_1 };

        // Assign probabilities based on:
        // 1. Group membership
        // 2. Prior probability
        // 3. Approximation ratio as confidence

        let confidence_factor = result.approximation_ratio;
        let mut probabilities = vec![0.0; n];

        for &idx in real_group {
            // Higher probability for real group members
            probabilities[idx] = confidence_factor * 0.7 + members[idx].prior_probability * 0.3;
        }

        // Give small probability to other group
        for i in 0..n {
            if !real_group.contains(&i) {
                probabilities[i] = (1.0 - confidence_factor) * 0.3 + members[i].prior_probability * 0.1;
            }
        }

        // Normalize
        let sum: f64 = probabilities.iter().sum();
        if sum > 0.0 {
            for p in &mut probabilities {
                *p /= sum;
            }
        }

        probabilities
    }

    /// Find most likely real spend
    fn find_most_likely(&self, probabilities: &[f64]) -> (usize, f64) {
        let (idx, &prob) = probabilities.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((0, &0.0));

        (idx, prob)
    }
}

impl RingSignatureQAOA<LocalSimulator> {
    /// Create with default local simulator
    pub fn default_local(ring_size: usize, depth: usize) -> Self {
        let backend = LocalSimulator::new(ring_size);
        Self::new(backend, depth)
    }
}

/// Helper to create ring members from raw data
pub fn create_ring_members(
    global_indices: &[u64],
    block_heights: &[u64],
    current_height: u64,
    prior_probs: Option<&[f64]>,
) -> Vec<RingMember> {
    let n = global_indices.len();
    let default_probs: Vec<f64> = vec![1.0 / n as f64; n];
    let probs = prior_probs.unwrap_or(&default_probs);

    global_indices.iter()
        .zip(block_heights.iter())
        .zip(probs.iter())
        .enumerate()
        .map(|(i, ((&gi, &bh), &pp))| RingMember {
            index: i,
            global_index: gi,
            block_height: bh,
            age_blocks: current_height.saturating_sub(bh),
            prior_probability: pp,
            public_key: format!("pk_{}", gi),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_analyzer_creation() {
        let analyzer = RingSignatureQAOA::default_local(16, 2);
        assert_eq!(analyzer.depth, 2);
    }

    #[test]
    fn test_create_ring_members() {
        let indices = vec![100, 200, 300, 400];
        let heights = vec![1000, 1500, 1800, 1900];
        let current = 2000;

        let members = create_ring_members(&indices, &heights, current, None);

        assert_eq!(members.len(), 4);
        assert_eq!(members[0].age_blocks, 1000);
        assert_eq!(members[3].age_blocks, 100);
    }

    #[test]
    fn test_simple_ring_analysis() {
        let analyzer = RingSignatureQAOA::default_local(4, 1)
            .with_shots(100);

        // Create test members with one obviously different from others
        let members = vec![
            RingMember {
                index: 0,
                global_index: 100,
                block_height: 1000,
                age_blocks: 1000, // Old
                prior_probability: 0.1,
                public_key: "pk_100".to_string(),
            },
            RingMember {
                index: 1,
                global_index: 200,
                block_height: 1800,
                age_blocks: 200, // Recent - likely real
                prior_probability: 0.7,
                public_key: "pk_200".to_string(),
            },
            RingMember {
                index: 2,
                global_index: 300,
                block_height: 1100,
                age_blocks: 900, // Old
                prior_probability: 0.1,
                public_key: "pk_300".to_string(),
            },
            RingMember {
                index: 3,
                global_index: 400,
                block_height: 1050,
                age_blocks: 950, // Old
                prior_probability: 0.1,
                public_key: "pk_400".to_string(),
            },
        ];

        let result = analyzer.analyze(&members).unwrap();

        assert!(result.probabilities.len() == 4);
        assert!(result.confidence > 0.0);

        // Member 1 has highest prior and is most recent - should have high prob
        // (This is a weak test due to quantum randomness)
        let sum: f64 = result.probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_ring_problem_construction() {
        let analyzer = RingSignatureQAOA::default_local(3, 1);

        let members = vec![
            RingMember {
                index: 0,
                global_index: 100,
                block_height: 1000,
                age_blocks: 1000,
                prior_probability: 0.8,
                public_key: "pk_100".to_string(),
            },
            RingMember {
                index: 1,
                global_index: 200,
                block_height: 1500,
                age_blocks: 500,
                prior_probability: 0.1,
                public_key: "pk_200".to_string(),
            },
            RingMember {
                index: 2,
                global_index: 300,
                block_height: 1400,
                age_blocks: 600,
                prior_probability: 0.1,
                public_key: "pk_300".to_string(),
            },
        ];

        let problem = analyzer.build_ring_problem(&members);

        assert_eq!(problem.num_nodes, 3);
        assert!(problem.edges.len() > 0);
    }

    #[test]
    fn test_with_quantum_walk() {
        let analyzer = RingSignatureQAOA::default_local(4, 1)
            .with_shots(50);

        let members = create_ring_members(
            &[100, 200, 300, 400],
            &[1000, 1800, 1100, 1050],
            2000,
            Some(&[0.1, 0.6, 0.2, 0.1]),
        );

        let result = analyzer.analyze_with_walk(&members).unwrap();

        assert!(result.centrality_scores.is_some());
        assert!(result.probabilities.len() == 4);
    }
}
