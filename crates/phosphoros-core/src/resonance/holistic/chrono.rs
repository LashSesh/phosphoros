//! Chronokrator - Expansion Engine with temporal dynamics

use super::channel::ResonanceChannel;
use std::f64::consts::PI;

/// Chronokrator - Temporal expansion engine
///
/// Manages multiple resonance channels with temporal dynamics,
/// computing total dynamics and triggering based on adaptive thresholds.
#[derive(Debug, Clone)]
pub struct Chronokrator {
    /// Array of resonance channels
    pub channels: Vec<ResonanceChannel>,
    /// Global oscillator Ω(t)
    pub omega_global: f64,
    /// Dynamic threshold Θ(t)
    pub theta_threshold: f64,
    /// Whether to adapt threshold based on history
    pub theta_adaptive: bool,
    /// History of total dynamics values
    pub dtotal_history: Vec<f64>,
}

impl Chronokrator {
    /// Create a new Chronokrator with specified number of channels
    pub fn new(num_channels: usize, theta_threshold: f64) -> Self {
        let channels = (0..num_channels)
            .map(|i| {
                let mut channel = ResonanceChannel::new(i);
                // Initialize with different phases for phase diversity
                channel.phi = (i as f64) * 2.0 * PI / (num_channels as f64);
                channel
            })
            .collect();

        Self {
            channels,
            omega_global: 1.0,
            theta_threshold,
            theta_adaptive: true,
            dtotal_history: Vec::new(),
        }
    }

    /// Calculate total dynamics: D_total(t) = (∏ D_i(t)) · Ω(t)
    pub fn total_dynamics(&mut self, t: f64) -> f64 {
        let product: f64 = self.channels.iter_mut().map(|ch| ch.evaluate(t)).product();

        let dtotal = product * self.omega_global;
        self.dtotal_history.push(dtotal);

        dtotal
    }

    /// Check trigger condition: D_total(t) > Θ(t)
    pub fn check_trigger(&self, dtotal: f64) -> bool {
        dtotal > self.theta_threshold
    }

    /// Update adaptive threshold based on recent history
    pub fn update_threshold(&mut self) {
        if !self.theta_adaptive || self.dtotal_history.len() < 10 {
            return;
        }

        let recent: Vec<f64> = self.dtotal_history.iter().rev().take(10).copied().collect();

        let mean = recent.iter().sum::<f64>() / recent.len() as f64;
        let variance =
            recent.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / recent.len() as f64;

        // Θ(t) = mean + 0.5·sqrt(variance)
        self.theta_threshold = mean + 0.5 * variance.sqrt();
    }

    /// Excalibrate: Generate directed action vector from gradient
    ///
    /// Normalizes the gradient to a unit vector for consistent direction.
    pub fn excalibrate(&self, gradient: &[f64; 5]) -> [f64; 5] {
        let norm = gradient.iter().map(|&x| x * x).sum::<f64>().sqrt();

        if norm < 1e-12 {
            return [0.0; 5];
        }

        gradient.map(|x| x / norm)
    }

    /// Reset the Chronokrator to initial state
    pub fn reset(&mut self) {
        for ch in &mut self.channels {
            ch.reset();
        }
        self.omega_global = 1.0;
        self.dtotal_history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chronokrator_creation() {
        let chrono = Chronokrator::new(4, 0.5);
        assert_eq!(chrono.channels.len(), 4);
        assert_eq!(chrono.theta_threshold, 0.5);
    }

    #[test]
    fn test_channel_phases_distributed() {
        let chrono = Chronokrator::new(4, 0.5);
        // Check that phases are evenly distributed
        for (i, ch) in chrono.channels.iter().enumerate() {
            let expected_phi = (i as f64) * 2.0 * PI / 4.0;
            assert!((ch.phi - expected_phi).abs() < 1e-10);
        }
    }

    #[test]
    fn test_total_dynamics() {
        let mut chrono = Chronokrator::new(2, 0.5);
        let dtotal = chrono.total_dynamics(0.0);
        assert!(dtotal > 0.0);
        assert_eq!(chrono.dtotal_history.len(), 1);
    }

    #[test]
    fn test_check_trigger() {
        let chrono = Chronokrator::new(2, 0.5);
        assert!(chrono.check_trigger(0.6));
        assert!(!chrono.check_trigger(0.4));
    }

    #[test]
    fn test_excalibrate() {
        let chrono = Chronokrator::new(2, 0.5);
        let gradient = [3.0, 4.0, 0.0, 0.0, 0.0];
        let result = chrono.excalibrate(&gradient);
        let norm = result.iter().map(|&x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_update_threshold() {
        let mut chrono = Chronokrator::new(2, 0.5);
        // Need at least 10 samples
        for i in 0..15 {
            chrono.dtotal_history.push(i as f64 * 0.1);
        }
        let old_threshold = chrono.theta_threshold;
        chrono.update_threshold();
        // Threshold should have been updated
        assert_ne!(chrono.theta_threshold, old_threshold);
    }
}
