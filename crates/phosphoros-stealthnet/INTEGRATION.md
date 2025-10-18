# PHOSPHOROS Stealth Integration Guide

This guide explains how to integrate the PHOSPHOROS stealth networking layer into your blockchain forensic analysis workflows.

## Quick Start

### 1. Add Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
phosphoros-stealthnet = { path = "../phosphoros-stealthnet" }
phosphoros-satellite = { path = "../phosphoros-satellite" }
```

### 2. Basic Configuration

Create a `config.yaml` with stealth settings:

```yaml
api:
  host: "0.0.0.0"
  port: 8080

analysis:
  knn_k: 8
  entropy_bins: 16
  resonance_threshold: 0.65
  max_snapshots: 32

stealth:
  enabled: true
  default_mode: Mimicry
  preferred_api: Slack
  proxy_enabled: false
  enable_logging: true
  enable_jitter: true
  min_jitter_ms: 100
  max_jitter_ms: 1000
```

### 3. Initialize Satellite with Stealth

```rust
use phosphoros_satellite::{SatelliteEngine, SatelliteConfig};

// Load configuration
let config = SatelliteConfig::load_from_path("config.yaml")?;

// Create satellite engine with stealth support
let engine = SatelliteEngine::new(config);

// Check if stealth is enabled
if engine.config().stealth.enabled {
    println!("Stealth mode active: {:?}", engine.config().stealth.default_mode);
}
```

## Use Cases

### Case 1: Basic Forensic Analysis with Mimicry

For standard blockchain analysis where you want to avoid detection:

```rust
use phosphoros_stealthnet::{StealthMode, StealthRequest, StealthProxy};

// Create stealth proxy with mimicry mode
let proxy = StealthProxy::new(StealthMode::Mimicry);

// Build forensic request
let request = StealthRequest::builder()
    .url("https://blockchain-explorer.com/api/address/analyze")
    .method("POST")
    .header("Content-Type", "application/json")
    .body_string(r#"{"address": "0xABC123..."}"#)
    .stealth_mode(StealthMode::Mimicry)
    .timing_jitter(500)
    .build()?;

// Transform through stealth layer
let stealth_request = proxy.transform(request)?;

// Use stealth_request for actual HTTP call
```

### Case 2: High-Security Investigation with Steganography

For sensitive investigations requiring maximum concealment:

```rust
use phosphoros_stealthnet::{
    StealthMode, StealthProxy, StealthRequest,
    ZeroWidthEncoder, SteganographyEncoder,
    PayloadShaper, ApiType,
};

// Create stealth proxy with steganography mode
let mut proxy = StealthProxy::new(StealthMode::Steganography);

// Encode sensitive payload
let encoder = ZeroWidthEncoder::new();
let sensitive_data = b"target_wallet_cluster_analysis";
let hidden_payload = encoder.encode(sensitive_data)?;

// Shape payload to look like legitimate API traffic
let mut shaper = PayloadShaper::new();
let shaped_request = shaper.shape_for_api(
    sensitive_data,
    ApiType::Slack  // Disguise as Slack API
)?;

println!("Payload concealed in {} chars of invisible data", 
         hidden_payload.chars().count());
```

### Case 3: Multi-Proxy Rotation for Large-Scale Analysis

For distributed analysis across multiple proxies:

```rust
use phosphoros_stealthnet::{
    ProxyConfig, ProxyProtocol, ProxyManager, RotationMode,
    StealthProxy, StealthMode,
};

// Create proxy manager
let manager = ProxyManager::new();

// Add multiple proxies
let proxies = vec![
    ProxyConfig::new(ProxyProtocol::Socks5, "proxy1.example.com".into(), 1080),
    ProxyConfig::new(ProxyProtocol::Socks5, "proxy2.example.com".into(), 1080),
    ProxyConfig::new(ProxyProtocol::Http, "proxy3.example.com".into(), 8080)
        .with_auth("user".into(), "pass".into()),
];

for proxy in proxies {
    manager.add_proxy(proxy)?;
}

// Set rotation mode
manager.set_mode(RotationMode::RoundRobin);

// Create stealth proxy
let mut stealth_proxy = StealthProxy::new(StealthMode::Adaptive);
*stealth_proxy.proxy_manager_mut() = manager;

// Each request will automatically rotate proxies
for wallet in wallets_to_analyze {
    let current_proxy = stealth_proxy.proxy_manager().acquire();
    println!("Using proxy: {:?}", current_proxy.map(|p| p.host));
    
    // Make request through current proxy
    // ...
}
```

### Case 4: Adaptive Mode for Dynamic Environments

Let the system choose the best stealth level automatically:

```rust
use phosphoros_stealthnet::{StealthMode, StealthProxy};

// Adaptive mode analyzes context and chooses appropriate stealth
let proxy = StealthProxy::new(StealthMode::Adaptive);

// System automatically:
// - Uses Open mode for low-risk operations
// - Switches to Mimicry for moderate risk
// - Activates Steganography for high-risk situations
```

## Dashboard Integration

### Accessing Stealth Controls

1. Start the PHOSPHOROS Dashboard:
   ```bash
   cargo run -p phosphoros-dashboard
   ```

2. Navigate to the **Stealth/Privacy** panel (🔒 icon)

3. Configure stealth settings:
   - Toggle stealth on/off
   - Select mode (Open/Mimicry/Steganography/Adaptive)
   - Configure proxy rotation
   - Adjust temporal jitter
   - Monitor active stealth tasks

### Panel Features

- **Real-time Statistics**: View active stealth tasks and proxy count
- **Mode Selection**: Quick switching between stealth modes
- **Proxy Management**: Add/remove proxies dynamically
- **Jitter Controls**: Fine-tune timing randomization
- **Compliance Notice**: Always visible legal disclaimer

## Advanced Configuration

### Fine-Tuning Stealth Behavior

```yaml
stealth:
  enabled: true
  default_mode: Adaptive
  
  # Preferred API for mimicry (optional)
  preferred_api: OpenAI
  
  # Proxy settings
  proxy_enabled: true
  proxies:
    - host: "socks-proxy.example.com"
      port: 1080
      protocol: "socks5"
      username: "user1"
      password: "secure_pass"
    
    - host: "http-proxy.example.com"
      port: 8080
      protocol: "http"
  
  proxy_rotation: Random  # RoundRobin, Random, Sequential
  
  # Compliance settings
  enable_logging: true  # Log all stealth operations
  
  # Timing randomization
  enable_jitter: true
  min_jitter_ms: 200
  max_jitter_ms: 2000
```

### Programmatic Configuration

```rust
use phosphoros_satellite::config::{StealthConfig, ProxyConfigEntry};
use phosphoros_stealthnet::{StealthMode, ApiType};

let stealth_config = StealthConfig {
    enabled: true,
    default_mode: StealthMode::Steganography,
    preferred_api: Some(ApiType::Discord),
    proxy_enabled: true,
    proxies: vec![
        ProxyConfigEntry {
            host: "proxy.example.com".into(),
            port: 1080,
            protocol: "socks5".into(),
            username: Some("user".into()),
            password: Some("pass".into()),
        }
    ],
    enable_logging: true,
    enable_jitter: true,
    min_jitter_ms: 100,
    max_jitter_ms: 1000,
    ..Default::default()
};
```

## Testing Stealth Features

### Unit Tests

Run the comprehensive test suite:

```bash
cargo test -p phosphoros-stealthnet
```

Tests cover:
- API mimicry generation (5 API types)
- Steganographic encoding/decoding
- Proxy rotation algorithms
- Payload transformation
- Request building and validation

### Integration Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end_stealth() {
        // Create stealth proxy
        let proxy = StealthProxy::new(StealthMode::Mimicry);
        
        // Build request
        let request = StealthRequest::builder()
            .url("https://test.example.com/api")
            .payload(b"test_data")
            .build()
            .unwrap();
        
        // Transform
        let result = proxy.transform(request);
        assert!(result.is_ok());
    }
}
```

### Example: Verify Steganography

```bash
cargo run --example basic_stealth
```

This example demonstrates:
- Zero-width encoding that appears invisible
- Proxy rotation patterns
- API mimicry templates
- Complete stealth workflow

## Monitoring and Logging

### Enable Compliance Logging

```rust
use phosphoros_satellite::SatelliteConfig;

let mut config = SatelliteConfig::default();
config.stealth.enable_logging = true;

// All stealth operations will be logged
```

### Log Output Example

```
[2025-10-18 14:00:00] INFO Stealth: Mode set to: Mimicry
[2025-10-18 14:00:01] INFO Stealth: Proxy rotation: Enabled
[2025-10-18 14:00:02] INFO Stealth: Task forensic-001 completed successfully
```

### Dashboard Monitoring

The **Stealth/Privacy** panel displays:
- Active stealth tasks count
- Configured proxies count
- Current stealth mode
- Logging status
- Task completion history

## Security Best Practices

### 1. Proxy Security
- Use trusted proxy providers only
- Rotate credentials regularly
- Monitor proxy health
- Implement automatic failover

### 2. Operational Security
- Always test in isolated environments first
- Use minimum necessary stealth level
- Enable logging for compliance
- Maintain proper documentation

### 3. Data Protection
- Payloads are never stored in plain text
- Memory is cleared after use
- No persistent credentials in configuration
- Secure credential management required

### 4. Compliance
- Review local laws before deployment
- Maintain audit trails as required
- Document authorization for operations
- Respect privacy and data protection laws

## Troubleshooting

### Stealth Requests Failing

1. Check proxy configuration:
   ```rust
   let proxy_config = ProxyConfig::new(...);
   proxy_config.validate()?;
   ```

2. Verify stealth mode compatibility
3. Test with Open mode first
4. Check network connectivity

### Performance Issues

- Reduce jitter range for faster operations
- Use Open mode when stealth not required
- Monitor proxy latency
- Consider proxy location

### Encoding Errors

- Verify data format before encoding
- Check for special characters in payloads
- Test steganography separately
- Review encoded data length

## Migration from Python Modules

If migrating from the Python stealth modules in `Upgrade.zip`:

### scorpiosync_generators.py → mimicry.rs
```python
# Python
payload = fake_api_request_generator()
```
```rust
// Rust
let mimicry = ApiMimicry::new();
let template = mimicry.generate_random()?;
```

### ghost_rpc.py → proxy.rs
```python
# Python
manager = GhostRPCManager(proxy_manager)
wave = manager.start_wave(...)
```
```rust
// Rust
let manager = ProxyManager::new();
manager.add_proxy(config)?;
```

### phantomload.py → request.rs + proxy.rs
```python
# Python
node = PhantomNode(...)
```
```rust
// Rust
let request = StealthRequest::builder()...build()?;
let proxy = StealthProxy::new(mode);
```

## API Reference

For complete API documentation:

```bash
cargo doc -p phosphoros-stealthnet --open
```

## Support

For issues or questions:
1. Check [README.md](README.md) for basic usage
2. Review this integration guide
3. Run examples in `examples/`
4. Open an issue on GitHub

## License

MIT OR Apache-2.0
