# PHOSPHOROS Konsolidierungs-Plan

## Aktueller Zustand

### Architektur
- **Desktop GUI** (iced): 12 Panels, autonome Services, direkter Library-Zugriff
- **Web GUI** (React): 8 Pages, HTTP API Calls, D3/ECharts Visualisierungen
- **Gateway** (Axum): REST API, OpenAPI Docs, Prometheus Metrics

### Probleme
1. Duplizierte Features zwischen Desktop und Web
2. Keine Synchronisation zwischen den GUIs
3. Gateway wird nur vom Web-Frontend genutzt
4. Verwirrende Architektur mit 3 Entry Points
5. Hohe Maintenance-Last (2x GUI-Code)

---

## OPTION 1: Unified Web-Only Interface [EMPFOHLEN]

### Konzept
Entferne Desktop GUI, konsolidiere alle Features in eine moderne Web-Anwendung.

### Neue Architektur
```
┌────────────────────────────────────────┐
│  PHOSPHOROS Unified Web Dashboard      │
│  (React + TypeScript + Vite)           │
│  Port: 3000 (Development)              │
│  Port: 80 (Production)                 │
├────────────────────────────────────────┤
│  Features:                             │
│  • 12 Panels (von Desktop migriert)    │
│  • Autonome Services (via Web Workers) │
│  • Real-time Updates (WebSocket)       │
│  • Command Palette (⌘+K)               │
│  • Dark/Light Theme                    │
│  • Export zu CSV/JSON                  │
│  • Offline PWA Support                 │
└──────────────┬─────────────────────────┘
               │
               │ HTTP + WebSocket
               ▼
┌────────────────────────────────────────┐
│  PHOSPHOROS Unified Gateway            │
│  (Axum + Tower + WebSocket)            │
│  Port: 8080                            │
├────────────────────────────────────────┤
│  Features:                             │
│  • REST API (/api/v1/*)                │
│  • WebSocket (/ws)                     │
│  • OpenAPI 3.0 Docs                    │
│  • Prometheus Metrics                  │
│  • Server-Sent Events (SSE)            │
│  • Background Task Queue               │
└──────────────┬─────────────────────────┘
               │
               ▼
┌────────────────────────────────────────┐
│  PHOSPHOROS Core Libraries             │
│  (phosphoros-core, satellite, etc.)    │
└────────────────────────────────────────┘
```

### Migration Steps

#### Phase 1: Gateway Enhancement (Week 1-2)
1. Erweitere Gateway um fehlende Endpoints:
   - `/api/v1/resonance/analyze` (von Desktop)
   - `/api/v1/wallet/derive` (BIP-39 Operations)
   - `/api/v1/cluster/compute` (Cluster Engine)
   - `/api/v1/workflows/execute` (Forensic Workflows)

2. Füge WebSocket Support hinzu:
   - Real-time Logs
   - Service Status Updates
   - Live Analysis Progress

3. Background Task System:
   - Queue für langläufige Analysen
   - Progress Tracking
   - Result Notification

#### Phase 2: Web Frontend Enhancement (Week 3-4)
1. Portiere fehlende Panels vom Desktop:
   - Forensic Workflows Panel
   - Stealth/Privacy Controls
   - System Log mit Filtering

2. Implementiere Web Workers:
   - Background Scraper (ersetzt Desktop Service)
   - Background Analyzer
   - Cluster Engine

3. Füge Real-time Features hinzu:
   - WebSocket Integration
   - Live Activity Feed
   - Server-Sent Events für Notifications

#### Phase 3: Testing & Migration (Week 5-6)
1. Feature-Parity Testing
2. Performance Benchmarking
3. User Acceptance Testing
4. Docker Compose Update
5. Documentation Update

#### Phase 4: Deprecation (Week 7)
1. Markiere `phosphoros-dashboard` als deprecated
2. Archiviere Desktop GUI Code
3. Update README mit neuer Architektur

### Vorteile
✅ Eine einzige UI zu maintainen
✅ Überall zugänglich (Browser)
✅ Keine Installation nötig
✅ Einfachere Architektur
✅ Gateway wird voll genutzt
✅ WebSocket für Real-time Updates
✅ Progressive Web App (PWA) möglich
✅ Einfacheres Deployment

### Nachteile
❌ Desktop GUI geht verloren (weniger Performance)
❌ Browser-Abhängigkeit
❌ Keine Offline-Nutzung (ohne PWA)
❌ Migration Effort

### Code-Beispiele

#### Erweiterter Gateway (Axum + WebSocket)
```rust
// crates/phosphoros-gateway/src/lib.rs
use axum::{
    Router,
    routing::{get, post},
    extract::{WebSocketUpgrade, State},
    response::IntoResponse,
};
use tower_http::cors::CorsLayer;

pub fn build_unified_gateway(engine: Arc<AnalysisEngine>) -> Router {
    Router::new()
        // REST API
        .nest("/api/v1", build_api_routes(engine.clone()))

        // WebSocket for real-time updates
        .route("/ws", get(websocket_handler))

        // Server-Sent Events for notifications
        .route("/events", get(sse_handler))

        // Static file serving (production)
        .nest_service("/", ServeDir::new("dist"))

        // CORS for development
        .layer(CorsLayer::permissive())

        .with_state(engine)
}

fn build_api_routes(engine: Arc<AnalysisEngine>) -> Router {
    Router::new()
        // Satellite endpoints (existing)
        .nest("/satellite", satellite_routes())

        // Resonance endpoints (from desktop)
        .route("/resonance/analyze", post(analyze_resonance))
        .route("/resonance/history", get(get_resonance_history))

        // Wallet endpoints
        .route("/wallet/derive", post(derive_addresses))
        .route("/wallet/import", post(import_seed))

        // Cluster endpoints
        .route("/cluster/compute", post(compute_clusters))
        .route("/cluster/list", get(list_clusters))

        // Workflow endpoints
        .route("/workflows/list", get(list_workflows))
        .route("/workflows/execute", post(execute_workflow))

        // Background tasks
        .route("/tasks/submit", post(submit_task))
        .route("/tasks/:id", get(get_task_status))

        .with_state(engine)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(engine): State<Arc<AnalysisEngine>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, engine))
}

async fn handle_socket(socket: WebSocket, engine: Arc<AnalysisEngine>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to engine events
    let mut event_rx = engine.subscribe_events();

    // Forward events to client
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let msg = serde_json::to_string(&event).unwrap();
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle client messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // Handle client commands
        }
    }
}
```

#### Web Dashboard mit WebSocket
```typescript
// phosphoros-web/src/hooks/useWebSocket.ts
import { useEffect, useRef, useState } from 'react';
import { useQueryClient } from '@tanstack/react-query';

interface EngineEvent {
  type: 'log' | 'analysis_complete' | 'service_status' | 'progress';
  payload: any;
}

export function useWebSocket(url: string) {
  const [isConnected, setIsConnected] = useState(false);
  const [lastEvent, setLastEvent] = useState<EngineEvent | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const queryClient = useQueryClient();

  useEffect(() => {
    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => {
      setIsConnected(true);
      console.log('WebSocket connected');
    };

    ws.onmessage = (event) => {
      const engineEvent: EngineEvent = JSON.parse(event.data);
      setLastEvent(engineEvent);

      // Invalidate queries based on event type
      switch (engineEvent.type) {
        case 'analysis_complete':
          queryClient.invalidateQueries({ queryKey: ['analyses'] });
          break;
        case 'service_status':
          queryClient.invalidateQueries({ queryKey: ['services'] });
          break;
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    ws.onclose = () => {
      setIsConnected(false);
      console.log('WebSocket disconnected');

      // Reconnect after 3 seconds
      setTimeout(() => {
        // Retry connection
      }, 3000);
    };

    return () => {
      ws.close();
    };
  }, [url, queryClient]);

  const send = (data: any) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(data));
    }
  };

  return { isConnected, lastEvent, send };
}
```

#### Background Services als Web Workers
```typescript
// phosphoros-web/src/workers/scraper.worker.ts
import { expose } from 'comlink';

class ScraperService {
  private running = false;
  private interval: number = 60000; // 1 minute

  async start(config: ScraperConfig) {
    this.running = true;

    while (this.running) {
      try {
        // Fetch data from chains
        const data = await this.scrape(config.chains);

        // Send to backend
        await fetch('/api/v1/satellite/ingest', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(data),
        });

        // Post progress to main thread
        self.postMessage({ type: 'progress', data });

      } catch (error) {
        self.postMessage({ type: 'error', error });
      }

      await new Promise(resolve => setTimeout(resolve, this.interval));
    }
  }

  stop() {
    this.running = false;
  }

  private async scrape(chains: string[]) {
    // Implement scraping logic
    return {};
  }
}

expose(ScraperService);
```

---

## OPTION 2: Tauri Hybrid App [PERFORMANCE-OPTIMIERT]

### Konzept
Ersetze beide GUIs durch **Tauri** (Rust Backend + Web Frontend).

### Neue Architektur
```
┌────────────────────────────────────────┐
│  PHOSPHOROS Tauri App                  │
├────────────────────────────────────────┤
│  Frontend: React + TypeScript + Vite  │
│  Backend: Rust (Tauri Core)           │
├────────────────────────────────────────┤
│  Features:                             │
│  • Native Performance                  │
│  • Direkter Rust Library Zugriff      │
│  • Kein HTTP Overhead                 │
│  • Cross-platform (Win/Mac/Linux)     │
│  • Auto-Updates                        │
│  • System Tray Integration            │
└──────────────┬─────────────────────────┘
               │
               │ Tauri Commands (IPC)
               ▼
┌────────────────────────────────────────┐
│  Tauri Backend (Rust)                  │
├────────────────────────────────────────┤
│  • phosphoros-core integration         │
│  • phosphoros-satellite integration    │
│  • Background Services (Tokio)         │
│  • File System Access                  │
│  • Native Notifications                │
└────────────────────────────────────────┘
```

### Vorteile
✅ Native Performance (wie Desktop)
✅ Modernes Web UI (wie Web)
✅ Direkter Library-Zugriff (kein HTTP)
✅ Cross-platform
✅ Kleinere Binary als Electron
✅ Auto-Update Support
✅ System Integration

### Nachteile
❌ Komplett neues Framework
❌ Hoher Migration Effort
❌ Tauri Lernkurve
❌ Keine Browser-Version

### Code-Beispiel

#### Tauri Command (Rust)
```rust
// src-tauri/src/commands.rs
use tauri::command;
use phosphoros_core::HolisticMatrix;
use phosphoros_satellite::AnalyticsPipeline;

#[command]
pub async fn analyze_resonance(
    seed_phrase: String,
    state: tauri::State<'_, AppState>,
) -> Result<ResonanceResult, String> {
    let matrix = HolisticMatrix::from_seed(&seed_phrase)
        .map_err(|e| e.to_string())?;

    let evaluation = matrix.evaluate(
        0.0,
        [1.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0],
        0.0,
    );

    Ok(ResonanceResult::from(evaluation))
}

#[command]
pub async fn compute_clusters(
    snapshot_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<ClusterResult, String> {
    let pipeline = AnalyticsPipeline::new();
    let snapshot = state.get_snapshot(&snapshot_id)?;

    let result = pipeline.run(&snapshot)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ClusterResult::from(result))
}
```

#### Frontend (React)
```typescript
// src/hooks/useTauri.ts
import { invoke } from '@tauri-apps/api/tauri';

export function useResonanceAnalysis() {
  return useMutation({
    mutationFn: async (seedPhrase: string) => {
      return await invoke<ResonanceResult>('analyze_resonance', {
        seedPhrase,
      });
    },
  });
}

export function useClusterComputation() {
  return useMutation({
    mutationFn: async (snapshotId: string) => {
      return await invoke<ClusterResult>('compute_clusters', {
        snapshotId,
      });
    },
  });
}
```

---

## OPTION 3: Gateway-Zentrische Architektur [MINIMALE ÄNDERUNG]

### Konzept
Behalte beide GUIs, aber **alle Datenzugriffe** laufen über Gateway.

### Neue Architektur
```
┌─────────────────────┐   ┌─────────────────────┐
│  Desktop GUI (iced) │   │  Web GUI (React)    │
│  Port: Native       │   │  Port: 3000         │
└──────────┬──────────┘   └──────────┬──────────┘
           │                         │
           │ HTTP API                │ HTTP API
           └──────────┬──────────────┘
                      │
                      ▼
           ┌─────────────────────┐
           │  Unified Gateway    │
           │  Port: 8080         │
           ├─────────────────────┤
           │  • REST API         │
           │  • WebSocket        │
           │  • OpenAPI          │
           │  • Metrics          │
           │  • State Sync       │
           └──────────┬──────────┘
                      │
                      ▼
           ┌─────────────────────┐
           │  Core Libraries     │
           └─────────────────────┘
```

### Änderungen

#### Desktop GUI Refactoring
- Entferne direkte Library-Calls
- Ersetze durch HTTP API Calls (wie Web)
- Nutze gemeinsamen API-Client

#### Gateway Enhancement
- Erweitere um alle Desktop-Features
- State Synchronisation zwischen Clients
- WebSocket für Live-Updates

### Vorteile
✅ Beide GUIs bleiben bestehen
✅ Konsistente Datenzugriffe
✅ State Synchronisation möglich
✅ Gateway wird voll genutzt

### Nachteile
❌ Weiterhin 2 GUIs zu maintainen
❌ Desktop verliert Performance (HTTP Overhead)
❌ Komplexe Synchronisation
❌ Löst nicht das Kernproblem

---

## EMPFEHLUNG: OPTION 1 (Unified Web)

### Warum?
1. **Einfachste Architektur**: Eine UI, ein Gateway, Core Libraries
2. **Geringste Maintenance-Last**: Nur eine Codebase für UI
3. **Maximale Zugänglichkeit**: Browser überall
4. **Moderne Stack**: React 18 + Vite ist ausgereift
5. **Gateway wird voll genutzt**: Kein ungenutzter Code

### Migration Timeline
- **Phase 1** (2 Wochen): Gateway erweitern
- **Phase 2** (2 Wochen): Web Frontend portieren
- **Phase 3** (2 Wochen): Testing & Polishing
- **Phase 4** (1 Woche): Deployment & Deprecation

**Total: 7 Wochen**

### Fallback
Wenn Web-Only zu riskant ist: **Option 2 (Tauri)** als Hybrid-Lösung.

---

## NÄCHSTE SCHRITTE

1. Entscheidung: Welche Option?
2. Feature-Liste finalisieren (was muss migriert werden?)
3. API Design für Gateway (OpenAPI Spec)
4. Prototyp bauen (1 Panel als PoC)
5. Migration planen

Soll ich mit einem Prototyp für **Option 1** starten?
