//! # PHOSPHOROS BIP39
//!
//! BIP39 multichain wallet support for PHOSPHOROS.
//!
//! ## Features
//!
//! - **BIP39 Mnemonics**: Support for 12 languages
//! - **Multichain**: Bitcoin, Ethereum, Substrate, Cosmos, Solana, Cardano, Monero
//! - **Derivation**: BIP32/44/49/84/86 support
//! - **Resonance Analysis**: Optional spectral analysis of wordlists
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! // Requires 'btc' feature for Bitcoin address generation
//! use phosphoros_bip39::{Mnemonic, WordlistLanguage, DerivationPath, MasterKey};
//! use phosphoros_bip39::multichain::{Blockchain, MultichainWallet};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create mnemonic from phrase
//!     let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
//!     let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English)?;
//!
//!     // Generate seed
//!     let seed = mnemonic.to_seed(None);
//!
//!     // Derive key
//!     let master = MasterKey::from_seed(seed, phosphoros_bip39::CurveType::Secp256k1);
//!     let path = DerivationPath::bitcoin(0, 0, 0);
//!     let key = master.derive(&path)?;
//!
//!     // Generate address (requires 'btc' feature)
//!     let wallet = MultichainWallet::new(key, Blockchain::Bitcoin);
//!     let address = wallet.address()?;
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod derivation;
mod error;
pub mod mnemonic;
pub mod multichain;
pub mod wordlist;

// Monero-specific cryptography
#[cfg(feature = "monero")]
pub mod monero;

// Bitcoin-specific address generation
#[cfg(feature = "btc")]
pub mod bitcoin;

// Ethereum/EVM-specific address generation
#[cfg(feature = "evm")]
pub mod ethereum;

// Optional resonance analysis module
#[cfg(feature = "resonance")]
pub mod analysis;

pub use derivation::{CurveType, DerivationPath, DerivedKey, MasterKey};
pub use error::{Error, Result};
pub use mnemonic::Mnemonic;
pub use multichain::{Blockchain, MultichainAddress, MultichainWallet};
pub use wordlist::WordlistLanguage;

// Re-export Bitcoin types when btc feature is enabled
#[cfg(feature = "btc")]
pub use bitcoin::{BitcoinAddress, BitcoinAddressType, BitcoinNetwork};

// Re-export Ethereum types when evm feature is enabled
#[cfg(feature = "evm")]
pub use ethereum::EthereumAddress;
