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

use app::PhosphorosApp;
use iced::{window, Settings, Size};
use tracing_subscriber::EnvFilter;

fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting PHOSPHOROS Dashboard...");

    // Configure window
    let window_settings = window::Settings {
        size: Size::new(1600.0, 900.0),
        min_size: Some(Size::new(1280.0, 720.0)),
        ..Default::default()
    };

    // Run application
    iced::application(
        "PHOSPHOROS - Living Lab Dashboard",
        PhosphorosApp::update,
        PhosphorosApp::view,
    )
    .settings(Settings {
        window: window_settings,
        ..Default::default()
    })
    .subscription(PhosphorosApp::subscription)
    .theme(PhosphorosApp::theme)
    .run_with(PhosphorosApp::new)
}
