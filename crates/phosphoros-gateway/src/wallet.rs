//! Wallet and BIP-39 API
//!
//! Provides endpoints for mnemonic phrase handling, multichain address derivation,
//! and wallet management.

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use phosphoros_bip39::{
    Blockchain, CurveType, DerivationPath, MasterKey, Mnemonic,
    MultichainWallet, WordlistLanguage,
};

/// Shared state for wallet operations
#[derive(Clone)]
pub struct WalletState {
    /// Currently loaded wallets (keyed by label)
    pub wallets: Arc<RwLock<std::collections::HashMap<String, WalletInfo>>>,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            wallets: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
}

/// Information about a loaded wallet
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WalletInfo {
    /// User-provided label
    pub label: String,
    /// Blockchain type
    pub blockchain: String,
    /// Generated addresses
    pub addresses: Vec<String>,
    /// Timestamp when imported
    pub imported_at: String,
}

/// Build the wallet management router
pub fn build_router(state: WalletState) -> Router {
    Router::new()
        .route("/import", post(import_mnemonic))
        .route("/derive", post(derive_addresses))
        .route("/list", get(list_wallets))
        .route("/remove", post(remove_wallet))
        .with_state(state)
}

/// Request to import a mnemonic phrase
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ImportRequest {
    /// Mnemonic phrase (12 or 24 words, space-separated)
    pub phrase: String,
    /// Language of the wordlist (default: English)
    #[serde(default = "default_language")]
    pub language: String,
    /// Label for tracking this wallet
    pub label: String,
}

fn default_language() -> String {
    "english".to_string()
}

/// Response from mnemonic import
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportResponse {
    /// Whether import was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Validated mnemonic (echoed back, without passphrase)
    pub mnemonic: Option<String>,
}

/// Request to derive addresses for multiple chains
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DeriveRequest {
    /// Mnemonic phrase (12 or 24 words, space-separated)
    pub phrase: String,
    /// Blockchains to derive addresses for
    pub blockchains: Vec<String>,
    /// Account index (default: 0)
    #[serde(default)]
    pub account: u32,
    /// Address index range (default: 0..1, i.e., single address)
    #[serde(default = "default_address_range")]
    pub address_range: std::ops::Range<u32>,
    /// Optional passphrase (BIP39 extension)
    #[serde(default)]
    pub passphrase: Option<String>,
    /// Label for storing this wallet
    #[serde(default)]
    pub label: Option<String>,
}

fn default_address_range() -> std::ops::Range<u32> {
    0..1
}

/// Response with derived addresses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeriveResponse {
    /// Whether derivation was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Derived addresses grouped by blockchain
    pub addresses: std::collections::HashMap<String, Vec<String>>,
}

/// List all loaded wallets
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListWalletsResponse {
    /// List of wallet metadata
    pub wallets: Vec<WalletInfo>,
}

/// Request to remove a wallet
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RemoveWalletRequest {
    /// Label of wallet to remove
    pub label: String,
}

/// Import a mnemonic phrase
///
/// Validates the mnemonic and stores it for later use.
#[utoipa::path(
    post,
    path = "/api/v1/wallet/import",
    tag = "wallet",
    request_body = ImportRequest,
    responses(
        (status = 200, description = "Mnemonic imported successfully", body = ImportResponse),
        (status = 400, description = "Invalid mnemonic phrase"),
        (status = 500, description = "Internal server error")
    )
)]
async fn import_mnemonic(
    State(state): State<WalletState>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, StatusCode> {
    // Parse language
    let lang = match req.language.to_lowercase().as_str() {
        "english" => WordlistLanguage::English,
        "chinese_simplified" => WordlistLanguage::ChineseSimplified,
        "chinese_traditional" => WordlistLanguage::ChineseTraditional,
        "french" => WordlistLanguage::French,
        "italian" => WordlistLanguage::Italian,
        "japanese" => WordlistLanguage::Japanese,
        "korean" => WordlistLanguage::Korean,
        "spanish" => WordlistLanguage::Spanish,
        "czech" => WordlistLanguage::Czech,
        "portuguese" => WordlistLanguage::Portuguese,
        _ => {
            return Ok(Json(ImportResponse {
                success: false,
                error: Some(format!("Unsupported language: {}", req.language)),
                mnemonic: None,
            }));
        }
    };

    // Validate mnemonic
    let mnemonic = match Mnemonic::from_phrase(&req.phrase, lang) {
        Ok(m) => m,
        Err(e) => {
            return Ok(Json(ImportResponse {
                success: false,
                error: Some(format!("Invalid mnemonic: {}", e)),
                mnemonic: None,
            }));
        }
    };

    // Store wallet info (we don't actually store the seed for security)
    let wallet_info = WalletInfo {
        label: req.label.clone(),
        blockchain: "multichain".to_string(),
        addresses: vec![],
        imported_at: chrono::Utc::now().to_rfc3339(),
    };

    state.wallets.write().insert(req.label.clone(), wallet_info);

    Ok(Json(ImportResponse {
        success: true,
        error: None,
        mnemonic: Some(mnemonic.phrase().to_string()),
    }))
}

/// Derive addresses for multiple blockchains
///
/// Generates addresses using BIP32/44/49/84 derivation paths.
#[utoipa::path(
    post,
    path = "/api/v1/wallet/derive",
    tag = "wallet",
    request_body = DeriveRequest,
    responses(
        (status = 200, description = "Addresses derived successfully", body = DeriveResponse),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn derive_addresses(
    State(state): State<WalletState>,
    Json(req): Json<DeriveRequest>,
) -> Result<Json<DeriveResponse>, StatusCode> {
    // Validate and parse mnemonic
    let mnemonic = match Mnemonic::from_phrase(&req.phrase, WordlistLanguage::English) {
        Ok(m) => m,
        Err(e) => {
            return Ok(Json(DeriveResponse {
                success: false,
                error: Some(format!("Invalid mnemonic: {}", e)),
                addresses: std::collections::HashMap::new(),
            }));
        }
    };

    // Generate seed
    let seed = mnemonic.to_seed(req.passphrase.as_deref());

    // Derive addresses for each blockchain
    let mut all_addresses = std::collections::HashMap::new();

    for blockchain_str in &req.blockchains {
        let blockchain = match parse_blockchain(blockchain_str) {
            Some(bc) => bc,
            None => {
                return Ok(Json(DeriveResponse {
                    success: false,
                    error: Some(format!("Unsupported blockchain: {}", blockchain_str)),
                    addresses: std::collections::HashMap::new(),
                }));
            }
        };

        let mut addresses = Vec::new();

        for address_index in req.address_range.clone() {
            // Create derivation path
            let path = match blockchain {
                Blockchain::Bitcoin => DerivationPath::bitcoin(req.account, 0, address_index),
                Blockchain::Ethereum => DerivationPath::ethereum(req.account, 0, address_index),
                _ => DerivationPath::bip44(coin_type_for_blockchain(&blockchain), req.account, 0, address_index),
            };

            // Derive key
            let curve = curve_type_for_blockchain(&blockchain);
            let master = MasterKey::from_seed(seed.clone(), curve);
            let derived = match master.derive(&path) {
                Ok(k) => k,
                Err(e) => {
                    return Ok(Json(DeriveResponse {
                        success: false,
                        error: Some(format!("Derivation failed: {}", e)),
                        addresses: std::collections::HashMap::new(),
                    }));
                }
            };

            // Generate address
            let wallet = MultichainWallet::new(derived, blockchain);
            let multichain_addr = match wallet.address() {
                Ok(addr) => addr,
                Err(e) => {
                    return Ok(Json(DeriveResponse {
                        success: false,
                        error: Some(format!("Address generation failed: {}", e)),
                        addresses: std::collections::HashMap::new(),
                    }));
                }
            };
            let address = multichain_addr.address;

            addresses.push(address);
        }

        all_addresses.insert(blockchain_str.clone(), addresses.clone());

        // Store wallet info if label provided
        if let Some(label) = &req.label {
            let wallet_info = WalletInfo {
                label: label.clone(),
                blockchain: blockchain_str.clone(),
                addresses,
                imported_at: chrono::Utc::now().to_rfc3339(),
            };
            state.wallets.write().insert(label.clone(), wallet_info);
        }
    }

    Ok(Json(DeriveResponse {
        success: true,
        error: None,
        addresses: all_addresses,
    }))
}

/// List all stored wallets
#[utoipa::path(
    get,
    path = "/api/v1/wallet/list",
    tag = "wallet",
    responses(
        (status = 200, description = "Wallet list retrieved", body = ListWalletsResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_wallets(
    State(state): State<WalletState>,
) -> Result<Json<ListWalletsResponse>, StatusCode> {
    let wallets = state.wallets.read();
    let wallet_list: Vec<WalletInfo> = wallets.values().cloned().collect();

    Ok(Json(ListWalletsResponse { wallets: wallet_list }))
}

/// Remove a wallet from storage
#[utoipa::path(
    post,
    path = "/api/v1/wallet/remove",
    tag = "wallet",
    request_body = RemoveWalletRequest,
    responses(
        (status = 200, description = "Wallet removed successfully"),
        (status = 404, description = "Wallet not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn remove_wallet(
    State(state): State<WalletState>,
    Json(req): Json<RemoveWalletRequest>,
) -> StatusCode {
    let mut wallets = state.wallets.write();
    match wallets.remove(&req.label) {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}

/// Parse blockchain string to enum
fn parse_blockchain(s: &str) -> Option<Blockchain> {
    match s.to_lowercase().as_str() {
        "bitcoin" | "btc" => Some(Blockchain::Bitcoin),
        "ethereum" | "eth" => Some(Blockchain::Ethereum),
        "polkadot" | "dot" | "kusama" | "ksm" | "substrate" => Some(Blockchain::Substrate),
        "cosmos" | "atom" => Some(Blockchain::Cosmos),
        "solana" | "sol" => Some(Blockchain::Solana),
        "cardano" | "ada" => Some(Blockchain::Cardano),
        "monero" | "xmr" => Some(Blockchain::Monero),
        _ => None,
    }
}

/// Get curve type for blockchain
fn curve_type_for_blockchain(blockchain: &Blockchain) -> CurveType {
    match blockchain {
        Blockchain::Bitcoin | Blockchain::Ethereum => CurveType::Secp256k1,
        Blockchain::Substrate => CurveType::Ed25519,
        Blockchain::Cosmos => CurveType::Secp256k1,
        Blockchain::Solana => CurveType::Ed25519,
        Blockchain::Cardano => CurveType::Ed25519,
        Blockchain::Monero => CurveType::Ed25519,
    }
}

/// Get BIP44 coin type for blockchain
fn coin_type_for_blockchain(blockchain: &Blockchain) -> u32 {
    match blockchain {
        Blockchain::Bitcoin => 0,
        Blockchain::Ethereum => 60,
        Blockchain::Substrate => 354, // Polkadot coin type
        Blockchain::Cosmos => 118,
        Blockchain::Solana => 501,
        Blockchain::Cardano => 1815,
        Blockchain::Monero => 128,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blockchain() {
        assert!(matches!(parse_blockchain("bitcoin"), Some(Blockchain::Bitcoin)));
        assert!(matches!(parse_blockchain("eth"), Some(Blockchain::Ethereum)));
        assert!(matches!(parse_blockchain("polkadot"), Some(Blockchain::Substrate)));
        assert!(matches!(parse_blockchain("unknown"), None));
    }

    #[test]
    fn test_wallet_state_default() {
        let state = WalletState::default();
        assert_eq!(state.wallets.read().len(), 0);
    }
}
