//! Multichain address generation
//!
//! Supports multiple blockchains with proper cryptographic implementations.
//!
//! ## Feature Flags
//!
//! - `monero` - Real Ed25519 derivation for Monero addresses
//! - `btc` - Real secp256k1 derivation for Bitcoin addresses (P2PKH, P2SH-P2WPKH, P2WPKH, P2TR)
//! - `evm` - Real secp256k1 derivation for Ethereum/EVM addresses
//! - `cosmos` - Real secp256k1 derivation for Cosmos SDK chains (bech32)

use crate::{derivation::DerivedKey, Error, Result};

#[cfg(feature = "monero")]
use crate::monero::{derive_monero_keys, generate_address, MoneroNetwork};

#[cfg(feature = "btc")]
use crate::bitcoin::{
    generate_address as btc_generate_address, BitcoinAddressType, BitcoinNetwork,
};

#[cfg(feature = "evm")]
use crate::ethereum::generate_address as eth_generate_address;

#[cfg(feature = "cosmos")]
use crate::cosmos::{generate_address as cosmos_generate_address, CosmosChain};

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
        match blockchain {
            // Bitcoin with real cryptography (btc feature)
            #[cfg(feature = "btc")]
            Blockchain::Bitcoin => {
                // Use proper Bitcoin address generation with secp256k1
                let btc_addr =
                    btc_generate_address(key, BitcoinAddressType::P2WPKH, BitcoinNetwork::Mainnet)?;

                Ok(Self {
                    address: btc_addr.address,
                    pubkey_hex: btc_addr.public_key_compressed,
                    blockchain,
                })
            }
            #[cfg(not(feature = "btc"))]
            Blockchain::Bitcoin => {
                // Placeholder when btc feature is not enabled
                Err(Error::UnsupportedBlockchain(
                    "Bitcoin support requires the 'btc' feature flag".into(),
                ))
            }

            // Monero with real cryptography (monero feature)
            #[cfg(feature = "monero")]
            Blockchain::Monero => {
                // Use proper Monero cryptography with Ed25519
                let monero_keys = derive_monero_keys(&key.private_key)?;
                let monero_addr = generate_address(&monero_keys, MoneroNetwork::Mainnet);

                // Combine spend and view public keys for pubkey_hex
                let mut combined_pubkey = Vec::with_capacity(64);
                combined_pubkey.extend_from_slice(&monero_keys.public_spend_key);
                combined_pubkey.extend_from_slice(&monero_keys.public_view_key);

                Ok(Self {
                    address: monero_addr.address,
                    pubkey_hex: hex::encode(combined_pubkey),
                    blockchain,
                })
            }
            #[cfg(not(feature = "monero"))]
            Blockchain::Monero => {
                // Placeholder when monero feature is not enabled
                Err(Error::UnsupportedBlockchain(
                    "Monero support requires the 'monero' feature flag".into(),
                ))
            }

            // Ethereum with real cryptography (evm feature)
            #[cfg(feature = "evm")]
            Blockchain::Ethereum => {
                // Use proper Ethereum address generation with secp256k1 + Keccak-256
                let eth_addr = eth_generate_address(key)?;

                Ok(Self {
                    address: eth_addr.address,
                    pubkey_hex: eth_addr.public_key_compressed,
                    blockchain,
                })
            }
            #[cfg(not(feature = "evm"))]
            Blockchain::Ethereum => {
                // Placeholder when evm feature is not enabled
                Err(Error::UnsupportedBlockchain(
                    "Ethereum support requires the 'evm' feature flag".into(),
                ))
            }

            // Cosmos with real cryptography (cosmos feature)
            #[cfg(feature = "cosmos")]
            Blockchain::Cosmos => {
                // Use proper Cosmos address generation with secp256k1 + bech32
                let cosmos_addr = cosmos_generate_address(key, CosmosChain::cosmos())?;

                Ok(Self {
                    address: cosmos_addr.address,
                    pubkey_hex: cosmos_addr.public_key_compressed,
                    blockchain,
                })
            }
            #[cfg(not(feature = "cosmos"))]
            Blockchain::Cosmos => {
                // Placeholder when cosmos feature is not enabled
                Err(Error::UnsupportedBlockchain(
                    "Cosmos support requires the 'cosmos' feature flag".into(),
                ))
            }

            // Other blockchains - placeholder implementations
            // TODO: Implement with proper cryptographic libraries
            _ => {
                let address = match blockchain {
                    Blockchain::Substrate => format!("5{}", &key.public_key_hex()[0..47]),
                    Blockchain::Solana => key.public_key_hex()[0..44].to_string(),
                    Blockchain::Cardano => format!("addr1{}", &key.public_key_hex()[0..53]),
                    Blockchain::Bitcoin
                    | Blockchain::Monero
                    | Blockchain::Ethereum
                    | Blockchain::Cosmos => {
                        unreachable!()
                    } // Handled above
                };

                Ok(Self {
                    address,
                    pubkey_hex: key.public_key_hex(),
                    blockchain,
                })
            }
        }
    }
}

/// Bitcoin-specific address generation (requires `btc` feature)
#[cfg(feature = "btc")]
impl MultichainAddress {
    /// Generate Bitcoin address with specific type
    ///
    /// # Arguments
    /// * `key` - The derived key
    /// * `address_type` - Type of Bitcoin address (P2PKH, P2SH-P2WPKH, P2WPKH, P2TR)
    /// * `network` - Bitcoin network (Mainnet, Testnet, Regtest)
    pub fn bitcoin_with_type(
        key: &DerivedKey,
        address_type: BitcoinAddressType,
        network: BitcoinNetwork,
    ) -> Result<Self> {
        let btc_addr = btc_generate_address(key, address_type, network)?;

        Ok(Self {
            address: btc_addr.address,
            pubkey_hex: btc_addr.public_key_compressed,
            blockchain: Blockchain::Bitcoin,
        })
    }

    /// Generate all Bitcoin address types for a key
    pub fn all_bitcoin_addresses(
        key: &DerivedKey,
        network: BitcoinNetwork,
    ) -> Result<Vec<Self>> {
        let types = [
            BitcoinAddressType::P2PKH,
            BitcoinAddressType::P2SHP2WPKH,
            BitcoinAddressType::P2WPKH,
            BitcoinAddressType::P2TR,
        ];

        let mut addresses = Vec::with_capacity(types.len());
        for addr_type in types {
            addresses.push(Self::bitcoin_with_type(key, addr_type, network)?);
        }
        Ok(addresses)
    }
}

/// Extended Monero address information (only with monero feature)
#[cfg(feature = "monero")]
#[derive(Debug, Clone)]
pub struct MoneroAddressInfo {
    /// Standard address
    pub address: String,
    /// Public spend key (hex)
    pub public_spend_key: String,
    /// Public view key (hex)
    pub public_view_key: String,
    /// Network type
    pub network: String,
}

#[cfg(feature = "monero")]
impl MoneroAddressInfo {
    /// Generate Monero address info from seed
    pub fn from_seed(seed: &[u8], network: MoneroNetwork) -> Result<Self> {
        let keys = derive_monero_keys(seed)?;
        let addr = generate_address(&keys, network);

        Ok(Self {
            address: addr.address,
            public_spend_key: hex::encode(keys.public_spend_key),
            public_view_key: hex::encode(keys.public_view_key),
            network: format!("{:?}", network),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::{CurveType, DerivedKey};

    fn mock_key() -> DerivedKey {
        // Use a valid secp256k1 private key for tests
        DerivedKey {
            private_key: vec![
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
                0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15,
                0xb0, 0xf0, 0x0a, 0x08,
            ],
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

    #[cfg(feature = "btc")]
    #[test]
    fn test_bitcoin_address_generation() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Bitcoin);
        assert!(addr.is_ok());
        let addr = addr.unwrap();
        // Default is P2WPKH (native SegWit)
        assert!(
            addr.address.starts_with("bc1q"),
            "Expected bc1q prefix: {}",
            addr.address
        );
    }

    #[cfg(feature = "btc")]
    #[test]
    fn test_all_bitcoin_address_types() {
        let key = mock_key();
        let addrs = MultichainAddress::all_bitcoin_addresses(&key, BitcoinNetwork::Mainnet);
        assert!(addrs.is_ok());
        let addrs = addrs.unwrap();
        assert_eq!(addrs.len(), 4);

        // Check each address type
        assert!(addrs[0].address.starts_with('1')); // P2PKH
        assert!(addrs[1].address.starts_with('3')); // P2SH-P2WPKH
        assert!(addrs[2].address.starts_with("bc1q")); // P2WPKH
        assert!(addrs[3].address.starts_with("bc1p")); // P2TR
    }

    #[cfg(not(feature = "btc"))]
    #[test]
    fn test_bitcoin_requires_feature() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Bitcoin);
        assert!(addr.is_err());
    }

    #[cfg(feature = "evm")]
    #[test]
    fn test_ethereum_address_generation() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Ethereum);
        assert!(addr.is_ok());
        let addr = addr.unwrap();
        // Ethereum addresses start with 0x and are 42 chars
        assert!(
            addr.address.starts_with("0x"),
            "Expected 0x prefix: {}",
            addr.address
        );
        assert_eq!(
            addr.address.len(),
            42,
            "Ethereum address should be 42 chars: {}",
            addr.address
        );
    }

    #[cfg(not(feature = "evm"))]
    #[test]
    fn test_ethereum_requires_feature() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Ethereum);
        assert!(addr.is_err());
    }

    #[cfg(feature = "cosmos")]
    #[test]
    fn test_cosmos_address_generation() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Cosmos);
        assert!(addr.is_ok());
        let addr = addr.unwrap();
        // Cosmos addresses start with cosmos1
        assert!(
            addr.address.starts_with("cosmos1"),
            "Expected cosmos1 prefix: {}",
            addr.address
        );
    }

    #[cfg(not(feature = "cosmos"))]
    #[test]
    fn test_cosmos_requires_feature() {
        let key = mock_key();
        let addr = MultichainAddress::from_key(&key, Blockchain::Cosmos);
        assert!(addr.is_err());
    }
}
