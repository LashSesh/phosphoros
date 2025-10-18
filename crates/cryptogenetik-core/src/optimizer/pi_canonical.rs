//! PI (Phase-Invarianz) Canonical operator

/// PI Canonical operator configuration
#[derive(Debug, Clone)]
pub struct PIConfig {
    /// Canonical factor
    pub canonical_factor: f64,
}

impl Default for PIConfig {
    fn default() -> Self {
        Self {
            canonical_factor: 1.0,
        }
    }
}

/// PI Canonical operator
///
/// Maintains phase invariance and canonical form
#[derive(Debug, Clone)]
pub struct PICanonical {
    config: PIConfig,
}

impl PICanonical {
    /// Create new PI operator
    pub fn new(config: PIConfig) -> Self {
        Self { config }
    }

    /// Apply canonical transformation
    pub fn apply(&self, v: [f64; 5]) -> [f64; 5] {
        // Normalize to canonical form
        let norm = v.iter().map(|&x| x * x).sum::<f64>().sqrt();

        if norm < 1e-12 {
            return v;
        }

        v.map(|x| x / norm * self.config.canonical_factor)
    }

    /// Check if vector is in canonical form
    pub fn is_canonical(&self, v: &[f64; 5]) -> bool {
        let norm = v.iter().map(|&x| x * x).sum::<f64>().sqrt();
        (norm - self.config.canonical_factor).abs() < 1e-6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_canonical() {
        let pi = PICanonical::new(PIConfig::default());
        let v = [2.0, 2.0, 2.0, 2.0, 2.0];
        let result = pi.apply(v);

        // Result should be normalized
        let norm: f64 = result.iter().map(|&x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_is_canonical() {
        let pi = PICanonical::new(PIConfig::default());
        let v = [1.0, 0.0, 0.0, 0.0, 0.0];
        assert!(pi.is_canonical(&v));

        let v2 = [2.0, 2.0, 2.0, 2.0, 2.0];
        assert!(!pi.is_canonical(&v2));
    }
}
