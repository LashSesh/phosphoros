# PHOSPHOROS Stealthnet

Stealth networking and invisible payload layer for PHOSPHOROS - enabling undetectable blockchain forensic analysis through traffic mimicry, steganography, and adaptive API camouflage.

## Features

### 🎭 Traffic Mimicry
Disguise forensic analysis traffic as legitimate API calls:
- **OpenAI API** - Chat completions and model queries
- **Slack API** - Workspace and conversation management
- **Telegram Bot API** - Message sending and updates
- **Discord API** - Channel and message operations
- **Generic REST** - Customizable API patterns

### 🔐 Steganography
Hide payloads using advanced encoding techniques:
- **Zero-Width Encoding** - Invisible characters in text (U+200B, U+200C)
- **Header Injection** - Base64-encoded data in HTTP headers
- **Query Parameter Encoding** - URL-safe steganographic parameters
- **Body Embedding** - JSON field injection with invisible data

### 🔀 Payload Shaping
Transform outbound data to appear legitimate:
- Adaptive payload transformation
- Random padding and obfuscation
- XOR-based payload encryption
- Context-aware format selection

### 🌐 Proxy Management
Route traffic through configurable proxies:
- **SOCKS5/HTTP/HTTPS** protocol support
- **Rotation modes**: Round-robin, Random, Sequential
- Authentication with username/password
- Automatic failover and health checks

### ⏱️ Temporal Jitter
Randomize request timing to avoid pattern detection:
- Configurable delay ranges (ms)
- Per-request randomization
- Burst prevention

## Stealth Modes

### Open Mode
Direct requests without concealment. Use for:
- Testing and development
- Non-sensitive operations
- Maximum performance

### Mimicry Mode
Traffic disguised as legitimate API calls. Use for:
- Standard forensic analysis
- Low-risk scanning operations
- Public blockchain analysis

### Steganography Mode
Full steganographic encoding of payloads. Use for:
- High-security operations
- Sensitive investigations
- Maximum concealment required

### Adaptive Mode
Intelligent mode selection based on context. Use for:
- Complex operations
- Variable threat environments
- Automated deployments

## Usage Examples

### Basic Stealth Request

```rust
use phosphoros_stealthnet::{StealthMode, StealthProxy, StealthRequest};

// Create a stealth proxy
let proxy = StealthProxy::new(StealthMode::Mimicry);

// Build a stealth request
let request = StealthRequest::builder()
    .url("https://api.example.com/blockchain/scan")
    .method("POST")
    .payload(b"wallet_address_to_analyze")
    .stealth_mode(StealthMode::Mimicry)
    .timing_jitter(500) // 500ms jitter
    .build()
    .unwrap();

// Transform through stealth layer
let stealth_request = proxy.transform(request).unwrap();
```

### Proxy Configuration

```rust
use phosphoros_stealthnet::{ProxyConfig, ProxyProtocol, ProxyManager, RotationMode};

// Create proxy manager
let manager = ProxyManager::new();

// Add proxies
let proxy1 = ProxyConfig::new(
    ProxyProtocol::Socks5,
    "proxy1.example.com".to_string(),
    1080,
).with_auth("user".to_string(), "pass".to_string());

manager.add_proxy(proxy1).unwrap();

// Set rotation mode
manager.set_mode(RotationMode::RoundRobin);

// Acquire next proxy
let proxy = manager.acquire().unwrap();
```

### API Mimicry

```rust
use phosphoros_stealthnet::{ApiMimicry, ApiType};

let mut mimicry = ApiMimicry::new();

// Generate a Slack API request template
let template = mimicry.generate_for_type(ApiType::Slack).unwrap();

// Randomize template structure
let randomized = mimicry.randomize_template(template);

// Add temporal jitter
let jitter_ms = mimicry.temporal_jitter(100, 1000);
```

### Steganography

```rust
use phosphoros_stealthnet::{ZeroWidthEncoder, SteganographyEncoder};

let encoder = ZeroWidthEncoder::new();

// Encode data
let data = b"sensitive_payload";
let encoded = encoder.encode(data).unwrap();

// Data is now invisible unicode characters
println!("Encoded: {}", encoded); // Appears empty but contains data

// Decode
let decoded = encoder.decode(&encoded).unwrap();
assert_eq!(decoded, data);
```

## Integration with PHOSPHOROS Satellite

The stealth layer integrates seamlessly with the Satellite forensic subsystem:

```rust
use phosphoros_satellite::{SatelliteEngine, SatelliteConfig};
use phosphoros_stealthnet::{StealthMode, StealthProxy};

// Load satellite configuration with stealth settings
let config = SatelliteConfig::load_from_path("config.yaml").unwrap();

// Stealth is configured via config.stealth
if config.stealth.enabled {
    println!("Stealth mode: {:?}", config.stealth.default_mode);
}

// Create satellite engine
let engine = SatelliteEngine::new(config);

// All outbound analysis requests can use stealth mode
```

## Configuration

Example YAML configuration for Satellite with stealth:

```yaml
stealth:
  enabled: true
  default_mode: Mimicry
  preferred_api: Slack
  proxy_enabled: true
  proxies:
    - host: "proxy1.example.com"
      port: 1080
      protocol: "socks5"
      username: "user"
      password: "pass"
  proxy_rotation: RoundRobin
  enable_logging: true
  enable_jitter: true
  min_jitter_ms: 100
  max_jitter_ms: 1000
```

## Dashboard Integration

Access stealth controls through the PHOSPHOROS Dashboard:

1. Navigate to **Stealth/Privacy** panel (🔒)
2. Toggle stealth mode on/off
3. Select stealth mode (Open/Mimicry/Steganography/Adaptive)
4. Configure proxy settings
5. Adjust temporal jitter
6. Monitor active stealth tasks

## Compliance and Legal Notice

⚠️ **IMPORTANT**: This stealth networking layer is intended ONLY for:

- **Legitimate forensic analysis** with proper authorization
- **Scientific research** in controlled environments
- **Defense and security purposes** with legal approval
- **Compliance-approved use cases** under supervision

### User Responsibilities

Users of this system are responsible for:
- Ensuring compliance with all applicable laws and regulations
- Obtaining proper authorization before conducting investigations
- Respecting privacy rights and data protection laws
- Maintaining audit logs as required by jurisdiction
- Using stealth features ethically and legally

### Prohibited Uses

This system must NOT be used for:
- Unauthorized access to computer systems
- Circumventing security measures without permission
- Privacy violations or illegal surveillance
- Any activity that violates local, state, or federal laws

### Logging and Compliance Mode

Enable compliance logging to maintain audit trails:

```rust
// Compliance mode logs all stealth operations
let config = StealthConfig {
    enable_logging: true,
    ..Default::default()
};
```

## Security Considerations

### Operational Security
- Rotate proxies regularly
- Use adaptive mode for high-security operations
- Monitor for detection attempts
- Maintain operational separation

### Data Protection
- Payloads are not stored in clear text
- Logs can be disabled for sensitive operations
- Memory is cleared after use
- No persistent credentials in code

### Best Practices
1. Always test in controlled environments first
2. Use minimum necessary stealth level
3. Maintain compliance documentation
4. Implement proper error handling
5. Monitor for anomalies

## Testing

Run the comprehensive test suite:

```bash
cargo test -p phosphoros-stealthnet
```

Tests cover:
- API mimicry generation
- Steganographic encoding/decoding
- Proxy rotation algorithms
- Payload transformation
- Request building and validation

## Performance

Stealth operations add minimal overhead:
- **Open mode**: ~0ms overhead
- **Mimicry mode**: ~1-5ms overhead
- **Steganography mode**: ~5-10ms overhead
- **Adaptive mode**: ~2-8ms overhead (dynamic)

Temporal jitter adds configured delay (100-1000ms default).

## Contributing

When contributing to stealth features:
1. Ensure all tests pass
2. Add tests for new functionality
3. Document security implications
4. Follow compliance guidelines
5. Review legal considerations

## License

Licensed under MIT OR Apache-2.0

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS" FOR LEGITIMATE FORENSIC AND RESEARCH PURPOSES ONLY. THE AUTHORS AND CONTRIBUTORS ARE NOT RESPONSIBLE FOR MISUSE OR ILLEGAL ACTIVITIES. USERS MUST ENSURE COMPLIANCE WITH ALL APPLICABLE LAWS.
