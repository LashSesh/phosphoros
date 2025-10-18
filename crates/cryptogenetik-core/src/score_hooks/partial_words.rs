//! Partial words matching hook

/// Partial words matching hook
#[derive(Debug, Clone)]
pub struct PartialWordsHook {
    /// Known word prefixes
    prefixes: Vec<String>,
}

impl PartialWordsHook {
    /// Create new partial words hook
    pub fn new() -> Self {
        Self {
            prefixes: vec![
                "aba".to_string(),
                "abi".to_string(),
                "abl".to_string(),
                // In production, would have full BIP39 prefix list
            ],
        }
    }

    /// Score based on partial word matches
    pub fn score(&self, words: &[String]) -> f64 {
        if words.is_empty() {
            return 0.0;
        }

        let mut matches = 0;
        for word in words {
            let prefix = word.chars().take(3).collect::<String>();
            if self.prefixes.iter().any(|p| p == &prefix) {
                matches += 1;
            }
        }

        matches as f64 / words.len() as f64
    }
}

impl Default for PartialWordsHook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_words() {
        let hook = PartialWordsHook::new();
        let words = vec!["abandon".to_string(), "ability".to_string()];
        let score = hook.score(&words);
        assert!(score > 0.0);
    }
}
