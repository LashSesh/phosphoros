use bip39::{Language, Mnemonic};
use ethers_core::utils::to_checksum;
use ethers_signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer};
use serde::Serialize;

use crate::error::{OuroborosError, Result};

#[derive(Debug, Clone, Serialize)]
pub struct SeedDetails {
    pub seed: String,
    pub address: String,
    pub private_key: String,
}

#[derive(Debug, Clone)]
pub struct SeedCrypto {
    language: Language,
}

impl Default for SeedCrypto {
    fn default() -> Self {
        Self {
            language: Language::English,
        }
    }
}

impl SeedCrypto {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn generate_seed(&self) -> Result<String> {
        let mnemonic = Mnemonic::generate_in(self.language, 12)
            .map_err(|e| OuroborosError::Crypto(e.to_string()))?;
        Ok(mnemonic.to_string())
    }

    pub fn seed_to_address(&self, seed_phrase: &str) -> Result<SeedDetails> {
        let parsed = Mnemonic::parse_in(self.language, seed_phrase)
            .map_err(|e| OuroborosError::Crypto(e.to_string()))?;
        let phrase = parsed.to_string();
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(phrase.as_str())
            .build()
            .map_err(|e| OuroborosError::Crypto(e.to_string()))?;

        let address = to_checksum(&wallet.address(), None);
        let private_key = hex::encode(wallet.signer().to_bytes());

        Ok(SeedDetails {
            seed: phrase,
            address,
            private_key,
        })
    }
}
