//! Monero daemon RPC client
//!
//! Provides access to Monero blockchain data through the daemon's JSON-RPC interface.
//! Supports both public and restricted RPC endpoints.

use crate::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Monero RPC client configuration
#[derive(Debug, Clone)]
pub struct MoneroRpcConfig {
    /// Daemon RPC URL (e.g., "http://127.0.0.1:18081")
    pub daemon_url: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Optional authentication (username:password)
    pub auth: Option<(String, String)>,
}

impl Default for MoneroRpcConfig {
    fn default() -> Self {
        Self {
            daemon_url: "http://127.0.0.1:18081".to_string(),
            timeout_secs: 30,
            auth: None,
        }
    }
}

impl MoneroRpcConfig {
    /// Create config for mainnet public node
    pub fn mainnet_public() -> Self {
        Self {
            daemon_url: "http://node.moneroworld.com:18089".to_string(),
            timeout_secs: 60,
            auth: None,
        }
    }

    /// Create config for stagenet
    pub fn stagenet() -> Self {
        Self {
            daemon_url: "http://127.0.0.1:38081".to_string(),
            timeout_secs: 30,
            auth: None,
        }
    }
}

/// Monero RPC client
pub struct MoneroRpcClient {
    config: MoneroRpcConfig,
    client: Client,
}

impl MoneroRpcClient {
    /// Create new RPC client
    pub fn new(config: MoneroRpcConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| Error::RpcConnection(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Make a JSON-RPC call
    async fn json_rpc<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: "0",
            method,
            params,
        };

        let url = format!("{}/json_rpc", self.config.daemon_url);
        let mut req = self.client.post(&url).json(&request);

        if let Some((user, pass)) = &self.config.auth {
            req = req.basic_auth(user, Some(pass));
        }

        let response: JsonRpcResponse<R> = req
            .send()
            .await
            .map_err(|e| Error::RpcConnection(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        if let Some(error) = response.error {
            return Err(Error::RpcRequest(format!(
                "{}: {}",
                error.code, error.message
            )));
        }

        response
            .result
            .ok_or_else(|| Error::InvalidResponse("Missing result".into()))
    }

    /// Get current blockchain height
    pub async fn get_height(&self) -> Result<u64> {
        #[derive(Deserialize)]
        struct HeightResponse {
            height: u64,
        }

        let response: HeightResponse = self.json_rpc("get_block_count", ()).await?;
        Ok(response.height)
    }

    /// Get block header by height
    pub async fn get_block_header_by_height(&self, height: u64) -> Result<BlockHeader> {
        #[derive(Serialize)]
        struct Params {
            height: u64,
        }

        #[derive(Deserialize)]
        struct Response {
            block_header: BlockHeader,
        }

        let response: Response = self
            .json_rpc("get_block_header_by_height", Params { height })
            .await?;
        Ok(response.block_header)
    }

    /// Get transaction data
    pub async fn get_transactions(&self, tx_hashes: &[String]) -> Result<Vec<TransactionInfo>> {
        #[derive(Serialize)]
        struct Params<'a> {
            txs_hashes: &'a [String],
            decode_as_json: bool,
        }

        let url = format!("{}/get_transactions", self.config.daemon_url);
        let mut req = self.client.post(&url).json(&Params {
            txs_hashes: tx_hashes,
            decode_as_json: true,
        });

        if let Some((user, pass)) = &self.config.auth {
            req = req.basic_auth(user, Some(pass));
        }

        let response: GetTransactionsResponse = req
            .send()
            .await
            .map_err(|e| Error::RpcConnection(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        if response.status != "OK" {
            return Err(Error::RpcRequest(format!(
                "get_transactions failed: {}",
                response.status
            )));
        }

        Ok(response.txs)
    }

    /// Get outputs for given amounts and indices (for ring member lookup)
    pub async fn get_outs(&self, outputs: Vec<OutputRequest>) -> Result<Vec<OutputInfo>> {
        #[derive(Serialize)]
        struct Params {
            outputs: Vec<OutputRequest>,
            get_txid: bool,
        }

        let url = format!("{}/get_outs", self.config.daemon_url);
        let mut req = self.client.post(&url).json(&Params {
            outputs,
            get_txid: true,
        });

        if let Some((user, pass)) = &self.config.auth {
            req = req.basic_auth(user, Some(pass));
        }

        let response: GetOutsResponse = req
            .send()
            .await
            .map_err(|e| Error::RpcConnection(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        if response.status != "OK" {
            return Err(Error::RpcRequest(format!(
                "get_outs failed: {}",
                response.status
            )));
        }

        Ok(response.outs)
    }

    /// Check if key images are spent
    pub async fn is_key_image_spent(&self, key_images: &[String]) -> Result<Vec<SpentStatus>> {
        #[derive(Serialize)]
        struct Params<'a> {
            key_images: &'a [String],
        }

        let url = format!("{}/is_key_image_spent", self.config.daemon_url);
        let mut req = self.client.post(&url).json(&Params { key_images });

        if let Some((user, pass)) = &self.config.auth {
            req = req.basic_auth(user, Some(pass));
        }

        let response: KeyImageSpentResponse = req
            .send()
            .await
            .map_err(|e| Error::RpcConnection(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        if response.status != "OK" {
            return Err(Error::RpcRequest(format!(
                "is_key_image_spent failed: {}",
                response.status
            )));
        }

        Ok(response
            .spent_status
            .into_iter()
            .map(|s| match s {
                0 => SpentStatus::Unspent,
                1 => SpentStatus::SpentInBlockchain,
                2 => SpentStatus::SpentInPool,
                _ => SpentStatus::Unknown,
            })
            .collect())
    }

    /// Get the output histogram for statistical analysis
    pub async fn get_output_histogram(
        &self,
        amounts: &[u64],
        min_count: u64,
        max_count: u64,
    ) -> Result<Vec<HistogramEntry>> {
        #[derive(Serialize)]
        struct Params<'a> {
            amounts: &'a [u64],
            min_count: u64,
            max_count: u64,
            unlocked: bool,
            recent_cutoff: u64,
        }

        #[derive(Deserialize)]
        struct Response {
            histogram: Vec<HistogramEntry>,
        }

        let response: Response = self
            .json_rpc(
                "get_output_histogram",
                Params {
                    amounts,
                    min_count,
                    max_count,
                    unlocked: true,
                    recent_cutoff: 0,
                },
            )
            .await?;

        Ok(response.histogram)
    }
}

// JSON-RPC structures
#[derive(Serialize)]
struct JsonRpcRequest<'a, T> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    params: T,
}

#[derive(Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

/// Block header information
#[derive(Debug, Clone, Deserialize)]
pub struct BlockHeader {
    /// Block height
    pub height: u64,
    /// Block hash
    pub hash: String,
    /// Block timestamp
    pub timestamp: u64,
    /// Number of transactions
    pub num_txes: u64,
    /// Difficulty
    pub difficulty: u64,
    /// Block reward
    pub reward: u64,
}

/// Transaction information from get_transactions
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionInfo {
    /// Transaction hash
    pub tx_hash: String,
    /// Block height (0 if in mempool)
    pub block_height: u64,
    /// Block timestamp
    pub block_timestamp: u64,
    /// Whether transaction is in mempool
    pub in_pool: bool,
    /// Decoded transaction JSON
    pub as_json: Option<String>,
}

/// Parsed transaction data
#[derive(Debug, Clone, Deserialize)]
pub struct ParsedTransaction {
    /// Version
    pub version: u8,
    /// Unlock time
    pub unlock_time: u64,
    /// Transaction inputs
    pub vin: Vec<TxInput>,
    /// Transaction outputs
    pub vout: Vec<TxOutput>,
    /// Extra data
    pub extra: Vec<u8>,
}

/// Transaction input
#[derive(Debug, Clone, Deserialize)]
pub struct TxInput {
    /// Key input data
    pub key: Option<KeyInput>,
}

/// Key input (for non-coinbase transactions)
#[derive(Debug, Clone, Deserialize)]
pub struct KeyInput {
    /// Amount (0 for RingCT)
    pub amount: u64,
    /// Key offsets (ring member indices)
    pub key_offsets: Vec<u64>,
    /// Key image
    pub k_image: String,
}

/// Transaction output
#[derive(Debug, Clone, Deserialize)]
pub struct TxOutput {
    /// Amount (0 for RingCT)
    pub amount: u64,
    /// Output target
    pub target: OutputTarget,
}

/// Output target (destination)
#[derive(Debug, Clone, Deserialize)]
pub struct OutputTarget {
    /// One-time public key
    pub key: Option<String>,
}

/// Output request for get_outs
#[derive(Debug, Clone, Serialize)]
pub struct OutputRequest {
    /// Amount (0 for RingCT)
    pub amount: u64,
    /// Global output index
    pub index: u64,
}

/// Output information from get_outs
#[derive(Debug, Clone, Deserialize)]
pub struct OutputInfo {
    /// One-time public key
    pub key: String,
    /// Whether output is unlocked
    pub unlocked: bool,
    /// Block height
    pub height: u64,
    /// Transaction hash
    pub txid: Option<String>,
}

#[derive(Deserialize)]
struct GetTransactionsResponse {
    status: String,
    #[serde(default)]
    txs: Vec<TransactionInfo>,
}

#[derive(Deserialize)]
struct GetOutsResponse {
    status: String,
    #[serde(default)]
    outs: Vec<OutputInfo>,
}

#[derive(Deserialize)]
struct KeyImageSpentResponse {
    status: String,
    #[serde(default)]
    spent_status: Vec<u8>,
}

/// Key image spent status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpentStatus {
    /// Not spent
    Unspent,
    /// Spent in blockchain
    SpentInBlockchain,
    /// Spent in mempool
    SpentInPool,
    /// Unknown status
    Unknown,
}

/// Output histogram entry
#[derive(Debug, Clone, Deserialize)]
pub struct HistogramEntry {
    /// Amount
    pub amount: u64,
    /// Total outputs
    pub total_instances: u64,
    /// Unlocked outputs
    pub unlocked_instances: u64,
    /// Recent outputs
    pub recent_instances: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = MoneroRpcConfig::default();
        assert_eq!(config.daemon_url, "http://127.0.0.1:18081");
    }

    #[test]
    fn test_config_mainnet() {
        let config = MoneroRpcConfig::mainnet_public();
        assert!(config.daemon_url.contains("moneroworld"));
    }
}
