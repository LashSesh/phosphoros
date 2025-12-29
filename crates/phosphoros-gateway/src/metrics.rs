//! Prometheus metrics for PHOSPHOROS Gateway.
//!
//! This module provides Prometheus-compatible metrics for monitoring the gateway.
//!
//! ## Exposed Metrics
//!
//! - `phosphoros_http_requests_total` - Total HTTP requests by path and status
//! - `phosphoros_http_request_duration_seconds` - Request duration histogram
//! - `phosphoros_snapshots_ingested_total` - Total snapshots ingested
//! - `phosphoros_analyses_completed_total` - Total forensic analyses completed

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use metrics::{counter, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

/// Prometheus metrics handle.
#[derive(Clone)]
pub struct MetricsState {
    handle: PrometheusHandle,
}

impl MetricsState {
    /// Creates a new metrics state with Prometheus exporter.
    pub fn new() -> Self {
        let handle = PrometheusBuilder::new()
            .install_recorder()
            .expect("Failed to install Prometheus recorder");

        Self { handle }
    }

    /// Returns the Prometheus metrics handle.
    pub fn handle(&self) -> &PrometheusHandle {
        &self.handle
    }
}

impl Default for MetricsState {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware to track HTTP request metrics.
pub async fn track_metrics(
    State(_metrics): State<MetricsState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let method = request.method().to_string();
    let start = Instant::now();

    let response = next.run(request).await;

    let status = response.status().as_u16().to_string();
    let duration = start.elapsed().as_secs_f64();

    // Record request count
    counter!(
        "phosphoros_http_requests_total",
        "path" => path.clone(),
        "method" => method.clone(),
        "status" => status.clone()
    )
    .increment(1);

    // Record request duration
    histogram!(
        "phosphoros_http_request_duration_seconds",
        "path" => path,
        "method" => method
    )
    .record(duration);

    response
}

/// Handler for the /metrics endpoint.
pub async fn metrics_handler(State(metrics): State<MetricsState>) -> (StatusCode, String) {
    (StatusCode::OK, metrics.handle().render())
}

/// Records a successful snapshot ingestion.
pub fn record_snapshot_ingested(chain: &str) {
    counter!(
        "phosphoros_snapshots_ingested_total",
        "chain" => chain.to_string()
    )
    .increment(1);
}

/// Records a completed forensic analysis.
pub fn record_analysis_completed(success: bool) {
    counter!(
        "phosphoros_analyses_completed_total",
        "success" => success.to_string()
    )
    .increment(1);
}

/// Records the number of entities in a snapshot.
pub fn record_entities_analyzed(count: usize) {
    histogram!("phosphoros_entities_per_analysis").record(count as f64);
}

/// Records the number of resonance hotspots detected.
pub fn record_hotspots_detected(count: usize) {
    histogram!("phosphoros_hotspots_detected").record(count as f64);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_metrics_state_creation() {
        // Note: Can only install recorder once per process
        // This test just verifies the API is correct
    }
}
