//! PHOSPHOROS Dashboard - Living Lab Interface
//!
//! Autonomous infogenetic system with real-time visualization and control.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

mod app;
mod config;
mod messages;
mod panels;
mod services;
mod state;
mod theme;
mod widgets;
mod integration;
mod tasks;

use app::PhosphorosApp;
use iced::Settings;
use tracing_subscriber::EnvFilter;

fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting PHOSPHOROS Dashboard...");

    // Run application with proper initialization
    iced::application(
        "PHOSPHOROS - Living Lab Dashboard",
        PhosphorosApp::update,
        PhosphorosApp::view,
    )
    .subscription(PhosphorosApp::subscription)
    .theme(PhosphorosApp::theme)
    .run_with(PhosphorosApp::new)
}
