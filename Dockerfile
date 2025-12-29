# PHOSPHOROS - Quantum-Enhanced Blockchain Forensics
# Multi-stage Docker build for production deployment

# ============================================================
# Build Stage
# ============================================================
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY ouroboros_dna/ ouroboros_dna/
COPY phosphoros-kryptogenetik/ phosphoros-kryptogenetik/

# Build release binaries
# Enable commonly used features for production
RUN cargo build --release \
    --package phosphoros-gateway \
    --package phosphoros-cli

# ============================================================
# Runtime Stage
# ============================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -U -s /bin/bash phosphoros

WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/phosphoros-gateway /usr/local/bin/
COPY --from=builder /app/target/release/phosphoros-cli /usr/local/bin/

# Copy documentation for reference
COPY README.md LAW_ENFORCEMENT_GUIDE.md ./docs/

# Change ownership
RUN chown -R phosphoros:phosphoros /app

# Switch to non-root user
USER phosphoros

# Expose gateway port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Environment variables
ENV RUST_LOG=info
ENV PHOSPHOROS_HOST=0.0.0.0
ENV PHOSPHOROS_PORT=8080

# Default command
CMD ["phosphoros-gateway"]

# ============================================================
# Development Stage (optional)
# ============================================================
FROM builder AS development

# Install development tools
RUN cargo install cargo-watch

WORKDIR /app

# Keep source mounted for development
VOLUME ["/app"]

CMD ["cargo", "watch", "-x", "run -p phosphoros-gateway"]
