//! 5-dimensional point in information space

use serde::{Deserialize, Serialize};

/// 5-dimensional point in information space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point5D {
    /// 5D coordinates [x, y, z, w, v]
    pub coords: [f64; 5],
}

impl Point5D {
    /// Create a new 5D point
    pub fn new(x: f64, y: f64, z: f64, w: f64, v: f64) -> Self {
        Self {
            coords: [x, y, z, w, v],
        }
    }

    /// Create a zero point
    pub fn zero() -> Self {
        Self { coords: [0.0; 5] }
    }

    /// Calculate L2 norm (Euclidean length)
    pub fn norm(&self) -> f64 {
        self.coords.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Normalize to unit length (L2 norm = 1)
    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n < 1e-12 {
            return *self;
        }
        Self {
            coords: self.coords.map(|x| x / n),
        }
    }

    /// Calculate dot product with another point
    pub fn dot(&self, other: &Self) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Calculate Euclidean distance to another point
    pub fn distance(&self, other: &Self) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Add another point (vector addition)
    pub fn add(&self, other: &Self) -> Self {
        let mut coords = [0.0; 5];
        for (i, coord) in coords.iter_mut().enumerate() {
            *coord = self.coords[i] + other.coords[i];
        }
        Self { coords }
    }

    /// Scale by a scalar value
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            coords: self.coords.map(|x| x * scalar),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point5d_norm() {
        let p = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
        assert!((p.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_point5d_dot() {
        let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);
        assert!((p1.dot(&p2)).abs() < 1e-10);
    }

    #[test]
    fn test_point5d_distance() {
        let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);
        assert!((p1.distance(&p2) - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_point5d_normalize() {
        let p = Point5D::new(3.0, 4.0, 0.0, 0.0, 0.0);
        let normalized = p.normalize();
        assert!((normalized.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_point5d_scale() {
        let p = Point5D::new(1.0, 2.0, 3.0, 4.0, 5.0);
        let scaled = p.scale(2.0);
        assert!((scaled.coords[0] - 2.0).abs() < 1e-10);
        assert!((scaled.coords[4] - 10.0).abs() < 1e-10);
    }
}
