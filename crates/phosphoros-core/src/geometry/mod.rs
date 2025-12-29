//! 5D Geometry primitives and Metatron Sacred Geometry
//!
//! This module provides the geometric foundation for PHOSPHOROS, including:
//!
//! - [`Point5D`]: 5-dimensional point/vector with full linear algebra operations
//! - [`MetatronGeometry`]: 13-node sacred geometry topology for 5D embedding
//!
//! ## 5D Information Space
//!
//! The 5D space coordinates are:
//! - `x, y, z`: Traditional spatial dimensions
//! - `w`: Temporal/4th dimension
//! - `v`: Semantic/information dimension
//!
//! ## Example: Point Operations
//!
//! ```rust
//! use phosphoros_core::Point5D;
//!
//! let a = Point5D::new(1.0, 2.0, 3.0, 4.0, 5.0);
//! let b = Point5D::new(5.0, 4.0, 3.0, 2.0, 1.0);
//!
//! // Dot product: 1*5 + 2*4 + 3*3 + 4*2 + 5*1 = 35
//! assert_eq!(a.dot(&b), 35.0);
//!
//! // Normalize to unit vector
//! let unit = a.normalize();
//! assert!((unit.norm() - 1.0).abs() < 1e-10);
//! ```
//!
//! ## Example: Metatron Embedding
//!
//! ```rust
//! use phosphoros_core::MetatronGeometry;
//!
//! let metatron = MetatronGeometry::new();
//!
//! // Embed an object hash into 5D space
//! let embedding = metatron.embed_object(0xDEADBEEF);
//!
//! // Embeddings are always normalized
//! assert!((embedding.norm() - 1.0).abs() < 1e-10);
//!
//! // Same hash always produces same embedding (deterministic)
//! let same = metatron.embed_object(0xDEADBEEF);
//! assert_eq!(embedding, same);
//! ```

mod metatron;
mod point5d;

pub use metatron::MetatronGeometry;
pub use point5d::Point5D;
