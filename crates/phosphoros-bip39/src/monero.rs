//! Monero cryptographic primitives and address generation
//!
//! Implements proper Monero key derivation and address encoding according to
//! the Monero protocol specification. This is critical for forensic analysis
//! as incorrect key/address handling would invalidate any investigation.
//!
//! Reference: https://github.com/monero-project/monero/blob/master/src/crypto/

#![cfg(feature = "monero")]

use crate::{Error, Result};

#[cfg(feature = "monero")]
use curve25519_dalek::{
    constants::ED25519_BASEPOINT_TABLE,
    scalar::Scalar,
};

#[cfg(feature = "monero")]
use sha3::{Digest, Keccak256};

/// Monero network type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneroNetwork {
    /// Mainnet (prefix 0x12 for standard addresses)
    Mainnet,
    /// Testnet (prefix 0x35)
    Testnet,
    /// Stagenet (prefix 0x18)
    Stagenet,
}

impl MoneroNetwork {
    /// Get the address prefix byte for standard addresses
    pub fn address_prefix(&self) -> u8 {
        match self {
            MoneroNetwork::Mainnet => 0x12,  // 18 decimal - starts with '4'
            MoneroNetwork::Testnet => 0x35,  // 53 decimal - starts with '9'
            MoneroNetwork::Stagenet => 0x18, // 24 decimal - starts with '5'
        }
    }

    /// Get the integrated address prefix
    pub fn integrated_prefix(&self) -> u8 {
        match self {
            MoneroNetwork::Mainnet => 0x13,  // 19 decimal
            MoneroNetwork::Testnet => 0x36,  // 54 decimal
            MoneroNetwork::Stagenet => 0x19, // 25 decimal
        }
    }

    /// Get the subaddress prefix
    pub fn subaddress_prefix(&self) -> u8 {
        match self {
            MoneroNetwork::Mainnet => 0x2A,  // 42 decimal - starts with '8'
            MoneroNetwork::Testnet => 0x3F,  // 63 decimal
            MoneroNetwork::Stagenet => 0x24, // 36 decimal
        }
    }
}

/// Monero key pair (spend and view keys)
#[derive(Clone)]
pub struct MoneroKeys {
    /// Private spend key (32 bytes)
    pub private_spend_key: [u8; 32],
    /// Private view key (32 bytes) - derived from spend key
    pub private_view_key: [u8; 32],
    /// Public spend key (32 bytes compressed Edwards point)
    pub public_spend_key: [u8; 32],
    /// Public view key (32 bytes compressed Edwards point)
    pub public_view_key: [u8; 32],
}

impl std::fmt::Debug for MoneroKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MoneroKeys")
            .field("private_spend_key", &"[REDACTED]")
            .field("private_view_key", &"[REDACTED]")
            .field("public_spend_key", &hex::encode(self.public_spend_key))
            .field("public_view_key", &hex::encode(self.public_view_key))
            .finish()
    }
}

/// Monero standard address
#[derive(Debug, Clone)]
pub struct MoneroAddress {
    /// Network type
    pub network: MoneroNetwork,
    /// Public spend key
    pub public_spend_key: [u8; 32],
    /// Public view key
    pub public_view_key: [u8; 32],
    /// Base58-encoded address string
    pub address: String,
}

/// Compute Keccak-256 hash (Monero uses Keccak, not SHA3-256)
#[cfg(feature = "monero")]
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

/// Reduce a 32-byte value to a valid Ed25519 scalar
/// This is critical for Monero's key derivation
#[cfg(feature = "monero")]
pub fn sc_reduce32(data: &[u8; 32]) -> [u8; 32] {
    // Create a 64-byte buffer with the 32-byte input padded with zeros
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(data);

    // Use Scalar::from_bytes_mod_order_wide for proper reduction
    let scalar = Scalar::from_bytes_mod_order_wide(&wide);
    scalar.to_bytes()
}

/// Derive public key from private key using Ed25519
#[cfg(feature = "monero")]
pub fn secret_key_to_public_key(secret: &[u8; 32]) -> Result<[u8; 32]> {
    let scalar = Scalar::from_bytes_mod_order(*secret);
    let point = &scalar * ED25519_BASEPOINT_TABLE;
    Ok(point.compress().to_bytes())
}

/// Derive Monero keys from a seed (typically from BIP39 mnemonic)
#[cfg(feature = "monero")]
pub fn derive_monero_keys(seed: &[u8]) -> Result<MoneroKeys> {
    if seed.len() < 32 {
        return Err(Error::InvalidMnemonic("Seed must be at least 32 bytes".into()));
    }

    // Take first 32 bytes and reduce to valid scalar for private spend key
    let mut seed_bytes = [0u8; 32];
    seed_bytes.copy_from_slice(&seed[..32]);
    let private_spend_key = sc_reduce32(&seed_bytes);

    // Derive private view key: H_s(private_spend_key)
    // This is the standard Monero derivation
    let view_key_hash = keccak256(&private_spend_key);
    let private_view_key = sc_reduce32(&view_key_hash);

    // Derive public keys
    let public_spend_key = secret_key_to_public_key(&private_spend_key)?;
    let public_view_key = secret_key_to_public_key(&private_view_key)?;

    Ok(MoneroKeys {
        private_spend_key,
        private_view_key,
        public_spend_key,
        public_view_key,
    })
}

/// Monero Base58 alphabet (different from Bitcoin's)
const MONERO_BASE58_ALPHABET: &[u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Encode bytes to Monero Base58
/// Monero uses a block-based encoding scheme different from Bitcoin
#[cfg(feature = "monero")]
pub fn base58_encode_monero(data: &[u8]) -> String {
    // Monero encodes in blocks of 8 bytes -> 11 characters
    // except the last block which may be smaller
    let full_blocks = data.len() / 8;
    let remainder = data.len() % 8;

    let mut result = String::new();

    // Process full 8-byte blocks
    for i in 0..full_blocks {
        let block = &data[i * 8..(i + 1) * 8];
        let encoded = encode_block(block, 11);
        result.push_str(&encoded);
    }

    // Process remainder
    if remainder > 0 {
        let block = &data[full_blocks * 8..];
        let encoded_len = match remainder {
            1 => 2,
            2 => 3,
            3 => 5,
            4 => 6,
            5 => 7,
            6 => 9,
            7 => 10,
            _ => unreachable!(),
        };
        let encoded = encode_block(block, encoded_len);
        result.push_str(&encoded);
    }

    result
}

/// Encode a single block to Base58
#[cfg(feature = "monero")]
fn encode_block(block: &[u8], output_len: usize) -> String {
    // Convert block to u64 (big-endian)
    let mut num: u128 = 0;
    for &byte in block {
        num = (num << 8) | (byte as u128);
    }

    // Convert to base58
    let mut chars = vec!['1'; output_len];
    for i in (0..output_len).rev() {
        let remainder = (num % 58) as usize;
        chars[i] = MONERO_BASE58_ALPHABET[remainder] as char;
        num /= 58;
    }

    chars.into_iter().collect()
}

/// Generate Monero standard address from keys
#[cfg(feature = "monero")]
pub fn generate_address(keys: &MoneroKeys, network: MoneroNetwork) -> MoneroAddress {
    // Address data: prefix (1 byte) + public_spend_key (32 bytes) + public_view_key (32 bytes)
    let mut data = Vec::with_capacity(69);
    data.push(network.address_prefix());
    data.extend_from_slice(&keys.public_spend_key);
    data.extend_from_slice(&keys.public_view_key);

    // Compute checksum: first 4 bytes of Keccak-256
    let checksum = keccak256(&data);
    data.extend_from_slice(&checksum[..4]);

    // Encode to Base58
    let address = base58_encode_monero(&data);

    MoneroAddress {
        network,
        public_spend_key: keys.public_spend_key,
        public_view_key: keys.public_view_key,
        address,
    }
}

/// Key image structure for tracking spent outputs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyImage {
    /// The key image bytes (32 bytes, compressed Edwards point)
    pub image: [u8; 32],
    /// Hex representation
    pub hex: String,
}

impl KeyImage {
    /// Create from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self {
            image: bytes,
            hex: hex::encode(bytes),
        }
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| Error::InvalidMnemonic(format!("Invalid key image hex: {}", e)))?;
        if bytes.len() != 32 {
            return Err(Error::InvalidMnemonic("Key image must be 32 bytes".into()));
        }
        let mut image = [0u8; 32];
        image.copy_from_slice(&bytes);
        Ok(Self::from_bytes(image))
    }
}

/// Transaction output reference for ring signature analysis
#[derive(Debug, Clone)]
pub struct TxOutputRef {
    /// Transaction hash
    pub tx_hash: String,
    /// Output index within the transaction
    pub output_index: u32,
    /// Public key of the output
    pub public_key: [u8; 32],
    /// Block height where this output was created
    pub block_height: u64,
    /// Timestamp of the block
    pub timestamp: u64,
    /// Amount (if known, 0 for RingCT outputs)
    pub amount: u64,
}

/// Ring member for analysis
#[derive(Debug, Clone)]
pub struct RingMember {
    /// Output reference
    pub output: TxOutputRef,
    /// Position in the ring (0 = oldest, n-1 = newest)
    pub ring_position: usize,
    /// Age in blocks at time of transaction
    pub age_blocks: u64,
    /// Whether this is likely the real input (heuristic analysis result)
    pub likely_real: Option<f64>,
}

/// Ring signature analysis result
#[derive(Debug, Clone)]
pub struct RingAnalysis {
    /// Key image being analyzed
    pub key_image: KeyImage,
    /// All ring members
    pub ring_members: Vec<RingMember>,
    /// Index of most likely real input (-1 if unknown)
    pub predicted_real_index: Option<usize>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Heuristics used
    pub heuristics_applied: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "monero")]
    fn test_keccak256() {
        // Test vector from Monero
        let input = b"";
        let hash = keccak256(input);
        let expected = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
        assert_eq!(hex::encode(hash), expected);
    }

    #[test]
    #[cfg(feature = "monero")]
    fn test_key_derivation() {
        // Use a non-zero test seed (BIP39 "abandon" x11 + "about" produces this)
        let seed: [u8; 32] = [
            0x5e, 0xb0, 0x0b, 0xbd, 0xdc, 0xf0, 0x69, 0x08,
            0x48, 0x89, 0xa8, 0xab, 0x91, 0x55, 0x56, 0x81,
            0x65, 0xf5, 0xc4, 0x53, 0xcc, 0xb8, 0x5e, 0x70,
            0x81, 0x1a, 0xae, 0xd6, 0xf6, 0xda, 0x5f, 0xc1,
        ];
        let keys = derive_monero_keys(&seed).unwrap();

        // Verify keys are valid (non-zero)
        assert_ne!(keys.private_spend_key, [0u8; 32]);
        assert_ne!(keys.public_spend_key, [0u8; 32]);
        assert_ne!(keys.public_view_key, [0u8; 32]);

        // Verify view key is different from spend key
        assert_ne!(keys.private_view_key, keys.private_spend_key);
        assert_ne!(keys.public_view_key, keys.public_spend_key);
    }

    #[test]
    #[cfg(feature = "monero")]
    fn test_address_generation() {
        let seed = [1u8; 32];
        let keys = derive_monero_keys(&seed).unwrap();
        let address = generate_address(&keys, MoneroNetwork::Mainnet);

        // Mainnet addresses start with '4'
        assert!(address.address.starts_with('4'));
        // Standard addresses are 95 characters
        assert_eq!(address.address.len(), 95);
    }

    #[test]
    fn test_key_image_from_hex() {
        let hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let ki = KeyImage::from_hex(hex).unwrap();
        assert_eq!(ki.hex, hex);
    }
}
