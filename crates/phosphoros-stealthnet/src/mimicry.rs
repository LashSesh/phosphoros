//! API mimicry for disguising traffic as legitimate API calls.

use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Result, StealthError};

/// API types that can be mimicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiType {
    /// OpenAI API
    OpenAI,
    /// Slack API
    Slack,
    /// Telegram Bot API
    Telegram,
    /// Discord API
    Discord,
    /// Generic REST API
    Generic,
}

impl ApiType {
    /// Returns all available API types.
    pub fn all() -> Vec<ApiType> {
        vec![
            ApiType::OpenAI,
            ApiType::Slack,
            ApiType::Telegram,
            ApiType::Discord,
            ApiType::Generic,
        ]
    }
}

impl std::fmt::Display for ApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiType::OpenAI => write!(f, "OpenAI"),
            ApiType::Slack => write!(f, "Slack"),
            ApiType::Telegram => write!(f, "Telegram"),
            ApiType::Discord => write!(f, "Discord"),
            ApiType::Generic => write!(f, "Generic"),
        }
    }
}

/// Template for mimicking API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTemplate {
    /// API type
    pub api_type: ApiType,
    /// HTTP method
    pub method: String,
    /// URL template
    pub url: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Body template (optional)
    pub body: Option<String>,
}

impl ApiTemplate {
    /// Creates a new API template.
    pub fn new(api_type: ApiType) -> Self {
        match api_type {
            ApiType::OpenAI => Self::openai(),
            ApiType::Slack => Self::slack(),
            ApiType::Telegram => Self::telegram(),
            ApiType::Discord => Self::discord(),
            ApiType::Generic => Self::generic(),
        }
    }

    fn openai() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer sk-...".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_type: ApiType::OpenAI,
            method: "POST".to_string(),
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            headers,
            body: Some(r#"{"model":"gpt-3.5-turbo","messages":[{"role":"user","content":"Hello"}]}"#.to_string()),
        }
    }

    fn slack() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer xoxb-...".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_type: ApiType::Slack,
            method: "GET".to_string(),
            url: "https://slack.com/api/conversations.list".to_string(),
            headers,
            body: None,
        }
    }

    fn telegram() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_type: ApiType::Telegram,
            method: "POST".to_string(),
            url: "https://api.telegram.org/botTOKEN/sendMessage".to_string(),
            headers,
            body: Some(r#"{"chat_id":123456,"text":"Hello"}"#.to_string()),
        }
    }

    fn discord() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bot TOKEN".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_type: ApiType::Discord,
            method: "POST".to_string(),
            url: "https://discord.com/api/v10/channels/CHANNEL_ID/messages".to_string(),
            headers,
            body: Some(r#"{"content":"Hello"}"#.to_string()),
        }
    }

    fn generic() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "Mozilla/5.0".to_string());

        Self {
            api_type: ApiType::Generic,
            method: "GET".to_string(),
            url: "https://api.example.com/v1/data".to_string(),
            headers,
            body: None,
        }
    }

    /// Converts to HTTP request string.
    pub fn to_http_request(&self) -> String {
        let mut lines = vec![format!("{} {} HTTP/1.1", self.method, self.url)];
        
        for (key, value) in &self.headers {
            lines.push(format!("{}: {}", key, value));
        }
        
        lines.push(String::new());
        
        if let Some(body) = &self.body {
            lines.push(body.clone());
        }
        
        lines.join("\r\n")
    }
}

/// API mimicry generator.
pub struct ApiMimicry {
    templates: Vec<ApiTemplate>,
    rng: rand::rngs::ThreadRng,
}

impl ApiMimicry {
    /// Creates a new API mimicry generator.
    pub fn new() -> Self {
        let templates = ApiType::all()
            .into_iter()
            .map(ApiTemplate::new)
            .collect();
        
        Self {
            templates,
            rng: rand::thread_rng(),
        }
    }

    /// Generates a random API request template.
    pub fn generate_random(&mut self) -> Result<ApiTemplate> {
        self.templates
            .choose(&mut self.rng)
            .cloned()
            .ok_or_else(|| StealthError::TransformationFailed("No templates available".to_string()))
    }

    /// Generates an API request for a specific type.
    pub fn generate_for_type(&self, api_type: ApiType) -> Result<ApiTemplate> {
        self.templates
            .iter()
            .find(|t| t.api_type == api_type)
            .cloned()
            .ok_or_else(|| StealthError::TransformationFailed(format!("Template not found for {:?}", api_type)))
    }

    /// Adds temporal jitter to requests (returns delay in milliseconds).
    pub fn temporal_jitter(&mut self, min_ms: u64, max_ms: u64) -> u64 {
        self.rng.gen_range(min_ms..=max_ms)
    }

    /// Randomizes request structure slightly.
    pub fn randomize_template(&mut self, mut template: ApiTemplate) -> ApiTemplate {
        // Add random query parameters for GET requests
        if template.method == "GET" && self.rng.gen_bool(0.5) {
            let param_keys = ["limit", "offset", "page", "per_page", "sort"];
            let param = param_keys.choose(&mut self.rng).unwrap();
            let value = self.rng.gen_range(1..100);
            
            if template.url.contains('?') {
                template.url.push_str(&format!("&{}={}", param, value));
            } else {
                template.url.push_str(&format!("?{}={}", param, value));
            }
        }
        
        // Vary user agent
        if template.headers.contains_key("User-Agent") && self.rng.gen_bool(0.3) {
            let user_agents = [
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
                "Mozilla/5.0 (X11; Linux x86_64)",
            ];
            if let Some(ua) = user_agents.choose(&mut self.rng) {
                template.headers.insert("User-Agent".to_string(), ua.to_string());
            }
        }
        
        template
    }
}

impl Default for ApiMimicry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_template_creation() {
        for api_type in ApiType::all() {
            let template = ApiTemplate::new(api_type);
            assert_eq!(template.api_type, api_type);
            assert!(!template.method.is_empty());
            assert!(!template.url.is_empty());
        }
    }

    #[test]
    fn test_api_mimicry_generate() {
        let mut mimicry = ApiMimicry::new();
        let template = mimicry.generate_random().unwrap();
        assert!(!template.url.is_empty());
    }

    #[test]
    fn test_api_mimicry_specific_type() {
        let mimicry = ApiMimicry::new();
        let template = mimicry.generate_for_type(ApiType::OpenAI).unwrap();
        assert_eq!(template.api_type, ApiType::OpenAI);
    }

    #[test]
    fn test_temporal_jitter() {
        let mut mimicry = ApiMimicry::new();
        let jitter = mimicry.temporal_jitter(10, 100);
        assert!(jitter >= 10 && jitter <= 100);
    }

    #[test]
    fn test_to_http_request() {
        let template = ApiTemplate::new(ApiType::Slack);
        let http_request = template.to_http_request();
        assert!(http_request.contains("GET"));
        assert!(http_request.contains("slack.com"));
    }
}
