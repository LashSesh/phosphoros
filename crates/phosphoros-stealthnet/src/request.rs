//! Stealth request building and handling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::Result;
use crate::mode::StealthMode;

/// Stealth request containing original data and transformation metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthRequest {
    /// Target URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Option<Vec<u8>>,
    /// Stealth mode for this request
    pub stealth_mode: StealthMode,
    /// Proxy configuration (optional)
    pub proxy: Option<String>,
    /// Timing jitter in milliseconds
    pub timing_jitter: Option<u64>,
    /// Metadata for tracking
    pub metadata: HashMap<String, String>,
}

impl StealthRequest {
    /// Creates a new stealth request builder.
    pub fn builder() -> StealthRequestBuilder {
        StealthRequestBuilder::new()
    }

    /// Gets the effective URL (may be modified by stealth layer).
    pub fn effective_url(&self) -> &str {
        &self.url
    }

    /// Gets the HTTP method.
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Gets headers.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Gets the body.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Gets the stealth mode.
    pub fn stealth_mode(&self) -> StealthMode {
        self.stealth_mode
    }

    /// Adds a header.
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /// Adds metadata.
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Builder for stealth requests.
#[derive(Debug, Default)]
pub struct StealthRequestBuilder {
    url: Option<String>,
    method: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
    stealth_mode: StealthMode,
    proxy: Option<String>,
    timing_jitter: Option<u64>,
    metadata: HashMap<String, String>,
}

impl StealthRequestBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            url: None,
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            stealth_mode: StealthMode::default(),
            proxy: None,
            timing_jitter: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the URL.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the HTTP method.
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    /// Adds a header.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets multiple headers.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Sets the body.
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the body from a string.
    pub fn body_string(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into().into_bytes());
        self
    }

    /// Sets the payload (alias for body).
    pub fn payload(self, payload: impl Into<Vec<u8>>) -> Self {
        self.body(payload)
    }

    /// Sets the stealth mode.
    pub fn stealth_mode(mut self, mode: StealthMode) -> Self {
        self.stealth_mode = mode;
        self
    }

    /// Sets the proxy URL.
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Sets timing jitter.
    pub fn timing_jitter(mut self, jitter_ms: u64) -> Self {
        self.timing_jitter = Some(jitter_ms);
        self
    }

    /// Adds metadata.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Builds the stealth request.
    pub fn build(self) -> Result<StealthRequest> {
        let url = self.url.ok_or_else(|| {
            crate::error::StealthError::InvalidConfig("URL is required".to_string())
        })?;

        Ok(StealthRequest {
            url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            stealth_mode: self.stealth_mode,
            proxy: self.proxy,
            timing_jitter: self.timing_jitter,
            metadata: self.metadata,
        })
    }
}

/// Response from a stealth request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Timing information
    pub timing: ResponseTiming,
    /// Original request metadata
    pub request_metadata: HashMap<String, String>,
}

impl StealthResponse {
    /// Creates a new stealth response.
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
            timing: ResponseTiming::default(),
            request_metadata: HashMap::new(),
        }
    }

    /// Gets the status code.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Gets the body as bytes.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Gets the body as a string (if valid UTF-8).
    pub fn body_string(&self) -> Option<String> {
        String::from_utf8(self.body.clone()).ok()
    }

    /// Gets response headers.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Checks if the response was successful.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}

/// Response timing information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseTiming {
    /// Time to establish connection (ms)
    pub connect_time_ms: Option<u64>,
    /// Time to first byte (ms)
    pub ttfb_ms: Option<u64>,
    /// Total request time (ms)
    pub total_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = StealthRequest::builder()
            .url("https://api.example.com/test")
            .method("POST")
            .header("Content-Type", "application/json")
            .body_string(r#"{"test": "data"}"#)
            .stealth_mode(StealthMode::Mimicry)
            .build()
            .unwrap();

        assert_eq!(request.url, "https://api.example.com/test");
        assert_eq!(request.method, "POST");
        assert!(request.headers.contains_key("Content-Type"));
        assert!(request.body.is_some());
        assert_eq!(request.stealth_mode, StealthMode::Mimicry);
    }

    #[test]
    fn test_request_builder_minimal() {
        let request = StealthRequest::builder()
            .url("https://example.com")
            .build()
            .unwrap();

        assert_eq!(request.method, "GET");
        assert_eq!(request.stealth_mode, StealthMode::Open);
    }

    #[test]
    fn test_request_builder_no_url() {
        let result = StealthRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_stealth_response() {
        let response = StealthResponse::new(200, b"test body".to_vec());
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), b"test body");
        assert!(response.is_success());
        assert_eq!(response.body_string().unwrap(), "test body");
    }

    #[test]
    fn test_response_is_success() {
        let success = StealthResponse::new(200, vec![]);
        assert!(success.is_success());

        let redirect = StealthResponse::new(301, vec![]);
        assert!(!redirect.is_success());

        let error = StealthResponse::new(404, vec![]);
        assert!(!error.is_success());
    }
}
