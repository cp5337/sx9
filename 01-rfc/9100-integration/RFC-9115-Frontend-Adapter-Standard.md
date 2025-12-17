# RFC-9115 — SX9 Frontend Adapter Standard

**Version:** 1.0  
**Status:** Draft  
**Date:** December 14, 2025  
**Applies To:** All SX9 Vertical Frontends (Orbital, Maritime, Cyber)  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9001, RFC-9002, RFC-9114

---

## Abstract

This RFC defines the canonical frontend adapter specification for Synaptix9 (SX9) vertical deployments. It establishes a uniform interface between TypeScript/React frontends and the SX9 Gateway, ensuring consistent port allocation, service discovery, health monitoring, and trivariate hash authentication across all verticals.

**Key Features:**

- Standardized Smart Crate deployment manifest
- TypeScript adapter with gateway bootstrap
- Automatic port registration via Port Manager (18104)
- Unified backend service discovery
- Environment-based vertical configuration

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    FRONTEND VERTICAL                            │
│              (Orbital / Maritime / Cyber)                       │
├─────────────────────────────────────────────────────────────────┤
│  React/TypeScript Application                                   │
│  ├─ adapter.ts (SX9_BACKEND)                                   │
│  ├─ bootstrapGateway()                                         │
│  └─ smart-crate.toml                                           │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 GATEWAY                                  │
├─────────────────────────────────────────────────────────────────┤
│  WebSocket (18120) │ REST (18121) │ gRPC (18122)               │
└──────────────────────┬──────────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
   Neural Mux    ATLAS Daemon    Hash Engine
    (18107)        (18106)         (18105)
```

---

## 2. Smart Crate Deployment Manifest

### 2.1 Standard Template

**File:** `smart-crate.toml`

```toml
[smart-crate]
name         = "sx9-frontend-${SX9_VERTICAL}"
version      = "1.2.0"
description  = "SX9 ${SX9_VERTICAL} vertical frontend"
target_env   = "synaptix9"
frontend     = "typescript"
backend      = "rust"
vertical     = "${SX9_VERTICAL}"           # orbital | maritime | cyber
build_stamp  = "${BUILD_DATE}"

[ports]
# Registered automatically by Port Manager (18104)
websocket    = 18120
rest         = 18121
grpc         = 18122

[backend]
neural_mux   = "http://localhost:18107"
atlas_daemon = "http://localhost:18106"
hash_engine  = "http://localhost:18105"
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
auth_header  = "SCH ${SX9_AUTH_TOKEN}"
tls          = true

[build]
cmd_prebuild = "pnpm install"
cmd_build    = "pnpm run build"
cmd_post     = "sx9ctl register --vertical ${SX9_VERTICAL}"
```

### 2.2 Manifest Validation

All manifests MUST:

- Include `[smart-crate]` section with `vertical` field
- Reference standard ports (18120-18122) for gateway
- Include `auth_header` with trivariate token
- Specify `cmd_post` for Port Manager registration

---

## 3. TypeScript Adapter

### 3.1 Standard Adapter Implementation

**File:** `src/lib/sx9-adapter.ts`

```typescript
/**
 * SX9 Backend Adapter
 * RFC-9115 Compliant
 */

export const SX9_BACKEND = {
  // Gateway Core (RFC-9114)
  WEBSOCKET: import.meta.env.VITE_SX9_WS ?? "ws://localhost:18120/ws",
  REST: import.meta.env.VITE_SX9_API ?? "http://localhost:18121/api/v1",
  GRPC: import.meta.env.VITE_SX9_RPC ?? "http://localhost:18122/grpc",

  // Monitoring (RFC-9114)
  HEALTH: "http://localhost:18108/health",
  METRICS: "http://localhost:18108/metrics",

  // Core Services (RFC-9114)
  HASH_ENGINE: "http://localhost:18105",
  NEURAL_MUX: "http://localhost:18107",
  ATLAS: "http://localhost:18106",

  // Data Plane (RFC-9005, RFC-9114)
  SUPABASE_GRAPH: "https://supabase.sx9.io/graphql/v1",
  CHROMA_CDN: "http://localhost:18125",
  R2_CDN: "http://localhost:18127",
} as const;

export interface GatewayHealth {
  status: "healthy" | "degraded" | "unhealthy";
  services: {
    neural_mux: boolean;
    atlas: boolean;
    hash_engine: boolean;
  };
  latency_ms: number;
}

/**
 * Bootstrap SX9 Gateway connectivity
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
      signal: AbortSignal.timeout(5000), // 5s timeout
    });

    if (!res.ok) {
      throw new Error(`Gateway health check failed: ${res.status}`);
    }

    const health: GatewayHealth = await res.json();
    const latency = performance.now() - startTime;

    console.log(`✅ SX9 Gateway online (${latency.toFixed(2)}ms)`);

    return {
      ...health,
      latency_ms: latency,
    };
  } catch (error) {
    console.error("❌ SX9 Gateway unavailable:", error);
    throw new Error("SX9 Gateway unavailable");
  }
}

/**
 * Validate trivariate hash token (RFC-9001)
 */
export function validateAuthToken(token: string): boolean {
  // Token format: triv:[SCH]_[CUID]_[UUID]
  const trivariatePattern =
    /^triv:[A-Za-z0-9+/]{16}_[A-Za-z0-9+/]{16}_[0-9a-f-]{36}$/;
  return trivariatePattern.test(token);
}

/**
 * Create authenticated fetch wrapper
 */
export function createAuthenticatedFetch(authToken: string) {
  return async (url: string, options: RequestInit = {}) => {
    return fetch(url, {
      ...options,
      headers: {
        ...options.headers,
        Authorization: `SCH ${authToken}`,
        "X-SX9-Vertical": import.meta.env.VITE_SX9_VERTICAL ?? "unknown",
      },
    });
  };
}
```

### 3.2 React Integration

**File:** `src/App.tsx`

```typescript
import { useEffect, useState } from 'react';
import { bootstrapGateway, type GatewayHealth } from './lib/sx9-adapter';

function App() {
  const [gatewayHealth, setGatewayHealth] = useState<GatewayHealth | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    bootstrapGateway()
      .then(health => {
        setGatewayHealth(health);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) return <div>Connecting to SX9 Gateway...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div>
      <h1>SX9 {import.meta.env.VITE_SX9_VERTICAL} Vertical</h1>
      <p>Gateway Status: {gatewayHealth?.status}</p>
      <p>Latency: {gatewayHealth?.latency_ms}ms</p>
    </div>
  );
}

export default App;
```

---

## 4. Environment Configuration

### 4.1 Required Environment Variables

**File:** `.env`

```bash
# Vertical Configuration
SX9_VERTICAL=orbital              # orbital | maritime | cyber

# Authentication (RFC-9001 trivariate token)
SX9_AUTH_TOKEN=triv:ABC123_DEF456_789-uuid

# Database Connections
NEON_URL=postgresql://user:pass@neon.tech/db

# Gateway Endpoints (Production)
VITE_SX9_WS=wss://gateway.sx9.io/ws
VITE_SX9_API=https://gateway.sx9.io/api/v1
VITE_SX9_RPC=https://gateway.sx9.io/grpc

# Development Overrides (optional)
# VITE_SX9_WS=ws://localhost:18120/ws
# VITE_SX9_API=http://localhost:18121/api/v1
# VITE_SX9_RPC=http://localhost:18122/grpc
```

### 4.2 Vertical-Specific Variables

Each vertical MAY define additional variables:

**Orbital:**

```bash
VITE_ORBITAL_TLE_SOURCE=https://celestrak.org/NORAD/elements/
VITE_ORBITAL_DEFAULT_SAT=ISS
```

**Maritime:**

```bash
VITE_MARITIME_AIS_SOURCE=wss://ais.sx9.io/stream
VITE_MARITIME_CHART_LAYER=nautical
```

**Cyber:**

```bash
VITE_CYBER_MITRE_API=https://attack.mitre.org/api/v1
VITE_CYBER_THREAT_FEED=https://feeds.sx9.io/cyber
```

---

## 5. Build and Deployment

### 5.1 Build Process

```bash
# 1. Install dependencies
pnpm install

# 2. Build frontend
pnpm run build

# 3. Register with Port Manager (automatic via smart-crate.toml)
sx9ctl register --vertical ${SX9_VERTICAL}
```

### 5.2 Port Manager Registration

The `sx9ctl register` command:

1. Reads `smart-crate.toml`
2. Validates vertical configuration
3. Registers ports with Port Manager (18104)
4. Generates trivariate hash for deployment
5. Updates gateway routing table

### 5.3 Health Check Integration

All frontends MUST implement health endpoint:

**File:** `public/health.json`

```json
{
  "service": "sx9-frontend-${SX9_VERTICAL}",
  "version": "1.2.0",
  "status": "healthy",
  "dependencies": {
    "gateway": "http://localhost:18120",
    "health_dashboard": "http://localhost:18108"
  }
}
```

---

## 6. Service Discovery

### 6.1 Automatic Service Discovery

Frontends discover backend services via:

1. **Environment Variables** (highest priority)
2. **smart-crate.toml** (build-time)
3. **Gateway Health Endpoint** (runtime)

```typescript
async function discoverServices(): Promise<ServiceMap> {
  // 1. Check environment
  if (import.meta.env.VITE_SX9_API) {
    return parseEnvServices();
  }

  // 2. Query gateway
  const res = await fetch(`${SX9_BACKEND.HEALTH}/services`);
  const services = await res.json();

  return services;
}
```

### 6.2 Service Map Structure

```typescript
interface ServiceMap {
  gateway: {
    websocket: string;
    rest: string;
    grpc: string;
  };
  core: {
    neural_mux: string;
    atlas: string;
    hash_engine: string;
  };
  data: {
    supabase: string;
    neon: string;
    chromadb: string;
    r2_cdn: string;
  };
  monitoring: {
    health: string;
    metrics: string;
    qa: string;
    plasma: string;
  };
}
```

---

## 7. WebSocket Integration

### 7.1 WebSocket Client

```typescript
import { SX9_BACKEND } from "./sx9-adapter";

export class SX9WebSocketClient {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;

  connect(authToken: string) {
    this.ws = new WebSocket(SX9_BACKEND.WEBSOCKET);

    this.ws.onopen = () => {
      console.log("✅ WebSocket connected");
      this.reconnectAttempts = 0;

      // Send authentication
      this.ws?.send(
        JSON.stringify({
          type: "auth",
          token: authToken,
        })
      );
    };

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.handleMessage(message);
    };

    this.ws.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    this.ws.onclose = () => {
      console.log("WebSocket closed");
      this.attemptReconnect(authToken);
    };
  }

  private attemptReconnect(authToken: string) {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);

      console.log(
        `Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`
      );

      setTimeout(() => this.connect(authToken), delay);
    }
  }

  private handleMessage(message: any) {
    // Handle incoming messages
    console.log("Received:", message);
  }

  send(data: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  disconnect() {
    this.ws?.close();
  }
}
```

---

## 8. Compliance Requirements

### 8.1 MUST Requirements

All SX9 frontends MUST:

- Include `smart-crate.toml` with vertical specification
- Implement `bootstrapGateway()` before app initialization
- Use trivariate hash authentication (RFC-9001)
- Register with Port Manager (18104) via `sx9ctl`
- Implement health check endpoint
- Use standard ports (18120-18122) for gateway

### 8.2 SHOULD Requirements

All SX9 frontends SHOULD:

- Implement WebSocket reconnection logic
- Cache service discovery results
- Monitor gateway health continuously
- Report metrics to Health Dashboard (18108)
- Use environment-based configuration

### 8.3 MAY Requirements

All SX9 frontends MAY:

- Implement custom vertical-specific services
- Add additional monitoring endpoints
- Extend `SX9_BACKEND` with vertical services
- Implement offline fallback mode

---

## 9. Testing and Validation

### 9.1 Adapter Validation

```bash
# Validate smart-crate.toml
sx9ctl validate smart-crate.toml

# Test gateway connectivity
sx9ctl test gateway --vertical orbital

# Verify port registration
sx9ctl ports list --vertical orbital
```

### 9.2 Integration Tests

```typescript
import { describe, it, expect } from "vitest";
import { bootstrapGateway, validateAuthToken } from "./sx9-adapter";

describe("SX9 Adapter", () => {
  it("should bootstrap gateway successfully", async () => {
    const health = await bootstrapGateway();
    expect(health.status).toBe("healthy");
    expect(health.latency_ms).toBeLessThan(100);
  });

  it("should validate trivariate tokens", () => {
    const validToken =
      "triv:ABC123DEF456GHIJ_KLM789NOP012QRS_550e8400-e29b-41d4-a716-446655440000";
    expect(validateAuthToken(validToken)).toBe(true);

    const invalidToken = "invalid-token";
    expect(validateAuthToken(invalidToken)).toBe(false);
  });
});
```

---

## 10. Migration Guide

### 10.1 Migrating Existing Frontends

For existing SX9 frontends not using this standard:

1. **Add smart-crate.toml** to repository root
2. **Create sx9-adapter.ts** in `src/lib/`
3. **Update App.tsx** to call `bootstrapGateway()`
4. **Add environment variables** to `.env`
5. **Run `sx9ctl register`** to register with Port Manager
6. **Test connectivity** with `sx9ctl test gateway`

### 10.2 Backward Compatibility

This RFC is backward compatible with:

- RFC-9114 Rev 1.1 (SX9 Gateway)
- RFC-9001 (Trivariate Hashing)
- RFC-9002 (Unicode Routing)

---

## 11. References

- RFC-9001: Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9114: SX9 Gateway Neural Retrofit
- RFC-9005: Unified Schema Specification

---

## 12. Revision History

| Version | Date       | Changes                                          |
| ------- | ---------- | ------------------------------------------------ |
| 1.0     | 2025-12-14 | Initial specification based on sx9-ui-adapter.md |

---

**Status:** This RFC is DRAFT and ready for review.
