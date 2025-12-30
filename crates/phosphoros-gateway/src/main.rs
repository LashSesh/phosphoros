//! PHOSPHOROS Gateway Server
//!
//! Starts the API gateway server that serves the web interface backend.

use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about = "PHOSPHOROS Gateway Server", long_about = None)]
struct Args {
    /// Host address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Enable Prometheus metrics
    #[arg(long, default_value_t = true)]
    metrics: bool,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "phosphoros_gateway=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Create the satellite engine
    let satellite_engine = Arc::new(phosphoros_satellite::SatelliteEngine::default());

    // Build the router
    let app = if args.metrics {
        let metrics_state = phosphoros_gateway::MetricsState::new();
        tracing::info!("Prometheus metrics enabled at /metrics");
        phosphoros_gateway::build_gateway_with_metrics(satellite_engine, metrics_state)
    } else {
        phosphoros_gateway::build_gateway(satellite_engine)
    };

    // Parse address
    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("PHOSPHOROS Gateway starting on http://{}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui/", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    tracing::info!("Shutdown signal received, stopping server...");
}
