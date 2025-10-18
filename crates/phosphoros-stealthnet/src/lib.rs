//! PHOSPHOROS Stealthnet - Stealth Networking and Invisible Payload Layer
//!
//! This crate provides stealth networking capabilities for PHOSPHOROS, enabling
//! undetectable blockchain forensic analysis through traffic mimicry, steganography,
//! and adaptive API camouflage.
//!
//! # Features
//!
//! - **Traffic Mimicry**: Imitate legitimate API traffic (Slack, Telegram, OpenAI, Discord)
//! - **Steganography**: Zero-width encoding, invisible payloads in headers, bodies, params
//! - **Payload Shaping**: Adaptive transformation for legitimate traffic patterns
//! - **Request Randomization**: Temporal, semantic, and structural variability
//! - **Proxy Management**: SOCKS5/HTTP proxy rotation and configuration
//!
//! # Usage
//!
//! ```
//! use phosphoros_stealthnet::{StealthMode, StealthProxy, StealthRequest};
//!
//! // Create a stealth proxy with mimicry mode
//! let proxy = StealthProxy::new(StealthMode::Mimicry);
//!
//! // Create a stealth request
//! let request = StealthRequest::builder()
//!     .url("https://api.example.com/scan")
//!     .payload(b"blockchain_data")
//!     .build()
//!     .unwrap();
//!
//! // Process through stealth layer
//! let stealth_request = proxy.transform(request).unwrap();
//! ```
//!
//! # Compliance and Legal Notice
//!
//! This stealth networking layer is intended ONLY for:
//! - Legitimate forensic analysis
//! - Scientific research
//! - Defense and security purposes with proper authorization
//! - Compliance-approved use cases
//!
//! Misuse of these capabilities may violate laws and regulations.
//! Users are responsible for ensuring compliance with applicable laws.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod mimicry;
pub mod mode;
pub mod payload;
pub mod proxy;
pub mod request;
pub mod steganography;

// Re-export commonly used types
pub use error::{Result, StealthError};
pub use mimicry::{ApiMimicry, ApiTemplate, ApiType};
pub use mode::StealthMode;
pub use payload::{PayloadShaper, PayloadTransformer};
pub use proxy::{ProxyConfig, ProxyManager, ProxyProtocol, RotationMode, StealthProxy};
pub use request::{StealthRequest, StealthRequestBuilder};
pub use steganography::{SteganographyEncoder, ZeroWidthEncoder};
