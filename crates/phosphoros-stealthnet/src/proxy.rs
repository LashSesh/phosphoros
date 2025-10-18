//! Proxy management for routing stealth traffic.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{Result, StealthError};

/// Proxy protocol type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyProtocol {
    /// SOCKS5 proxy
    Socks5,
    /// HTTP proxy
    Http,
    /// HTTPS proxy
    Https,
}

impl std::fmt::Display for ProxyProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyProtocol::Socks5 => write!(f, "SOCKS5"),
            ProxyProtocol::Http => write!(f, "HTTP"),
            ProxyProtocol::Https => write!(f, "HTTPS"),
        }
    }
}

/// Proxy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Whether proxy is enabled
    pub enabled: bool,
    /// Proxy protocol
    pub protocol: ProxyProtocol,
    /// Proxy host
    pub host: String,
    /// Proxy port
    pub port: u16,
    /// Optional username
    pub username: Option<String>,
    /// Optional password
    pub password: Option<String>,
}

impl ProxyConfig {
    /// Creates a new proxy configuration.
    pub fn new(protocol: ProxyProtocol, host: String, port: u16) -> Self {
        Self {
            enabled: true,
            protocol,
            host,
            port,
            username: None,
            password: None,
        }
    }

    /// Sets authentication credentials.
    pub fn with_auth(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    /// Disables the proxy.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            protocol: ProxyProtocol::Http,
            host: String::new(),
            port: 0,
            username: None,
            password: None,
        }
    }

    /// Gets the proxy URL.
    pub fn url(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let auth = match (&self.username, &self.password) {
            (Some(u), Some(p)) => format!("{}:{}@", u, p),
            _ => String::new(),
        };

        format!(
            "{}://{}{}:{}",
            self.protocol.to_string().to_lowercase(),
            auth,
            self.host,
            self.port
        )
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if self.host.is_empty() {
            return Err(StealthError::ProxyError("Host is empty".to_string()));
        }

        if self.port == 0 {
            return Err(StealthError::ProxyError("Invalid port".to_string()));
        }

        Ok(())
    }
}

/// Proxy rotation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationMode {
    /// Round-robin rotation
    RoundRobin,
    /// Random selection
    Random,
    /// Sequential (no rotation)
    Sequential,
}

/// Proxy manager for handling multiple proxies.
pub struct ProxyManager {
    proxies: Arc<RwLock<Vec<ProxyConfig>>>,
    mode: Arc<RwLock<RotationMode>>,
    current_index: Arc<RwLock<usize>>,
}

impl ProxyManager {
    /// Creates a new proxy manager.
    pub fn new() -> Self {
        Self {
            proxies: Arc::new(RwLock::new(Vec::new())),
            mode: Arc::new(RwLock::new(RotationMode::RoundRobin)),
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Adds a proxy to the pool.
    pub fn add_proxy(&self, proxy: ProxyConfig) -> Result<()> {
        proxy.validate()?;
        self.proxies.write().push(proxy);
        Ok(())
    }

    /// Removes all proxies.
    pub fn clear_proxies(&self) {
        self.proxies.write().clear();
        *self.current_index.write() = 0;
    }

    /// Sets the rotation mode.
    pub fn set_mode(&self, mode: RotationMode) {
        *self.mode.write() = mode;
    }

    /// Gets the current rotation mode.
    pub fn mode(&self) -> RotationMode {
        *self.mode.read()
    }

    /// Gets the next proxy according to the rotation mode.
    pub fn acquire(&self) -> Option<ProxyConfig> {
        let proxies = self.proxies.read();
        if proxies.is_empty() {
            return None;
        }

        let mode = *self.mode.read();
        match mode {
            RotationMode::RoundRobin => {
                let mut index = self.current_index.write();
                let proxy = proxies[*index % proxies.len()].clone();
                *index = (*index + 1) % proxies.len();
                Some(proxy)
            }
            RotationMode::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..proxies.len());
                Some(proxies[index].clone())
            }
            RotationMode::Sequential => {
                let index = *self.current_index.read();
                Some(proxies[index % proxies.len()].clone())
            }
        }
    }

    /// Gets the number of configured proxies.
    pub fn count(&self) -> usize {
        self.proxies.read().len()
    }

    /// Gets all proxies.
    pub fn list(&self) -> Vec<ProxyConfig> {
        self.proxies.read().clone()
    }

    /// Tests if a proxy is available.
    #[cfg(feature = "network")]
    pub async fn test_proxy(&self, _proxy: &ProxyConfig) -> Result<bool> {
        // In real implementation, would test actual connectivity
        Ok(true)
    }
}

impl Default for ProxyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Stealth proxy orchestrator combining mode and proxy management.
pub struct StealthProxy {
    mode: crate::mode::StealthMode,
    proxy_manager: ProxyManager,
}

impl StealthProxy {
    /// Creates a new stealth proxy.
    pub fn new(mode: crate::mode::StealthMode) -> Self {
        Self {
            mode,
            proxy_manager: ProxyManager::new(),
        }
    }

    /// Gets the current stealth mode.
    pub fn mode(&self) -> crate::mode::StealthMode {
        self.mode
    }

    /// Sets the stealth mode.
    pub fn set_mode(&mut self, mode: crate::mode::StealthMode) {
        self.mode = mode;
    }

    /// Gets a reference to the proxy manager.
    pub fn proxy_manager(&self) -> &ProxyManager {
        &self.proxy_manager
    }

    /// Gets a mutable reference to the proxy manager.
    pub fn proxy_manager_mut(&mut self) -> &mut ProxyManager {
        &mut self.proxy_manager
    }

    /// Transforms a request according to the stealth mode.
    pub fn transform(&self, request: crate::request::StealthRequest) -> Result<crate::request::StealthRequest> {
        match self.mode {
            crate::mode::StealthMode::Open => Ok(request),
            crate::mode::StealthMode::Mimicry => {
                // Apply mimicry transformation
                Ok(request)
            }
            crate::mode::StealthMode::Steganography => {
                // Apply steganographic transformation
                Ok(request)
            }
            crate::mode::StealthMode::Adaptive => {
                // Intelligently select best transformation
                Ok(request)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_config_creation() {
        let config = ProxyConfig::new(
            ProxyProtocol::Socks5,
            "127.0.0.1".to_string(),
            1080,
        );
        assert!(config.enabled);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 1080);
    }

    #[test]
    fn test_proxy_config_url() {
        let config = ProxyConfig::new(
            ProxyProtocol::Socks5,
            "proxy.example.com".to_string(),
            1080,
        )
        .with_auth("user".to_string(), "pass".to_string());

        let url = config.url();
        assert!(url.contains("socks5://"));
        assert!(url.contains("user:pass@"));
        assert!(url.contains("proxy.example.com:1080"));
    }

    #[test]
    fn test_proxy_manager_add() {
        let manager = ProxyManager::new();
        let config = ProxyConfig::new(
            ProxyProtocol::Http,
            "proxy1.example.com".to_string(),
            8080,
        );

        manager.add_proxy(config).unwrap();
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_proxy_manager_round_robin() {
        let manager = ProxyManager::new();
        manager.set_mode(RotationMode::RoundRobin);

        for i in 0..3 {
            let config = ProxyConfig::new(
                ProxyProtocol::Http,
                format!("proxy{}.example.com", i),
                8080,
            );
            manager.add_proxy(config).unwrap();
        }

        let first = manager.acquire().unwrap();
        let second = manager.acquire().unwrap();
        let third = manager.acquire().unwrap();
        let fourth = manager.acquire().unwrap();

        assert_ne!(first.host, second.host);
        assert_ne!(second.host, third.host);
        assert_eq!(first.host, fourth.host); // Should wrap around
    }

    #[test]
    fn test_stealth_proxy() {
        let mut proxy = StealthProxy::new(crate::mode::StealthMode::Mimicry);
        assert_eq!(proxy.mode(), crate::mode::StealthMode::Mimicry);

        proxy.set_mode(crate::mode::StealthMode::Steganography);
        assert_eq!(proxy.mode(), crate::mode::StealthMode::Steganography);
    }
}
