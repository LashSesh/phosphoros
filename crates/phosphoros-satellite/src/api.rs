//! REST API for the Satellite subsystem (optional, feature-gated).
//!
//! This module provides HTTP endpoints for ingesting snapshots, running analyses,
//! and retrieving reports.

#![cfg(feature = "api")]

use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::engine::SatelliteEngine;
use crate::models::{AnalysisReport, AnalysisRequest, SnapshotIngest};

/// Shared state for API handlers.
#[derive(Clone)]
pub struct ApiState {
    engine: Arc<SatelliteEngine>,
}

/// Builds the API router with all endpoints.
pub fn build_router(engine: Arc<SatelliteEngine>) -> Router {
    let state = ApiState { engine };
    Router::new()
        .route("/health", get(health))
        .route("/v1/snapshots", get(list_snapshots).post(ingest_snapshot))
        .route("/v1/analyze/:id", post(run_analysis))
        .route("/v1/reports/latest", get(latest_report))
        .with_state(state)
}

async fn health() -> Json<impl Serialize> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn list_snapshots(State(state): State<ApiState>) -> Json<Vec<serde_json::Value>> {
    let snapshots = state
        .engine
        .list_snapshots()
        .into_iter()
        .map(|record| {
            serde_json::json!({
                "id": record.id,
                "label": record.label,
                "captured_at": record.captured_at,
                "entities": record.observations.len(),
            })
        })
        .collect();
    Json(snapshots)
}

async fn ingest_snapshot(
    State(state): State<ApiState>,
    Json(payload): Json<SnapshotIngest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let record = state
        .engine
        .ingest(payload)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    tracing::info!(snapshot_id = %record.id, "snapshot ingested via API");
    Ok(Json(serde_json::json!({
        "id": record.id,
        "captured_at": record.captured_at,
    })))
}

async fn run_analysis(
    State(state): State<ApiState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(payload): Json<AnalysisRequest>,
) -> Result<Json<AnalysisReport>, StatusCode> {
    let report = state
        .engine
        .analyze(id, payload)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(report))
}

async fn latest_report(State(state): State<ApiState>) -> Result<Json<AnalysisReport>, StatusCode> {
    if let Some(report) = state.engine.last_report() {
        Ok(Json(report))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
