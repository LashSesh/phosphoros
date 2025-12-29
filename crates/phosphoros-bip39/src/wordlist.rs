//! BIP39 Wordlist support for multiple languages

use std::fmt;

/// BIP39 Wordlist languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordlistLanguage {
    /// English wordlist (most common)
    English,
    /// Chinese (Simplified)
    ChineseSimplified,
    /// Chinese (Traditional)
    ChineseTraditional,
    /// French
    French,
    /// Italian
    Italian,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
    /// Spanish
    Spanish,
    /// Czech
    Czech,
    /// Portuguese
    Portuguese,
}

impl WordlistLanguage {
    /// Get all supported languages
    pub fn all() -> Vec<Self> {
        vec![
            Self::English,
            Self::ChineseSimplified,
            Self::ChineseTraditional,
            Self::French,
            Self::Italian,
            Self::Japanese,
            Self::Korean,
            Self::Spanish,
            Self::Czech,
            Self::Portuguese,
        ]
    }

    /// Get language name
    pub fn name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ChineseSimplified => "Chinese (Simplified)",
            Self::ChineseTraditional => "Chinese (Traditional)",
            Self::French => "French",
            Self::Italian => "Italian",
            Self::Japanese => "Japanese",
            Self::Korean => "Korean",
            Self::Spanish => "Spanish",
            Self::Czech => "Czech",
            Self::Portuguese => "Portuguese",
        }
    }

    /// Convert to bip39 crate language
    pub(crate) fn to_bip39_language(self) -> bip39::Language {
        match self {
            Self::English => bip39::Language::English,
            Self::ChineseSimplified => bip39::Language::SimplifiedChinese,
            Self::ChineseTraditional => bip39::Language::TraditionalChinese,
            Self::French => bip39::Language::French,
            Self::Italian => bip39::Language::Italian,
            Self::Japanese => bip39::Language::Japanese,
            Self::Korean => bip39::Language::Korean,
            Self::Spanish => bip39::Language::Spanish,
            Self::Czech => bip39::Language::Czech,
            Self::Portuguese => bip39::Language::Portuguese,
        }
    }
}

impl fmt::Display for WordlistLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_languages() {
        let langs = WordlistLanguage::all();
        assert_eq!(langs.len(), 10);
    }

    #[test]
    fn test_language_name() {
        assert_eq!(WordlistLanguage::English.name(), "English");
        assert_eq!(WordlistLanguage::Japanese.name(), "Japanese");
    }
}
