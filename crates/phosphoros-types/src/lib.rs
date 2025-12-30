//! Shared Types for PHOSPHOROS Ecosystem
//!
//! This crate provides fundamental types used across all PHOSPHOROS crates:
//! - Performance Triplet (ψ, ρ, ω)
//! - Metatron Geometry (13-node topology)
//! - Spectral Signatures
//!
//! # Usage
//!
//! ```rust
//! use phosphoros_types::{PerformanceTriplet, SpectralSignature};
//!
//! let triplet = PerformanceTriplet::new(0.8, 0.9, 0.7);
//! assert!(triplet.resonance() > 0.5);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod triplet;
mod geometry;
mod spectral;

pub use triplet::PerformanceTriplet;
pub use geometry::{MetatronNode, MetatronGeometry, Point5D};
pub use spectral::{SpectralSignature, SpectralSignatureBuilder};
