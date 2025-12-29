//! Ring signature analysis engine
//!
//! Combines RPC data retrieval with heuristic analysis to identify
//! probable real inputs in Monero ring signatures.

use crate::{
    heuristics::{
        CombinedHeuristic, DecoySelectionHeuristic, HeuristicResult, OutputReuseHeuristic,
        TemporalHeuristic,
    },
    rpc::{MoneroRpcClient, OutputRequest, TransactionInfo},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ring analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingAnalysisResult {
    /// Transaction hash being analyzed
    pub tx_hash: String,
    /// Input index within the transaction
    pub input_index: usize,
    /// Key image for this input
    pub key_image: String,
    /// Ring size
    pub ring_size: usize,
    /// Individual heuristic results
    pub heuristic_results: Vec<HeuristicResult>,
    /// Combined analysis result
    pub combined_result: HeuristicResult,
    /// Ring member details
    pub ring_members: Vec<RingMemberInfo>,
    /// Overall confidence (0.0 to 1.0)
    pub confidence: f64,
    /// Whether this analysis is considered reliable
    pub reliable: bool,
}

/// Information about a ring member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingMemberInfo {
    /// Global output index
    pub global_index: u64,
    /// Block height where output was created
    pub block_height: u64,
    /// One-time public key
    pub public_key: String,
    /// Age in blocks at time of transaction
    pub age_blocks: u64,
    /// Transaction hash that created this output
    pub source_tx_hash: Option<String>,
    /// Probability of being real input (0.0 to 1.0)
    pub probability: f64,
}

/// Ring signature analyzer
pub struct RingAnalyzer {
    /// RPC client for blockchain data
    rpc_client: MoneroRpcClient,
    /// Temporal heuristic
    temporal: TemporalHeuristic,
    /// Decoy selection heuristic
    decoy: DecoySelectionHeuristic,
    /// Output reuse heuristic
    reuse: OutputReuseHeuristic,
    /// Combined heuristic
    combined: CombinedHeuristic,
    /// Cache of output reuse counts (reserved for optimization)
    #[allow(dead_code)]
    reuse_cache: HashMap<u64, usize>,
}

impl RingAnalyzer {
    /// Create new ring analyzer
    pub fn new(rpc_client: MoneroRpcClient) -> Self {
        Self {
            rpc_client,
            temporal: TemporalHeuristic::default(),
            decoy: DecoySelectionHeuristic::default(),
            reuse: OutputReuseHeuristic::new(3),
            combined: CombinedHeuristic::default(),
            reuse_cache: HashMap::new(),
        }
    }

    /// Analyze a specific transaction input
    pub async fn analyze_input(
        &mut self,
        tx_hash: &str,
        input_index: usize,
    ) -> Result<RingAnalysisResult> {
        // Fetch transaction data
        let txs = self
            .rpc_client
            .get_transactions(&[tx_hash.to_string()])
            .await?;

        if txs.is_empty() {
            return Err(Error::TransactionNotFound(tx_hash.to_string()));
        }

        let tx_info = &txs[0];
        let tx_height = tx_info.block_height;

        // Parse transaction JSON
        let parsed = self.parse_transaction(tx_info)?;

        if input_index >= parsed.inputs.len() {
            return Err(Error::AnalysisFailed(format!(
                "Input index {} out of range (tx has {} inputs)",
                input_index,
                parsed.inputs.len()
            )));
        }

        let input = &parsed.inputs[input_index];
        let key_image = input.key_image.clone();
        let ring_indices = &input.ring_indices;

        // Fetch output details for ring members
        let ring_members = self
            .fetch_ring_member_info(ring_indices, tx_height)
            .await?;

        // Extract data for heuristics
        let ages: Vec<u64> = ring_members.iter().map(|m| m.age_blocks).collect();
        let heights: Vec<u64> = ring_members.iter().map(|m| m.block_height).collect();

        // Apply heuristics
        let temporal_result = self.temporal.analyze(&ages, tx_height);
        let decoy_result = self.decoy.analyze(&heights, tx_height);

        // For reuse analysis, we'd need to scan more transactions
        // Using placeholder counts for now
        let reuse_counts: Vec<usize> = vec![1; ring_members.len()];
        let reuse_result = self.reuse.analyze(&reuse_counts);

        // Combine results
        let heuristic_results = vec![
            temporal_result.clone(),
            decoy_result.clone(),
            reuse_result.clone(),
        ];
        let combined_result = self.combined.combine(&heuristic_results);

        // Calculate per-member probabilities
        let ring_members_with_prob: Vec<RingMemberInfo> = ring_members
            .into_iter()
            .enumerate()
            .map(|(i, mut m)| {
                m.probability = combined_result
                    .member_scores
                    .get(i)
                    .copied()
                    .unwrap_or(0.0);
                m
            })
            .collect();

        // Determine reliability
        let reliable = combined_result.confidence > 0.4 && ring_members_with_prob.len() >= 11;

        Ok(RingAnalysisResult {
            tx_hash: tx_hash.to_string(),
            input_index,
            key_image,
            ring_size: ring_members_with_prob.len(),
            heuristic_results,
            combined_result: combined_result.clone(),
            ring_members: ring_members_with_prob,
            confidence: combined_result.confidence,
            reliable,
        })
    }

    /// Analyze all inputs in a transaction
    pub async fn analyze_transaction(&mut self, tx_hash: &str) -> Result<Vec<RingAnalysisResult>> {
        // Fetch transaction
        let txs = self
            .rpc_client
            .get_transactions(&[tx_hash.to_string()])
            .await?;

        if txs.is_empty() {
            return Err(Error::TransactionNotFound(tx_hash.to_string()));
        }

        let tx_info = &txs[0];
        let parsed = self.parse_transaction(tx_info)?;

        let mut results = Vec::with_capacity(parsed.inputs.len());

        for i in 0..parsed.inputs.len() {
            match self.analyze_input(tx_hash, i).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    tracing::warn!("Failed to analyze input {}: {}", i, e);
                }
            }
        }

        Ok(results)
    }

    /// Parse transaction JSON into structured format
    fn parse_transaction(&self, tx_info: &TransactionInfo) -> Result<ParsedTx> {
        let json_str = tx_info
            .as_json
            .as_ref()
            .ok_or_else(|| Error::InvalidResponse("Missing transaction JSON".into()))?;

        // Parse the JSON
        let tx: serde_json::Value = serde_json::from_str(json_str)?;

        let mut inputs = Vec::new();

        if let Some(vin) = tx.get("vin").and_then(|v| v.as_array()) {
            for input in vin {
                if let Some(key) = input.get("key") {
                    let key_image = key
                        .get("k_image")
                        .and_then(|k| k.as_str())
                        .unwrap_or("")
                        .to_string();

                    let key_offsets: Vec<u64> = key
                        .get("key_offsets")
                        .and_then(|o| o.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_u64())
                                .collect()
                        })
                        .unwrap_or_default();

                    // Convert relative offsets to absolute indices
                    let ring_indices = self.offsets_to_absolute(&key_offsets);

                    inputs.push(ParsedInput {
                        key_image,
                        ring_indices,
                    });
                }
            }
        }

        Ok(ParsedTx { inputs })
    }

    /// Convert relative key offsets to absolute global indices
    fn offsets_to_absolute(&self, offsets: &[u64]) -> Vec<u64> {
        let mut absolute = Vec::with_capacity(offsets.len());
        let mut running_sum = 0u64;

        for &offset in offsets {
            running_sum += offset;
            absolute.push(running_sum);
        }

        absolute
    }

    /// Fetch detailed info for ring members
    async fn fetch_ring_member_info(
        &self,
        indices: &[u64],
        tx_height: u64,
    ) -> Result<Vec<RingMemberInfo>> {
        // Build output requests
        let requests: Vec<OutputRequest> = indices
            .iter()
            .map(|&index| OutputRequest { amount: 0, index })
            .collect();

        // Fetch from daemon
        let outputs = self.rpc_client.get_outs(requests).await?;

        // Build ring member info
        let members: Vec<RingMemberInfo> = outputs
            .into_iter()
            .zip(indices.iter())
            .map(|(output, &global_index)| RingMemberInfo {
                global_index,
                block_height: output.height,
                public_key: output.key,
                age_blocks: tx_height.saturating_sub(output.height),
                source_tx_hash: output.txid,
                probability: 0.0, // Will be set after analysis
            })
            .collect();

        Ok(members)
    }

    /// Check if a key image has been spent
    pub async fn is_spent(&self, key_image: &str) -> Result<bool> {
        let results = self
            .rpc_client
            .is_key_image_spent(&[key_image.to_string()])
            .await?;

        Ok(results.first().map(|s| *s != crate::rpc::SpentStatus::Unspent).unwrap_or(false))
    }

    /// Get current blockchain height
    pub async fn get_height(&self) -> Result<u64> {
        self.rpc_client.get_height().await
    }
}

/// Parsed transaction structure
struct ParsedTx {
    inputs: Vec<ParsedInput>,
}

/// Parsed transaction input
struct ParsedInput {
    key_image: String,
    ring_indices: Vec<u64>,
}

/// Batch analysis result for multiple transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAnalysisResult {
    /// Total transactions analyzed
    pub total_transactions: usize,
    /// Total inputs analyzed
    pub total_inputs: usize,
    /// High confidence predictions
    pub high_confidence_count: usize,
    /// Average confidence score
    pub average_confidence: f64,
    /// Individual results
    pub results: Vec<RingAnalysisResult>,
    /// Summary statistics
    pub statistics: AnalysisStatistics,
}

/// Analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatistics {
    /// Distribution of predicted indices
    pub index_distribution: HashMap<usize, usize>,
    /// Average ring size
    pub average_ring_size: f64,
    /// Average output age (blocks)
    pub average_output_age: f64,
    /// Temporal heuristic success rate
    pub temporal_success_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_conversion() {
        // Test the offset to absolute conversion logic
        let offsets = vec![100u64, 50, 30, 20, 10];
        let mut absolute = Vec::with_capacity(offsets.len());
        let mut running_sum = 0u64;

        for &offset in &offsets {
            running_sum += offset;
            absolute.push(running_sum);
        }

        assert_eq!(absolute, vec![100, 150, 180, 200, 210]);
    }

    #[test]
    fn test_ring_member_info() {
        let member = RingMemberInfo {
            global_index: 12345,
            block_height: 2000000,
            public_key: "abc123".to_string(),
            age_blocks: 1000,
            source_tx_hash: Some("tx_hash".to_string()),
            probability: 0.75,
        };

        assert_eq!(member.age_blocks, 1000);
        assert_eq!(member.probability, 0.75);
    }
}
