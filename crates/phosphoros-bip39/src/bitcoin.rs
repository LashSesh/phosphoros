//! Bitcoin address generation
//!
//! Implements proper Bitcoin address derivation with support for:
//! - P2PKH (Legacy addresses starting with '1')
//! - P2SH-P2WPKH (Wrapped SegWit addresses starting with '3')
//! - P2WPKH (Native SegWit addresses starting with 'bc1q')
//! - P2TR (Taproot addresses starting with 'bc1p')

use bitcoin::hashes::{ripemd160, sha256, Hash};
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::{Address, Network, PrivateKey, PublicKey};

use crate::{derivation::DerivedKey, Error, Result};

/// Bitcoin address type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinAddressType {
    /// P2PKH - Pay to Public Key Hash (legacy, starts with '1')
    P2PKH,
    /// P2SH-P2WPKH - Wrapped SegWit (starts with '3')
    P2SHP2WPKH,
    /// P2WPKH - Native SegWit (starts with 'bc1q')
    P2WPKH,
    /// P2TR - Taproot (starts with 'bc1p')
    P2TR,
}

impl BitcoinAddressType {
    /// Get the expected address prefix for mainnet
    pub fn expected_prefix(&self) -> &'static str {
        match self {
            Self::P2PKH => "1",
            Self::P2SHP2WPKH => "3",
            Self::P2WPKH => "bc1q",
            Self::P2TR => "bc1p",
        }
    }
}

/// Bitcoin network type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinNetwork {
    /// Bitcoin Mainnet
    Mainnet,
    /// Bitcoin Testnet
    Testnet,
    /// Bitcoin Regtest
    Regtest,
}

impl From<BitcoinNetwork> for Network {
    fn from(net: BitcoinNetwork) -> Self {
        match net {
            BitcoinNetwork::Mainnet => Network::Bitcoin,
            BitcoinNetwork::Testnet => Network::Testnet,
            BitcoinNetwork::Regtest => Network::Regtest,
        }
    }
}

/// Bitcoin address with metadata
#[derive(Debug, Clone)]
pub struct BitcoinAddress {
    /// The address string
    pub address: String,
    /// Address type
    pub address_type: BitcoinAddressType,
    /// Network
    pub network: BitcoinNetwork,
    /// Compressed public key (33 bytes, hex)
    pub public_key_compressed: String,
    /// Public key hash (HASH160 = RIPEMD160(SHA256(pubkey)))
    pub pubkey_hash: String,
}

/// Generate Bitcoin address from derived key
///
/// # Arguments
/// * `key` - The derived key containing the private key
/// * `address_type` - Type of address to generate
/// * `network` - Bitcoin network (mainnet, testnet, regtest)
///
/// # Returns
/// Bitcoin address with full metadata
pub fn generate_address(
    key: &DerivedKey,
    address_type: BitcoinAddressType,
    network: BitcoinNetwork,
) -> Result<BitcoinAddress> {
    let secp = Secp256k1::new();

    // Parse private key
    if key.private_key.len() != 32 {
        return Err(Error::InvalidKey(format!(
            "Invalid private key length: expected 32, got {}",
            key.private_key.len()
        )));
    }

    let secret_key = SecretKey::from_slice(&key.private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid secp256k1 private key: {}", e)))?;

    let private_key = PrivateKey::new(secret_key, network.into());
    let public_key = PublicKey::from_private_key(&secp, &private_key);

    // Get compressed public key (33 bytes)
    let compressed_pubkey = public_key.inner.serialize();

    // Calculate HASH160 (RIPEMD160(SHA256(pubkey)))
    let sha256_hash = sha256::Hash::hash(&compressed_pubkey);
    let pubkey_hash = ripemd160::Hash::hash(sha256_hash.as_ref());

    // Generate address based on type
    let address = match address_type {
        BitcoinAddressType::P2PKH => {
            // Legacy P2PKH address
            Address::p2pkh(&public_key, network.into())
        }
        BitcoinAddressType::P2SHP2WPKH => {
            // P2SH-wrapped P2WPKH (nested SegWit)
            Address::p2shwpkh(&public_key, network.into())
                .map_err(|e| Error::InvalidKey(format!("P2SH-P2WPKH generation failed: {}", e)))?
        }
        BitcoinAddressType::P2WPKH => {
            // Native SegWit P2WPKH
            Address::p2wpkh(&public_key, network.into())
                .map_err(|e| Error::InvalidKey(format!("P2WPKH generation failed: {}", e)))?
        }
        BitcoinAddressType::P2TR => {
            // Taproot P2TR (key-path spend only)
            let internal_key = public_key.inner.x_only_public_key().0;
            // Create taproot address from internal key (no script tree)
            Address::p2tr(&secp, internal_key, None, network.into())
        }
    };

    Ok(BitcoinAddress {
        address: address.to_string(),
        address_type,
        network,
        public_key_compressed: hex::encode(compressed_pubkey),
        pubkey_hash: hex::encode(pubkey_hash.as_byte_array()),
    })
}

/// Generate all Bitcoin address types for a key
pub fn generate_all_addresses(
    key: &DerivedKey,
    network: BitcoinNetwork,
) -> Result<Vec<BitcoinAddress>> {
    let types = [
        BitcoinAddressType::P2PKH,
        BitcoinAddressType::P2SHP2WPKH,
        BitcoinAddressType::P2WPKH,
        BitcoinAddressType::P2TR,
    ];

    let mut addresses = Vec::with_capacity(types.len());
    for addr_type in types {
        addresses.push(generate_address(key, addr_type, network)?);
    }
    Ok(addresses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::CurveType;

    fn test_key() -> DerivedKey {
        // Known test vector: SHA256("test") first 32 bytes as private key
        // This gives us a predictable key for testing
        let private_key = vec![
            0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
            0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15,
            0xb0, 0xf0, 0x0a, 0x08,
        ];
        DerivedKey {
            private_key,
            public_key: vec![], // Will be computed
            chain_code: vec![0u8; 32],
            path: "m/44'/0'/0'/0/0".to_string(),
            curve: CurveType::Secp256k1,
        }
    }

    #[test]
    fn test_p2pkh_address() {
        let key = test_key();
        let result = generate_address(&key, BitcoinAddressType::P2PKH, BitcoinNetwork::Mainnet);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert!(
            addr.address.starts_with('1'),
            "P2PKH should start with '1': {}",
            addr.address
        );
    }

    #[test]
    fn test_p2sh_p2wpkh_address() {
        let key = test_key();
        let result =
            generate_address(&key, BitcoinAddressType::P2SHP2WPKH, BitcoinNetwork::Mainnet);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert!(
            addr.address.starts_with('3'),
            "P2SH-P2WPKH should start with '3': {}",
            addr.address
        );
    }

    #[test]
    fn test_p2wpkh_address() {
        let key = test_key();
        let result = generate_address(&key, BitcoinAddressType::P2WPKH, BitcoinNetwork::Mainnet);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert!(
            addr.address.starts_with("bc1q"),
            "P2WPKH should start with 'bc1q': {}",
            addr.address
        );
    }

    #[test]
    fn test_p2tr_address() {
        let key = test_key();
        let result = generate_address(&key, BitcoinAddressType::P2TR, BitcoinNetwork::Mainnet);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert!(
            addr.address.starts_with("bc1p"),
            "P2TR should start with 'bc1p': {}",
            addr.address
        );
    }

    #[test]
    fn test_all_addresses() {
        let key = test_key();
        let result = generate_all_addresses(&key, BitcoinNetwork::Mainnet);
        assert!(result.is_ok());
        let addrs = result.unwrap();
        assert_eq!(addrs.len(), 4);

        // All should have same pubkey hash (except P2TR which uses different derivation)
        assert_eq!(addrs[0].pubkey_hash, addrs[1].pubkey_hash);
        assert_eq!(addrs[1].pubkey_hash, addrs[2].pubkey_hash);
    }

    #[test]
    fn test_testnet_addresses() {
        let key = test_key();
        let result = generate_address(&key, BitcoinAddressType::P2PKH, BitcoinNetwork::Testnet);
        assert!(result.is_ok());
        let addr = result.unwrap();
        // Testnet P2PKH starts with 'm' or 'n'
        assert!(
            addr.address.starts_with('m') || addr.address.starts_with('n'),
            "Testnet P2PKH should start with 'm' or 'n': {}",
            addr.address
        );
    }
}
