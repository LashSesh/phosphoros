//! Metatron Geometry
//!
//! 13-node sacred geometry topology embedded in 5D space.

use serde::{Deserialize, Serialize};

/// A node in the Metatron geometry
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MetatronNode {
    /// Node index (0-12)
    pub index: usize,
    /// 5D position
    pub position: Point5D,
    /// Node type
    pub node_type: NodeType,
}

/// Type of node in the Metatron structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Central node (index 0)
    Center,
    /// Peripheral nodes (indices 1-12)
    Peripheral,
}

impl MetatronNode {
    /// Create a new node
    pub fn new(index: usize, position: Point5D) -> Self {
        Self {
            index,
            position,
            node_type: if index == 0 {
                NodeType::Center
            } else {
                NodeType::Peripheral
            },
        }
    }

    /// Check if this is the central node
    pub fn is_center(&self) -> bool {
        self.node_type == NodeType::Center
    }
}

/// 5-dimensional point
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point5D {
    /// Coordinates
    pub coords: [f64; 5],
}

impl Point5D {
    /// Create a new 5D point
    pub fn new(coords: [f64; 5]) -> Self {
        Self { coords }
    }

    /// Create the origin
    pub fn origin() -> Self {
        Self::new([0.0; 5])
    }

    /// Create from individual coordinates
    pub fn from_coords(x0: f64, x1: f64, x2: f64, x3: f64, x4: f64) -> Self {
        Self::new([x0, x1, x2, x3, x4])
    }

    /// Get coordinate by index
    pub fn get(&self, index: usize) -> f64 {
        self.coords.get(index).copied().unwrap_or(0.0)
    }

    /// Set coordinate by index
    pub fn set(&mut self, index: usize, value: f64) {
        if index < 5 {
            self.coords[index] = value;
        }
    }

    /// Calculate L2 norm
    pub fn norm(&self) -> f64 {
        self.coords.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    /// Normalize to unit length
    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n < 1e-10 {
            Self::origin()
        } else {
            Self::new(self.coords.map(|x| x / n))
        }
    }

    /// Dot product
    pub fn dot(&self, other: &Self) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Distance to another point
    pub fn distance(&self, other: &Self) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Add two points
    pub fn add(&self, other: &Self) -> Self {
        let mut result = [0.0; 5];
        for i in 0..5 {
            result[i] = self.coords[i] + other.coords[i];
        }
        Self::new(result)
    }

    /// Subtract two points
    pub fn sub(&self, other: &Self) -> Self {
        let mut result = [0.0; 5];
        for i in 0..5 {
            result[i] = self.coords[i] - other.coords[i];
        }
        Self::new(result)
    }

    /// Scale by scalar
    pub fn scale(&self, scalar: f64) -> Self {
        Self::new(self.coords.map(|x| x * scalar))
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mut result = [0.0; 5];
        for i in 0..5 {
            result[i] = self.coords[i] * (1.0 - t) + other.coords[i] * t;
        }
        Self::new(result)
    }
}

impl Default for Point5D {
    fn default() -> Self {
        Self::origin()
    }
}

impl std::fmt::Display for Point5D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:.3}, {:.3}, {:.3}, {:.3}, {:.3}]",
            self.coords[0],
            self.coords[1],
            self.coords[2],
            self.coords[3],
            self.coords[4]
        )
    }
}

/// The complete 13-node Metatron geometry
#[derive(Debug, Clone)]
pub struct MetatronGeometry {
    /// All 13 nodes
    pub nodes: [MetatronNode; 13],
}

impl MetatronGeometry {
    /// Create the standard Metatron geometry
    ///
    /// The geometry consists of:
    /// - 1 central node at origin
    /// - 12 peripheral nodes arranged symmetrically
    pub fn new() -> Self {
        use std::f64::consts::PI;

        let mut nodes = [MetatronNode::new(0, Point5D::origin()); 13];

        // Central node
        nodes[0] = MetatronNode::new(0, Point5D::origin());

        // 12 peripheral nodes
        // First 6 in the primary plane
        for i in 0..6 {
            let angle = (i as f64) * PI / 3.0;
            let coords = [
                angle.cos(),
                angle.sin(),
                0.0,
                0.0,
                0.0,
            ];
            nodes[i + 1] = MetatronNode::new(i + 1, Point5D::new(coords));
        }

        // Next 6 in the secondary plane (rotated in higher dimensions)
        for i in 0..6 {
            let angle = (i as f64) * PI / 3.0 + PI / 6.0;
            let coords = [
                0.5 * angle.cos(),
                0.5 * angle.sin(),
                (3.0_f64).sqrt() / 2.0,
                0.0,
                0.0,
            ];
            nodes[i + 7] = MetatronNode::new(i + 7, Point5D::new(coords));
        }

        Self { nodes }
    }

    /// Get node by index
    pub fn get_node(&self, index: usize) -> Option<&MetatronNode> {
        self.nodes.get(index)
    }

    /// Get the central node
    pub fn center(&self) -> &MetatronNode {
        &self.nodes[0]
    }

    /// Get all peripheral nodes
    pub fn peripherals(&self) -> &[MetatronNode] {
        &self.nodes[1..]
    }

    /// Find the nearest node to a point
    pub fn nearest_node(&self, point: &Point5D) -> &MetatronNode {
        self.nodes
            .iter()
            .min_by(|a, b| {
                a.position
                    .distance(point)
                    .partial_cmp(&b.position.distance(point))
                    .unwrap()
            })
            .unwrap()
    }

    /// Embed a point into the Metatron geometry
    /// Returns weights for each node
    pub fn embed(&self, point: &Point5D) -> [f64; 13] {
        let mut weights = [0.0; 13];
        let mut total = 0.0;

        for (i, node) in self.nodes.iter().enumerate() {
            let dist = node.position.distance(point);
            // Use inverse distance weighting with softening
            let w = 1.0 / (dist + 0.1);
            weights[i] = w;
            total += w;
        }

        // Normalize
        if total > 0.0 {
            for w in &mut weights {
                *w /= total;
            }
        }

        weights
    }

    /// Reconstruct a point from Metatron weights
    pub fn reconstruct(&self, weights: &[f64; 13]) -> Point5D {
        let mut result = [0.0; 5];

        for (i, &w) in weights.iter().enumerate() {
            for j in 0..5 {
                result[j] += w * self.nodes[i].position.coords[j];
            }
        }

        Point5D::new(result)
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
    fn test_point5d_creation() {
        let p = Point5D::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!((p.get(0) - 1.0).abs() < 1e-10);
        assert!((p.get(4) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point5d_norm() {
        let p = Point5D::new([1.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((p.norm() - 1.0).abs() < 1e-10);

        let q = Point5D::new([1.0, 1.0, 1.0, 1.0, 1.0]);
        assert!((q.norm() - 5.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_metatron_creation() {
        let geom = MetatronGeometry::new();
        assert_eq!(geom.nodes.len(), 13);
        assert!(geom.center().is_center());
        assert_eq!(geom.peripherals().len(), 12);
    }

    #[test]
    fn test_metatron_embed() {
        let geom = MetatronGeometry::new();
        let weights = geom.embed(&Point5D::origin());

        // Sum of weights should be 1
        let sum: f64 = weights.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);

        // Center should have highest weight for origin
        assert!(weights[0] > weights[1]);
    }

    #[test]
    fn test_metatron_roundtrip() {
        let geom = MetatronGeometry::new();

        // Embed and reconstruct a node position
        let original = geom.nodes[1].position;
        let weights = geom.embed(&original);
        let reconstructed = geom.reconstruct(&weights);

        // Should be close to original
        assert!(original.distance(&reconstructed) < 0.5);
    }
}
