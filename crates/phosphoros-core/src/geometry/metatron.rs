//! Metatron Sacred Geometry - 13-Node topological structure with 5D projection
//!
//! The Metatron geometry provides a canonical embedding topology for mapping
//! arbitrary objects into 5D space. It consists of 13 nodes arranged in a
//! sacred geometric pattern.
//!
//! ## Node Structure
//!
//! ```text
//!        ┌─────────────────────┐
//!        │  Metatron 13-Node   │
//!        └─────────────────────┘
//!
//!     Node 0: Center (0, 0, 0)
//!
//!     Nodes 1-6: Hexagon ring
//!       ╱ ╲
//!      3   2
//!     │     │
//!     4  0  1
//!     │     │
//!      5   6
//!       ╲ ╱
//!
//!     Nodes 7-12: Cube vertices
//!     (±0.5, ±0.5, ±0.5)
//! ```
//!
//! ## 5D Projection
//!
//! Each 3D node is projected to 5D as `[x, y, z, r, φ]` where:
//! - `x, y, z`: Original 3D coordinates
//! - `r`: Radial distance from origin
//! - `φ`: Angular position (atan2(y, x))
//!
//! ## Usage
//!
//! ```rust
//! use phosphoros_core::MetatronGeometry;
//!
//! let metatron = MetatronGeometry::new();
//!
//! // Embed any object via its hash
//! let embedding = metatron.embed_object(42u64);
//!
//! // Access specific nodes
//! let center = metatron.get_node_5d(0);  // Center node
//! let hex1 = metatron.get_node_5d(1);    // First hexagon node
//! ```

use super::Point5D;

/// Metatron Sacred Geometry with 13 canonical nodes.
///
/// Provides deterministic embedding of objects into 5D space via weighted
/// interpolation of canonical nodes. The same hash always produces the
/// same embedding.
///
/// # Node Layout
///
/// - **Node 0**: Center at origin
/// - **Nodes 1-6**: Hexagonal ring in XY plane
/// - **Nodes 7-12**: Cube vertices (subset)
///
/// # Example
///
/// ```rust
/// use phosphoros_core::MetatronGeometry;
///
/// let metatron = MetatronGeometry::default();
///
/// // Different hashes produce different (but deterministic) embeddings
/// let a = metatron.embed_object(123);
/// let b = metatron.embed_object(456);
/// assert_ne!(a, b);
///
/// // Same hash always produces same result
/// let a2 = metatron.embed_object(123);
/// assert_eq!(a, a2);
/// ```
pub struct MetatronGeometry {
    #[allow(dead_code)]
    nodes_3d: [[f64; 3]; 13],
    projection_5d: [Point5D; 13],
}

impl MetatronGeometry {
    /// Create a new Metatron geometry with canonical 13-node structure
    pub fn new() -> Self {
        let sqrt3 = 3.0_f64.sqrt();

        // 13 canonical Metatron nodes
        // 0: Center
        // 1-6: Hexagon
        // 7-12: Cube vertices
        let nodes_3d = [
            [0.0, 0.0, 0.0],           // 0: Center
            [1.0, 0.0, 0.0],           // 1: H1
            [0.5, sqrt3 / 2.0, 0.0],   // 2: H2
            [-0.5, sqrt3 / 2.0, 0.0],  // 3: H3
            [-1.0, 0.0, 0.0],          // 4: H4
            [-0.5, -sqrt3 / 2.0, 0.0], // 5: H5
            [0.5, -sqrt3 / 2.0, 0.0],  // 6: H6
            [0.5, 0.5, 0.5],           // 7: Q1
            [0.5, 0.5, -0.5],          // 8: Q2
            [0.5, -0.5, 0.5],          // 9: Q3
            [0.5, -0.5, -0.5],         // 10: Q4
            [-0.5, 0.5, 0.5],          // 11: Q5
            [-0.5, 0.5, -0.5],         // 12: Q6
        ];

        // Project to 5D: [x, y, z, r, φ]
        let projection_5d = nodes_3d.map(|node| {
            let x = node[0];
            let y = node[1];
            let z = node[2];
            let r = (x * x + y * y + z * z).sqrt();
            let phi = y.atan2(x);
            Point5D::new(x, y, z, r, phi).normalize()
        });

        Self {
            nodes_3d,
            projection_5d,
        }
    }

    /// Embed an object (represented by hash) into 5D via Metatron topology
    pub fn embed_object(&self, obj_hash: u64) -> Point5D {
        let weights = self.hash_to_weights(obj_hash);

        let mut embedding = Point5D::zero();
        for (i, weight) in weights.iter().enumerate() {
            let node = self.projection_5d[i];
            embedding = embedding.add(&node.scale(*weight));
        }

        embedding.normalize()
    }

    /// Convert hash to 13 probability weights (deterministic)
    fn hash_to_weights(&self, h: u64) -> [f64; 13] {
        let mut weights = [0.0; 13];
        for (i, weight) in weights.iter_mut().enumerate() {
            let seed = h.wrapping_add(i as u64 * 0x9e3779b9);
            *weight = ((seed as f64 * 0.0001).sin()).abs();
        }

        let sum: f64 = weights.iter().sum();
        weights.iter_mut().for_each(|w| *w /= sum + 1e-10);
        weights
    }

    /// Get a specific 5D node by index (0-12)
    pub fn get_node_5d(&self, index: usize) -> Point5D {
        self.projection_5d[index]
    }
}

impl Default for MetatronGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metatron_geometry() {
        let metatron = MetatronGeometry::new();
        // Center node (0,0,0,0,0) normalizes to itself (zero vector)
        let node1 = metatron.get_node_5d(1);
        assert!((node1.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_embedding_deterministic() {
        let metatron = MetatronGeometry::new();
        let hash = 0x1234_5678_9abc_def0;
        let emb1 = metatron.embed_object(hash);
        let emb2 = metatron.embed_object(hash);
        assert_eq!(emb1, emb2);
    }

    #[test]
    fn test_embedding_normalized() {
        let metatron = MetatronGeometry::new();
        let emb = metatron.embed_object(42);
        assert!((emb.norm() - 1.0).abs() < 1e-10);
    }
}
