//! Application state management

use crate::panels::{PanelId, PanelState};
use crate::services::ServiceManager;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Maximum number of notifications to keep
const MAX_NOTIFICATIONS: usize = 10;

/// Maximum number of log entries to keep
const MAX_LOG_ENTRIES: usize = 1000;

/// Main application state
#[derive(Debug)]
pub struct AppState {
    /// Current active panel
    pub active_panel: PanelId,

    /// Panel states
    pub panels: PanelState,

    /// Service manager
    pub service_manager: ServiceManager,

    /// Notifications queue
    pub notifications: VecDeque<Notification>,

    /// Log entries
    pub logs: VecDeque<LogEntry>,

    /// Theme mode
    pub dark_mode: bool,

    /// Sidebar collapsed state
    pub sidebar_collapsed: bool,

    /// Window dimensions
    pub window_size: (f32, f32),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_panel: PanelId::Home,
            panels: PanelState::default(),
            service_manager: ServiceManager::default(),
            notifications: VecDeque::with_capacity(MAX_NOTIFICATIONS),
            logs: VecDeque::with_capacity(MAX_LOG_ENTRIES),
            dark_mode: true, // Default to dark mode as per spec
            sidebar_collapsed: false,
            window_size: (1600.0, 900.0),
        }
    }
}

impl AppState {
    /// Add a notification
    pub fn add_notification(&mut self, notification: Notification) {
        if self.notifications.len() >= MAX_NOTIFICATIONS {
            self.notifications.pop_front();
        }
        self.notifications.push_back(notification);
    }

    /// Remove a notification by index
    pub fn remove_notification(&mut self, index: usize) {
        if index < self.notifications.len() {
            self.notifications.remove(index);
        }
    }

    /// Add a log entry
    pub fn add_log(&mut self, entry: LogEntry) {
        if self.logs.len() >= MAX_LOG_ENTRIES {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
    }

    /// Clear all logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Get filtered logs
    pub fn filtered_logs(&self, filter: &str) -> Vec<&LogEntry> {
        if filter.is_empty() {
            self.logs.iter().collect()
        } else {
            self.logs
                .iter()
                .filter(|log| log.message.to_lowercase().contains(&filter.to_lowercase()))
                .collect()
        }
    }
}

/// Notification types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: usize,
    /// Notification type
    pub kind: NotificationKind,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Notification kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationKind {
    /// Information
    Info,
    /// Success
    Success,
    /// Warning
    Warning,
    /// Error
    Error,
}

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Log level
    pub level: LogLevel,
    /// Module/source
    pub source: String,
    /// Message
    pub message: String,
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// Debug
    Debug,
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
