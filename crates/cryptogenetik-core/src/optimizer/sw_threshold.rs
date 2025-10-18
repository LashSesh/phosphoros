//! SW (Schwellenwert) Threshold operator

/// SW Threshold operator configuration
#[derive(Debug, Clone)]
pub struct SWConfig {
    /// Threshold value
    pub threshold: f64,
    /// Scaling factor
    pub scale: f64,
}

impl Default for SWConfig {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            scale: 1.0,
        }
    }
}

/// SW Threshold operator
///
/// Applies adaptive thresholding based on score
#[derive(Debug, Clone)]
pub struct SWThreshold {
    config: SWConfig,
}

impl SWThreshold {
    /// Create new SW operator
    pub fn new(config: SWConfig) -> Self {
        Self { config }
    }

    /// Apply threshold operation
    pub fn apply(&self, v: [f64; 5], score: f64) -> Option<[f64; 5]> {
        if score >= self.config.threshold {
            // Pass through with scaling
            Some(v.map(|x| x * self.config.scale))
        } else {
            // Below threshold - gate
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw_threshold() {
        let sw = SWThreshold::new(SWConfig::default());
        let v = [1.0, 1.0, 1.0, 1.0, 1.0];
        
        // Above threshold
        assert!(sw.apply(v, 0.6).is_some());
        
        // Below threshold
        assert!(sw.apply(v, 0.4).is_none());
    }
}
