//! Ethereum/EVM address generation
//!
//! Implements proper Ethereum address derivation using:
//! - secp256k1 elliptic curve (via k256)
//! - Keccak-256 hashing
//! - EIP-55 checksum (mixed-case encoding)

use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};

use crate::{derivation::DerivedKey, Error, Result};

/// Ethereum address with metadata
#[derive(Debug, Clone)]
pub struct EthereumAddress {
    /// The address string (with 0x prefix, EIP-55 checksummed)
    pub address: String,
    /// The address string (lowercase, no checksum)
    pub address_lowercase: String,
    /// Uncompressed public key (65 bytes with 04 prefix, hex)
    pub public_key_uncompressed: String,
    /// Compressed public key (33 bytes, hex)
    pub public_key_compressed: String,
    /// Keccak-256 hash of public key (for derivation verification)
    pub pubkey_hash: String,
}

/// Generate Ethereum address from derived key
///
/// # Arguments
/// * `key` - The derived key containing the private key
///
/// # Returns
/// Ethereum address with EIP-55 checksum and full metadata
pub fn generate_address(key: &DerivedKey) -> Result<EthereumAddress> {
    // Parse private key
    if key.private_key.len() != 32 {
        return Err(Error::InvalidKey(format!(
            "Invalid private key length: expected 32, got {}",
            key.private_key.len()
        )));
    }

    // Create signing key from private key bytes
    let signing_key = SigningKey::from_slice(&key.private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid secp256k1 private key: {}", e)))?;

    // Get verifying (public) key
    let verifying_key = signing_key.verifying_key();

    // Get uncompressed public key (65 bytes with 04 prefix)
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_uncompressed = public_key_point.as_bytes();

    // Get compressed public key (33 bytes)
    let public_key_compressed = verifying_key.to_encoded_point(true);

    // For Ethereum address derivation, we need the public key without the 04 prefix
    // Ethereum uses the last 64 bytes (x,y coordinates)
    let pubkey_without_prefix = &public_key_uncompressed[1..]; // Skip 0x04 prefix

    // Keccak-256 hash of public key (without prefix)
    let pubkey_hash = Keccak256::digest(pubkey_without_prefix);

    // Take last 20 bytes for address
    let address_bytes = &pubkey_hash[12..32]; // bytes 12-31 = 20 bytes

    // Create lowercase address
    let address_lowercase = format!("0x{}", hex::encode(address_bytes));

    // Apply EIP-55 checksum
    let address_checksummed = apply_eip55_checksum(address_bytes);

    Ok(EthereumAddress {
        address: address_checksummed,
        address_lowercase,
        public_key_uncompressed: hex::encode(public_key_uncompressed),
        public_key_compressed: hex::encode(public_key_compressed.as_bytes()),
        pubkey_hash: hex::encode(pubkey_hash),
    })
}

/// Apply EIP-55 mixed-case checksum to Ethereum address
///
/// Uppercase letters are used when the corresponding hex digit
/// in the Keccak-256 hash of the lowercase address is >= 8
fn apply_eip55_checksum(address_bytes: &[u8]) -> String {
    let address_hex = hex::encode(address_bytes);

    // Hash the lowercase address (without 0x)
    let hash = Keccak256::digest(address_hex.as_bytes());

    let mut checksummed = String::with_capacity(42);
    checksummed.push_str("0x");

    for (i, c) in address_hex.chars().enumerate() {
        // Get the nibble from the hash at position i
        let hash_byte = hash[i / 2];
        let hash_nibble = if i % 2 == 0 {
            hash_byte >> 4
        } else {
            hash_byte & 0x0f
        };

        // If hash nibble >= 8, uppercase; otherwise lowercase
        if hash_nibble >= 8 && c.is_ascii_alphabetic() {
            checksummed.push(c.to_ascii_uppercase());
        } else {
            checksummed.push(c);
        }
    }

    checksummed
}

/// Validate EIP-55 checksum for an Ethereum address
///
/// Per EIP-55, addresses that are all lowercase or all uppercase are valid
/// (no checksum applied). Mixed-case addresses must have the correct checksum.
pub fn validate_checksum(address: &str) -> bool {
    let address = address.strip_prefix("0x").unwrap_or(address);

    if address.len() != 40 {
        return false;
    }

    // Check if hex is valid
    let bytes = match hex::decode(address.to_lowercase()) {
        Ok(b) => b,
        Err(_) => return false,
    };

    // All lowercase or all uppercase - valid (no checksum applied)
    if address == address.to_lowercase() || address == address.to_uppercase() {
        return true;
    }

    // Mixed case - verify checksum
    let expected = apply_eip55_checksum(&bytes);
    let expected_without_prefix = expected.strip_prefix("0x").unwrap_or(&expected);

    address == expected_without_prefix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::CurveType;

    fn test_key() -> DerivedKey {
        // Known test vector
        let private_key = vec![
            0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
            0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15,
            0xb0, 0xf0, 0x0a, 0x08,
        ];
        DerivedKey {
            private_key,
            public_key: vec![],
            chain_code: vec![0u8; 32],
            path: "m/44'/60'/0'/0/0".to_string(),
            curve: CurveType::Secp256k1,
        }
    }

    #[test]
    fn test_ethereum_address_generation() {
        let key = test_key();
        let result = generate_address(&key);
        assert!(result.is_ok());
        let addr = result.unwrap();

        // Check address format
        assert!(addr.address.starts_with("0x"), "Should have 0x prefix");
        assert_eq!(
            addr.address.len(),
            42,
            "Address should be 42 chars (0x + 40 hex)"
        );
        assert!(addr.address_lowercase.starts_with("0x"));
        assert_eq!(addr.address_lowercase.len(), 42);

        // Check public key formats
        assert_eq!(
            addr.public_key_uncompressed.len(),
            130,
            "Uncompressed pubkey: 65 bytes = 130 hex"
        );
        assert!(
            addr.public_key_uncompressed.starts_with("04"),
            "Uncompressed pubkey should start with 04"
        );
        assert_eq!(
            addr.public_key_compressed.len(),
            66,
            "Compressed pubkey: 33 bytes = 66 hex"
        );
    }

    #[test]
    fn test_eip55_checksum() {
        // Known EIP-55 test vectors
        let test_cases = [
            "5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "fB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
            "dbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
            "D1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
        ];

        for addr in test_cases {
            let bytes = hex::decode(addr.to_lowercase()).unwrap();
            let checksummed = apply_eip55_checksum(&bytes);
            assert_eq!(
                checksummed,
                format!("0x{}", addr),
                "Checksum mismatch for {}",
                addr
            );
        }
    }

    #[test]
    fn test_validate_checksum() {
        // Valid checksummed addresses
        assert!(validate_checksum(
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        ));
        assert!(validate_checksum(
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359"
        ));

        // Invalid (wrong case)
        assert!(!validate_checksum(
            "0x5AAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        )); // 'A' should be 'a'

        // All lowercase is valid (no checksum)
        assert!(validate_checksum(
            "0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed"
        ));
    }

    #[test]
    fn test_consistent_address_derivation() {
        let key = test_key();
        let addr1 = generate_address(&key).unwrap();
        let addr2 = generate_address(&key).unwrap();

        // Same key should always produce same address
        assert_eq!(addr1.address, addr2.address);
        assert_eq!(addr1.address_lowercase, addr2.address_lowercase);
    }
}
