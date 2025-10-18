//! Torus Topology - S¹ × S¹ phase space structure

use std::f64::consts::PI;

/// Torus Topology - S¹ × S¹ representing spatial and temporal phase circles
///
/// The torus provides a natural phase space for holistic resonance,
/// where both spatial and temporal phases evolve on circular manifolds.
#[derive(Debug, Clone, Copy)]
pub struct TorusTopology {
    /// Spatial phase (S¹_space)
    pub spatial_phase: f64,
    /// Temporal phase (S¹_time)
    pub temporal_phase: f64,
}

impl TorusTopology {
    /// Create a new torus topology at origin
    pub fn new() -> Self {
        Self {
            spatial_phase: 0.0,
            temporal_phase: 0.0,
        }
    }

    /// Create with initial phases
    pub fn with_phases(spatial_phase: f64, temporal_phase: f64) -> Self {
        Self {
            spatial_phase: spatial_phase % (2.0 * PI),
            temporal_phase: temporal_phase % (2.0 * PI),
        }
    }

    /// Update phases (wraps around 2π)
    pub fn update(&mut self, delta_space: f64, delta_time: f64) {
        self.spatial_phase = (self.spatial_phase + delta_space) % (2.0 * PI);
        self.temporal_phase = (self.temporal_phase + delta_time) % (2.0 * PI);
    }

    /// Get 3D coordinates on torus surface
    ///
    /// # Arguments
    /// * `major_radius` - Distance from torus center to tube center
    /// * `minor_radius` - Radius of the tube
    pub fn coordinates(&self, major_radius: f64, minor_radius: f64) -> [f64; 3] {
        let theta = self.spatial_phase;
        let phi = self.temporal_phase;

        [
            (major_radius + minor_radius * phi.cos()) * theta.cos(),
            (major_radius + minor_radius * phi.cos()) * theta.sin(),
            minor_radius * phi.sin(),
        ]
    }

    /// Reset to origin
    pub fn reset(&mut self) {
        self.spatial_phase = 0.0;
        self.temporal_phase = 0.0;
    }
}

impl Default for TorusTopology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_creation() {
        let torus = TorusTopology::new();
        assert_eq!(torus.spatial_phase, 0.0);
        assert_eq!(torus.temporal_phase, 0.0);
    }

    #[test]
    fn test_update_wraps() {
        let mut torus = TorusTopology::new();
        torus.update(3.0 * PI, 4.0 * PI);
        // Should wrap around
        assert!((torus.spatial_phase - PI).abs() < 1e-10);
        assert!(torus.temporal_phase.abs() < 1e-10);
    }

    #[test]
    fn test_coordinates() {
        let torus = TorusTopology::new();
        let coords = torus.coordinates(2.0, 1.0);
        // At (0, 0), should be [major_radius + minor_radius, 0, 0]
        assert!((coords[0] - 3.0).abs() < 1e-10);
        assert!(coords[1].abs() < 1e-10);
        assert!(coords[2].abs() < 1e-10);
    }

    #[test]
    fn test_with_phases() {
        let torus = TorusTopology::with_phases(PI / 2.0, PI / 4.0);
        assert!((torus.spatial_phase - PI / 2.0).abs() < 1e-10);
        assert!((torus.temporal_phase - PI / 4.0).abs() < 1e-10);
    }
}
