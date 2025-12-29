//! Spectral Signatures
//!
//! Spectral signatures encode the (ψ, ρ, ω) triplet with additional metadata.

use crate::triplet::PerformanceTriplet;
use serde::{Deserialize, Serialize};

/// Spectral Signature with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralSignature {
    /// The core performance triplet
    pub triplet: PerformanceTriplet,
    /// Timestamp when signature was computed
    pub timestamp: u64,
    /// Source identifier (e.g., address, seed, tx hash)
    pub source: String,
    /// Additional tags
    pub tags: Vec<String>,
    /// Confidence score [0, 1]
    pub confidence: f64,
}

impl SpectralSignature {
    /// Create a new spectral signature
    pub fn new(triplet: PerformanceTriplet, source: impl Into<String>) -> Self {
        Self {
            triplet,
            timestamp: 0,
            source: source.into(),
            tags: Vec::new(),
            confidence: 1.0,
        }
    }

    /// Create from individual components
    pub fn from_components(
        psi: f64,
        rho: f64,
        omega: f64,
        source: impl Into<String>,
    ) -> Self {
        Self::new(PerformanceTriplet::new(psi, rho, omega), source)
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Get the resonance score
    pub fn resonance(&self) -> f64 {
        self.triplet.resonance()
    }

    /// Get ψ component
    pub fn psi(&self) -> f64 {
        self.triplet.psi
    }

    /// Get ρ component
    pub fn rho(&self) -> f64 {
        self.triplet.rho
    }

    /// Get ω component
    pub fn omega(&self) -> f64 {
        self.triplet.omega
    }

    /// Check if signature is above threshold
    pub fn is_resonant(&self, threshold: f64) -> bool {
        self.resonance() >= threshold
    }

    /// Calculate similarity to another signature
    pub fn similarity(&self, other: &Self) -> f64 {
        1.0 / (1.0 + self.triplet.distance(&other.triplet))
    }

    /// Weighted confidence score
    pub fn weighted_resonance(&self) -> f64 {
        self.resonance() * self.confidence
    }
}

impl Default for SpectralSignature {
    fn default() -> Self {
        Self::new(PerformanceTriplet::default(), "unknown")
    }
}

impl std::fmt::Display for SpectralSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spectral[{}]: {} (conf={:.2})",
            self.source, self.triplet, self.confidence
        )
    }
}

/// Builder for spectral signatures
pub struct SpectralSignatureBuilder {
    triplet: PerformanceTriplet,
    source: String,
    timestamp: u64,
    tags: Vec<String>,
    confidence: f64,
}

impl SpectralSignatureBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            triplet: PerformanceTriplet::default(),
            source: String::new(),
            timestamp: 0,
            tags: Vec::new(),
            confidence: 1.0,
        }
    }

    /// Set triplet
    pub fn triplet(mut self, triplet: PerformanceTriplet) -> Self {
        self.triplet = triplet;
        self
    }

    /// Set components
    pub fn components(mut self, psi: f64, rho: f64, omega: f64) -> Self {
        self.triplet = PerformanceTriplet::new(psi, rho, omega);
        self
    }

    /// Set source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Add tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set confidence
    pub fn confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Build the signature
    pub fn build(self) -> SpectralSignature {
        SpectralSignature {
            triplet: self.triplet,
            source: self.source,
            timestamp: self.timestamp,
            tags: self.tags,
            confidence: self.confidence,
        }
    }
}

impl Default for SpectralSignatureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_creation() {
        let sig = SpectralSignature::from_components(0.8, 0.9, 0.7, "test");
        assert!((sig.psi() - 0.8).abs() < 1e-10);
        assert_eq!(sig.source, "test");
    }

    #[test]
    fn test_spectral_builder() {
        let sig = SpectralSignatureBuilder::new()
            .components(0.5, 0.5, 0.5)
            .source("builder_test")
            .tag("test")
            .confidence(0.9)
            .build();

        assert_eq!(sig.tags.len(), 1);
        assert!((sig.confidence - 0.9).abs() < 1e-10);
    }

    #[test]
    fn test_similarity() {
        let a = SpectralSignature::from_components(0.5, 0.5, 0.5, "a");
        let b = SpectralSignature::from_components(0.5, 0.5, 0.5, "b");
        let c = SpectralSignature::from_components(1.0, 1.0, 1.0, "c");

        assert!((a.similarity(&b) - 1.0).abs() < 1e-10);
        assert!(a.similarity(&c) < a.similarity(&b));
    }
}
