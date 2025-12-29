//! Resonance Engine trait and evaluation types
//!
//! This module defines the core trait that all resonance engines must implement,
//! along with the evaluation result type that determines whether a state passes
//! through the resonance gate or is blocked.

use serde::{Deserialize, Serialize};

/// Reason why a resonance evaluation was gated (blocked)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateReason {
    /// Low coherence (phase synchronization insufficient)
    LowCoherence,
    /// High fluctuation (instability detected)
    HighFluctuation,
    /// Monolith criterion failed (geometric conditions not met)
    MonolithFailed,
    /// Insufficient resonance threshold
    InsufficientResonance,
    /// Other custom gate condition
    Other,
}

/// Result of a resonance evaluation
///
/// Either the state is gated (blocked) with a reason, or it produces an output
/// vector and score that can be used for further processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Evaluation {
    /// State was gated (blocked) - no output
    Gated {
        /// The reason for gating
        reason: GateReason,
    },
    /// State passed - produced output vector and score
    Output {
        /// The 5D output vector
        vector: [f64; 5],
        /// The resonance score (0.0 - 1.0)
        score: f64,
    },
}

impl Evaluation {
    /// Check if this evaluation produced output (not gated)
    pub fn is_output(&self) -> bool {
        matches!(self, Evaluation::Output { .. })
    }

    /// Check if this evaluation was gated (blocked)
    pub fn is_gated(&self) -> bool {
        matches!(self, Evaluation::Gated { .. })
    }

    /// Get the output vector and score, if available
    pub fn output(&self) -> Option<([f64; 5], f64)> {
        match self {
            Evaluation::Output { vector, score } => Some((*vector, *score)),
            Evaluation::Gated { .. } => None,
        }
    }

    /// Get the gate reason, if gated
    pub fn gate_reason(&self) -> Option<GateReason> {
        match self {
            Evaluation::Gated { reason } => Some(*reason),
            Evaluation::Output { .. } => None,
        }
    }
}

/// Core trait for resonance engines
///
/// A resonance engine evaluates input states (perception, intention, gradient, geometry)
/// and produces either an output vector with score, or gates (blocks) the state with a reason.
///
/// All resonance engines must:
/// - Be deterministic (same inputs -> same outputs)
/// - Have resettable state
/// - Provide access to internal state for inspection
pub trait ResonanceEngine: Send + Sync {
    /// Internal state type that can be inspected
    type State;

    /// Reset the engine to initial state
    fn reset(&mut self);

    /// Evaluate a state at time t
    ///
    /// # Arguments
    /// * `t` - Time parameter (for temporal dynamics)
    /// * `perception` - Current 5D perception vector
    /// * `intention` - Intended 5D direction vector
    /// * `gradient` - Gradient field at current position
    /// * `theta` - Angular parameter (geometry-dependent)
    ///
    /// # Returns
    /// Either `Evaluation::Output` with resulting vector and score,
    /// or `Evaluation::Gated` with the reason for blocking.
    fn evaluate(
        &mut self,
        t: f64,
        perception: [f64; 5],
        intention: [f64; 5],
        gradient: [f64; 5],
        theta: f64,
    ) -> Evaluation;

    /// Get the current internal state
    fn state(&self) -> &Self::State;

    /// Get a mutable reference to the internal state
    fn state_mut(&mut self) -> &mut Self::State;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluation_is_output() {
        let eval = Evaluation::Output {
            vector: [1.0, 2.0, 3.0, 4.0, 5.0],
            score: 0.8,
        };
        assert!(eval.is_output());
        assert!(!eval.is_gated());
    }

    #[test]
    fn test_evaluation_is_gated() {
        let eval = Evaluation::Gated {
            reason: GateReason::LowCoherence,
        };
        assert!(eval.is_gated());
        assert!(!eval.is_output());
    }

    #[test]
    fn test_evaluation_output() {
        let vec = [1.0, 2.0, 3.0, 4.0, 5.0];
        let eval = Evaluation::Output {
            vector: vec,
            score: 0.8,
        };
        assert_eq!(eval.output(), Some((vec, 0.8)));
    }

    #[test]
    fn test_evaluation_gate_reason() {
        let eval = Evaluation::Gated {
            reason: GateReason::MonolithFailed,
        };
        assert_eq!(eval.gate_reason(), Some(GateReason::MonolithFailed));
    }
}
