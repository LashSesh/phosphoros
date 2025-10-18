//! Steganography encoding for hiding payloads.

use crate::error::{Result, StealthError};

/// Trait for steganographic encoding.
pub trait SteganographyEncoder {
    /// Encodes data into a steganographic format.
    fn encode(&self, data: &[u8]) -> Result<String>;
    
    /// Decodes data from a steganographic format.
    fn decode(&self, encoded: &str) -> Result<Vec<u8>>;
}

/// Zero-width character encoder for hiding data.
pub struct ZeroWidthEncoder;

impl ZeroWidthEncoder {
    /// Creates a new zero-width encoder.
    pub fn new() -> Self {
        Self
    }

    /// Encodes a byte as zero-width characters.
    fn encode_byte(&self, byte: u8) -> String {
        let binary = format!("{:08b}", byte);
        binary
            .chars()
            .map(|bit| {
                if bit == '1' {
                    '\u{200B}' // Zero-width space
                } else {
                    '\u{200C}' // Zero-width non-joiner
                }
            })
            .collect()
    }

    /// Decodes zero-width characters to a byte.
    fn decode_byte(&self, chars: &[char]) -> Result<u8> {
        if chars.len() != 8 {
            return Err(StealthError::EncodingFailed(
                format!("Invalid zero-width sequence length: expected 8, got {}", chars.len()),
            ));
        }

        let binary: String = chars
            .iter()
            .map(|c| {
                if *c == '\u{200B}' {
                    '1'
                } else if *c == '\u{200C}' {
                    '0'
                } else {
                    '?'
                }
            })
            .collect();

        if binary.contains('?') {
            return Err(StealthError::EncodingFailed(
                "Invalid zero-width character".to_string(),
            ));
        }

        u8::from_str_radix(&binary, 2)
            .map_err(|e| StealthError::EncodingFailed(format!("Binary parsing failed: {}", e)))
    }
}

impl Default for ZeroWidthEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl SteganographyEncoder for ZeroWidthEncoder {
    fn encode(&self, data: &[u8]) -> Result<String> {
        Ok(data.iter().map(|&byte| self.encode_byte(byte)).collect())
    }

    fn decode(&self, encoded: &str) -> Result<Vec<u8>> {
        let chars: Vec<char> = encoded.chars().collect();
        let mut result = Vec::new();

        for chunk in chars.chunks(8) {
            result.push(self.decode_byte(chunk)?);
        }

        Ok(result)
    }
}

/// Header injection encoder for hiding data in HTTP headers.
pub struct HeaderInjectionEncoder {
    /// Header name to use
    header_name: String,
}

impl HeaderInjectionEncoder {
    /// Creates a new header injection encoder.
    pub fn new(header_name: String) -> Self {
        Self { header_name }
    }

    /// Encodes data as base64 in a header value.
    pub fn encode_to_header(&self, data: &[u8]) -> (String, String) {
        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        (self.header_name.clone(), encoded)
    }

    /// Decodes data from a header value.
    pub fn decode_from_header(&self, value: &str) -> Result<Vec<u8>> {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD
            .decode(value)
            .map_err(|e| StealthError::EncodingFailed(format!("Base64 decode failed: {}", e)))
    }
}

/// Query parameter encoder for hiding data in URL parameters.
pub struct QueryParamEncoder {
    /// Parameter name to use
    param_name: String,
}

impl QueryParamEncoder {
    /// Creates a new query parameter encoder.
    pub fn new(param_name: String) -> Self {
        Self { param_name }
    }

    /// Encodes data into a URL parameter.
    pub fn encode_to_param(&self, data: &[u8]) -> (String, String) {
        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data);
        (self.param_name.clone(), encoded)
    }

    /// Decodes data from a URL parameter.
    pub fn decode_from_param(&self, value: &str) -> Result<Vec<u8>> {
        use base64::Engine as _;
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(value)
            .map_err(|e| StealthError::EncodingFailed(format!("Base64 decode failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_width_encode_decode() {
        let encoder = ZeroWidthEncoder::new();
        let data = b"Hello, World!";
        
        let encoded = encoder.encode(data).unwrap();
        assert!(!encoded.is_empty());
        assert!(encoded.contains('\u{200B}') || encoded.contains('\u{200C}'));
        
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_zero_width_encode_byte() {
        let encoder = ZeroWidthEncoder::new();
        let encoded = encoder.encode_byte(0b10101010);
        // Each character is a zero-width character
        let char_count = encoded.chars().count();
        assert_eq!(char_count, 8);
    }

    #[test]
    fn test_header_injection() {
        let encoder = HeaderInjectionEncoder::new("X-Custom-Data".to_string());
        let data = b"secret payload";
        
        let (name, value) = encoder.encode_to_header(data);
        assert_eq!(name, "X-Custom-Data");
        
        let decoded = encoder.decode_from_header(&value).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_query_param_encoding() {
        let encoder = QueryParamEncoder::new("token".to_string());
        let data = b"hidden data";
        
        let (name, value) = encoder.encode_to_param(data);
        assert_eq!(name, "token");
        assert!(!value.contains('='));
        
        let decoded = encoder.decode_from_param(&value).unwrap();
        assert_eq!(decoded, data);
    }
}
