//! WebSocket Support for Real-time Updates
//!
//! Provides WebSocket endpoint for live updates from the gateway, including
//! analysis progress, service status, logs, and event notifications.

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use utoipa::ToSchema;

/// WebSocket state with event broadcaster
#[derive(Clone)]
pub struct WebSocketState {
    /// Broadcast channel for events (supports multiple subscribers)
    pub event_tx: broadcast::Sender<GatewayEvent>,
}

impl WebSocketState {
    /// Create new WebSocket state
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(100); // Buffer up to 100 events
        Self { event_tx }
    }

    /// Broadcast an event to all connected clients
    pub fn broadcast(&self, event: GatewayEvent) {
        let _ = self.event_tx.send(event); // Ignore errors if no listeners
    }
}

impl Default for WebSocketState {
    fn default() -> Self {
        Self::new()
    }
}

/// Gateway event types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum GatewayEvent {
    /// Log message
    Log {
        /// Timestamp (RFC3339)
        timestamp: String,
        /// Log level (info, warn, error, debug)
        level: String,
        /// Log message
        message: String,
        /// Optional source component
        source: Option<String>,
    },
    /// Analysis progress update
    AnalysisProgress {
        /// Analysis ID
        id: String,
        /// Progress percentage (0-100)
        progress: u32,
        /// Current step description
        step: String,
    },
    /// Analysis completed
    AnalysisComplete {
        /// Analysis ID
        id: String,
        /// Whether analysis was successful
        success: bool,
        /// Optional error message
        error: Option<String>,
    },
    /// Service status update
    ServiceStatus {
        /// Service name (scraper, analyzer, cluster_engine)
        service: String,
        /// Service status (running, stopped, error)
        status: String,
        /// Optional status message
        message: Option<String>,
    },
    /// Resonance evaluation completed
    ResonanceEvaluated {
        /// Timestamp
        timestamp: String,
        /// Resonance score
        score: Option<f64>,
        /// Whether it was gated
        gated: bool,
        /// Optional label
        label: Option<String>,
    },
    /// Wallet address derived
    WalletDerived {
        /// Timestamp
        timestamp: String,
        /// Blockchain
        blockchain: String,
        /// Number of addresses generated
        count: usize,
    },
    /// Cluster computed
    ClusterComputed {
        /// Timestamp
        timestamp: String,
        /// Snapshot ID
        snapshot_id: String,
        /// Number of clusters found
        num_clusters: usize,
    },
    /// Generic notification
    Notification {
        /// Notification level (info, warning, error, success)
        level: String,
        /// Title
        title: String,
        /// Message body
        message: String,
    },
}

/// Build the WebSocket router
pub fn build_router(state: WebSocketState) -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

/// WebSocket upgrade handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle a WebSocket connection
async fn handle_socket(socket: WebSocket, state: WebSocketState) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to event broadcast channel
    let mut event_rx = state.event_tx.subscribe();

    // Send initial connection confirmation
    let connect_msg = GatewayEvent::Notification {
        level: "info".to_string(),
        title: "Connected".to_string(),
        message: "WebSocket connection established".to_string(),
    };

    if let Ok(json) = serde_json::to_string(&connect_msg) {
        let _ = sender.send(Message::Text(json)).await;
    }

    // Spawn task to forward events to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            // Serialize event to JSON
            let json = match serde_json::to_string(&event) {
                Ok(j) => j,
                Err(e) => {
                    tracing::warn!("Failed to serialize event: {}", e);
                    continue;
                }
            };

            // Send to client
            if sender.send(Message::Text(json)).await.is_err() {
                // Client disconnected
                break;
            }
        }
    });

    // Spawn task to handle client messages (for potential commands)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Parse client command (e.g., subscribe to specific events)
                    tracing::debug!("Received WebSocket message: {}", text);
                    // TODO: Implement command handling
                }
                Message::Close(_) => {
                    tracing::debug!("WebSocket close frame received");
                    break;
                }
                Message::Ping(_) => {
                    // Axum handles pong automatically
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish (connection close)
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    tracing::debug!("WebSocket connection closed");
}

/// Helper functions to emit events from other modules

/// Emit a log event
pub fn emit_log(state: &WebSocketState, level: &str, message: &str, source: Option<&str>) {
    let event = GatewayEvent::Log {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: level.to_string(),
        message: message.to_string(),
        source: source.map(|s| s.to_string()),
    };
    state.broadcast(event);
}

/// Emit analysis progress
pub fn emit_analysis_progress(state: &WebSocketState, id: &str, progress: u32, step: &str) {
    let event = GatewayEvent::AnalysisProgress {
        id: id.to_string(),
        progress,
        step: step.to_string(),
    };
    state.broadcast(event);
}

/// Emit analysis completion
pub fn emit_analysis_complete(state: &WebSocketState, id: &str, success: bool, error: Option<String>) {
    let event = GatewayEvent::AnalysisComplete {
        id: id.to_string(),
        success,
        error,
    };
    state.broadcast(event);
}

/// Emit service status update
pub fn emit_service_status(state: &WebSocketState, service: &str, status: &str, message: Option<String>) {
    let event = GatewayEvent::ServiceStatus {
        service: service.to_string(),
        status: status.to_string(),
        message,
    };
    state.broadcast(event);
}

/// Emit resonance evaluation
pub fn emit_resonance_evaluated(
    state: &WebSocketState,
    score: Option<f64>,
    gated: bool,
    label: Option<String>,
) {
    let event = GatewayEvent::ResonanceEvaluated {
        timestamp: chrono::Utc::now().to_rfc3339(),
        score,
        gated,
        label,
    };
    state.broadcast(event);
}

/// Emit wallet derivation
pub fn emit_wallet_derived(state: &WebSocketState, blockchain: &str, count: usize) {
    let event = GatewayEvent::WalletDerived {
        timestamp: chrono::Utc::now().to_rfc3339(),
        blockchain: blockchain.to_string(),
        count,
    };
    state.broadcast(event);
}

/// Emit cluster computation
pub fn emit_cluster_computed(state: &WebSocketState, snapshot_id: &str, num_clusters: usize) {
    let event = GatewayEvent::ClusterComputed {
        timestamp: chrono::Utc::now().to_rfc3339(),
        snapshot_id: snapshot_id.to_string(),
        num_clusters,
    };
    state.broadcast(event);
}

/// Emit notification
pub fn emit_notification(state: &WebSocketState, level: &str, title: &str, message: &str) {
    let event = GatewayEvent::Notification {
        level: level.to_string(),
        title: title.to_string(),
        message: message.to_string(),
    };
    state.broadcast(event);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_state_creation() {
        let state = WebSocketState::new();
        // Should be able to broadcast without subscribers
        state.broadcast(GatewayEvent::Notification {
            level: "info".to_string(),
            title: "Test".to_string(),
            message: "Test message".to_string(),
        });
    }

    #[test]
    fn test_event_serialization() {
        let event = GatewayEvent::Log {
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            level: "info".to_string(),
            message: "Test log".to_string(),
            source: Some("test".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"Log\""));
        assert!(json.contains("Test log"));
    }
}
