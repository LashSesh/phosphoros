//! Basic stealth networking example
//!
//! This example demonstrates the core features of the phosphoros-stealthnet crate:
//! - Creating stealth requests
//! - Using different stealth modes
//! - Configuring proxies
//! - Applying steganography
//!
//! Run with: cargo run --example basic_stealth

use phosphoros_stealthnet::{
    ApiMimicry, ApiType, ProxyConfig, ProxyManager, ProxyProtocol, RotationMode,
    SteganographyEncoder, StealthMode, StealthProxy, StealthRequest, ZeroWidthEncoder,
};

fn main() {
    println!("=== PHOSPHOROS Stealthnet Basic Example ===\n");

    // 1. Create a simple stealth request
    println!("1. Creating a basic stealth request:");
    let request = StealthRequest::builder()
        .url("https://api.example.com/blockchain/scan")
        .method("POST")
        .header("Content-Type", "application/json")
        .payload(b"wallet_address_to_analyze")
        .stealth_mode(StealthMode::Open)
        .build()
        .unwrap();

    println!("   URL: {}", request.url);
    println!("   Method: {}", request.method());
    println!("   Mode: {}\n", request.stealth_mode());

    // 2. Demonstrate stealth modes
    println!("2. Available stealth modes:");
    for mode in StealthMode::all() {
        println!(
            "   {} (Level {}): {}",
            mode,
            mode.level(),
            mode.description()
        );
    }
    println!();

    // 3. API Mimicry
    println!("3. API Mimicry - Disguising traffic:");
    let mut mimicry = ApiMimicry::new();

    for api_type in [ApiType::Slack, ApiType::OpenAI, ApiType::Telegram] {
        let template = mimicry.generate_for_type(api_type).unwrap();
        println!("   {} API template:", api_type);
        println!("     Method: {}", template.method);
        println!("     URL: {}", template.url);
        println!("     Headers: {} keys", template.headers.len());

        // Add temporal jitter
        let jitter = mimicry.temporal_jitter(100, 500);
        println!("     Temporal jitter: {}ms", jitter);
    }
    println!();

    // 4. Steganography
    println!("4. Steganographic encoding:");
    let encoder = ZeroWidthEncoder::new();
    let secret_data = b"SECRET_PAYLOAD";

    println!("   Original data: {:?}", String::from_utf8_lossy(secret_data));

    let encoded = encoder.encode(secret_data).unwrap();
    println!("   Encoded data length: {} chars", encoded.chars().count());
    println!("   Encoded (appears invisible): \"{}\"", encoded);

    let decoded = encoder.decode(&encoded).unwrap();
    println!("   Decoded data: {:?}", String::from_utf8_lossy(&decoded));
    println!(
        "   Match: {}",
        if decoded == secret_data { "✓" } else { "✗" }
    );
    println!();

    // 5. Proxy Management
    println!("5. Proxy configuration and rotation:");
    let manager = ProxyManager::new();

    // Add some example proxies
    let proxies = vec![
        ProxyConfig::new(
            ProxyProtocol::Socks5,
            "proxy1.example.com".to_string(),
            1080,
        ),
        ProxyConfig::new(
            ProxyProtocol::Http,
            "proxy2.example.com".to_string(),
            8080,
        )
        .with_auth("user".to_string(), "pass".to_string()),
        ProxyConfig::new(
            ProxyProtocol::Https,
            "proxy3.example.com".to_string(),
            443,
        ),
    ];

    for proxy in proxies {
        manager.add_proxy(proxy).unwrap();
    }

    println!("   Added {} proxies", manager.count());

    // Demonstrate rotation modes
    for rotation_mode in [
        RotationMode::RoundRobin,
        RotationMode::Random,
        RotationMode::Sequential,
    ] {
        manager.set_mode(rotation_mode);
        println!("\n   Rotation mode: {:?}", rotation_mode);

        for i in 0..3 {
            if let Some(proxy) = manager.acquire() {
                println!("     Request {}: {}", i + 1, proxy.host);
            }
        }
    }
    println!();

    // 6. Combining features with StealthProxy
    println!("6. Using StealthProxy for complete stealth:");
    let mut stealth_proxy = StealthProxy::new(StealthMode::Mimicry);

    // Configure proxies
    let proxy_cfg = ProxyConfig::new(
        ProxyProtocol::Socks5,
        "stealth-proxy.example.com".to_string(),
        9050,
    );
    stealth_proxy
        .proxy_manager_mut()
        .add_proxy(proxy_cfg)
        .unwrap();

    println!("   Stealth proxy configured:");
    println!("     Mode: {}", stealth_proxy.mode());
    println!(
        "     Proxies: {}",
        stealth_proxy.proxy_manager().count()
    );

    // Create a stealth request
    let request = StealthRequest::builder()
        .url("https://blockchain-api.example.com/analyze")
        .method("POST")
        .payload(b"forensic_data")
        .stealth_mode(StealthMode::Mimicry)
        .timing_jitter(250)
        .metadata("task_id", "forensic-001")
        .build()
        .unwrap();

    println!("\n   Created stealth request:");
    println!("     URL: {}", request.url);
    println!("     Stealth mode: {}", request.stealth_mode());
    println!(
        "     Timing jitter: {}ms",
        request.timing_jitter.unwrap_or(0)
    );

    // Transform through stealth layer
    let transformed = stealth_proxy.transform(request).unwrap();
    println!("     ✓ Request transformed through stealth layer");
    println!("     Final URL: {}", transformed.url);

    println!("\n=== Example complete! ===");
    println!("\nNote: This example demonstrates API usage.");
    println!("For actual network requests, enable the 'network' feature.");
}
