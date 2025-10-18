//! Resonance integration using phosphoros-core
//!
//! Provides real resonance analysis and 5D spectral operations.

use phosphoros_core::{HolisticMatrix, ResonanceEngine, Point5D, SpectralSignature};
use thiserror::Error;

/// Resonance integration error
#[derive(Debug, Error)]
pub enum ResonanceError {
    /// Analysis error
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

/// Resonance analysis result
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// Best resonance score
    pub best_resonance: f64,
    /// Spectral signature (ψ, ρ, ω)
    pub signature: (f64, f64, f64),
    /// 5D point
    pub point_5d: [f64; 5],
}

/// Resonance integration
pub struct ResonanceIntegration {
    engine: HolisticMatrix,
}

impl Default for ResonanceIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceIntegration {
    /// Create a new resonance integration
    pub fn new() -> Self {
        Self {
            engine: HolisticMatrix::default_config(),
        }
    }
    
    /// Analyze a point in 5D space
    pub fn analyze_point(&mut self, perception: [f64; 5], intention: [f64; 5]) -> Result<ResonanceResult, ResonanceError> {
        // Reset engine for fresh analysis
        self.engine.reset();
        
        // Calculate gradient (difference between intention and perception)
        let gradient = [
            intention[0] - perception[0],
            intention[1] - perception[1],
            intention[2] - perception[2],
            intention[3] - perception[3],
            intention[4] - perception[4],
        ];
        
        // Evaluate using holistic matrix
        let eval = self.engine.evaluate(1.0, perception, intention, gradient, 0.5);
        
        match eval {
            phosphoros_core::Evaluation::Output { vector, score } => {
                // Calculate spectral signature from 5D point coordinates
                let psi = (vector[0] + vector[1]) / 2.0;
                let rho = (vector[2] + vector[3]) / 2.0;
                let omega = vector[4];
                
                Ok(ResonanceResult {
                    best_resonance: score,
                    signature: (psi, rho, omega),
                    point_5d: vector,
                })
            }
            phosphoros_core::Evaluation::Gated { reason } => {
                Err(ResonanceError::AnalysisError(format!("Gated: {:?}", reason)))
            }
        }
    }
    
    /// Batch analyze multiple points
    pub fn batch_analyze(&mut self, points: Vec<([f64; 5], [f64; 5])>) -> Vec<ResonanceResult> {
        points.into_iter()
            .filter_map(|(perception, intention)| {
                self.analyze_point(perception, intention).ok()
            })
            .collect()
    }
    
    /// Calculate resonance from spectral signature
    pub fn calculate_resonance(psi: f64, rho: f64, omega: f64) -> f64 {
        psi * rho * omega
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_point() {
        let mut integration = ResonanceIntegration::new();
        let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
        let intention = [0.6, 0.6, 0.6, 0.6, 0.6];
        
        let result = integration.analyze_point(perception, intention);
        assert!(result.is_ok());
        
        let res = result.unwrap();
        assert!(res.best_resonance >= 0.0);
        assert!(res.best_resonance <= 1.0);
    }
    
    #[test]
    fn test_calculate_resonance() {
        let resonance = ResonanceIntegration::calculate_resonance(0.8, 0.9, 0.7);
        assert!((resonance - 0.504).abs() < 0.001);
    }
}
