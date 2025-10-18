//! BIP39 Mnemonic generation and validation

use crate::wordlist::WordlistLanguage;
use crate::Error;
use bip39::Mnemonic as Bip39Mnemonic;
use sha2::{Digest, Sha256};

/// BIP39 Mnemonic phrase
pub struct Mnemonic {
    inner: Bip39Mnemonic,
    language: WordlistLanguage,
}

impl Mnemonic {
    /// Create a new mnemonic from words
    pub fn from_words(words: &[String], language: WordlistLanguage) -> Result<Self, Error> {
        let phrase = words.join(" ");
        let inner = Bip39Mnemonic::parse_in(language.to_bip39_language(), &phrase)
            .map_err(|e| Error::InvalidMnemonic(e.to_string()))?;

        Ok(Self { inner, language })
    }

    /// Create from phrase string
    pub fn from_phrase(phrase: &str, language: WordlistLanguage) -> Result<Self, Error> {
        let inner = Bip39Mnemonic::parse_in(language.to_bip39_language(), phrase)
            .map_err(|e| Error::InvalidMnemonic(e.to_string()))?;

        Ok(Self { inner, language })
    }

    /// Generate a new random mnemonic
    pub fn generate(word_count: usize, language: WordlistLanguage) -> Result<Self, Error> {
        let entropy_bits = match word_count {
            12 => 128,
            15 => 160,
            18 => 192,
            21 => 224,
            24 => 256,
            _ => return Err(Error::InvalidWordCount(word_count)),
        };

        // Generate random entropy
        let entropy_bytes = entropy_bits / 8;
        let mut entropy = vec![0u8; entropy_bytes];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut entropy);

        let inner = Bip39Mnemonic::from_entropy_in(language.to_bip39_language(), &entropy)
            .map_err(|e| Error::MnemonicGeneration(e.to_string()))?;

        Ok(Self { inner, language })
    }

    /// Get words as vector
    pub fn words(&self) -> Vec<String> {
        self.inner.words().map(|w| w.to_string()).collect()
    }

    /// Get phrase as string
    pub fn phrase(&self) -> String {
        self.inner.to_string()
    }

    /// Get entropy bytes
    pub fn entropy(&self) -> Vec<u8> {
        self.inner.to_entropy()
    }

    /// Get language
    pub fn language(&self) -> WordlistLanguage {
        self.language
    }

    /// Convert to seed with optional passphrase
    pub fn to_seed(&self, passphrase: Option<&str>) -> Vec<u8> {
        self.inner.to_seed(passphrase.unwrap_or("")).to_vec()
    }

    /// Validate mnemonic
    pub fn validate(&self) -> bool {
        // The bip39 crate validates on parse, so if we have a Mnemonic it's valid
        true
    }

    /// Calculate checksum
    pub fn checksum(&self) -> u8 {
        let entropy = self.entropy();
        let hash = Sha256::digest(&entropy);
        hash[0]
    }
}

// Need to add rand to dependencies
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English).unwrap();
        assert_eq!(mnemonic.words().len(), 12);
    }

    #[test]
    fn test_words_roundtrip() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English).unwrap();
        let words = mnemonic.words();
        let mnemonic2 = Mnemonic::from_words(&words, WordlistLanguage::English).unwrap();
        assert_eq!(mnemonic.phrase(), mnemonic2.phrase());
    }

    #[test]
    fn test_to_seed() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English).unwrap();
        let seed = mnemonic.to_seed(None);
        assert_eq!(seed.len(), 64);
    }

    #[test]
    fn test_validate() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_phrase(phrase, WordlistLanguage::English).unwrap();
        assert!(mnemonic.validate());
    }
}
