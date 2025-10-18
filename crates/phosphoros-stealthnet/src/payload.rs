//! Payload transformation and shaping utilities.

use serde_json::Value;

use crate::error::{Result, StealthError};
use crate::mimicry::{ApiMimicry, ApiTemplate, ApiType};
use crate::steganography::{SteganographyEncoder, ZeroWidthEncoder};

/// Trait for transforming payloads.
pub trait PayloadTransformer {
    /// Transforms a raw payload into a stealthy format.
    fn transform(&self, payload: &[u8]) -> Result<Vec<u8>>;
}

/// Payload shaper for adaptive transformation.
pub struct PayloadShaper {
    mimicry: ApiMimicry,
    encoder: ZeroWidthEncoder,
}

impl PayloadShaper {
    /// Creates a new payload shaper.
    pub fn new() -> Self {
        Self {
            mimicry: ApiMimicry::new(),
            encoder: ZeroWidthEncoder::new(),
        }
    }

    /// Shapes payload to look like a specific API type.
    pub fn shape_for_api(&mut self, payload: &[u8], api_type: ApiType) -> Result<String> {
        let template = self.mimicry.generate_for_type(api_type)?;
        self.embed_in_template(payload, template)
    }

    /// Embeds payload in an API template body.
    fn embed_in_template(&self, payload: &[u8], mut template: ApiTemplate) -> Result<String> {
        // Encode payload using steganography
        let encoded = self.encoder.encode(payload)?;
        
        // If template has a body, inject steganographic payload
        if let Some(body) = template.body {
            // Try to parse as JSON
            if let Ok(mut json) = serde_json::from_str::<Value>(&body) {
                // Add steganographic data to a field
                if let Some(obj) = json.as_object_mut() {
                    // Find a text field or create one
                    if let Some(Value::String(text)) = obj.values_mut().find(|v| v.is_string()) {
                        text.push_str(&encoded);
                    } else {
                        obj.insert("metadata".to_string(), Value::String(encoded));
                    }
                    template.body = Some(serde_json::to_string(&json)?);
                } else {
                    template.body = Some(format!("{}{}", body, encoded));
                }
            } else {
                // Not JSON, just append
                template.body = Some(format!("{}{}", body, encoded));
            }
        } else {
            // No body, add as invisible text
            template.body = Some(encoded);
        }
        
        Ok(template.to_http_request())
    }

    /// Extracts original payload from shaped data.
    pub fn extract_from_shaped(&self, shaped: &str) -> Result<Vec<u8>> {
        // Find zero-width characters in the shaped data
        let zero_width: String = shaped
            .chars()
            .filter(|c| *c == '\u{200B}' || *c == '\u{200C}')
            .collect();
        
        if zero_width.is_empty() {
            return Err(StealthError::EncodingFailed(
                "No steganographic data found".to_string(),
            ));
        }
        
        self.encoder.decode(&zero_width)
    }

    /// Adds random padding to payload.
    pub fn add_padding(&self, mut payload: Vec<u8>, target_size: usize) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        while payload.len() < target_size {
            payload.push(rng.gen());
        }
        
        payload
    }

    /// Removes padding from payload (assumes last bytes are padding).
    pub fn remove_padding(&self, payload: Vec<u8>, original_size: usize) -> Vec<u8> {
        payload.into_iter().take(original_size).collect()
    }
}

impl Default for PayloadShaper {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple payload obfuscator using XOR.
pub struct XorObfuscator {
    key: Vec<u8>,
}

impl XorObfuscator {
    /// Creates a new XOR obfuscator with the given key.
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    /// Obfuscates data using XOR.
    pub fn obfuscate(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ self.key[i % self.key.len()])
            .collect()
    }

    /// Deobfuscates data (same as obfuscate for XOR).
    pub fn deobfuscate(&self, data: &[u8]) -> Vec<u8> {
        self.obfuscate(data)
    }
}

impl PayloadTransformer for XorObfuscator {
    fn transform(&self, payload: &[u8]) -> Result<Vec<u8>> {
        Ok(self.obfuscate(payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_shaper() {
        let mut shaper = PayloadShaper::new();
        let payload = b"test data";
        
        let shaped = shaper.shape_for_api(payload, ApiType::Slack).unwrap();
        assert!(shaped.contains("slack.com"));
        
        let extracted = shaper.extract_from_shaped(&shaped).unwrap();
        assert_eq!(extracted, payload);
    }

    #[test]
    fn test_padding() {
        let shaper = PayloadShaper::new();
        let payload = vec![1, 2, 3, 4, 5];
        let original_size = payload.len();
        
        let padded = shaper.add_padding(payload.clone(), 100);
        assert_eq!(padded.len(), 100);
        
        let unpadded = shaper.remove_padding(padded, original_size);
        assert_eq!(unpadded, payload);
    }

    #[test]
    fn test_xor_obfuscator() {
        let obfuscator = XorObfuscator::new(vec![0xAB, 0xCD, 0xEF]);
        let data = b"Hello, World!";
        
        let obfuscated = obfuscator.obfuscate(data);
        assert_ne!(obfuscated, data);
        
        let deobfuscated = obfuscator.deobfuscate(&obfuscated);
        assert_eq!(deobfuscated, data);
    }

    #[test]
    fn test_xor_transformer() {
        let transformer = XorObfuscator::new(vec![0x42]);
        let data = b"test";
        
        let transformed = transformer.transform(data).unwrap();
        let back = transformer.transform(&transformed).unwrap();
        assert_eq!(back, data);
    }
}
