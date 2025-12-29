//! Holistic Matrix - Complete integration of resonance engines

use super::{
    chrono::Chronokrator,
    kosmo::{Kosmokrator, PhaseState},
    mandorla::MandorlaField,
    monolith::Monolith,
    torus::TorusTopology,
};
use crate::resonance::traits::{Evaluation, GateReason, ResonanceEngine};
use std::f64::consts::PI;

/// Complete state of the Holistic Matrix
#[derive(Debug, Clone)]
pub struct MatrixState {
    /// Current simulation time
    pub time: f64,
    /// Kosmokrator coherence measure κ(t)
    pub kosmokrator_coherence: f64,
    /// Chronokrator total divergence D_total
    pub chronokrator_dtotal: f64,
    /// Whether the Monolith was triggered
    pub monolith_triggered: bool,
    /// Torus spatial phase φ_s
    pub torus_spatial: f64,
    /// Torus temporal phase φ_t
    pub torus_temporal: f64,
    /// Number of outputs generated
    pub output_count: usize,
}

/// O.P.H.A.N. (Orbital Projection Hyperstructure Asymmetric Node)
#[derive(Debug, Clone)]
struct Ophan {
    pub id: usize,
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    pub phase: f64,
    pub amplitude: f64,
}

impl Ophan {
    fn new(id: usize, phase: f64) -> Self {
        Self {
            id,
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            phase,
            amplitude: 1.0,
        }
    }

    fn update(&mut self, signal: f64, learning_rate: f64) {
        self.psi = (1.0 - learning_rate) * self.psi + learning_rate * signal;
        self.psi = self.psi.clamp(0.1, 2.0);
    }
}

/// Central Konus (convergence cone)
#[derive(Debug, Clone)]
struct Konus {
    pub phase: f64,
    pub frequency: f64,
    pub convergence_field: [f64; 5],
}

impl Konus {
    fn new() -> Self {
        Self {
            phase: 0.0,
            frequency: 1.0,
            convergence_field: [0.0; 5],
        }
    }

    fn update_convergence(&mut self, ophanim: &[Ophan]) {
        for (i, ophan) in ophanim.iter().enumerate() {
            if i < 5 {
                self.convergence_field[i] = ophan.psi * ophan.rho * ophan.omega;
            }
        }
    }
}

/// O.P.H.A.N. Array - 4 Ophanim + Konus
#[derive(Debug, Clone)]
struct OphanArray {
    pub ophanim: Vec<Ophan>,
    pub konus: Konus,
    pub learning_rate: f64,
}

impl OphanArray {
    fn new() -> Self {
        let ophanim = (0..4)
            .map(|i| {
                let phase = (i as f64) * PI / 2.0; // 0°, 90°, 180°, 270°
                Ophan::new(i, phase)
            })
            .collect();

        Self {
            ophanim,
            konus: Konus::new(),
            learning_rate: 0.1,
        }
    }

    fn update_from_vector(&mut self, vector: &[f64; 5]) {
        for (i, ophan) in self.ophanim.iter_mut().enumerate() {
            let signal = if i < vector.len() {
                vector[i]
            } else {
                vector.iter().sum::<f64>() / vector.len() as f64
            };

            ophan.update(signal, self.learning_rate);
        }

        self.konus.update_convergence(&self.ophanim);
    }

    fn convergence_field(&self) -> [f64; 5] {
        self.konus.convergence_field
    }
}

/// Pfauenthron - Integration of O.P.H.A.N., Mandorla, and Monolith
#[derive(Debug, Clone)]
struct Pfauenthron {
    pub ophan_array: OphanArray,
    pub mandorla: MandorlaField,
    pub monolith: Monolith,
}

impl Pfauenthron {
    fn new(eta_threshold: f64) -> Self {
        Self {
            ophan_array: OphanArray::new(),
            mandorla: MandorlaField::new(eta_threshold),
            monolith: Monolith::new(),
        }
    }

    fn evaluate(
        &mut self,
        perception: [f64; 5],
        intention: [f64; 5],
        gradient: [f64; 5],
        theta: f64,
        t: f64,
    ) -> Option<[f64; 5]> {
        // Update O.P.H.A.N. Array
        self.ophan_array.update_from_vector(&perception);

        // Update Mandorla
        self.mandorla.update_gabriel(perception);
        self.mandorla.update_oriphiel(intention);

        // Check Mandorla singularity
        let mandorla_mag = self.mandorla.magnitude();

        // Monolith evaluation
        let konus_field = self.ophan_array.convergence_field();
        let triggered = self
            .monolith
            .evaluate_criterion(&gradient, &konus_field, theta, t);

        if triggered && mandorla_mag >= self.mandorla.eta_threshold {
            Some(self.monolith.excalibrate())
        } else {
            None
        }
    }
}

/// Holistic Matrix - Complete integration of all resonance engines
///
/// This is the main resonance engine that integrates:
/// - Kosmokrator (exclusion via Proof-of-Resonance)
/// - Chronokrator (temporal expansion)
/// - Pfauenthron (O.P.H.A.N. + Mandorla + Monolith)
/// - Torus topology (S¹ × S¹ phase space)
#[derive(Debug, Clone)]
pub struct HolisticMatrix {
    /// Kosmokrator - Exclusion engine with Proof-of-Resonance
    pub kosmokrator: Kosmokrator,
    /// Chronokrator - Temporal expansion and divergence tracking
    pub chronokrator: Chronokrator,
    /// Pfauenthron - O.P.H.A.N. array with Mandorla field
    pub pfauenthron: Pfauenthron,
    /// Torus topology - S¹ × S¹ phase space embedding
    pub torus: TorusTopology,
    /// Current simulation time
    pub time: f64,
    /// History of 5D output vectors
    pub output_history: Vec<Option<[f64; 5]>>,
    /// Cached matrix state for performance
    cached_state: MatrixState,
}

impl HolisticMatrix {
    /// Create a new Holistic Matrix with specified parameters
    ///
    /// # Arguments
    /// * `kappa_threshold` - Coherence threshold for Kosmokrator
    /// * `num_channels` - Number of resonance channels for Chronokrator
    /// * `theta_threshold` - Dynamics threshold for Chronokrator
    /// * `eta_threshold` - Mandorla singularity threshold
    pub fn new(
        kappa_threshold: f64,
        num_channels: usize,
        theta_threshold: f64,
        eta_threshold: f64,
    ) -> Self {
        Self {
            kosmokrator: Kosmokrator::new(kappa_threshold, 0.05, 0.1),
            chronokrator: Chronokrator::new(num_channels, theta_threshold),
            pfauenthron: Pfauenthron::new(eta_threshold),
            torus: TorusTopology::new(),
            time: 0.0,
            output_history: Vec::new(),
            cached_state: MatrixState {
                time: 0.0,
                kosmokrator_coherence: 0.0,
                chronokrator_dtotal: 0.0,
                monolith_triggered: false,
                torus_spatial: 0.0,
                torus_temporal: 0.0,
                output_count: 0,
            },
        }
    }

    /// Create with default parameters
    pub fn default_config() -> Self {
        Self::new(0.8, 8, 0.5, 0.7)
    }

    /// Main evaluation cycle
    fn evaluate_cycle(
        &mut self,
        input_state: PhaseState,
        perception: [f64; 5],
        intention: [f64; 5],
        gradient: [f64; 5],
        delta_time: f64,
    ) -> Evaluation {
        self.time += delta_time;

        // 1. KOSMOKRATOR: Proof-of-Resonance
        let por_passed = self.kosmokrator.check_por(&input_state);

        if !por_passed {
            self.output_history.push(None);
            return Evaluation::Gated {
                reason: GateReason::LowCoherence,
            };
        }

        // 2. CHRONOKRATOR: Dynamics check
        let dtotal = self.chronokrator.total_dynamics(self.time);
        let trigger = self.chronokrator.check_trigger(dtotal);

        if !trigger {
            self.output_history.push(None);
            return Evaluation::Gated {
                reason: GateReason::InsufficientResonance,
            };
        }

        // 3. PFAUENTHRON: Monolith singularity
        let action = self.pfauenthron.evaluate(
            perception,
            intention,
            gradient,
            self.chronokrator.theta_threshold,
            self.time,
        );

        // 4. TORUS: Update topology
        let kappa = self.kosmokrator.coherence(&input_state);
        self.torus.update(kappa * 0.1, dtotal * 0.1);

        // 5. Adaptive threshold update
        self.chronokrator.update_threshold();

        match action {
            Some(vector) => {
                self.output_history.push(Some(vector));
                // Calculate score from resonance components
                let score = dtotal * kappa;
                Evaluation::Output { vector, score }
            }
            None => {
                self.output_history.push(None);
                Evaluation::Gated {
                    reason: GateReason::MonolithFailed,
                }
            }
        }
    }

    /// Get complete state export
    pub fn get_state(&self) -> MatrixState {
        MatrixState {
            time: self.time,
            kosmokrator_coherence: self
                .kosmokrator
                .coherence_history
                .last()
                .copied()
                .unwrap_or(0.0),
            chronokrator_dtotal: self
                .chronokrator
                .dtotal_history
                .last()
                .copied()
                .unwrap_or(0.0),
            monolith_triggered: self.pfauenthron.monolith.singularity_triggered,
            torus_spatial: self.torus.spatial_phase,
            torus_temporal: self.torus.temporal_phase,
            output_count: self.output_history.iter().filter(|o| o.is_some()).count(),
        }
    }

    fn update_cached_state(&mut self) {
        self.cached_state = self.get_state();
    }
}

impl ResonanceEngine for HolisticMatrix {
    type State = MatrixState;

    fn reset(&mut self) {
        self.kosmokrator.reset();
        self.chronokrator.reset();
        self.pfauenthron.mandorla.reset();
        self.pfauenthron.monolith.reset();
        self.torus.reset();
        self.time = 0.0;
        self.output_history.clear();
        self.update_cached_state();
    }

    fn evaluate(
        &mut self,
        t: f64,
        perception: [f64; 5],
        intention: [f64; 5],
        gradient: [f64; 5],
        _theta: f64,
    ) -> Evaluation {
        // Create phase state from perception
        let mut state = PhaseState::new(perception.len());
        for (i, &p) in perception.iter().enumerate() {
            state.amplitudes[i] = p.abs();
            state.phases[i] = if p < 0.0 { PI } else { 0.0 };
        }
        state.normalize();

        // Evaluate with time delta
        let delta_time = if self.time > 0.0 { t - self.time } else { 0.01 };
        let result = self.evaluate_cycle(state, perception, intention, gradient, delta_time);
        self.update_cached_state();
        result
    }

    fn state(&self) -> &Self::State {
        &self.cached_state
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.cached_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holistic_matrix_creation() {
        let matrix = HolisticMatrix::new(0.8, 4, 0.5, 0.7);
        assert_eq!(matrix.time, 0.0);
    }

    #[test]
    fn test_holistic_matrix_evaluate() {
        let mut matrix = HolisticMatrix::default_config();
        let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
        let intention = [0.6, 0.6, 0.6, 0.6, 0.6];
        let gradient = [1.0, 0.0, 0.0, 0.0, 0.0];

        let result = matrix.evaluate(1.0, perception, intention, gradient, 0.5);
        // Should get some result (either Output or Gated)
        assert!(result.is_output() || result.is_gated());
    }

    #[test]
    fn test_holistic_matrix_reset() {
        let mut matrix = HolisticMatrix::default_config();
        matrix.time = 10.0;
        matrix.reset();
        assert_eq!(matrix.time, 0.0);
        assert_eq!(matrix.output_history.len(), 0);
    }
}
