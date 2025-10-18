//! # PHOSPHOROS Kryptogenetik Core
//!
//! Prä-holographisches System zur 5D-Skalarprojektion
//! Integration von Triton, Metatron-IUL und Gabriel Cells
//!
//! ## Quick Start
//!
//! ```rust
//! use phosphoros_kryptogenetik::PhosphorosCore;
//!
//! let mut core = PhosphorosCore::new();
//! let seed_words = vec!["abandon", "ability", "able"];
//! let embeddings = core.embed_seed_phrase(&seed_words);
//! core.create_infogenom("analyzer".to_string(), 8);
//! let result = core.explore_keyspace("analyzer", 0x1337, 500)
//!     .expect("Exploration failed");
//! println!("Best Resonance: {:.6}", result.best_resonance);
//! ```
//!
//! ## Architecture
//!
//! - **Geometry**: 5D-Embedding via Metatron Sacred Geometry (13-Node → 5D)
//! - **Resonance**: Gabriel Cells with (ψ, ρ, ω) spectral signatures
//! - **Spiral**: Triton Golden Spiral with Ouroboros feedback
//! - **Alchemy**: Solve et Coagula with Merkaba-Gate Logic
//! - **Explorer**: QDASH exploration engine with TIC crystallization

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod alchemy;
pub mod bridge;
pub mod explorer;
pub mod geometry;
pub mod resonance;
pub mod spiral;

#[cfg(feature = "bip39-integration")]
pub mod bip39_ext;

mod error;

pub use alchemy::{InformationAlchemyEvaluator, SolveCoagulaDecision};
pub use bridge::PhosphorosCore;
pub use error::{Error, Result};
pub use explorer::{ExplorationResult, QDASHExplorer, TICCrystal};
pub use geometry::{MetatronGeometry, Point5D};
pub use resonance::{GabrielCell, Infogenom, SpectralSignature};
pub use spiral::TritonSpiralGenerator;
