//! Solve Coagula Decision - Merkaba Gate Logic

use crate::resonance::SpectralSignature;

/// Solve et Coagula decision engine with Merkaba Gate logic
pub struct SolveCoagulaDecision {
    phi_threshold: f64,
    theta_dynamic: f64,
    theta_ema_gamma: f64,
    spike_delta: f64,
    gate_open: bool,
    phase_history: Vec<(String, f64, f64)>,
}

impl SolveCoagulaDecision {
    /// Create a new decision engine with default parameters
    pub fn new() -> Self {
        Self {
            phi_threshold: 0.5,
            theta_dynamic: 0.5,
            theta_ema_gamma: 0.2,
            spike_delta: 0.05,
            gate_open: false,
            phase_history: Vec::new(),
        }
    }

    /// Evaluate phase (liquid/solid) based on order measure Φ
    pub fn evaluate_phase(&mut self, signature: SpectralSignature) -> String {
        let d = signature.resonance();
        let kappa = 5.0;
        let theta_adaptive = 0.125;

        // Order measure Φ = σ(κ[D - Θ])
        let phi = 1.0 / (1.0 + (-kappa * (d - theta_adaptive)).exp());

        let phase = if phi >= self.phi_threshold {
            "fest"
        } else {
            "flüssig"
        };

        self.phase_history.push((phase.to_string(), phi, d));
        phase.to_string()
    }

    /// Check Proof of Resonance condition
    pub fn check_proof_of_resonance(&self, signature: SpectralSignature) -> bool {
        signature.psi >= 0.1
            && signature.rho >= 0.1
            && signature.omega >= 0.1
            && signature.omega <= 0.9
    }

    /// Evaluate Merkaba Gate condition
    pub fn check_merkaba_gate(&mut self, signature: SpectralSignature) -> bool {
        let por_condition = self.check_proof_of_resonance(signature);
        let phase = self.evaluate_phase(signature);
        let phase_condition = phase == "fest";
        let spike_condition = signature.resonance() > (self.theta_dynamic + self.spike_delta);

        // Gate decision
        self.gate_open = por_condition && phase_condition && spike_condition;

        // Update dynamic threshold (EMA)
        self.theta_dynamic = (1.0 - self.theta_ema_gamma) * self.theta_dynamic
            + self.theta_ema_gamma * signature.resonance();

        self.gate_open
    }

    /// Check if gate is currently open
    pub fn is_gate_open(&self) -> bool {
        self.gate_open
    }
}

impl Default for SolveCoagulaDecision {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_creation() {
        let decision = SolveCoagulaDecision::new();
        assert!(!decision.is_gate_open());
    }

    #[test]
    fn test_proof_of_resonance() {
        let decision = SolveCoagulaDecision::new();
        let sig = SpectralSignature::new(0.5, 0.5, 0.5);
        assert!(decision.check_proof_of_resonance(sig));
    }

    #[test]
    fn test_merkaba_gate() {
        let mut decision = SolveCoagulaDecision::new();
        let sig = SpectralSignature::new(0.9, 0.9, 0.8);
        let _ = decision.check_merkaba_gate(sig);
        // Gate may or may not be open depending on conditions
    }
}
