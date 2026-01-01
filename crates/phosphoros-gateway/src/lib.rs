//! PHOSPHOROS Gateway - Unified API Gateway
//!
//! This crate provides a unified API gateway that exposes PHOSPHOROS functionality
//! including the Satellite forensic analysis subsystem and core resonance features.
//!
//! ## OpenAPI Documentation
//!
//! The gateway includes auto-generated OpenAPI 3.0 documentation accessible at:
//! - `/swagger-ui/` - Interactive Swagger UI
//! - `/api-docs/openapi.json` - Raw OpenAPI specification
//!
//! ## Prometheus Metrics
//!
//! Prometheus-compatible metrics are exposed at:
//! - `/metrics` - Prometheus metrics endpoint

#![forbid(unsafe_code)]

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod cluster;
pub mod metrics;
pub mod resonance;
pub mod wallet;
pub mod websocket;

pub use cluster::ClusterState;
pub use metrics::{MetricsState, record_analysis_completed, record_snapshot_ingested};
pub use phosphoros_satellite;
pub use resonance::ResonanceState;
pub use wallet::WalletState;
pub use websocket::WebSocketState;

/// OpenAPI documentation for the PHOSPHOROS Gateway.
///
/// This includes all endpoints from the Satellite subsystem and gateway health checks.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "PHOSPHOROS Gateway API",
        version = "1.0.0",
        description = "Unified API Gateway for PHOSPHOROS blockchain forensic analysis platform.\n\n\
            PHOSPHOROS provides advanced resonance-based analytics for blockchain forensics, \
            including anomaly detection, cluster analysis, and topological pattern recognition.",
        license(name = "MIT OR Apache-2.0"),
        contact(
            name = "PHOSPHOROS Project Team",
            url = "https://github.com/LashSesh/phosphoros"
        )
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "https://api.phosphoros.io", description = "Production server")
    ),
    tags(
        (name = "gateway", description = "Gateway health and info endpoints"),
        (name = "satellite", description = "Satellite forensic analysis subsystem"),
        (name = "snapshots", description = "Blockchain snapshot management"),
        (name = "analysis", description = "Forensic analysis operations"),
        (name = "resonance", description = "5D resonance analysis and spectral signatures"),
        (name = "wallet", description = "BIP-39 wallet management and address derivation"),
        (name = "cluster", description = "Blockchain entity clustering and community detection")
    ),
    paths(
        root,
        health
    ),
    components(schemas(
        GatewayInfo,
        HealthResponse,
        phosphoros_satellite::models::EntityObservation,
        phosphoros_satellite::models::SnapshotIngest,
        phosphoros_satellite::models::SnapshotRecord,
        phosphoros_satellite::models::AnalysisRequest,
        phosphoros_satellite::models::AnalysisReport,
        phosphoros_satellite::models::ResonanceHotspot,
        phosphoros_satellite::models::AnomalyScore,
        phosphoros_satellite::models::TopologySummary,
        phosphoros_satellite::models::EntropySummary,
        resonance::AnalyzeRequest,
        resonance::AnalyzeResponse,
        resonance::SpectralRequest,
        resonance::SpectralResponse,
        resonance::ResonanceRecord,
        wallet::ImportRequest,
        wallet::ImportResponse,
        wallet::DeriveRequest,
        wallet::DeriveResponse,
        wallet::WalletInfo,
        wallet::ListWalletsResponse,
        wallet::RemoveWalletRequest,
        cluster::Cluster,
        cluster::ComputeRequest,
        cluster::ComputeResponse,
        cluster::ClusterSummary,
        cluster::Entity,
        cluster::ListClustersResponse,
        cluster::ClusterMembersResponse
    ))
)]
pub struct ApiDoc;

/// Gateway information response.
#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct GatewayInfo {
    /// Service name
    pub service: String,
    /// API version
    pub version: String,
    /// Available endpoints
    pub endpoints: std::collections::HashMap<String, String>,
}

/// Health check response.
#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Gateway version
    pub version: String,
}

/// Builds the complete PHOSPHOROS gateway router.
///
/// This includes all satellite endpoints, resonance analysis, wallet management,
/// cluster analysis, WebSocket support, health checks, OpenAPI documentation, and Prometheus metrics.
pub fn build_gateway(
    satellite_engine: Arc<phosphoros_satellite::SatelliteEngine>,
    resonance_state: ResonanceState,
    wallet_state: WalletState,
    cluster_state: ClusterState,
    websocket_state: WebSocketState,
) -> Router {
    let satellite_router = phosphoros_satellite::build_router(satellite_engine);
    let resonance_router = resonance::build_router(resonance_state);
    let wallet_router = wallet::build_router(wallet_state);
    let cluster_router = cluster::build_router(cluster_state);
    let websocket_router = websocket::build_router(websocket_state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/satellite", satellite_router)
        .nest("/api/v1/resonance", resonance_router)
        .nest("/api/v1/wallet", wallet_router)
        .nest("/api/v1/cluster", cluster_router)
        .merge(websocket_router) // WebSocket at /ws
}

/// Builds the gateway router with Prometheus metrics enabled.
///
/// This is the recommended entry point for production deployments.
pub fn build_gateway_with_metrics(
    satellite_engine: Arc<phosphoros_satellite::SatelliteEngine>,
    resonance_state: ResonanceState,
    wallet_state: WalletState,
    cluster_state: ClusterState,
    websocket_state: WebSocketState,
    metrics_state: MetricsState,
) -> Router {
    let satellite_router = phosphoros_satellite::build_router(satellite_engine);
    let resonance_router = resonance::build_router(resonance_state);
    let wallet_router = wallet::build_router(wallet_state);
    let cluster_router = cluster::build_router(cluster_state);
    let websocket_router = websocket::build_router(websocket_state);

    // Create a separate router for metrics with its own state
    let metrics_router = Router::new()
        .route("/metrics", get(metrics::metrics_handler))
        .with_state(metrics_state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(metrics_router)
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/satellite", satellite_router)
        .nest("/api/v1/resonance", resonance_router)
        .nest("/api/v1/wallet", wallet_router)
        .nest("/api/v1/cluster", cluster_router)
        .merge(websocket_router) // WebSocket at /ws
}

/// Gateway root endpoint with service information.
#[utoipa::path(
    get,
    path = "/",
    tag = "gateway",
    responses(
        (status = 200, description = "Gateway information", body = GatewayInfo)
    )
)]
async fn root() -> Json<GatewayInfo> {
    let mut endpoints = std::collections::HashMap::new();
    endpoints.insert("/".to_string(), "This help message".to_string());
    endpoints.insert("/health".to_string(), "Gateway health check".to_string());
    endpoints.insert("/metrics".to_string(), "Prometheus metrics".to_string());
    endpoints.insert("/swagger-ui/".to_string(), "Interactive API documentation".to_string());
    endpoints.insert("/api-docs/openapi.json".to_string(), "OpenAPI 3.0 specification".to_string());

    // Satellite endpoints
    endpoints.insert("/satellite/health".to_string(), "Satellite subsystem health".to_string());
    endpoints.insert("/satellite/v1/snapshots".to_string(), "Manage blockchain snapshots".to_string());
    endpoints.insert("/satellite/v1/analyze/:id".to_string(), "Run forensic analysis".to_string());
    endpoints.insert("/satellite/v1/reports/latest".to_string(), "Get latest analysis report".to_string());

    // Resonance endpoints
    endpoints.insert("/api/v1/resonance/analyze".to_string(), "Analyze 5D resonance state".to_string());
    endpoints.insert("/api/v1/resonance/spectral".to_string(), "Compute spectral signature metrics".to_string());
    endpoints.insert("/api/v1/resonance/history".to_string(), "Get resonance evaluation history".to_string());

    // Wallet endpoints
    endpoints.insert("/api/v1/wallet/import".to_string(), "Import BIP-39 mnemonic phrase".to_string());
    endpoints.insert("/api/v1/wallet/derive".to_string(), "Derive multichain addresses".to_string());
    endpoints.insert("/api/v1/wallet/list".to_string(), "List stored wallets".to_string());

    // Cluster endpoints
    endpoints.insert("/api/v1/cluster/compute".to_string(), "Compute entity clusters".to_string());
    endpoints.insert("/api/v1/cluster/list".to_string(), "List all clusters".to_string());
    endpoints.insert("/api/v1/cluster/:snapshot_id".to_string(), "Get clusters for snapshot".to_string());
    endpoints.insert("/api/v1/cluster/:snapshot_id/:cluster_id/members".to_string(), "Get cluster members".to_string());

    // WebSocket endpoint
    endpoints.insert("/ws".to_string(), "WebSocket for real-time updates".to_string());

    Json(GatewayInfo {
        service: "PHOSPHOROS Gateway".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        endpoints,
    })
}

/// Health check endpoint.
#[utoipa::path(
    get,
    path = "/health",
    tag = "gateway",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_builds() {
        let engine = Arc::new(phosphoros_satellite::SatelliteEngine::default());
        let resonance = ResonanceState::default();
        let wallet = WalletState::default();
        let cluster = ClusterState::default();
        let websocket = WebSocketState::default();
        let _router = build_gateway(engine, resonance, wallet, cluster, websocket);
    }

    #[test]
    fn test_openapi_spec() {
        let doc = ApiDoc::openapi();
        assert_eq!(doc.info.title, "PHOSPHOROS Gateway API");
        assert!(!doc.paths.paths.is_empty());
    }
}
