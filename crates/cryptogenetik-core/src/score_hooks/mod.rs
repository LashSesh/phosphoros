//! Score hooks - Early scoring mechanisms

pub mod checksum;
pub mod partial_words;
pub mod pattern;

pub use checksum::ChecksumHook;
pub use partial_words::PartialWordsHook;
pub use pattern::PatternHook;

/// Hook set combining multiple early-score mechanisms
#[derive(Debug, Clone)]
pub struct HookSet {
    pub checksum: Option<ChecksumHook>,
    pub partial_words: Option<PartialWordsHook>,
    pub pattern: Option<PatternHook>,
}

impl HookSet {
    /// Create new empty hook set
    pub fn new() -> Self {
        Self {
            checksum: None,
            partial_words: None,
            pattern: None,
        }
    }

    /// Enable all hooks
    pub fn all() -> Self {
        Self {
            checksum: Some(ChecksumHook::new()),
            partial_words: Some(PartialWordsHook::new()),
            pattern: Some(PatternHook::new()),
        }
    }

    /// Calculate early score from all enabled hooks
    pub fn early_score(&self, data: &[u8], words: &[String]) -> f64 {
        let mut score = 0.0;
        let mut count = 0;

        if let Some(ref hook) = self.checksum {
            score += hook.score(data);
            count += 1;
        }

        if let Some(ref hook) = self.partial_words {
            score += hook.score(words);
            count += 1;
        }

        if let Some(ref hook) = self.pattern {
            score += hook.score(data);
            count += 1;
        }

        if count > 0 {
            score / count as f64
        } else {
            0.5 // Default neutral score
        }
    }
}

impl Default for HookSet {
    fn default() -> Self {
        Self::new()
    }
}
