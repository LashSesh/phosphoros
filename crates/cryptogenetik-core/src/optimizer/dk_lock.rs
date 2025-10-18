//! DK (Divergenz-Kontrolle) Lock operator

/// DK Lock operator configuration
#[derive(Debug, Clone)]
pub struct DKConfig {
    /// Lock strength
    pub strength: f64,
    /// Divergence limit
    pub divergence_limit: f64,
}

impl Default for DKConfig {
    fn default() -> Self {
        Self {
            strength: 0.5,
            divergence_limit: 2.0,
        }
    }
}

/// DK Lock operator
///
/// Controls divergence and locks onto promising regions
#[derive(Debug, Clone)]
pub struct DKLock {
    config: DKConfig,
    reference: Option<[f64; 5]>,
}

impl DKLock {
    /// Create new DK operator
    pub fn new(config: DKConfig) -> Self {
        Self {
            config,
            reference: None,
        }
    }

    /// Set reference point
    pub fn set_reference(&mut self, reference: [f64; 5]) {
        self.reference = Some(reference);
    }

    /// Apply lock operation
    pub fn apply(&self, v: [f64; 5]) -> [f64; 5] {
        if let Some(ref_point) = self.reference {
            // Calculate divergence
            let divergence = self.calculate_divergence(&v, &ref_point);

            if divergence > self.config.divergence_limit {
                // Pull back towards reference
                self.pull_towards_reference(v, ref_point)
            } else {
                v
            }
        } else {
            v
        }
    }

    fn calculate_divergence(&self, v: &[f64; 5], reference: &[f64; 5]) -> f64 {
        v.iter()
            .zip(reference.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    fn pull_towards_reference(&self, v: [f64; 5], reference: [f64; 5]) -> [f64; 5] {
        let mut result = v;
        for i in 0..5 {
            result[i] = v[i] + self.config.strength * (reference[i] - v[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dk_without_reference() {
        let dk = DKLock::new(DKConfig::default());
        let v = [1.0, 1.0, 1.0, 1.0, 1.0];
        let result = dk.apply(v);
        assert_eq!(result, v);
    }

    #[test]
    fn test_dk_with_reference() {
        let mut dk = DKLock::new(DKConfig::default());
        dk.set_reference([0.0, 0.0, 0.0, 0.0, 0.0]);
        
        let v = [5.0, 5.0, 5.0, 5.0, 5.0];
        let result = dk.apply(v);
        
        // Result should be pulled towards reference
        for &val in &result {
            assert!(val < 5.0);
        }
    }
}
