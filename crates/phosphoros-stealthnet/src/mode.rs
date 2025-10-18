//! Stealth modes for different levels of concealment.

use serde::{Deserialize, Serialize};

/// Stealth mode determining the level of traffic concealment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum StealthMode {
    /// No stealth - direct requests
    #[default]
    Open,
    
    /// Traffic mimicry - requests appear as legitimate API calls
    Mimicry,
    
    /// Full steganography - payloads hidden using zero-width encoding
    Steganography,
    
    /// Adaptive mode - dynamically selects best concealment based on context
    Adaptive,
}

impl StealthMode {
    /// Returns a human-readable description of the mode.
    pub fn description(&self) -> &'static str {
        match self {
            StealthMode::Open => "Direct requests without concealment",
            StealthMode::Mimicry => "Requests disguised as legitimate API traffic",
            StealthMode::Steganography => "Payloads hidden using steganographic encoding",
            StealthMode::Adaptive => "Dynamically adapts concealment to context",
        }
    }

    /// Returns the stealth level (0-3, higher is more stealthy).
    pub fn level(&self) -> u8 {
        match self {
            StealthMode::Open => 0,
            StealthMode::Mimicry => 1,
            StealthMode::Steganography => 2,
            StealthMode::Adaptive => 3,
        }
    }

    /// Returns all available stealth modes.
    pub fn all() -> Vec<StealthMode> {
        vec![
            StealthMode::Open,
            StealthMode::Mimicry,
            StealthMode::Steganography,
            StealthMode::Adaptive,
        ]
    }
}

impl std::fmt::Display for StealthMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StealthMode::Open => write!(f, "Open"),
            StealthMode::Mimicry => write!(f, "Mimicry"),
            StealthMode::Steganography => write!(f, "Steganography"),
            StealthMode::Adaptive => write!(f, "Adaptive"),
        }
    }
}
