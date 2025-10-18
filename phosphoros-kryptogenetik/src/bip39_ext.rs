//! BIP39 integration extensions (feature-gated)

#[cfg(feature = "bip39-integration")]
use bip39::{Language, Mnemonic};

#[cfg(feature = "bip39-integration")]
use crate::{Error, PhosphorosCore, Point5D, Result};

#[cfg(feature = "bip39-integration")]
impl PhosphorosCore {
    /// Parse and embed a BIP39 mnemonic phrase
    pub fn embed_mnemonic(&self, phrase: &str) -> Result<Vec<Point5D>> {
        let _mnemonic = Mnemonic::parse_in_normalized(Language::English, phrase)
            .map_err(|e| Error::InvalidConfiguration(format!("Invalid BIP39 phrase: {}", e)))?;

        let words: Vec<&str> = phrase.split_whitespace().collect();
        Ok(self.embed_seed_phrase(&words))
    }
}

#[cfg(all(test, feature = "bip39-integration"))]
mod tests {
    use super::*;

    #[test]
    fn test_embed_mnemonic() {
        let core = PhosphorosCore::new();
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = core.embed_mnemonic(phrase);
        assert!(result.is_ok());
        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), 12);
    }
}
