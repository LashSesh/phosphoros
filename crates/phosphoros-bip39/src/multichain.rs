//! Multichain address generation (simplified)

use crate::{derivation::DerivedKey, Error, Result};

/// Supported blockchains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blockchain {
    /// Bitcoin
    Bitcoin,
    /// Ethereum
    Ethereum,
    /// Substrate (Polkadot, Kusama, etc.)
    Substrate,
    /// Cosmos
    Cosmos,
    /// Solana
    Solana,
    /// Cardano
    Cardano,
    /// Monero
    Monero,
}

impl Blockchain {
    /// Get blockchain name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bitcoin => "Bitcoin",
            Self::Ethereum => "Ethereum",
            Self::Substrate => "Substrate",
            Self::Cosmos => "Cosmos",
            Self::Solana => "Solana",
            Self::Cardano => "Cardano",
            Self::Monero => "Monero",
        }
    }

    /// Get coin type for BIP44
    pub fn coin_type(&self) -> u32 {
        match self {
            Self::Bitcoin => 0,
            Self::Ethereum => 60,
            Self::Cosmos => 118,
            Self::Solana => 501,
            Self::Cardano => 1815,
            Self::Substrate => 354, // Polkadot
            Self::Monero => 128,
        }
    }
}

/// Multichain wallet
pub struct MultichainWallet {
    /// Derived key
    pub key: DerivedKey,
    /// Blockchain
    pub blockchain: Blockchain,
}

impl MultichainWallet {
    /// Create wallet from derived key
    pub fn new(key: DerivedKey, blockchain: Blockchain) -> Self {
        Self { key, blockchain }
    }

    /// Generate address for this blockchain
    pub fn address(&self) -> Result<MultichainAddress> {
        MultichainAddress::from_key(&self.key, self.blockchain)
    }
}

/// Multichain address
pub struct MultichainAddress {
    /// Address string
    pub address: String,
    /// Public key hex
    pub pubkey_hex: String,
    /// Blockchain
    pub blockchain: Blockchain,
}

impl MultichainAddress {
    /// Generate address from derived key
    pub fn from_key(key: &DerivedKey, blockchain: Blockchain) -> Result<Self> {
        // Simplified implementation - in production would use proper address encoding
        let address = match blockchain {
            Blockchain::Bitcoin => format!("1{}", &key.public_key_hex()[0..33]),
            Blockchain::Ethereum => format!("0x{}", &key.public_key_hex()[0..40]),
            Blockchain::Substrate => format!("5{}", &key.public_key_hex()[0..47]),
            Blockchain::Cosmos => format!("cosmos1{}", &key.public_key_hex()[0..39]),
            Blockchain::Solana => key.public_key_hex()[0..44].to_string(),
            Blockchain::Cardano => format!("addr1{}", &key.public_key_hex()[0..53]),
            Blockchain::Monero => format!("4{}", &key.public_key_hex()[0..94]),
        };

        Ok(Self {
            address,
            pubkey_hex: key.public_key_hex(),
            blockchain,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::{CurveType, DerivedKey};

    fn mock_key() -> DerivedKey {
        DerivedKey {
            private_key: vec![1u8; 32],
            public_key: vec![2u8; 33],
            chain_code: vec![3u8; 32],
            path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::Secp256k1,
        }
    }

    #[test]
    fn test_blockchain_names() {
        assert_eq!(Blockchain::Bitcoin.name(), "Bitcoin");
        assert_eq!(Blockchain::Ethereum.name(), "Ethereum");
    }

    #[test]
    fn test_coin_types() {
        assert_eq!(Blockchain::Bitcoin.coin_type(), 0);
        assert_eq!(Blockchain::Ethereum.coin_type(), 60);
    }

    #[test]
    fn test_address_generation() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Bitcoin);
        assert!(addr.is_ok());
        let addr = addr.unwrap();
        assert!(addr.address.starts_with('1'));
    }
}
