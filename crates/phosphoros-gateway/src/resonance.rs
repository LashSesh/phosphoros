//! Resonance Analysis API
//!
//! Provides endpoints for 5D spectral resonance evaluation, spectral signature
//! computation, and holistic matrix analysis.

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use phosphoros_core::{Evaluation, GateReason, HolisticMatrix, ResonanceEngine, SpectralSignature};

/// Shared state for resonance analysis
#[derive(Clone)]
pub struct ResonanceState {
    /// Thread-safe resonance engine
    pub engine: Arc<RwLock<HolisticMatrix>>,
    /// History of resonance evaluations
    pub history: Arc<RwLock<Vec<ResonanceRecord>>>,
}

impl Default for ResonanceState {
    fn default() -> Self {
        Self {
            engine: Arc::new(RwLock::new(HolisticMatrix::default_config())),
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

/// Build the resonance analysis router
pub fn build_router(state: ResonanceState) -> Router {
    Router::new()
        .route("/analyze", post(analyze_resonance))
        .route("/spectral", post(compute_spectral))
        .route("/history", get(get_history))
        .route("/history/clear", post(clear_history))
        .with_state(state)
}

/// Request to analyze resonance of a state
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyzeRequest {
    /// Time parameter
    pub t: f64,
    /// Perception vector (5D)
    pub perception: [f64; 5],
    /// Intention vector (5D)
    pub intention: [f64; 5],
    /// Gradient vector (5D)
    pub gradient: [f64; 5],
    /// Theta parameter (phase angle)
    pub theta: f64,
    /// Optional label for tracking
    #[serde(default)]
    pub label: Option<String>,
}

/// Response from resonance analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum AnalyzeResponse {
    /// Analysis succeeded and produced an output
    Output {
        /// Action vector in 5D space
        vector: [f64; 5],
        /// Resonance score (0.0 to 1.0)
        score: f64,
        /// Optional label (echoed from request)
        label: Option<String>,
    },
    /// Analysis was gated (blocked) by resonance criteria
    Gated {
        /// Reason for gating
        reason: String,
        /// Optional label (echoed from request)
        label: Option<String>,
    },
}

/// Request to compute spectral signature
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SpectralRequest {
    /// Psi (ψ) - coherence/semantics
    pub psi: f64,
    /// Rho (ρ) - density/structure
    pub rho: f64,
    /// Omega (ω) - frequency/phase
    pub omega: f64,
}

/// Response with spectral signature metrics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SpectralResponse {
    /// Coherence (ψ)
    pub psi: f64,
    /// Density (ρ)
    pub rho: f64,
    /// Frequency (ω)
    pub omega: f64,
    /// Resonance (D = ψ·ρ·ω, invariant formula)
    pub resonance: f64,
    /// Energy (L2 norm)
    pub energy: f64,
}

/// Historical record of a resonance evaluation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResonanceRecord {
    /// Timestamp of evaluation
    pub timestamp: String,
    /// Input parameters
    pub request: AnalyzeRequest,
    /// Output result
    pub response: AnalyzeResponse,
}

/// Analyze resonance of a 5D state
///
/// Evaluates the holistic matrix with given perception, intention, and gradient
/// vectors. Returns either an action vector or a gating reason.
#[utoipa::path(
    post,
    path = "/api/v1/resonance/analyze",
    tag = "resonance",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, description = "Resonance analysis completed", body = AnalyzeResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn analyze_resonance(
    State(state): State<ResonanceState>,
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, StatusCode> {
    // Evaluate using the holistic matrix
    let mut engine = state.engine.write();
    let evaluation = engine.evaluate(req.t, req.perception, req.intention, req.gradient, req.theta);

    // Convert evaluation to response
    let response = match evaluation {
        Evaluation::Output { vector, score } => AnalyzeResponse::Output {
            vector,
            score,
            label: req.label.clone(),
        },
        Evaluation::Gated { reason } => AnalyzeResponse::Gated {
            reason: gate_reason_to_string(reason),
            label: req.label.clone(),
        },
    };

    // Record in history
    let record = ResonanceRecord {
        timestamp: chrono::Utc::now().to_rfc3339(),
        request: req,
        response: response.clone(),
    };
    state.history.write().push(record);

    Ok(Json(response))
}

/// Compute spectral signature metrics
///
/// Given (ψ, ρ, ω), computes resonance (D = ψ·ρ·ω) and energy metrics.
#[utoipa::path(
    post,
    path = "/api/v1/resonance/spectral",
    tag = "resonance",
    request_body = SpectralRequest,
    responses(
        (status = 200, description = "Spectral metrics computed", body = SpectralResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn compute_spectral(
    Json(req): Json<SpectralRequest>,
) -> Result<Json<SpectralResponse>, StatusCode> {
    let sig = SpectralSignature::new(req.psi, req.rho, req.omega);

    Ok(Json(SpectralResponse {
        psi: sig.psi,
        rho: sig.rho,
        omega: sig.omega,
        resonance: sig.resonance(),
        energy: sig.energy(),
    }))
}

/// Get resonance evaluation history
///
/// Returns all recorded resonance evaluations with timestamps.
#[utoipa::path(
    get,
    path = "/api/v1/resonance/history",
    tag = "resonance",
    responses(
        (status = 200, description = "Resonance history retrieved", body = Vec<ResonanceRecord>),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_history(
    State(state): State<ResonanceState>,
) -> Result<Json<Vec<ResonanceRecord>>, StatusCode> {
    let history = state.history.read();
    Ok(Json(history.clone()))
}

/// Clear resonance history
///
/// Removes all stored resonance evaluation records.
#[utoipa::path(
    post,
    path = "/api/v1/resonance/history/clear",
    tag = "resonance",
    responses(
        (status = 200, description = "History cleared successfully"),
        (status = 500, description = "Internal server error")
    )
)]
async fn clear_history(State(state): State<ResonanceState>) -> StatusCode {
    state.history.write().clear();
    StatusCode::OK
}

/// Convert GateReason to human-readable string
fn gate_reason_to_string(reason: GateReason) -> String {
    match reason {
        GateReason::LowCoherence => "Low coherence (ψ below threshold)".to_string(),
        GateReason::HighFluctuation => "High fluctuation (chaotic state)".to_string(),
        GateReason::MonolithFailed => "Monolith criterion not met (geometric alignment)".to_string(),
        GateReason::InsufficientResonance => "Insufficient resonance threshold".to_string(),
        GateReason::Other => "Gated by custom criterion".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_signature() {
        let sig = SpectralSignature::new(0.8, 0.6, 0.5);
        assert_eq!(sig.psi, 0.8);
        assert_eq!(sig.rho, 0.6);
        assert_eq!(sig.omega, 0.5);
        assert!((sig.resonance() - 0.24).abs() < 1e-10); // ψ·ρ·ω = 0.8*0.6*0.5
    }

    #[test]
    fn test_resonance_state_default() {
        let state = ResonanceState::default();
        assert_eq!(state.history.read().len(), 0);
    }
}
