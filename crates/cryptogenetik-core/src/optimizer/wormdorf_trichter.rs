//! Wormdorf-Trichter (WT) operator - Topological contraction along principal curvature

/// Wormdorf-Trichter operator configuration
#[derive(Debug, Clone)]
pub struct WTConfig {
    /// Contraction factor λ ∈ (0,1)
    pub lambda: f64,
    /// Maximum steps
    pub max_steps: usize,
}

impl Default for WTConfig {
    fn default() -> Self {
        Self {
            lambda: 0.8,
            max_steps: 10,
        }
    }
}

/// Wormdorf-Trichter operator
///
/// Applies topological contraction to reduce the search space diameter.
/// Contracts along principal curvature direction.
#[derive(Debug, Clone)]
pub struct WormdorfTrichter {
    config: WTConfig,
}

impl WormdorfTrichter {
    /// Create new WT operator
    pub fn new(config: WTConfig) -> Self {
        Self { config }
    }

    /// Apply contraction to vector
    ///
    /// v' = v + λ · ∇(curvature)
    pub fn apply(&self, v: [f64; 5], gradient: [f64; 5], _score: f64) -> [f64; 5] {
        let mut result = v;
        
        // Compute principal curvature direction (simplified)
        // In full implementation, this would use Hessian analysis
        let curvature_dir = self.principal_curvature_direction(&gradient);

        // Contract along curvature
        for i in 0..5 {
            result[i] += self.config.lambda * curvature_dir[i];
        }

        result
    }

    fn principal_curvature_direction(&self, gradient: &[f64; 5]) -> [f64; 5] {
        // Simplified: use negative normalized gradient
        let norm = gradient.iter().map(|&x| x * x).sum::<f64>().sqrt();
        if norm < 1e-12 {
            return [0.0; 5];
        }
        gradient.map(|x| -x / norm * 0.1)
    }

    /// Check if contraction should continue
    pub fn should_continue(&self, step: usize, delta: f64) -> bool {
        step < self.config.max_steps && delta.abs() > 1e-6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wt_apply() {
        let wt = WormdorfTrichter::new(WTConfig::default());
        let v = [1.0, 1.0, 1.0, 1.0, 1.0];
        let gradient = [0.5, 0.5, 0.5, 0.5, 0.5];
        let result = wt.apply(v, gradient, 0.8);
        
        // Result should be different from input
        assert_ne!(result, v);
    }

    #[test]
    fn test_should_continue() {
        let wt = WormdorfTrichter::new(WTConfig::default());
        assert!(wt.should_continue(0, 0.1));
        assert!(!wt.should_continue(100, 0.1));
        assert!(!wt.should_continue(0, 1e-10));
    }
}
