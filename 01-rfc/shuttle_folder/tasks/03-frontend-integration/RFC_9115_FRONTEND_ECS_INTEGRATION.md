# RFC-9115 FRONTEND ADAPTER → ECS INTEGRATION

**TypeScript/React Frontend Integration with Three-Layer ECS Architecture**

---

## 🎯 EXECUTIVE SUMMARY:

```
FRONTEND (TypeScript/React)
    ↓ WebSocket/REST/gRPC
GATEWAY (18120/18121/18122)
    ↓ Neural Mux (<250ns routing)
ECS LAYERS (apecs → Legion → ATLAS)
    ↓ Trivariate Hash Authentication
BACKEND SERVICES (Supabase, Neon, ChromaDB, R2)
```

**Key Integration Points:**
- ✅ SX9 Gateway ports (18120-18122)
- ✅ Trivariate hash authentication (RFC-9001)
- ✅ Smart Crate deployment manifest
- ✅ Three-layer ECS backend
- ✅ Real-time WebSocket events
- ✅ Health monitoring integration

---

## 📊 COMPLETE ARCHITECTURE:

```
┌─────────────────────────────────────────────────────────────────┐
│  FRONTEND LAYER (TypeScript/React)                             │
│  ═════════════════════════════════                             │
│  • Smart Crate TOML manifest                                   │
│  • sx9-adapter.ts (Gateway bootstrap)                          │
│  • WebSocket client (port 18120)                               │
│  • REST client (port 18121)                                    │
│  • gRPC client (port 18122)                                    │
│  • Trivariate auth token validation                            │
│  • Environment-based vertical config                           │
└─────────────────────────────────────────────────────────────────┘
    ↓ HTTP/WS/gRPC (TLS)
┌─────────────────────────────────────────────────────────────────┐
│  GATEWAY LAYER (SX9 Gateway - RFC-9114)                        │
│  ═════════════════════════════════════                         │
│  • Port 18120: WebSocket (real-time events)                    │
│  • Port 18121: REST API (HTTP/JSON)                           │
│  • Port 18122: gRPC (binary protocol)                         │
│  • Neural Mux routing (<250ns)                                 │
│  • Trivariate hash verification                                │
│  • Service discovery                                           │
└─────────────────────────────────────────────────────────────────┘
    ↓ Internal routing
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: ATLAS DAEMON (Cognitive - 1ms OODA)                 │
│  ═════════════════════════════════════                         │
│  • Port 18106: ATLAS cognitive loop                            │
│  • OODA cycle (Observe/Orient/Decide/Act)                      │
│  • Convergence calculation (H1/H2)                             │
│  • Nonagon 9-vertex analysis                                   │
│  • Crystal realm resonance                                     │
│  • Frontend event orchestration                                │
│  • WebSocket event publishing                                  │
└─────────────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: LEGION ECS (Hot-Path - <1µs)                        │
│  ═════════════════════════════════                             │
│  • SlotGraph entity routing                                    │
│  • Unicode trigger execution                                   │
│  • Delta position tracking (6-decimal)                         │
│  • Ring Bus L2 messaging                                       │
│  • Frontend entity state synchronization                       │
│  • Real-time event streaming                                   │
│  INTEGERS ONLY - NO STRINGS!                                   │
└─────────────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: apecs (Cold-Path - Async I/O)                       │
│  ═════════════════════════════════                             │
│  • Database queries (Supabase/Neon)                            │
│  • File uploads (R2 CDN)                                       │
│  • Vector search (ChromaDB)                                    │
│  • Configuration loading                                       │
│  • Health checks                                               │
│  • Metrics collection                                          │
│  STRINGS ALLOWED - I/O OPS                                     │
└─────────────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────────────┐
│  DATA LAYER (Backend Services)                                │
│  ═══════════════════════════                                   │
│  • Supabase GraphQL (https://supabase.sx9.io)                 │
│  • Neon Postgres (RFC-9005 schema)                            │
│  • ChromaDB Vector CDN (port 18125)                           │
│  • R2 CDN Subscriber (port 18127)                             │
│  • Neo4j Graph (GLAF)                                          │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔌 FRONTEND → GATEWAY CONNECTION:

### **smart-crate.toml (Deployment Manifest)**
```toml
[smart-crate]
name         = "sx9-frontend-${SX9_VERTICAL}"
version      = "1.2.0"
vertical     = "${SX9_VERTICAL}"  # orbital | maritime | cyber
frontend     = "typescript"
backend      = "rust"

[ports]
# Gateway endpoints (auto-registered via Port Manager 18104)
websocket    = 18120  # Real-time events
rest         = 18121  # HTTP/JSON API
grpc         = 18122  # Binary protocol

[backend]
# Layer 3 (ATLAS)
atlas_daemon = "http://localhost:18106"

# Layer 2 (Neural Mux routing)
neural_mux   = "http://localhost:18107"

# Hash engine (trivariate auth)
hash_engine  = "http://localhost:18105"

# Layer 1 (Data services)
database_api = "https://supabase.sx9.io/graphql/v1"
neon_core    = "postgres://${NEON_URL}"
chromadb     = "http://localhost:18125"
r2_cdn       = "http://localhost:18127"

[monitoring]
health       = "http://localhost:18108/health"
metrics      = "http://localhost:18108/metrics"
qa_engine    = "http://localhost:18109"
plasma       = "http://localhost:18110"

[security]
# Trivariate hash authentication (RFC-9001)
auth_header  = "SCH ${SX9_AUTH_TOKEN}"
tls          = true
```

### **TypeScript Adapter (sx9-adapter.ts)**
```typescript
/**
 * SX9 Backend Adapter - ECS Integration
 * Connects TypeScript frontend to three-layer ECS backend
 */

export const SX9_BACKEND = {
  // GATEWAY (Ports 18120-18122)
  WEBSOCKET: import.meta.env.VITE_SX9_WS ?? "ws://localhost:18120/ws",
  REST: import.meta.env.VITE_SX9_API ?? "http://localhost:18121/api/v1",
  GRPC: import.meta.env.VITE_SX9_RPC ?? "http://localhost:18122/grpc",

  // LAYER 3: ATLAS (Cognitive)
  ATLAS: "http://localhost:18106",
  
  // LAYER 2: Neural Mux (Hot-Path Routing)
  NEURAL_MUX: "http://localhost:18107",
  
  // Hash Engine (Trivariate Auth)
  HASH_ENGINE: "http://localhost:18105",

  // LAYER 1: Data Services (Cold-Path)
  SUPABASE_GRAPH: "https://supabase.sx9.io/graphql/v1",
  CHROMA_CDN: "http://localhost:18125",
  R2_CDN: "http://localhost:18127",

  // Monitoring
  HEALTH: "http://localhost:18108/health",
  METRICS: "http://localhost:18108/metrics",
  QA_ENGINE: "http://localhost:18109",
  PLASMA: "http://localhost:18110",
} as const;

export interface GatewayHealth {
  status: "healthy" | "degraded" | "unhealthy";
  services: {
    atlas: boolean;        // Layer 3 cognitive
    neural_mux: boolean;   // Layer 2 routing
    hash_engine: boolean;  // Auth
    legion_world: boolean; // Layer 2 ECS
    apecs_runtime: boolean; // Layer 1 async
  };
  latency_ms: number;
  ecs_tick: number;  // Current Legion tick
}

/**
 * Bootstrap SX9 Gateway - Verify ECS backend connectivity
 * MUST be called before app initialization
 */
export async function bootstrapGateway(): Promise<GatewayHealth> {
  const startTime = performance.now();

  try {
    const res = await fetch(`${SX9_BACKEND.HEALTH}`, {
      method: "GET",
      headers: {
        Accept: "application/json",
      },
      signal: AbortSignal.timeout(5000),
    });

    if (!res.ok) {
      throw new Error(`Gateway health check failed: ${res.status}`);
    }

    const health: GatewayHealth = await res.json();
    const latency = performance.now() - startTime;

    console.log(`✅ SX9 Gateway online (${latency.toFixed(2)}ms)`);
    console.log(`   ATLAS: ${health.services.atlas ? '✓' : '✗'}`);
    console.log(`   Neural Mux: ${health.services.neural_mux ? '✓' : '✗'}`);
    console.log(`   Legion ECS: ${health.services.legion_world ? '✓' : '✗'}`);
    console.log(`   ECS Tick: ${health.ecs_tick}`);

    return {
      ...health,
      latency_ms: latency,
    };
  } catch (error) {
    console.error("❌ SX9 Gateway unavailable:", error);
    throw new Error("SX9 Gateway unavailable - ECS backend offline");
  }
}

/**
 * Validate trivariate hash token (RFC-9001)
 * Format: triv:[SCH]_[CUID]_[UUID]
 */
export function validateAuthToken(token: string): boolean {
  const trivariatePattern =
    /^triv:[A-Za-z0-9+/]{16}_[A-Za-z0-9+/]{16}_[0-9a-f-]{36}$/;
  return trivariatePattern.test(token);
}

/**
 * Create authenticated fetch wrapper with trivariate hash
 */
export function createAuthenticatedFetch(authToken: string) {
  if (!validateAuthToken(authToken)) {
    throw new Error("Invalid trivariate auth token (RFC-9001 violation)");
  }

  return async (url: string, options: RequestInit = {}) => {
    return fetch(url, {
      ...options,
      headers: {
        ...options.headers,
        Authorization: `SCH ${authToken}`,  // RFC-9001 trivariate hash
        "X-SX9-Vertical": import.meta.env.VITE_SX9_VERTICAL ?? "unknown",
        "X-ECS-Layer": "frontend",  // ECS layer identification
      },
    });
  };
}
```

---

## 🔄 REAL-TIME EVENT FLOW:

### **WebSocket Client (ECS Event Stream)**
```typescript
import { SX9_BACKEND } from "./sx9-adapter";

export interface ECSEvent {
  type: "entity_update" | "delta_change" | "atlas_decision" | "health_update";
  layer: "atlas" | "legion" | "apecs";
  timestamp: number;
  tick?: number;  // Legion ECS tick
  data: any;
}

export class SX9WebSocketClient {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private eventHandlers: Map<string, (event: ECSEvent) => void> = new Map();

  connect(authToken: string) {
    this.ws = new WebSocket(SX9_BACKEND.WEBSOCKET);

    this.ws.onopen = () => {
      console.log("✅ WebSocket connected to ECS backend");
      this.reconnectAttempts = 0;

      // Send trivariate authentication
      this.ws?.send(
        JSON.stringify({
          type: "auth",
          token: authToken,
          layer: "frontend",
        })
      );
    };

    this.ws.onmessage = (event) => {
      const ecsEvent: ECSEvent = JSON.parse(event.data);
      this.handleECSEvent(ecsEvent);
    };

    this.ws.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    this.ws.onclose = () => {
      console.log("WebSocket closed - reconnecting to ECS backend...");
      this.attemptReconnect(authToken);
    };
  }

  private handleECSEvent(event: ECSEvent) {
    console.log(`[${event.layer}] ${event.type}`, event.data);
    
    // Route to registered handlers
    const handler = this.eventHandlers.get(event.type);
    if (handler) {
      handler(event);
    }
  }

  /**
   * Subscribe to ECS events by type
   */
  on(eventType: string, handler: (event: ECSEvent) => void) {
    this.eventHandlers.set(eventType, handler);
  }

  /**
   * Send command to ECS backend (routed via Neural Mux)
   */
  sendCommand(command: {
    type: string;
    target_layer: "atlas" | "legion" | "apecs";
    data: any;
  }) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({
        ...command,
        timestamp: Date.now(),
      }));
    } else {
      console.error("WebSocket not connected - cannot send command");
    }
  }

  private attemptReconnect(authToken: string) {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);

      console.log(
        `Reconnecting to ECS backend in ${delay}ms (attempt ${this.reconnectAttempts})`
      );

      setTimeout(() => this.connect(authToken), delay);
    }
  }

  disconnect() {
    this.ws?.close();
  }
}
```

---

## 🎯 REACT INTEGRATION (FULL EXAMPLE):

```typescript
import { useEffect, useState } from "react";
import {
  bootstrapGateway,
  type GatewayHealth,
  SX9_BACKEND,
} from "./lib/sx9-adapter";
import { SX9WebSocketClient, type ECSEvent } from "./lib/sx9-websocket";

function App() {
  const [gatewayHealth, setGatewayHealth] = useState<GatewayHealth | null>(null);
  const [wsClient] = useState(() => new SX9WebSocketClient());
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [ecsEvents, setEcsEvents] = useState<ECSEvent[]>([]);

  useEffect(() => {
    // 1. Bootstrap gateway (verify ECS backend)
    bootstrapGateway()
      .then((health) => {
        setGatewayHealth(health);
        
        // 2. Connect WebSocket for real-time ECS events
        const authToken = import.meta.env.VITE_SX9_AUTH_TOKEN;
        wsClient.connect(authToken);

        // 3. Subscribe to ECS events
        wsClient.on("entity_update", (event) => {
          console.log("Legion entity updated:", event.data);
          setEcsEvents((prev) => [...prev, event]);
        });

        wsClient.on("delta_change", (event) => {
          console.log("Delta position changed:", event.data);
          setEcsEvents((prev) => [...prev, event]);
        });

        wsClient.on("atlas_decision", (event) => {
          console.log("ATLAS decision:", event.data);
          setEcsEvents((prev) => [...prev, event]);
        });

        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });

    return () => {
      wsClient.disconnect();
    };
  }, [wsClient]);

  if (loading) {
    return (
      <div className="loading">
        <h2>Connecting to SX9 ECS Backend...</h2>
        <p>Verifying ATLAS, Legion, and apecs layers...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error">
        <h2>ECS Backend Connection Failed</h2>
        <p>{error}</p>
        <button onClick={() => window.location.reload()}>Retry</button>
      </div>
    );
  }

  return (
    <div className="app">
      <header>
        <h1>SX9 {import.meta.env.VITE_SX9_VERTICAL} Vertical</h1>
        <div className="health-status">
          <span className={`status ${gatewayHealth?.status}`}>
            {gatewayHealth?.status}
          </span>
          <span className="latency">{gatewayHealth?.latency_ms}ms</span>
          <span className="tick">Tick: {gatewayHealth?.ecs_tick}</span>
        </div>
      </header>

      <main>
        <section className="ecs-layers">
          <h2>ECS Layer Status</h2>
          <div className="layer">
            <span className="layer-name">LAYER 3: ATLAS</span>
            <span className={gatewayHealth?.services.atlas ? "online" : "offline"}>
              {gatewayHealth?.services.atlas ? "✓ Online" : "✗ Offline"}
            </span>
          </div>
          <div className="layer">
            <span className="layer-name">LAYER 2: Legion</span>
            <span className={gatewayHealth?.services.legion_world ? "online" : "offline"}>
              {gatewayHealth?.services.legion_world ? "✓ Online" : "✗ Offline"}
            </span>
          </div>
          <div className="layer">
            <span className="layer-name">LAYER 1: apecs</span>
            <span className={gatewayHealth?.services.apecs_runtime ? "online" : "offline"}>
              {gatewayHealth?.services.apecs_runtime ? "✓ Online" : "✗ Offline"}
            </span>
          </div>
        </section>

        <section className="ecs-events">
          <h2>Real-Time ECS Events</h2>
          <div className="event-stream">
            {ecsEvents.slice(-10).reverse().map((event, i) => (
              <div key={i} className={`event ${event.layer}`}>
                <span className="event-layer">[{event.layer}]</span>
                <span className="event-type">{event.type}</span>
                {event.tick && <span className="event-tick">Tick {event.tick}</span>}
                <pre>{JSON.stringify(event.data, null, 2)}</pre>
              </div>
            ))}
          </div>
        </section>

        <section className="actions">
          <h2>ECS Commands</h2>
          <button onClick={() => {
            wsClient.sendCommand({
              type: "trigger_atlas_ooda",
              target_layer: "atlas",
              data: { reason: "manual_trigger" },
            });
          }}>
            Trigger ATLAS OODA Cycle
          </button>

          <button onClick={() => {
            wsClient.sendCommand({
              type: "query_legion_entities",
              target_layer: "legion",
              data: { filter: "active" },
            });
          }}>
            Query Legion Entities
          </button>

          <button onClick={() => {
            wsClient.sendCommand({
              type: "fetch_data",
              target_layer: "apecs",
              data: { source: "supabase" },
            });
          }}>
            Fetch Data (apecs)
          </button>
        </section>
      </main>
    </div>
  );
}

export default App;
```

---

## 📊 DATA FLOW EXAMPLES:

### **Example 1: User Clicks "Trigger ATLAS OODA"**
```
USER ACTION: Click button
    ↓
FRONTEND: wsClient.sendCommand({ type: "trigger_atlas_ooda", target_layer: "atlas" })
    ↓
GATEWAY (18120): WebSocket message → Neural Mux routing
    ↓
LAYER 3 (ATLAS): Receives command, starts 1ms OODA cycle
    ├─ OBSERVE: Gather current state
    ├─ ORIENT: Calculate convergence (H1/H2)
    ├─ DECIDE: Check crystal resonance, SDT gate
    └─ ACT: Execute decision
    ↓
LAYER 2 (Legion): Update entities based on ATLAS decision (<1µs)
    ├─ Update SlotGraphTaskEntity
    ├─ Modify delta positions
    └─ Fire Unicode triggers
    ↓
GATEWAY (18120): Publish "atlas_decision" event via WebSocket
    ↓
FRONTEND: wsClient receives event → Update UI
    ↓
USER: Sees real-time ATLAS decision in event stream
```

### **Example 2: Backend Entity Update**
```
LAYER 2 (Legion): Entity state changes (tool execution complete)
    ↓
Ring Bus L2: Publish entity_update event
    ↓
GATEWAY (18120): Forward to connected WebSocket clients
    ↓
FRONTEND: wsClient.on("entity_update") → Handler called
    ↓
REACT: setState() → UI updates with new entity data
    ↓
USER: Sees updated entity in real-time
```

### **Example 3: Database Query**
```
FRONTEND: Click "Fetch Data"
    ↓
GATEWAY (18121): REST POST /api/v1/query
    ↓
LAYER 1 (apecs): Receive query request (async I/O allowed)
    ├─ Query Supabase GraphQL
    ├─ Query Neon Postgres
    └─ Aggregate results
    ↓
GATEWAY (18121): Return JSON response
    ↓
FRONTEND: Receive data → Update UI
```

---

## ✅ COMPLETE INTEGRATION CHECKLIST:

```
FRONTEND SETUP:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ smart-crate.toml (deployment manifest)
✅ sx9-adapter.ts (Gateway bootstrap + auth)
✅ sx9-websocket.ts (Real-time ECS events)
✅ .env (SX9_AUTH_TOKEN + vertical config)
✅ bootstrapGateway() call in App.tsx
✅ WebSocket event handlers (entity_update, delta_change, atlas_decision)
✅ Trivariate hash validation
✅ Health monitoring display

GATEWAY INTEGRATION:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Port 18120: WebSocket (real-time events)
✅ Port 18121: REST API (HTTP/JSON)
✅ Port 18122: gRPC (binary protocol)
✅ Neural Mux routing (<250ns)
✅ Port Manager registration (18104)

ECS BACKEND CONNECTIVITY:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ LAYER 3 (ATLAS): Port 18106, 1ms OODA loop
✅ LAYER 2 (Legion): Hot-path entity sync, <1µs updates
✅ LAYER 1 (apecs): Database queries, async I/O
✅ Hash Engine: Port 18105, trivariate auth verification
✅ Health Dashboard: Port 18108, continuous monitoring

DATA SERVICES:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Supabase GraphQL: https://supabase.sx9.io
✅ Neon Postgres: RFC-9005 schema
✅ ChromaDB Vector CDN: Port 18125
✅ R2 CDN Subscriber: Port 18127

READY FOR DEPLOYMENT! 🚀
```

**The frontend is now fully integrated with the three-layer ECS architecture!**