//! Integration bridge between Satellite and PHOSPHOROS Core.
//!
//! This module provides trait implementations and adapters that allow
//! Satellite analysis to work seamlessly with PHOSPHOROS resonance engines.

use phosphoros_core::{Point5D, SpectralSignature};

use crate::error::Result;
use crate::models::EntityObservation;

/// Trait for converting entities to PHOSPHOROS 5D space.
pub trait ToPoint5D {
    /// Converts this entity to a 5D point for resonance analysis.
    fn to_point5d(&self) -> Result<Point5D>;
}

impl ToPoint5D for EntityObservation {
    fn to_point5d(&self) -> Result<Point5D> {
        // Pad or truncate features to exactly 5 dimensions
        let mut coords = [0.0; 5];
        for (i, &val) in self.features.iter().take(5).enumerate() {
            coords[i] = val;
        }
        Ok(Point5D::new(coords[0], coords[1], coords[2], coords[3], coords[4]))
    }
}

/// Trait for extracting spectral signatures from analysis results.
pub trait ToSpectralSignature {
    /// Extracts a spectral signature (ψ, ρ, ω) from this entity.
    fn to_spectral_signature(&self) -> Result<SpectralSignature>;
}

impl ToSpectralSignature for EntityObservation {
    fn to_spectral_signature(&self) -> Result<SpectralSignature> {
        // Extract first 3 features as spectral signature components
        let psi = self.features.get(0).copied().unwrap_or(0.5);
        let rho = self.features.get(1).copied().unwrap_or(0.5);
        let omega = self.features.get(2).copied().unwrap_or(0.5);
        
        Ok(SpectralSignature::new(psi, rho, omega))
    }
}

/// Converts a resonance magnitude to a spectral signature.
///
/// This is useful for converting Satellite resonance hotspots into
/// PHOSPHOROS-compatible spectral signatures.
pub fn resonance_to_spectral(magnitude: f64) -> SpectralSignature {
    // Use magnitude as coherence (ψ)
    // Set density (ρ) and frequency (ω) to balanced values
    SpectralSignature::new(magnitude, 0.7, 0.7)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_entity_to_point5d() {
        let entity = EntityObservation {
            id: Uuid::new_v4(),
            address: "0xtest".to_string(),
            features: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            connections: vec![],
            metadata: HashMap::new(),
        };
        
        let point = entity.to_point5d().unwrap();
        assert_eq!(point.coords, [0.1, 0.2, 0.3, 0.4, 0.5]);
    }

    #[test]
    fn test_entity_to_spectral() {
        let entity = EntityObservation {
            id: Uuid::new_v4(),
            address: "0xtest".to_string(),
            features: vec![0.8, 0.6, 0.7],
            connections: vec![],
            metadata: HashMap::new(),
        };
        
        let sig = entity.to_spectral_signature().unwrap();
        assert_eq!(sig.psi, 0.8);
        assert_eq!(sig.rho, 0.6);
        assert_eq!(sig.omega, 0.7);
    }

    #[test]
    fn test_resonance_to_spectral() {
        let sig = resonance_to_spectral(0.9);
        assert_eq!(sig.psi, 0.9);
    }
}
