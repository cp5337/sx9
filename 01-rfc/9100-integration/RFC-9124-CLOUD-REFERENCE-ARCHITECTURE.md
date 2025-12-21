# RFC-9124: Cloud Reference Architecture — CTAS Primary, Forge Upsell

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Depends On:** RFC-9120, RFC-9121, RFC-9122, RFC-9123

---

## Abstract

RFC-9124 specifies the cloud reference architecture for SX9's two primary verticals: CTAS (Convergent Threat Analysis System) as the core product and Forge as the premium upsell tier. The architecture uses Cloudflare for edge/gateway and GCP Cloud Run for compute, staying within free/low-cost tiers for demos while providing a production-ready structure that scales with revenue.

**Business Model:**
- **CTAS** = The product analysts use daily
- **Forge** = The upsell that lets customers build on CTAS

**Design Principles:**
1. CTAS is always the primary tenant — Forge is gated
2. Zero-cost demo capability with production structure
3. Same infrastructure serves both tiers (feature flags, not separate deployments)
4. Reference implementation that mirrors local gold disk architecture

---

## 1. Business Context

### 1.1 Target Customers

| Segment | CTAS Users | Forge Unlock | Example Orgs |
|---------|------------|--------------|--------------|
| **SOF/IC Direct** | 50-200 analysts | Unlikely (use as-is) | USSOCOM, DIA, CIA |
| **Large Contractors** | 500-5000 analysts | Primary upsell target | Booz Allen, SAIC, Leidos, GD-IT, Raytheon |
| **Mid-size SDVOSB** | 50-200 analysts | Secondary target | Credence, Barbaricum, MDW |
| **LE/DHS** | 100-500 analysts | Case-by-case | FBI, HSI, CBP |

### 1.2 Revenue Model

```
CTAS Base License
├── Per-seat subscription ($X/analyst/month)
├── Intel feed tier (Basic → Premium → Custom)
└── Support tier (Standard → Priority → Dedicated)

Forge Unlock (Premium Tier)
├── Platform fee (flat annual)
├── Per-developer seat ($Y/dev/month)
├── Private registry allocation
├── Custom pattern authoring
└── Priority factory queue
```

### 1.3 Value Proposition

**CTAS alone:**
> "Your analysts correlate threats 10x faster than manual methods."

**CTAS + Forge:**
> "Your analysts build mission-specific tools in hours, not months. Stop waiting for vendor roadmap. Your team owns the capability."

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CLOUD REFERENCE ARCHITECTURE                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CLOUDFLARE EDGE                                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                                                                       │  │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │  │
│  │  │   Workers   │    │     KV      │    │     R2      │              │  │
│  │  │             │    │             │    │             │              │  │
│  │  │ • Gateway   │    │ • Sessions  │    │ • Artifacts │              │  │
│  │  │ • Auth      │    │ • Patterns  │    │ • Reports   │              │  │
│  │  │ • Routing   │    │ • Cache     │    │ • Gold Disk │              │  │
│  │  │ • Tier Gate │    │ • Feature   │    │   Backups   │              │  │
│  │  │             │    │   Flags     │    │             │              │  │
│  │  └──────┬──────┘    └─────────────┘    └─────────────┘              │  │
│  │         │                                                            │  │
│  │         │  100k req/day free                                        │  │
│  └─────────┼────────────────────────────────────────────────────────────┘  │
│            │                                                                │
│            │                                                                │
│  GCP CLOUD RUN                                                             │
│  ┌─────────┼────────────────────────────────────────────────────────────┐  │
│  │         ▼                                                             │  │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │  │
│  │  │                      CTAS SERVICES (Tier 1)                      │ │  │
│  │  │                                                                   │ │  │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │ │  │
│  │  │  │ Threat   │  │ Pattern  │  │ Adversary│  │ Intel    │        │ │  │
│  │  │  │ Analysis │  │ Matching │  │ Modeling │  │ Feeds    │        │ │  │
│  │  │  │ API      │  │ API      │  │ API      │  │ API      │        │ │  │
│  │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │ │  │
│  │  │                                                                   │ │  │
│  │  └───────────────────────────────────────────────────────────────────┘ │  │
│  │                                                                         │  │
│  │  ┌─────────────────────────────────────────────────────────────────┐   │  │
│  │  │                    FORGE SERVICES (Tier 2 - Gated)               │  │  │
│  │  │                                                                   │  │  │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │  │  │
│  │  │  │ Forge    │  │ Lightning│  │ Pattern  │  │ Registry │        │  │  │
│  │  │  │ API      │  │ QA API   │  │ Authoring│  │ API      │        │  │  │
│  │  │  │          │  │          │  │ API      │  │          │        │  │  │
│  │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │  │  │
│  │  │                                                                   │  │  │
│  │  │  🔒 Requires Forge license                                       │  │  │
│  │  └───────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                         │  │
│  │  2M req/month free                                                     │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  GCP SUPPORTING SERVICES                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  Firestore (1GB free)  │  Cloud Storage  │  Secret Manager          │   │
│  │  • User profiles       │  • Larger files │  • API keys              │   │
│  │  • Org settings        │  • Exports      │  • Credentials           │   │
│  │  • Audit logs          │                 │                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Cloudflare Layer

### 3.1 API Gateway Worker

```typescript
// workers/gateway/src/index.ts
// Main API gateway - routes to CTAS or Forge based on tier

import { Router } from 'itty-router';

interface Env {
  SESSIONS: KVNamespace;
  PATTERNS: KVNamespace;
  FEATURES: KVNamespace;
  ARTIFACTS: R2Bucket;
  CTAS_API: string;
  FORGE_API: string;
}

interface Session {
  userId: string;
  orgId: string;
  tier: 'ctas' | 'ctas_forge' | 'enterprise';
  permissions: string[];
  expiresAt: number;
}

const router = Router();

// ═══════════════════════════════════════════════════════════════════════════════
// AUTHENTICATION
// ═══════════════════════════════════════════════════════════════════════════════

async function authenticate(request: Request, env: Env): Promise<Session | null> {
  const authHeader = request.headers.get('Authorization');
  if (!authHeader?.startsWith('Bearer ')) {
    return null;
  }
  
  const token = authHeader.slice(7);
  const sessionKey = `session:${token}`;
  
  const sessionData = await env.SESSIONS.get(sessionKey, 'json');
  if (!sessionData) {
    return null;
  }
  
  const session = sessionData as Session;
  if (session.expiresAt < Date.now()) {
    await env.SESSIONS.delete(sessionKey);
    return null;
  }
  
  return session;
}

function requireTier(session: Session, requiredTier: 'ctas' | 'ctas_forge' | 'enterprise'): boolean {
  const tierHierarchy = {
    'ctas': 1,
    'ctas_forge': 2,
    'enterprise': 3,
  };
  
  return tierHierarchy[session.tier] >= tierHierarchy[requiredTier];
}

// ═══════════════════════════════════════════════════════════════════════════════
// CTAS ROUTES (Tier 1 - All customers)
// ═══════════════════════════════════════════════════════════════════════════════

// Threat Analysis
router.post('/api/v1/analyze/threat', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  // Forward to CTAS API
  return fetch(`${env.CTAS_API}/analyze/threat`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Org-Id': session.orgId,
      'X-User-Id': session.userId,
    },
    body: request.body,
  });
});

// Pattern Matching
router.post('/api/v1/patterns/match', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  // Check cache first
  const body = await request.json();
  const cacheKey = `pattern:match:${JSON.stringify(body)}`;
  const cached = await env.PATTERNS.get(cacheKey);
  
  if (cached) {
    return new Response(cached, {
      headers: { 'Content-Type': 'application/json', 'X-Cache': 'HIT' },
    });
  }
  
  // Forward to CTAS API
  const response = await fetch(`${env.CTAS_API}/patterns/match`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  
  const result = await response.text();
  
  // Cache for 5 minutes
  await env.PATTERNS.put(cacheKey, result, { expirationTtl: 300 });
  
  return new Response(result, {
    headers: { 'Content-Type': 'application/json', 'X-Cache': 'MISS' },
  });
});

// Adversary Profiles
router.get('/api/v1/adversary/:id', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  const { id } = request.params;
  return fetch(`${env.CTAS_API}/adversary/${id}`, {
    headers: { 'X-Org-Id': session.orgId },
  });
});

// Intel Feeds
router.get('/api/v1/feeds', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  return fetch(`${env.CTAS_API}/feeds`, {
    headers: { 'X-Org-Id': session.orgId },
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// FORGE ROUTES (Tier 2 - Premium customers only)
// ═══════════════════════════════════════════════════════════════════════════════

// Tier gate middleware for Forge routes
async function forgeGate(request: Request, env: Env): Promise<Response | null> {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  if (!requireTier(session, 'ctas_forge')) {
    return new Response(JSON.stringify({
      error: 'Forge license required',
      upgrade_url: 'https://sx9.dev/upgrade',
      message: 'This feature requires CTAS + Forge tier. Contact sales for upgrade.',
    }), { status: 403 });
  }
  
  return null; // Continue to handler
}

// Forge API - Prompt generation
router.post('/api/v1/forge/prompt', async (request, env) => {
  const gateResponse = await forgeGate(request, env);
  if (gateResponse) return gateResponse;
  
  const session = await authenticate(request, env);
  
  return fetch(`${env.FORGE_API}/prompt`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Org-Id': session!.orgId,
      'X-User-Id': session!.userId,
    },
    body: request.body,
  });
});

// Forge API - Lightning QA
router.post('/api/v1/forge/qa/analyze', async (request, env) => {
  const gateResponse = await forgeGate(request, env);
  if (gateResponse) return gateResponse;
  
  const session = await authenticate(request, env);
  
  return fetch(`${env.FORGE_API}/qa/analyze`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Org-Id': session!.orgId,
    },
    body: request.body,
  });
});

// Forge API - Pattern authoring
router.post('/api/v1/forge/patterns', async (request, env) => {
  const gateResponse = await forgeGate(request, env);
  if (gateResponse) return gateResponse;
  
  const session = await authenticate(request, env);
  
  // Require pattern:write permission
  if (!session!.permissions.includes('pattern:write')) {
    return new Response(JSON.stringify({
      error: 'Permission denied',
      required: 'pattern:write',
    }), { status: 403 });
  }
  
  return fetch(`${env.FORGE_API}/patterns`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Org-Id': session!.orgId,
      'X-User-Id': session!.userId,
    },
    body: request.body,
  });
});

// Forge API - Private registry
router.get('/api/v1/forge/registry/:org/*', async (request, env) => {
  const gateResponse = await forgeGate(request, env);
  if (gateResponse) return gateResponse;
  
  const session = await authenticate(request, env);
  const { org } = request.params;
  
  // Can only access own org's registry
  if (org !== session!.orgId) {
    return new Response(JSON.stringify({ error: 'Access denied' }), { status: 403 });
  }
  
  return fetch(`${env.FORGE_API}/registry/${org}${request.url.split(org)[1]}`, {
    headers: { 'X-Org-Id': session!.orgId },
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// ARTIFACT ROUTES (R2)
// ═══════════════════════════════════════════════════════════════════════════════

router.get('/api/v1/artifacts/:key', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  const { key } = request.params;
  
  // Artifacts are org-scoped
  const objectKey = `${session.orgId}/${key}`;
  const object = await env.ARTIFACTS.get(objectKey);
  
  if (!object) {
    return new Response(JSON.stringify({ error: 'Not found' }), { status: 404 });
  }
  
  return new Response(object.body, {
    headers: {
      'Content-Type': object.httpMetadata?.contentType || 'application/octet-stream',
    },
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH & META
// ═══════════════════════════════════════════════════════════════════════════════

router.get('/health', () => {
  return new Response(JSON.stringify({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: '1.0.0',
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
});

router.get('/api/v1/tier', async (request, env) => {
  const session = await authenticate(request, env);
  if (!session) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }
  
  const features = {
    ctas: {
      threat_analysis: true,
      pattern_matching: true,
      adversary_profiles: true,
      intel_feeds: true,
    },
    forge: {
      prompt_generation: requireTier(session, 'ctas_forge'),
      lightning_qa: requireTier(session, 'ctas_forge'),
      pattern_authoring: requireTier(session, 'ctas_forge'),
      private_registry: requireTier(session, 'ctas_forge'),
    },
  };
  
  return new Response(JSON.stringify({
    tier: session.tier,
    org: session.orgId,
    features,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
});

// 404 handler
router.all('*', () => {
  return new Response(JSON.stringify({ error: 'Not found' }), { status: 404 });
});

export default {
  fetch: router.handle,
};
```

### 3.2 Cloudflare Configuration

```toml
# wrangler.toml

name = "sx9-gateway"
main = "src/index.ts"
compatibility_date = "2024-01-01"

# KV Namespaces
[[kv_namespaces]]
binding = "SESSIONS"
id = "xxx"

[[kv_namespaces]]
binding = "PATTERNS"
id = "xxx"

[[kv_namespaces]]
binding = "FEATURES"
id = "xxx"

# R2 Bucket
[[r2_buckets]]
binding = "ARTIFACTS"
bucket_name = "sx9-artifacts"

# Environment variables (secrets via `wrangler secret put`)
[vars]
CTAS_API = "https://ctas-api-xxx.run.app"
FORGE_API = "https://forge-api-xxx.run.app"

# Routes
[[routes]]
pattern = "api.sx9.dev/*"
zone_name = "sx9.dev"

# Demo environment
[env.demo]
vars = { CTAS_API = "https://ctas-demo-xxx.run.app", FORGE_API = "https://forge-demo-xxx.run.app" }
```

---

## 4. GCP Cloud Run Layer

### 4.1 CTAS API Service

```dockerfile
# Dockerfile.ctas-api

FROM rust:1.74-slim AS builder

WORKDIR /build

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build
COPY src ./src
RUN cargo build --release

# Runtime
FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /build/target/release/ctas-api /usr/local/bin/

EXPOSE 8080

ENV PORT=8080
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/ctas-api"]
```

```rust
// ctas-api/src/main.rs

use axum::{
    routing::{get, post},
    Router, Json, Extension,
    extract::{Path, State},
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    // Database connections, etc.
}

// ═══════════════════════════════════════════════════════════════════════════════
// THREAT ANALYSIS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct ThreatAnalysisRequest {
    indicators: Vec<String>,
    context: Option<String>,
    depth: Option<u32>,
}

#[derive(Serialize)]
struct ThreatAnalysisResponse {
    threat_id: String,
    confidence: f64,
    classifications: Vec<ThreatClassification>,
    related_adversaries: Vec<String>,
    recommended_actions: Vec<String>,
}

#[derive(Serialize)]
struct ThreatClassification {
    category: String,
    subcategory: String,
    confidence: f64,
}

async fn analyze_threat(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(request): Json<ThreatAnalysisRequest>,
) -> Json<ThreatAnalysisResponse> {
    let org_id = headers.get("X-Org-Id").map(|h| h.to_str().unwrap_or(""));
    
    // Threat analysis logic here
    // This would integrate with your existing CTAS engine
    
    Json(ThreatAnalysisResponse {
        threat_id: uuid::Uuid::new_v4().to_string(),
        confidence: 0.87,
        classifications: vec![
            ThreatClassification {
                category: "IED".to_string(),
                subcategory: "VBIED".to_string(),
                confidence: 0.92,
            }
        ],
        related_adversaries: vec!["APT-29".to_string()],
        recommended_actions: vec!["Escalate to JIEDDO".to_string()],
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// PATTERN MATCHING
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct PatternMatchRequest {
    input: String,
    pattern_types: Option<Vec<String>>,
    threshold: Option<f64>,
}

#[derive(Serialize)]
struct PatternMatchResponse {
    matches: Vec<PatternMatch>,
    processing_time_ms: u64,
}

#[derive(Serialize)]
struct PatternMatch {
    pattern_id: String,
    pattern_name: String,
    confidence: f64,
    matched_segments: Vec<String>,
}

async fn match_patterns(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PatternMatchRequest>,
) -> Json<PatternMatchResponse> {
    let start = std::time::Instant::now();
    
    // Pattern matching logic here
    
    Json(PatternMatchResponse {
        matches: vec![
            PatternMatch {
                pattern_id: "ied-001".to_string(),
                pattern_name: "Standard VBIED Signature".to_string(),
                confidence: 0.89,
                matched_segments: vec!["vehicle approach".to_string(), "timing circuit".to_string()],
            }
        ],
        processing_time_ms: start.elapsed().as_millis() as u64,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// ADVERSARY PROFILES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
struct AdversaryProfile {
    id: String,
    name: String,
    aliases: Vec<String>,
    ttps: Vec<String>,
    attribution_confidence: f64,
    last_activity: String,
}

async fn get_adversary(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<AdversaryProfile> {
    // Lookup adversary profile
    
    Json(AdversaryProfile {
        id: id.clone(),
        name: "APT-29".to_string(),
        aliases: vec!["Cozy Bear".to_string(), "The Dukes".to_string()],
        ttps: vec!["T1566".to_string(), "T1059".to_string()],
        attribution_confidence: 0.85,
        last_activity: "2024-12-15".to_string(),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    
    let state = Arc::new(AppState {});
    
    let app = Router::new()
        .route("/analyze/threat", post(analyze_threat))
        .route("/patterns/match", post(match_patterns))
        .route("/adversary/:id", get(get_adversary))
        .route("/feeds", get(|| async { Json(serde_json::json!({"feeds": []})) }))
        .route("/health", get(|| async { Json(serde_json::json!({"status": "healthy"})) }))
        .with_state(state);
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    tracing::info!("CTAS API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 4.2 Forge API Service

```rust
// forge-api/src/main.rs

use axum::{
    routing::{get, post},
    Router, Json, Extension,
    extract::{Path, State},
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    // Forge-specific state
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROMPT GENERATION (Thalmic + Pattern Resolution)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct PromptRequest {
    intent: String,
    context: Option<String>,
}

#[derive(Serialize)]
struct PromptResponse {
    session_id: String,
    thalmic_score: f64,
    pattern_resolved: String,
    interview: serde_json::Value,
    canonical_prompt: String,
    ready_for_factory: bool,
}

async fn generate_prompt(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(request): Json<PromptRequest>,
) -> Json<PromptResponse> {
    let org_id = headers.get("X-Org-Id").map(|h| h.to_str().unwrap_or("unknown"));
    
    // This is the RFC-9120 pipeline:
    // 1. Thalmic filter scores clarity
    // 2. Pattern resolver maps to design pattern
    // 3. Interview auto-populates
    // 4. Canonical prompt assembles
    
    Json(PromptResponse {
        session_id: uuid::Uuid::new_v4().to_string(),
        thalmic_score: 0.87,
        pattern_resolved: "reactor".to_string(),
        interview: serde_json::json!({
            "identity": {
                "name": "example-adapter",
                "type": "crate",
            },
            "capabilities": {
                "primary": ["data transformation"],
            },
        }),
        canonical_prompt: "# SX9-PROMPT v4.0\n...",
        ready_for_factory: true,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// LIGHTNING QA
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct QARequest {
    code: String,
    language: String,
}

#[derive(Serialize)]
struct QAResponse {
    grade: String,
    score: f64,
    passed: bool,
    dimensions: QADimensions,
    anti_patterns: Vec<AntiPattern>,
    refactor_directives: Vec<RefactorDirective>,
}

#[derive(Serialize)]
struct QADimensions {
    structure: f64,
    complexity: f64,
    patterns: f64,
    security: f64,
}

#[derive(Serialize)]
struct AntiPattern {
    pattern_type: String,
    severity: String,
    location: String,
    remediation: String,
}

#[derive(Serialize)]
struct RefactorDirective {
    directive_type: String,
    priority: String,
    action: String,
}

async fn analyze_qa(
    State(state): State<Arc<AppState>>,
    Json(request): Json<QARequest>,
) -> Json<QAResponse> {
    // RFC-9121 Lightning QA pipeline
    
    Json(QAResponse {
        grade: "B".to_string(),
        score: 78.5,
        passed: false,
        dimensions: QADimensions {
            structure: 85.0,
            complexity: 72.0,
            patterns: 80.0,
            security: 77.0,
        },
        anti_patterns: vec![
            AntiPattern {
                pattern_type: "UnboundedChannel".to_string(),
                severity: "medium".to_string(),
                location: "src/lib.rs:45".to_string(),
                remediation: "Use bounded channel with explicit capacity".to_string(),
            }
        ],
        refactor_directives: vec![
            RefactorDirective {
                directive_type: "BoundChannel".to_string(),
                priority: "medium".to_string(),
                action: "Replace unbounded_channel() with channel(100)".to_string(),
            }
        ],
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// PATTERN AUTHORING
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct PatternCreateRequest {
    name: String,
    description: String,
    constraints: serde_json::Value,
    interview_template: serde_json::Value,
}

#[derive(Serialize)]
struct PatternCreateResponse {
    pattern_id: String,
    name: String,
    status: String,
}

async fn create_pattern(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(request): Json<PatternCreateRequest>,
) -> Json<PatternCreateResponse> {
    let org_id = headers.get("X-Org-Id").map(|h| h.to_str().unwrap_or("unknown"));
    
    // Store pattern in org's private registry
    
    Json(PatternCreateResponse {
        pattern_id: format!("{}:{}", org_id.unwrap_or("unknown"), request.name),
        name: request.name,
        status: "active".to_string(),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    
    let state = Arc::new(AppState {});
    
    let app = Router::new()
        .route("/prompt", post(generate_prompt))
        .route("/qa/analyze", post(analyze_qa))
        .route("/patterns", post(create_pattern))
        .route("/health", get(|| async { Json(serde_json::json!({"status": "healthy"})) }))
        .with_state(state);
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    tracing::info!("Forge API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## 5. Terraform Configuration

### 5.1 Project Structure

```
terraform/
├── environments/
│   ├── demo/
│   │   └── main.tf
│   └── prod/
│       └── main.tf
├── modules/
│   ├── cloudflare/
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   └── gcp/
│       ├── main.tf
│       ├── variables.tf
│       └── outputs.tf
└── main.tf
```

### 5.2 GCP Module

```hcl
# terraform/modules/gcp/main.tf

variable "project_id" {
  type = string
}

variable "region" {
  type    = string
  default = "us-central1"
}

variable "environment" {
  type = string
}

# ═══════════════════════════════════════════════════════════════════════════════
# CLOUD RUN - CTAS API
# ═══════════════════════════════════════════════════════════════════════════════

resource "google_cloud_run_v2_service" "ctas_api" {
  name     = "ctas-api-${var.environment}"
  location = var.region
  
  template {
    containers {
      image = "gcr.io/${var.project_id}/ctas-api:latest"
      
      resources {
        limits = {
          cpu    = "1"
          memory = "512Mi"
        }
      }
      
      env {
        name  = "RUST_LOG"
        value = "info"
      }
      
      env {
        name  = "ENVIRONMENT"
        value = var.environment
      }
    }
    
    scaling {
      min_instance_count = 0  # Scale to zero (free tier friendly)
      max_instance_count = 10
    }
  }
  
  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }
}

# Allow unauthenticated (gateway handles auth)
resource "google_cloud_run_v2_service_iam_member" "ctas_api_public" {
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.ctas_api.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# ═══════════════════════════════════════════════════════════════════════════════
# CLOUD RUN - FORGE API
# ═══════════════════════════════════════════════════════════════════════════════

resource "google_cloud_run_v2_service" "forge_api" {
  name     = "forge-api-${var.environment}"
  location = var.region
  
  template {
    containers {
      image = "gcr.io/${var.project_id}/forge-api:latest"
      
      resources {
        limits = {
          cpu    = "2"      # Forge needs more compute for QA
          memory = "1Gi"
        }
      }
      
      env {
        name  = "RUST_LOG"
        value = "info"
      }
    }
    
    scaling {
      min_instance_count = 0
      max_instance_count = 5
    }
  }
}

resource "google_cloud_run_v2_service_iam_member" "forge_api_public" {
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.forge_api.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# ═══════════════════════════════════════════════════════════════════════════════
# FIRESTORE (User data, org settings)
# ═══════════════════════════════════════════════════════════════════════════════

resource "google_firestore_database" "sx9" {
  project     = var.project_id
  name        = "(default)"
  location_id = var.region
  type        = "FIRESTORE_NATIVE"
}

# ═══════════════════════════════════════════════════════════════════════════════
# SECRET MANAGER
# ═══════════════════════════════════════════════════════════════════════════════

resource "google_secret_manager_secret" "api_keys" {
  secret_id = "api-keys-${var.environment}"
  
  replication {
    auto {}
  }
}

# ═══════════════════════════════════════════════════════════════════════════════
# OUTPUTS
# ═══════════════════════════════════════════════════════════════════════════════

output "ctas_api_url" {
  value = google_cloud_run_v2_service.ctas_api.uri
}

output "forge_api_url" {
  value = google_cloud_run_v2_service.forge_api.uri
}
```

### 5.3 Cloudflare Module

```hcl
# terraform/modules/cloudflare/main.tf

variable "account_id" {
  type = string
}

variable "zone_id" {
  type = string
}

variable "environment" {
  type = string
}

variable "ctas_api_url" {
  type = string
}

variable "forge_api_url" {
  type = string
}

# ═══════════════════════════════════════════════════════════════════════════════
# KV NAMESPACES
# ═══════════════════════════════════════════════════════════════════════════════

resource "cloudflare_workers_kv_namespace" "sessions" {
  account_id = var.account_id
  title      = "sx9-sessions-${var.environment}"
}

resource "cloudflare_workers_kv_namespace" "patterns" {
  account_id = var.account_id
  title      = "sx9-patterns-${var.environment}"
}

resource "cloudflare_workers_kv_namespace" "features" {
  account_id = var.account_id
  title      = "sx9-features-${var.environment}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# R2 BUCKET
# ═══════════════════════════════════════════════════════════════════════════════

resource "cloudflare_r2_bucket" "artifacts" {
  account_id = var.account_id
  name       = "sx9-artifacts-${var.environment}"
  location   = "ENAM"  # Eastern North America
}

# ═══════════════════════════════════════════════════════════════════════════════
# WORKER
# ═══════════════════════════════════════════════════════════════════════════════

resource "cloudflare_worker_script" "gateway" {
  account_id = var.account_id
  name       = "sx9-gateway-${var.environment}"
  content    = file("${path.module}/workers/gateway/dist/index.js")
  module     = true
  
  kv_namespace_binding {
    name         = "SESSIONS"
    namespace_id = cloudflare_workers_kv_namespace.sessions.id
  }
  
  kv_namespace_binding {
    name         = "PATTERNS"
    namespace_id = cloudflare_workers_kv_namespace.patterns.id
  }
  
  kv_namespace_binding {
    name         = "FEATURES"
    namespace_id = cloudflare_workers_kv_namespace.features.id
  }
  
  r2_bucket_binding {
    name        = "ARTIFACTS"
    bucket_name = cloudflare_r2_bucket.artifacts.name
  }
  
  plain_text_binding {
    name = "CTAS_API"
    text = var.ctas_api_url
  }
  
  plain_text_binding {
    name = "FORGE_API"
    text = var.forge_api_url
  }
}

# ═══════════════════════════════════════════════════════════════════════════════
# DNS & ROUTES
# ═══════════════════════════════════════════════════════════════════════════════

resource "cloudflare_worker_route" "api" {
  zone_id     = var.zone_id
  pattern     = var.environment == "prod" ? "api.sx9.dev/*" : "api-${var.environment}.sx9.dev/*"
  script_name = cloudflare_worker_script.gateway.name
}

# ═══════════════════════════════════════════════════════════════════════════════
# OUTPUTS
# ═══════════════════════════════════════════════════════════════════════════════

output "gateway_url" {
  value = var.environment == "prod" ? "https://api.sx9.dev" : "https://api-${var.environment}.sx9.dev"
}

output "sessions_kv_id" {
  value = cloudflare_workers_kv_namespace.sessions.id
}

output "patterns_kv_id" {
  value = cloudflare_workers_kv_namespace.patterns.id
}
```

### 5.4 Demo Environment

```hcl
# terraform/environments/demo/main.tf

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

provider "google" {
  project = var.gcp_project_id
  region  = "us-central1"
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# ═══════════════════════════════════════════════════════════════════════════════
# GCP RESOURCES
# ═══════════════════════════════════════════════════════════════════════════════

module "gcp" {
  source = "../../modules/gcp"
  
  project_id  = var.gcp_project_id
  region      = "us-central1"
  environment = "demo"
}

# ═══════════════════════════════════════════════════════════════════════════════
# CLOUDFLARE RESOURCES
# ═══════════════════════════════════════════════════════════════════════════════

module "cloudflare" {
  source = "../../modules/cloudflare"
  
  account_id   = var.cloudflare_account_id
  zone_id      = var.cloudflare_zone_id
  environment  = "demo"
  ctas_api_url = module.gcp.ctas_api_url
  forge_api_url = module.gcp.forge_api_url
}

# ═══════════════════════════════════════════════════════════════════════════════
# OUTPUTS
# ═══════════════════════════════════════════════════════════════════════════════

output "api_url" {
  value = module.cloudflare.gateway_url
}

output "ctas_api_url" {
  value = module.gcp.ctas_api_url
}

output "forge_api_url" {
  value = module.gcp.forge_api_url
}
```

---

## 6. Cost Analysis

### 6.1 Free Tier Limits

| Service | Free Tier | Demo Usage Estimate | Headroom |
|---------|-----------|---------------------|----------|
| **Cloudflare Workers** | 100k req/day | ~10k req/day | 90% |
| **Cloudflare KV** | 100k reads/day, 1k writes/day | ~5k reads, ~100 writes | 95% |
| **Cloudflare R2** | 10GB storage, 10M Class A, 10M Class B | ~1GB, ~100k ops | 90% |
| **GCP Cloud Run** | 2M req/month, 360k vCPU-sec | ~200k req, ~50k vCPU-sec | 85% |
| **GCP Firestore** | 1GB storage, 50k reads/day | ~100MB, ~5k reads | 90% |

### 6.2 When You'll Hit Limits

| Milestone | Expected Usage | Action |
|-----------|----------------|--------|
| **Demo (1-10 users)** | Well under free tier | No cost |
| **Pilot (10-50 users)** | May hit Cloud Run limits | ~$20/month |
| **Early customers (50-200 users)** | KV writes may spike | ~$50/month |
| **Growth (200+ users)** | Need reserved capacity | ~$200/month |

---

## 7. Local-Cloud Sync

### 7.1 Gold Disk to R2 Sync

```bash
#!/bin/bash
# scripts/sync-gold-to-r2.sh
# Sync local gold disk artifacts to Cloudflare R2

set -euo pipefail

BUCKET="sx9-artifacts-demo"
GOLD_DIR="/path/to/gold-disk"

# Sync birth certificates
rclone sync "${GOLD_DIR}/birth-certificates/" "r2:${BUCKET}/gold/birth-certificates/"

# Sync QA reports
rclone sync "${GOLD_DIR}/qa-reports/" "r2:${BUCKET}/gold/qa-reports/"

# Sync blessed crate archives
rclone sync "${GOLD_DIR}/crates/" "r2:${BUCKET}/gold/crates/"

# Update manifest
rclone copy "${GOLD_DIR}/gold-disk-manifest.toml" "r2:${BUCKET}/gold/"

echo "✅ Gold disk synced to R2"
```

### 7.2 Pattern Registry Sync

```bash
#!/bin/bash
# scripts/sync-patterns-to-kv.sh
# Sync local pattern registry to Cloudflare KV

set -euo pipefail

KV_NAMESPACE_ID="xxx"

# Read local pattern registry
for pattern_file in /path/to/patterns/*.toml; do
    name=$(basename "$pattern_file" .toml)
    content=$(cat "$pattern_file" | base64)
    
    # Upload to KV
    curl -X PUT "https://api.cloudflare.com/client/v4/accounts/${CF_ACCOUNT_ID}/storage/kv/namespaces/${KV_NAMESPACE_ID}/values/pattern:${name}" \
        -H "Authorization: Bearer ${CF_API_TOKEN}" \
        -H "Content-Type: text/plain" \
        --data "${content}"
    
    echo "Synced pattern: ${name}"
done

echo "✅ Patterns synced to KV"
```

---

## 8. Demo Flow

### 8.1 CTAS Demo Script

```bash
# 1. Show threat analysis
curl -X POST https://api-demo.sx9.dev/api/v1/analyze/threat \
  -H "Authorization: Bearer $DEMO_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"indicators": ["192.168.1.1", "malware.exe", "c2.evil.com"]}'

# 2. Show pattern matching
curl -X POST https://api-demo.sx9.dev/api/v1/patterns/match \
  -H "Authorization: Bearer $DEMO_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"input": "Vehicle approach with modified trunk, cellular trigger"}'

# 3. Show adversary profile
curl https://api-demo.sx9.dev/api/v1/adversary/apt-29 \
  -H "Authorization: Bearer $DEMO_TOKEN"
```

### 8.2 Forge Demo Script (For upgraded customers)

```bash
# 1. Show tier check
curl https://api-demo.sx9.dev/api/v1/tier \
  -H "Authorization: Bearer $FORGE_TOKEN"
# Returns: {"tier": "ctas_forge", "features": {"forge": {"prompt_generation": true, ...}}}

# 2. Show prompt generation
curl -X POST https://api-demo.sx9.dev/api/v1/forge/prompt \
  -H "Authorization: Bearer $FORGE_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"intent": "I need an adapter that converts STIX 2.1 to our internal threat format"}'

# 3. Show Lightning QA
curl -X POST https://api-demo.sx9.dev/api/v1/forge/qa/analyze \
  -H "Authorization: Bearer $FORGE_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"code": "fn main() { todo!() }", "language": "rust"}'
# Returns: {"grade": "F", "anti_patterns": [{"type": "TodoMacro", ...}]}
```

### 8.3 Tier Gate Demo

```bash
# Try to access Forge with CTAS-only token
curl -X POST https://api-demo.sx9.dev/api/v1/forge/prompt \
  -H "Authorization: Bearer $CTAS_ONLY_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"intent": "test"}'

# Returns 403:
# {
#   "error": "Forge license required",
#   "upgrade_url": "https://sx9.dev/upgrade",
#   "message": "This feature requires CTAS + Forge tier. Contact sales for upgrade."
# }
```

---

## 9. Implementation Checklist

### Phase 1: GCP Foundation (Week 1)
- [ ] Create GCP project
- [ ] Set up Cloud Run
- [ ] Deploy CTAS API stub
- [ ] Deploy Forge API stub
- [ ] Configure Firestore

### Phase 2: Cloudflare Edge (Week 2)
- [ ] Set up Cloudflare account
- [ ] Create KV namespaces
- [ ] Create R2 bucket
- [ ] Deploy gateway Worker
- [ ] Configure DNS routes

### Phase 3: Integration (Week 3)
- [ ] Connect Worker to Cloud Run
- [ ] Implement auth flow
- [ ] Implement tier gating
- [ ] Test end-to-end

### Phase 4: Sync & Gold Disk (Week 4)
- [ ] R2 sync scripts
- [ ] KV pattern sync
- [ ] Gold disk backup to R2
- [ ] Restore-from-cloud capability

### Phase 5: Demo Polish (Week 5)
- [ ] Demo tokens & accounts
- [ ] Demo scripts
- [ ] Demo data (sanitized)
- [ ] Runbook for live demos

---

## 10. References

- RFC-9120: Prompt Forge v4
- RFC-9121: Lightning QA Engine
- RFC-9122: Git Workflow
- RFC-9123: Gold Disk Architecture
- Cloudflare Workers Documentation
- GCP Cloud Run Documentation

---

*End of RFC-9124*
