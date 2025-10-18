//! PHOSPHOROS Gateway - Unified API Gateway
//!
//! This crate provides a unified API gateway that exposes PHOSPHOROS functionality
//! including the Satellite forensic analysis subsystem and core resonance features.

#![forbid(unsafe_code)]

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use serde::Serialize;

pub use phosphoros_satellite;

/// Builds the complete PHOSPHOROS gateway router.
///
/// This includes all satellite endpoints and health checks.
pub fn build_gateway(satellite_engine: Arc<phosphoros_satellite::SatelliteEngine>) -> Router {
    let satellite_router = phosphoros_satellite::build_router(satellite_engine);
    
    Router::new()
        .route("/", get(root))
        .nest("/satellite", satellite_router)
}

async fn root() -> Json<impl Serialize> {
    Json(serde_json::json!({
        "service": "PHOSPHOROS Gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "/": "This help message",
            "/satellite/health": "Satellite subsystem health",
            "/satellite/v1/snapshots": "Manage blockchain snapshots",
            "/satellite/v1/analyze/:id": "Run forensic analysis",
            "/satellite/v1/reports/latest": "Get latest analysis report",
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_builds() {
        let engine = Arc::new(phosphoros_satellite::SatelliteEngine::default());
        let _router = build_gateway(engine);
    }
}

