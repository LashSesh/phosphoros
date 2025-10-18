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
//! ```rust
//! use phosphoros_bip39::{Mnemonic, WordlistLanguage, DerivationPath, MasterKey};
//! use phosphoros_bip39::multichain::{Blockchain, MultichainWallet};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create mnemonic from phrase
//! let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
//! let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English)?;
//!
//! // Generate seed
//! let seed = mnemonic.to_seed(None);
//!
//! // Derive key
//! let master = MasterKey::from_seed(seed, phosphoros_bip39::CurveType::Secp256k1);
//! let path = DerivationPath::bitcoin(0, 0, 0);
//! let key = master.derive(&path)?;
//!
//! // Generate address
//! let wallet = MultichainWallet::new(key, Blockchain::Bitcoin);
//! let address = wallet.address()?;
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
pub mod derivation;
pub mod mnemonic;
pub mod multichain;
pub mod wordlist;

// Optional resonance analysis module
#[cfg(feature = "resonance")]
pub mod analysis;

pub use error::{Error, Result};
pub use derivation::{CurveType, DerivationPath, DerivedKey, MasterKey};
pub use mnemonic::Mnemonic;
pub use multichain::{Blockchain, MultichainAddress, MultichainWallet};
pub use wordlist::WordlistLanguage;

