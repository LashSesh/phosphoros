//! Pattern matching hook

/// Pattern matching hook
#[derive(Debug, Clone)]
pub struct PatternHook;

impl PatternHook {
    /// Create new pattern hook
    pub fn new() -> Self {
        Self
    }

    /// Score based on pattern detection (simplified)
    pub fn score(&self, data: &[u8]) -> f64 {
        // Simple pattern score: detect repetitions
        if data.len() < 2 {
            return 0.5;
        }

        let mut pattern_score: f64 = 0.0;
        for i in 0..data.len() - 1 {
            if data[i] == data[i + 1] {
                pattern_score += 0.1;
            }
        }

        pattern_score.min(1.0)
    }
}

impl Default for PatternHook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_score() {
        let hook = PatternHook::new();
        let data = b"aabbccdd";
        let score = hook.score(data);
        assert!(score > 0.0);
    }
}
