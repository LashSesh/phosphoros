# PHOSPHOROS API Documentation

## Overview

The PHOSPHOROS Gateway exposes a RESTful API with automatic OpenAPI 3.0 documentation.

**Base URL:** `http://localhost:8080`

---

## Authentication

Currently, the API does not require authentication. Future versions will support JWT/OAuth2.

---

## Endpoints

### Gateway

#### GET /
Service information and available endpoints.

**Response:**
```json
{
  "service": "PHOSPHOROS Gateway",
  "version": "0.1.0",
  "endpoints": {
    "/": "This help message",
    "/health": "Gateway health check",
    "/metrics": "Prometheus metrics",
    "/swagger-ui/": "Interactive API documentation"
  }
}
```

#### GET /health
Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

#### GET /metrics
Prometheus-compatible metrics endpoint.

**Response:** Plain text Prometheus format

---

### Satellite Forensics

#### GET /satellite/health
Satellite subsystem health check.

#### GET /satellite/v1/snapshots
List all ingested blockchain snapshots.

**Response:**
```json
[
  {
    "id": "uuid",
    "label": "btc-block-800000",
    "entity_count": 1000,
    "ingested_at": "2025-01-01T00:00:00Z"
  }
]
```

#### POST /satellite/v1/snapshots
Ingest a new blockchain snapshot.

**Request Body:**
```json
{
  "label": "btc-block-800000",
  "observations": [
    {
      "id": "uuid",
      "address": "bc1q...",
      "features": [0.1, 0.2, 0.3, 0.4, 0.5]
    }
  ]
}
```

**Response:**
```json
{
  "id": "uuid",
  "label": "btc-block-800000",
  "entity_count": 1,
  "ingested_at": "2025-01-01T00:00:00Z"
}
```

#### POST /satellite/v1/analyze/:snapshot_id
Run forensic analysis on a snapshot.

**Request Body:**
```json
{
  "run_clustering": true,
  "run_anomaly_detection": true,
  "run_topology": true
}
```

**Response:**
```json
{
  "snapshot_id": "uuid",
  "analyzed_at": "2025-01-01T00:00:00Z",
  "hotspots": [...],
  "anomalies": [...],
  "topology": {...},
  "entropy": {...}
}
```

#### GET /satellite/v1/reports/latest
Get the most recent analysis report.

---

## Prometheus Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `phosphoros_http_requests_total` | Counter | Total HTTP requests |
| `phosphoros_http_request_duration_seconds` | Histogram | Request latency |
| `phosphoros_snapshots_ingested_total` | Counter | Snapshots ingested |
| `phosphoros_analyses_completed_total` | Counter | Analyses completed |

---

## Interactive Documentation

Visit `/swagger-ui/` for interactive API documentation with try-it-out functionality.

---

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `NOT_FOUND` | 404 | Resource not found |
| `INVALID_REQUEST` | 400 | Invalid request body |
| `INTERNAL_ERROR` | 500 | Server error |
