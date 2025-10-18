//! Wallet integration using phosphoros-bip39
//!
//! Provides real mnemonic import and address generation.

use phosphoros_bip39::{Mnemonic, WordlistLanguage, DerivationPath, MasterKey, CurveType};
use phosphoros_bip39::multichain::{Blockchain, MultichainWallet};
use crate::panels::{SeedInfo, AddressInfo};
use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

/// Wallet integration error
#[derive(Debug, Error)]
pub enum WalletError {
    /// Invalid mnemonic
    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),
    
    /// Derivation error
    #[error("Derivation error: {0}")]
    DerivationError(String),
    
    /// Address generation error
    #[error("Address generation error: {0}")]
    AddressError(String),
}

/// Wallet integration
pub struct WalletIntegration;

impl WalletIntegration {
    /// Import mnemonic and generate addresses
    pub fn import_mnemonic(phrase: &str) -> Result<SeedInfo, WalletError> {
        // Parse mnemonic
        let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English)
            .map_err(|e| WalletError::InvalidMnemonic(e.to_string()))?;
        
        // Generate seed
        let seed = mnemonic.to_seed(None);
        
        // Generate addresses for multiple chains
        let mut addresses = Vec::new();
        
        // Bitcoin addresses
        if let Ok(btc_addr) = Self::generate_address(&seed, Blockchain::Bitcoin, 0) {
            addresses.push(btc_addr);
        }
        
        // Ethereum addresses
        if let Ok(eth_addr) = Self::generate_address(&seed, Blockchain::Ethereum, 0) {
            addresses.push(eth_addr);
        }
        
        // Create masked mnemonic
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let masked = if words.len() >= 12 {
            format!("{} {} {} ... {} {} {}", 
                words[0], words[1], words[2],
                words[words.len()-3], words[words.len()-2], words[words.len()-1])
        } else {
            "***".to_string()
        };
        
        Ok(SeedInfo {
            id: Uuid::new_v4().to_string(),
            mnemonic_masked: masked,
            addresses,
            imported_at: Utc::now(),
        })
    }
    
    /// Generate address for a specific blockchain
    fn generate_address(seed: &[u8], blockchain: Blockchain, account: u32) -> Result<AddressInfo, WalletError> {
        let curve = match blockchain {
            Blockchain::Ethereum => CurveType::Secp256k1,
            Blockchain::Bitcoin => CurveType::Secp256k1,
            _ => CurveType::Secp256k1,
        };
        
        let master = MasterKey::from_seed(seed.to_vec(), curve);
        
        let path = match blockchain {
            Blockchain::Bitcoin => DerivationPath::bip44(0, account, 0, 0),
            Blockchain::Ethereum => DerivationPath::bip44(60, account, 0, 0),
            _ => DerivationPath::bip44(0, account, 0, 0),
        };
        
        let key = master.derive(&path)
            .map_err(|e| WalletError::DerivationError(e.to_string()))?;
        
        let wallet = MultichainWallet::new(key, blockchain);
        let multichain_addr = wallet.address()
            .map_err(|e| WalletError::AddressError(e.to_string()))?;
        
        Ok(AddressInfo {
            address: multichain_addr.address,
            chain: format!("{:?}", blockchain),
            path: format!("{:?}", path),
        })
    }
    
    /// Validate mnemonic phrase
    pub fn validate_mnemonic(phrase: &str) -> bool {
        Mnemonic::from_phrase(phrase, WordlistLanguage::English).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_import_valid_mnemonic() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = WalletIntegration::import_mnemonic(phrase);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.addresses.is_empty());
    }
    
    #[test]
    fn test_validate_mnemonic() {
        let valid = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        assert!(WalletIntegration::validate_mnemonic(valid));
        
        let invalid = "not a valid mnemonic phrase";
        assert!(!WalletIntegration::validate_mnemonic(invalid));
    }
}
