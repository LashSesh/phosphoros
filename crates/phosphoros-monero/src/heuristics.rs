//! Ring signature analysis heuristics
//!
//! Implements scientifically documented heuristics for analyzing Monero ring signatures.
//! These methods are based on published academic research and real-world analysis.
//!
//! ## References
//!
//! - Möser et al. "An Empirical Analysis of Traceability in the Monero Blockchain"
//! - Kumar et al. "A Traceability Analysis of Monero's Blockchain"
//! - Miller et al. "An Empirical Analysis of Linkability in the Monero Blockchain"
//! - Vijayakumaran "Analysis of CryptoNote Transaction Graphs"

use serde::{Deserialize, Serialize};

/// Result of applying a heuristic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicResult {
    /// Name of the heuristic
    pub heuristic_name: String,
    /// Index of predicted real input (0-based within ring)
    pub predicted_index: Option<usize>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Detailed scores for each ring member
    pub member_scores: Vec<f64>,
    /// Explanation of the result
    pub explanation: String,
}

/// Temporal analysis heuristic
///
/// Based on the observation that real transaction inputs tend to be
/// among the most recently created outputs. The Monero wallet selects
/// decoys using a gamma distribution, but the real input is often newer.
#[derive(Debug, Clone)]
pub struct TemporalHeuristic {
    /// Gamma distribution shape parameter (Monero default: ~19.28)
    pub gamma_shape: f64,
    /// Minimum age for suspicion (blocks)
    pub min_age_blocks: u64,
    /// Weight for recency in scoring
    pub recency_weight: f64,
}

impl Default for TemporalHeuristic {
    fn default() -> Self {
        Self {
            gamma_shape: 19.28,
            min_age_blocks: 10,
            recency_weight: 0.7,
        }
    }
}

impl TemporalHeuristic {
    /// Analyze ring members based on temporal patterns
    ///
    /// # Arguments
    /// * `ring_ages` - Ages of ring members in blocks (at time of transaction)
    /// * `tx_block_height` - Block height of the transaction being analyzed
    ///
    /// # Returns
    /// Analysis result with predicted real input
    pub fn analyze(&self, ring_ages: &[u64], tx_block_height: u64) -> HeuristicResult {
        if ring_ages.is_empty() {
            return HeuristicResult {
                heuristic_name: "Temporal Analysis".to_string(),
                predicted_index: None,
                confidence: 0.0,
                member_scores: vec![],
                explanation: "Empty ring".to_string(),
            };
        }

        // Calculate scores based on age distribution
        let mut scores: Vec<f64> = ring_ages
            .iter()
            .map(|&age| self.age_to_score(age))
            .collect();

        // Normalize scores
        let max_score = scores.iter().cloned().fold(0.0, f64::max);
        if max_score > 0.0 {
            for score in &mut scores {
                *score /= max_score;
            }
        }

        // Find the most likely real input (highest score = most recent)
        let (predicted_index, max_normalized) = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &s)| (i, s))
            .unwrap_or((0, 0.0));

        // Calculate confidence based on score distribution
        let score_variance = self.calculate_variance(&scores);
        let confidence = (max_normalized * (1.0 - score_variance)).clamp(0.0, 1.0);

        // Check for "guess-newest" heuristic
        let newest_index = ring_ages
            .iter()
            .enumerate()
            .min_by_key(|(_, &age)| age)
            .map(|(i, _)| i)
            .unwrap_or(0);

        let explanation = if newest_index == predicted_index {
            format!(
                "Newest output (age {} blocks) is most likely real input. \
                 Guess-newest heuristic applies.",
                ring_ages[newest_index]
            )
        } else {
            format!(
                "Output at index {} (age {} blocks) has highest probability score.",
                predicted_index, ring_ages[predicted_index]
            )
        };

        HeuristicResult {
            heuristic_name: "Temporal Analysis (Guess-Newest)".to_string(),
            predicted_index: Some(predicted_index),
            confidence,
            member_scores: scores,
            explanation,
        }
    }

    /// Convert age to probability score
    /// Newer outputs get higher scores based on gamma distribution
    fn age_to_score(&self, age: u64) -> f64 {
        if age < self.min_age_blocks {
            return 1.0; // Very recent outputs are highly likely to be real
        }

        // Inverse gamma-like scoring (newer = higher score)
        let age_f = age as f64;
        let scale = 50.0; // Typical scale parameter

        // Higher score for newer outputs
        1.0 / (1.0 + (age_f / scale).powf(self.gamma_shape / 10.0))
    }

    fn calculate_variance(&self, scores: &[f64]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
        variance.sqrt()
    }
}

/// Decoy selection analysis heuristic
///
/// Analyzes whether the decoy selection follows expected patterns.
/// Real wallets use specific algorithms; deviations may indicate
/// the real input or reveal wallet software fingerprints.
#[derive(Debug, Clone)]
pub struct DecoySelectionHeuristic {
    /// Expected ring size
    pub expected_ring_size: usize,
    /// Whether to check for uniform distribution
    pub check_uniformity: bool,
}

impl Default for DecoySelectionHeuristic {
    fn default() -> Self {
        Self {
            expected_ring_size: 16, // Current Monero ring size
            check_uniformity: true,
        }
    }
}

impl DecoySelectionHeuristic {
    /// Analyze decoy selection patterns
    ///
    /// # Arguments
    /// * `ring_heights` - Block heights where ring members were created
    /// * `tx_height` - Block height of the transaction
    ///
    /// # Returns
    /// Analysis result indicating potential anomalies
    pub fn analyze(&self, ring_heights: &[u64], tx_height: u64) -> HeuristicResult {
        if ring_heights.len() < 2 {
            return HeuristicResult {
                heuristic_name: "Decoy Selection Analysis".to_string(),
                predicted_index: None,
                confidence: 0.0,
                member_scores: vec![],
                explanation: "Insufficient ring members".to_string(),
            };
        }

        // Calculate ages
        let ages: Vec<u64> = ring_heights
            .iter()
            .map(|&h| tx_height.saturating_sub(h))
            .collect();

        // Check for outputs that don't match expected gamma distribution
        let mut anomaly_scores: Vec<f64> = Vec::with_capacity(ages.len());
        let total_age: u64 = ages.iter().sum();
        let avg_age = total_age as f64 / ages.len() as f64;

        for &age in &ages {
            // Outputs significantly different from expected distribution
            let deviation = ((age as f64 - avg_age) / avg_age).abs();

            // Very new outputs or very old outputs are anomalous
            let age_anomaly = if age < 10 {
                0.8 // Very new - likely real
            } else if age > avg_age as u64 * 3 {
                0.3 // Very old - likely decoy
            } else {
                0.5 // Normal range
            };

            anomaly_scores.push((1.0 - deviation.min(1.0)) * age_anomaly);
        }

        // Find most likely real input
        let (predicted_index, max_score) = anomaly_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &s)| (i, s))
            .unwrap_or((0, 0.0));

        // Calculate confidence
        let confidence = max_score * 0.6; // Decoy analysis is less reliable

        let explanation = format!(
            "Analyzed {} ring members. Output at index {} shows unusual selection pattern \
             (age {} blocks, avg age {:.0} blocks).",
            ring_heights.len(),
            predicted_index,
            ages[predicted_index],
            avg_age
        );

        HeuristicResult {
            heuristic_name: "Decoy Selection Pattern".to_string(),
            predicted_index: Some(predicted_index),
            confidence,
            member_scores: anomaly_scores,
            explanation,
        }
    }

    /// Check if ring size matches expected value
    pub fn check_ring_size(&self, actual_size: usize) -> bool {
        actual_size == self.expected_ring_size
    }
}

/// Output reuse detection heuristic
///
/// Detects when an output appears in multiple transactions as a ring member,
/// which can reveal information about the real input.
#[derive(Debug, Clone, Default)]
pub struct OutputReuseHeuristic {
    /// Minimum times an output must appear to be considered overused
    pub min_reuse_count: usize,
}

impl OutputReuseHeuristic {
    /// Create new output reuse detector
    pub fn new(min_reuse_count: usize) -> Self {
        Self { min_reuse_count }
    }

    /// Analyze output reuse patterns
    ///
    /// # Arguments
    /// * `output_reuse_counts` - How many times each ring member appears in other rings
    ///
    /// # Returns
    /// Analysis result indicating likely decoys (overused outputs)
    pub fn analyze(&self, output_reuse_counts: &[usize]) -> HeuristicResult {
        if output_reuse_counts.is_empty() {
            return HeuristicResult {
                heuristic_name: "Output Reuse Analysis".to_string(),
                predicted_index: None,
                confidence: 0.0,
                member_scores: vec![],
                explanation: "No reuse data".to_string(),
            };
        }

        // Higher reuse count = more likely to be a decoy (not the real input)
        let max_reuse = *output_reuse_counts.iter().max().unwrap_or(&0) as f64;

        let scores: Vec<f64> = output_reuse_counts
            .iter()
            .map(|&count| {
                if max_reuse > 0.0 {
                    // Lower reuse = higher chance of being real
                    1.0 - (count as f64 / max_reuse)
                } else {
                    0.5
                }
            })
            .collect();

        // Find least reused output (most likely real)
        let (predicted_index, min_reuse) = output_reuse_counts
            .iter()
            .enumerate()
            .min_by_key(|(_, &count)| count)
            .map(|(i, &c)| (i, c))
            .unwrap_or((0, 0));

        let confidence = if max_reuse > 0.0 {
            (1.0 - (min_reuse as f64 / max_reuse)) * 0.5
        } else {
            0.0
        };

        let explanation = format!(
            "Output at index {} has lowest reuse count ({}), suggesting it may be the real input. \
             Max reuse in ring: {}.",
            predicted_index, min_reuse, max_reuse as usize
        );

        HeuristicResult {
            heuristic_name: "Output Reuse Pattern".to_string(),
            predicted_index: Some(predicted_index),
            confidence,
            member_scores: scores,
            explanation,
        }
    }
}

/// Combined heuristic analyzer
///
/// Combines multiple heuristics with weighted voting
#[derive(Debug, Clone)]
pub struct CombinedHeuristic {
    /// Weight for temporal heuristic
    pub temporal_weight: f64,
    /// Weight for decoy selection heuristic
    pub decoy_weight: f64,
    /// Weight for output reuse heuristic
    pub reuse_weight: f64,
}

impl Default for CombinedHeuristic {
    fn default() -> Self {
        Self {
            temporal_weight: 0.5,
            decoy_weight: 0.3,
            reuse_weight: 0.2,
        }
    }
}

impl CombinedHeuristic {
    /// Combine multiple heuristic results
    pub fn combine(&self, results: &[HeuristicResult]) -> HeuristicResult {
        if results.is_empty() {
            return HeuristicResult {
                heuristic_name: "Combined Analysis".to_string(),
                predicted_index: None,
                confidence: 0.0,
                member_scores: vec![],
                explanation: "No heuristic results to combine".to_string(),
            };
        }

        // Get the ring size from first result
        let ring_size = results[0].member_scores.len();
        if ring_size == 0 {
            return HeuristicResult {
                heuristic_name: "Combined Analysis".to_string(),
                predicted_index: None,
                confidence: 0.0,
                member_scores: vec![],
                explanation: "Empty ring".to_string(),
            };
        }

        // Combine scores with weights
        let weights = [self.temporal_weight, self.decoy_weight, self.reuse_weight];
        let mut combined_scores = vec![0.0; ring_size];

        for (result, &weight) in results.iter().zip(weights.iter()) {
            for (i, &score) in result.member_scores.iter().enumerate() {
                if i < ring_size {
                    combined_scores[i] += score * weight * result.confidence;
                }
            }
        }

        // Normalize
        let total_weight: f64 = results
            .iter()
            .zip(weights.iter())
            .map(|(r, &w)| w * r.confidence)
            .sum();

        if total_weight > 0.0 {
            for score in &mut combined_scores {
                *score /= total_weight;
            }
        }

        // Find best prediction
        let (predicted_index, max_score) = combined_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &s)| (i, s))
            .unwrap_or((0, 0.0));

        let heuristics_used: Vec<&str> = results.iter().map(|r| r.heuristic_name.as_str()).collect();

        HeuristicResult {
            heuristic_name: "Combined Multi-Heuristic Analysis".to_string(),
            predicted_index: Some(predicted_index),
            confidence: max_score,
            member_scores: combined_scores,
            explanation: format!(
                "Combined {} heuristics: {}. Predicted real input at index {} with {:.1}% confidence.",
                results.len(),
                heuristics_used.join(", "),
                predicted_index,
                max_score * 100.0
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_heuristic_newest() {
        let heuristic = TemporalHeuristic::default();

        // Ring with ages: the newest (age 5) should be predicted
        let ages = vec![1000, 500, 5, 2000, 750];
        let result = heuristic.analyze(&ages, 3000);

        assert_eq!(result.predicted_index, Some(2)); // Index 2 has age 5
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_decoy_selection_heuristic() {
        let heuristic = DecoySelectionHeuristic::default();

        // Heights where outputs were created
        let heights = vec![1000, 1500, 2900, 2000, 2500];
        let tx_height = 3000;

        let result = heuristic.analyze(&heights, tx_height);

        assert!(result.predicted_index.is_some());
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_output_reuse_heuristic() {
        let heuristic = OutputReuseHeuristic::new(5);

        // Reuse counts: output at index 3 has lowest reuse
        let reuse_counts = vec![10, 15, 8, 2, 12];

        let result = heuristic.analyze(&reuse_counts);

        assert_eq!(result.predicted_index, Some(3)); // Lowest reuse
    }

    #[test]
    fn test_combined_heuristic() {
        let combined = CombinedHeuristic::default();

        let results = vec![
            HeuristicResult {
                heuristic_name: "Test1".to_string(),
                predicted_index: Some(2),
                confidence: 0.8,
                member_scores: vec![0.1, 0.2, 0.9, 0.3, 0.1],
                explanation: "Test".to_string(),
            },
            HeuristicResult {
                heuristic_name: "Test2".to_string(),
                predicted_index: Some(2),
                confidence: 0.6,
                member_scores: vec![0.2, 0.1, 0.8, 0.2, 0.1],
                explanation: "Test".to_string(),
            },
        ];

        let result = combined.combine(&results);

        assert_eq!(result.predicted_index, Some(2));
        assert!(result.confidence > 0.5);
    }
}
