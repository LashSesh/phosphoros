//! Cosmos/Tendermint address generation
//!
//! Implements proper Cosmos address derivation using:
//! - secp256k1 elliptic curve (via k256)
//! - SHA256 + RIPEMD160 hashing
//! - Bech32 encoding with configurable HRP

use bech32::{ToBase32, Variant};
use k256::ecdsa::SigningKey;
use sha2::{Digest, Sha256};

use crate::{derivation::DerivedKey, Error, Result};

/// Common Cosmos chain prefixes
#[derive(Debug, Clone)]
pub struct CosmosChain {
    /// Human-readable part for bech32
    pub hrp: String,
    /// Chain name
    pub name: String,
    /// Coin type for BIP44 (usually 118 for Cosmos SDK chains)
    pub coin_type: u32,
}

impl CosmosChain {
    /// Cosmos Hub (ATOM)
    pub fn cosmos() -> Self {
        Self {
            hrp: "cosmos".to_string(),
            name: "Cosmos Hub".to_string(),
            coin_type: 118,
        }
    }

    /// Osmosis
    pub fn osmosis() -> Self {
        Self {
            hrp: "osmo".to_string(),
            name: "Osmosis".to_string(),
            coin_type: 118,
        }
    }

    /// Juno
    pub fn juno() -> Self {
        Self {
            hrp: "juno".to_string(),
            name: "Juno".to_string(),
            coin_type: 118,
        }
    }

    /// Celestia
    pub fn celestia() -> Self {
        Self {
            hrp: "celestia".to_string(),
            name: "Celestia".to_string(),
            coin_type: 118,
        }
    }

    /// Create custom Cosmos chain
    pub fn custom(hrp: &str, name: &str, coin_type: u32) -> Self {
        Self {
            hrp: hrp.to_string(),
            name: name.to_string(),
            coin_type,
        }
    }
}

/// Cosmos address with metadata
#[derive(Debug, Clone)]
pub struct CosmosAddress {
    /// The bech32 address
    pub address: String,
    /// Compressed public key (33 bytes, hex)
    pub public_key_compressed: String,
    /// Address bytes (20 bytes, hex) - RIPEMD160(SHA256(pubkey))
    pub address_bytes: String,
    /// Chain configuration
    pub chain: CosmosChain,
}

/// Generate Cosmos address from derived key
///
/// # Arguments
/// * `key` - The derived key containing the private key
/// * `chain` - Chain configuration with HRP
///
/// # Returns
/// Cosmos address with bech32 encoding
pub fn generate_address(key: &DerivedKey, chain: CosmosChain) -> Result<CosmosAddress> {
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

    // Get compressed public key (33 bytes)
    let public_key_compressed = verifying_key.to_encoded_point(true);
    let pubkey_bytes = public_key_compressed.as_bytes();

    // Calculate address: RIPEMD160(SHA256(compressed_pubkey))
    let sha256_hash = Sha256::digest(pubkey_bytes);

    // Use our own RIPEMD160 implementation or import it
    let address_bytes = ripemd160_hash(&sha256_hash);

    // Encode as bech32
    let address = bech32::encode(&chain.hrp, address_bytes.to_base32(), Variant::Bech32)
        .map_err(|e| Error::AddressGenerationFailed(format!("Bech32 encoding failed: {}", e)))?;

    Ok(CosmosAddress {
        address,
        public_key_compressed: hex::encode(pubkey_bytes),
        address_bytes: hex::encode(&address_bytes),
        chain,
    })
}

/// RIPEMD-160 hash implementation
/// (Using a simple implementation to avoid adding another dependency)
fn ripemd160_hash(data: &[u8]) -> [u8; 20] {

    // RIPEMD-160 constants
    const K_LEFT: [u32; 5] = [0x00000000, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e];
    const K_RIGHT: [u32; 5] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0x00000000];

    const R_LEFT: [usize; 80] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9,
        5, 2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8,
        12, 4, 13, 3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
    ];

    const R_RIGHT: [usize; 80] = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8,
        12, 4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11,
        15, 0, 5, 12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
    ];

    const S_LEFT: [u32; 80] = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12,
        15, 9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14,
        15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11,
        8, 5, 6,
    ];

    const S_RIGHT: [u32; 80] = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7,
        12, 7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11,
        14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13,
        11, 11,
    ];

    fn f(j: usize, x: u32, y: u32, z: u32) -> u32 {
        match j {
            0..=15 => x ^ y ^ z,
            16..=31 => (x & y) | (!x & z),
            32..=47 => (x | !y) ^ z,
            48..=63 => (x & z) | (y & !z),
            64..=79 => x ^ (y | !z),
            _ => unreachable!(),
        }
    }

    // Pad message
    let mut msg = data.to_vec();
    let original_len = msg.len();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    let bit_len = (original_len as u64) * 8;
    msg.extend_from_slice(&bit_len.to_le_bytes());

    // Initial hash values
    let mut h: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

    // Process each 512-bit block
    for chunk in msg.chunks(64) {
        let mut x = [0u32; 16];
        for (i, word) in chunk.chunks(4).enumerate() {
            x[i] = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
        }

        let mut al = h[0];
        let mut bl = h[1];
        let mut cl = h[2];
        let mut dl = h[3];
        let mut el = h[4];

        let mut ar = h[0];
        let mut br = h[1];
        let mut cr = h[2];
        let mut dr = h[3];
        let mut er = h[4];

        for j in 0..80 {
            let round = j / 16;

            // Left side
            let t = al
                .wrapping_add(f(j, bl, cl, dl))
                .wrapping_add(x[R_LEFT[j]])
                .wrapping_add(K_LEFT[round])
                .rotate_left(S_LEFT[j])
                .wrapping_add(el);
            al = el;
            el = dl;
            dl = cl.rotate_left(10);
            cl = bl;
            bl = t;

            // Right side
            let t = ar
                .wrapping_add(f(79 - j, br, cr, dr))
                .wrapping_add(x[R_RIGHT[j]])
                .wrapping_add(K_RIGHT[round])
                .rotate_left(S_RIGHT[j])
                .wrapping_add(er);
            ar = er;
            er = dr;
            dr = cr.rotate_left(10);
            cr = br;
            br = t;
        }

        let t = h[1].wrapping_add(cl).wrapping_add(dr);
        h[1] = h[2].wrapping_add(dl).wrapping_add(er);
        h[2] = h[3].wrapping_add(el).wrapping_add(ar);
        h[3] = h[4].wrapping_add(al).wrapping_add(br);
        h[4] = h[0].wrapping_add(bl).wrapping_add(cr);
        h[0] = t;
    }

    let mut result = [0u8; 20];
    for (i, &val) in h.iter().enumerate() {
        result[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivation::CurveType;

    fn test_key() -> DerivedKey {
        let private_key = vec![
            0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
            0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15,
            0xb0, 0xf0, 0x0a, 0x08,
        ];
        DerivedKey {
            private_key,
            public_key: vec![],
            chain_code: vec![0u8; 32],
            path: "m/44'/118'/0'/0/0".to_string(),
            curve: CurveType::Secp256k1,
        }
    }

    #[test]
    fn test_cosmos_address_generation() {
        let key = test_key();
        let result = generate_address(&key, CosmosChain::cosmos());
        assert!(result.is_ok());
        let addr = result.unwrap();

        // Check address format
        assert!(
            addr.address.starts_with("cosmos1"),
            "Should have cosmos1 prefix: {}",
            addr.address
        );
    }

    #[test]
    fn test_osmosis_address_generation() {
        let key = test_key();
        let result = generate_address(&key, CosmosChain::osmosis());
        assert!(result.is_ok());
        let addr = result.unwrap();

        assert!(
            addr.address.starts_with("osmo1"),
            "Should have osmo1 prefix: {}",
            addr.address
        );
    }

    #[test]
    fn test_consistent_address_derivation() {
        let key = test_key();
        let addr1 = generate_address(&key, CosmosChain::cosmos()).unwrap();
        let addr2 = generate_address(&key, CosmosChain::cosmos()).unwrap();

        assert_eq!(addr1.address, addr2.address);
        assert_eq!(addr1.address_bytes, addr2.address_bytes);
    }

    #[test]
    fn test_ripemd160_known_vector() {
        // Test vector: RIPEMD160("")
        let empty_hash = ripemd160_hash(&[]);
        let expected = hex::decode("9c1185a5c5e9fc54612808977ee8f548b2258d31").unwrap();
        assert_eq!(&empty_hash[..], &expected[..]);
    }
}
