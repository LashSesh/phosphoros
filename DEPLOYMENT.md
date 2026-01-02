# PHOSPHOROS Deployment Guide

## 🎯 Consolidated Architecture

After the GUI consolidation (Phase 1 + Phase 2), PHOSPHOROS now features a **unified web-only interface** powered by a comprehensive Gateway API.

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    User Access Layer                         │
│  Browser (http://localhost:3000)                            │
│  ├── React 18 Web Dashboard (SPA)                           │
│  ├── TypeScript API Client (15 hooks)                       │
│  └── WebSocket Real-time Updates (8 event types)            │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ HTTP + WebSocket
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Nginx Reverse Proxy                       │
│  ├── /           → SPA routing (index.html fallback)        │
│  ├── /api/*      → Gateway API (http://gateway:8080)        │
│  └── /ws         → WebSocket (ws://gateway:8080/ws)         │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              PHOSPHOROS Gateway (Axum + Tokio)              │
│  ├── Satellite Forensics API (/satellite/v1/*)             │
│  ├── Resonance API          (/api/v1/resonance/*)          │
│  ├── Wallet API             (/api/v1/wallet/*)             │
│  ├── Cluster API            (/api/v1/cluster/*)            │
│  ├── WebSocket Server       (/ws)                           │
│  ├── OpenAPI/Swagger        (/swagger-ui/)                  │
│  └── Prometheus Metrics     (/metrics)                      │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Core Analysis Engines                     │
│  ├── Satellite Engine   (Blockchain Forensics)             │
│  ├── Resonance Engine   (5D Spectral Analysis)             │
│  ├── BIP-39 Engine      (Multichain Derivation)            │
│  └── Cluster Engine     (KNN/DBSCAN/Hierarchical)          │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start with Docker Compose

### Prerequisites

- Docker 24.0+
- Docker Compose v2.0+
- 4GB RAM minimum
- 10GB disk space

### One-Command Deployment

```bash
# Clone repository
git clone https://github.com/your-org/phosphoros.git
cd phosphoros

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f
```

### Access Points

Once deployed, access PHOSPHOROS at:

| Service | URL | Description |
|---------|-----|-------------|
| **Web Dashboard** | http://localhost:3000 | Main user interface |
| **Gateway API** | http://localhost:8080 | REST API endpoints |
| **Swagger UI** | http://localhost:8080/swagger-ui/ | API documentation |
| **WebSocket** | ws://localhost:3000/ws | Real-time events |
| **Metrics** | http://localhost:8080/metrics | Prometheus metrics |
| **Health Check** | http://localhost:8080/health | Service status |

---

## 📦 Services

### Gateway Service

**Container:** `phosphoros_gateway`
**Port:** 8080
**Image:** Built from root `Dockerfile`

Provides the main API server with:
- 25+ REST API endpoints
- 8 WebSocket event types
- OpenAPI 3.0 specification
- Prometheus metrics export

**Environment Variables:**
```yaml
RUST_LOG: info                 # Logging level
PHOSPHOROS_HOST: 0.0.0.0       # Bind address
PHOSPHOROS_PORT: 8080          # Gateway port
```

### Web Dashboard Service

**Container:** `phosphoros_web`
**Port:** 3000 (mapped to 80 internally)
**Image:** Built from `phosphoros-web/Dockerfile`

Provides the React web interface with:
- Single-page application (SPA)
- API proxy via Nginx (`/api` → `gateway:8080`)
- WebSocket proxy (`/ws` → `gateway:8080/ws`)
- Static asset serving

---

## 🔧 Configuration

### Docker Compose Profiles

#### Production (Default)
```bash
docker-compose up -d
```

Starts:
- `gateway` (production mode)
- `web` (Nginx + static build)

#### Development Mode
```bash
docker-compose --profile dev up -d
```

Starts:
- `gateway-dev` (hot reload with cargo-watch)
- `web` (still uses production build)

### Custom Configuration

Create `.env` file in project root:

```env
# Gateway Configuration
RUST_LOG=debug
PHOSPHOROS_HOST=0.0.0.0
PHOSPHOROS_PORT=8080

# Web Configuration
WEB_PORT=3000
API_URL=http://gateway:8080
```

---

## 🛠️ Manual Build & Run

### Build Gateway

```bash
# Build Rust binary
cargo build --release -p phosphoros-gateway

# Run gateway
./target/release/phosphoros-gateway
```

Gateway will start on `http://0.0.0.0:8080`

### Build Web Dashboard

```bash
cd phosphoros-web

# Install dependencies
npm install

# Development server (with hot reload)
npm run dev
# Access: http://localhost:5173

# Production build
npm run build
# Output: dist/ directory

# Preview production build
npm run preview
```

### Serve Production Build

```bash
# Using nginx
cd phosphoros-web/dist
python3 -m http.server 3000

# Or use any static file server
```

---

## 🧪 Testing the Deployment

### 1. Health Check

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "0.7.0"
}
```

### 2. Gateway Info

```bash
curl http://localhost:8080/
```

Expected response:
```json
{
  "service": "PHOSPHOROS Gateway",
  "version": "0.7.0",
  "endpoints": {
    "/satellite/v1": "Blockchain forensics",
    "/api/v1/resonance": "5D spectral analysis",
    "/api/v1/wallet": "BIP-39 multichain derivation",
    "/api/v1/cluster": "Entity clustering",
    "/ws": "WebSocket events"
  }
}
```

### 3. WebSocket Connection Test

```bash
# Using websocat (install: cargo install websocat)
websocat ws://localhost:3000/ws

# Should receive events like:
# {"type":"Log","timestamp":"...","level":"info","message":"..."}
```

### 4. Web Dashboard

Open browser to `http://localhost:3000`

Test features:
- **Resonance Page** (`/resonance`) - 5D spectral analysis
- **Wallet Page** (`/wallet`) - BIP-39 address derivation
- **Cluster Page** (`/cluster`) - Entity clustering

---

## 📊 Monitoring

### Prometheus Metrics

Gateway exposes metrics at `/metrics`:

```bash
curl http://localhost:8080/metrics
```

Key metrics:
- HTTP request counts and latencies
- WebSocket connection count
- Analysis operation timings
- Memory usage

### Docker Logs

```bash
# View all logs
docker-compose logs

# Follow gateway logs
docker-compose logs -f gateway

# Follow web logs
docker-compose logs -f web
```

---

## 🔒 Production Hardening

### HTTPS Setup

Add SSL certificates and update `docker-compose.yml`:

```yaml
web:
  ports:
    - "443:443"
  volumes:
    - ./certs:/etc/nginx/certs:ro
```

Update Nginx config in `phosphoros-web/Dockerfile` to use SSL.

### Security Headers

Nginx configuration includes:
- `X-Real-IP` forwarding
- `X-Forwarded-For` proxying
- `X-Forwarded-Proto` detection

Consider adding:
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `Strict-Transport-Security` (for HTTPS)

### Resource Limits

Add to `docker-compose.yml`:

```yaml
gateway:
  deploy:
    resources:
      limits:
        cpus: '2'
        memory: 2G
      reservations:
        cpus: '1'
        memory: 512M
```

---

## 🐛 Troubleshooting

### Gateway won't start

Check logs:
```bash
docker-compose logs gateway
```

Common issues:
- Port 8080 already in use: `lsof -i :8080`
- Build failure: Check Rust version (`cargo --version`)
- Missing dependencies: Re-run `cargo build --release`

### Web dashboard shows "Cannot connect to API"

1. Check if gateway is running:
   ```bash
   docker-compose ps gateway
   curl http://localhost:8080/health
   ```

2. Check Nginx proxy logs:
   ```bash
   docker-compose logs web
   ```

3. Verify network connectivity:
   ```bash
   docker-compose exec web ping gateway
   ```

### WebSocket connection fails

1. Check browser console for errors
2. Verify WebSocket endpoint:
   ```bash
   curl -i -N -H "Connection: Upgrade" \
        -H "Upgrade: websocket" \
        http://localhost:8080/ws
   ```
3. Check Nginx WebSocket proxy configuration

### Build fails

```bash
# Clean and rebuild
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

---

## 🔄 Updates & Maintenance

### Update PHOSPHOROS

```bash
# Pull latest code
git pull origin main

# Rebuild and restart
docker-compose down
docker-compose build
docker-compose up -d
```

### Backup Data

Currently, PHOSPHOROS runs in-memory. For persistent storage:

1. Add volume mounts in `docker-compose.yml`
2. Configure gateway to use persistent backend
3. Backup mounted volumes regularly

### Scale Deployment

For high-load scenarios:

```yaml
web:
  deploy:
    replicas: 3

gateway:
  deploy:
    replicas: 2
```

Add load balancer in front of services.

---

## 📚 API Documentation

Access interactive API documentation:
- **Swagger UI**: http://localhost:8080/swagger-ui/
- **OpenAPI JSON**: http://localhost:8080/openapi.json

### Example API Calls

#### Resonance Analysis
```bash
curl -X POST http://localhost:8080/api/v1/resonance/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "psi": 0.85,
    "rho": 0.92,
    "omega": 0.78,
    "label": "test-entity"
  }'
```

#### Wallet Derivation
```bash
curl -X POST http://localhost:8080/api/v1/wallet/derive \
  -H "Content-Type: application/json" \
  -d '{
    "phrase": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    "blockchains": ["bitcoin", "ethereum"],
    "account": 0,
    "address_range": {"start": 0, "end": 5}
  }'
```

#### Cluster Analysis
```bash
curl -X POST http://localhost:8080/api/v1/cluster/compute \
  -H "Content-Type: application/json" \
  -d '{
    "snapshot_id": "test-cluster",
    "entities": [
      {"address": "0x1234", "features": [0.1, 0.2, 0.3]},
      {"address": "0x5678", "features": [0.2, 0.3, 0.4]}
    ],
    "algorithm": "knn",
    "k": 3
  }'
```

---

## 🏗️ Architecture Changes (Phase 1 + 2)

### What Changed

**Before Consolidation:**
- 2 separate GUIs (iced desktop + React web)
- Gateway only used by web frontend
- No WebSocket support
- Limited API coverage

**After Consolidation:**
- Single unified web interface
- 25 Gateway API endpoints (up from 11)
- Full WebSocket real-time updates
- Complete type-safe TypeScript client
- All features accessible via web browser

### Migration Path

The desktop GUI (`phosphoros-dashboard`) is deprecated but still available in the codebase. To use it:

```bash
cargo run -p phosphoros-dashboard
```

However, **all future development focuses on the web interface**.

---

## 🎓 Learning Resources

- **Consolidation Plan**: See `CONSOLIDATION_PLAN.md`
- **Phase 2 Status**: See `PHASE_2_STATUS.md`
- **Main README**: See `README.md`
- **API Reference**: http://localhost:8080/swagger-ui/

---

**Last Updated:** 2026-01-01
**Version:** 0.7.0
**Status:** Production Ready ✅
