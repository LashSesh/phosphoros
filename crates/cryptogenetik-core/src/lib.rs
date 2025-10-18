//! # Cryptogenetik Core
//!
//! Search and optimization core for PHOSPHOROS cryptogenetik system.
//!
//! ## Features
//!
//! - **Search Pipeline**: Triton search with resonance evaluation
//! - **Operators**: WT (contraction), SW (threshold), DK (lock), PI (canonical)
//! - **Score Hooks**: Early scoring with checksum, partial words, patterns
//! - **Resonance Integration**: Pluggable resonance engines via trait
//!
//! ## Quick Start
//!
//! ```rust
//! use cryptogenetik_core::{PhosphorosCore, HookSet, OperatorSet};
//! use phosphoros_core::HolisticMatrix;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create resonance engine
//! let engine = HolisticMatrix::default_config();
//!
//! // Create search core
//! let mut core = PhosphorosCore::default_config(engine);
//!
//! // Explore
//! let words = vec!["test".to_string()];
//! let result = core.explore(words, 1000, 12345)?;
//!
//! println!("Best resonance: {}", result.best_resonance);
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
pub mod bridge;
pub mod optimizer;
pub mod score_hooks;
pub mod triton;

pub use bridge::{Outcome, PhosphorosCore};
pub use error::{Error, Result};
pub use optimizer::OperatorSet;
pub use score_hooks::HookSet;
pub use triton::{SearchResult, TritonPipeline};

