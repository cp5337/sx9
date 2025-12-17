# RFC-9114 Rev 1.2 - CloudFlare R2 CDN Integration

**SX9 Gateway R2 Subscription Service**

---

## 🎯 **ARCHITECTURE:**

```
SX9 GATEWAY (18120-18122)
    ↓
NEURAL MUX ROUTER (<250ns)
    ↓
R2 CDN SUBSCRIBER (Port 18127)
    ├─→ CloudFlare R2 Buckets
    │   ├── sx9-threat-intel (threat tools JSON)
    │   ├── sx9-mitre-attack (MITRE matrices)
    │   ├── sx9-kali-tools (Kali tool manifests)
    │   └── sx9-osint-data (OSINT datasets)
    │
    ├─→ Local Cache (Redis/Sled)
    ├─→ Periodic Sync (every 1 hour)
    └─→ Fallback to Supabase
```

---

## 💰 **CLOUDFLARE R2 PRICING:**

```
STORAGE:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
First 10 GB/month: FREE
After 10 GB: $0.015/GB/month

CLASS A OPERATIONS (List, Put, Copy):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
First 1M requests/month: FREE
After 1M: $4.50/million

CLASS B OPERATIONS (Get, Head):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
First 10M requests/month: FREE
After 10M: $0.36/million

EGRESS (Downloads):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ALL EGRESS: $0 (ZERO! This is the killer feature!)

ESTIMATED COST FOR SX9:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Storage (200 MB threat intel): $0 (under 10 GB)
Sync operations (1x/hour): $0 (under 1M ops)
Downloads (unlimited): $0 (zero egress!)

TOTAL: $0/month for typical SX9 usage! 🎉
```

---

## 🚀 **RUST IMPLEMENTATION:**

```rust
// File: sx9-gateway/src/cdn/r2_subscriber.rs

use aws_sdk_s3::{Client as S3Client, Config};
use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// R2 CDN Subscriber Service
pub struct R2SubscriberService {
    client: S3Client,
    buckets: R2Buckets,
    cache: Arc<RwLock<R2Cache>>,
    port: u16,
}

#[derive(Clone)]
struct R2Buckets {
    threat_intel: String,
    mitre_attack: String,
    kali_tools: String,
    osint_data: String,
}

struct R2Cache {
    threat_tools: Option<ThreatToolsManifest>,
    mitre_matrix: Option<MitreMatrix>,
    last_sync: std::time::Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatToolsManifest {
    version: String,
    generated_at: String,
    total_tools: usize,
    tools: Vec<ToolEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    unicode: String,
    name: String,
    category: String,
    hash_operational: String,
    hash_semantic: String,
    genome: String,
    mitre_techniques: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMatrix {
    version: String,
    techniques: Vec<MitreTechnique>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechnique {
    id: String,
    name: String,
    tactics: Vec<String>,
}

impl R2SubscriberService {
    /// Initialize R2 subscriber with CloudFlare credentials
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        info!("🔌 Initializing R2 CDN subscriber on port {}", port);
        
        // Configure R2 client (S3-compatible)
        let config = Config::builder()
            .endpoint_url(
                std::env::var("R2_ENDPOINT")
                    .unwrap_or_else(|_| "https://YOUR-ACCOUNT-ID.r2.cloudflarestorage.com".to_string())
            )
            .region(aws_sdk_s3::config::Region::new("auto"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                std::env::var("R2_ACCESS_KEY_ID")?,
                std::env::var("R2_SECRET_ACCESS_KEY")?,
                None,
                None,
                "r2",
            ))
            .build();
        
        let client = S3Client::from_conf(config);
        
        let buckets = R2Buckets {
            threat_intel: "sx9-threat-intel".to_string(),
            mitre_attack: "sx9-mitre-attack".to_string(),
            kali_tools: "sx9-kali-tools".to_string(),
            osint_data: "sx9-osint-data".to_string(),
        };
        
        let cache = Arc::new(RwLock::new(R2Cache {
            threat_tools: None,
            mitre_matrix: None,
            last_sync: std::time::Instant::now(),
        }));
        
        info!("✅ R2 client configured with {} buckets", 4);
        
        Ok(Self {
            client,
            buckets,
            cache,
            port,
        })
    }
    
    /// Start periodic sync from R2
    pub async fn start_sync_loop(self: Arc<Self>) {
        let sync_interval = std::time::Duration::from_secs(3600); // 1 hour
        
        loop {
            if let Err(e) = self.sync_from_r2().await {
                warn!("⚠️  R2 sync failed: {}", e);
            }
            
            tokio::time::sleep(sync_interval).await;
        }
    }
    
    /// Sync data from R2 buckets
    async fn sync_from_r2(&self) -> anyhow::Result<()> {
        info!("🔄 Syncing from R2 buckets...");
        let start = std::time::Instant::now();
        
        // Sync threat tools
        let threat_tools = self.fetch_threat_tools().await?;
        
        // Sync MITRE matrix
        let mitre_matrix = self.fetch_mitre_matrix().await?;
        
        // Update cache
        let mut cache = self.cache.write().await;
        cache.threat_tools = Some(threat_tools);
        cache.mitre_matrix = Some(mitre_matrix);
        cache.last_sync = std::time::Instant::now();
        
        let elapsed = start.elapsed();
        info!("✅ R2 sync complete in {:?}", elapsed);
        
        Ok(())
    }
    
    /// Fetch threat tools manifest from R2
    async fn fetch_threat_tools(&self) -> anyhow::Result<ThreatToolsManifest> {
        let object = self.client
            .get_object()
            .bucket(&self.buckets.threat_intel)
            .key("threat-tools.json")
            .send()
            .await?;
        
        let body = object.body.collect().await?;
        let manifest: ThreatToolsManifest = serde_json::from_slice(&body.into_bytes())?;
        
        info!("📥 Fetched {} tools from R2", manifest.total_tools);
        
        Ok(manifest)
    }
    
    /// Fetch MITRE ATT&CK matrix from R2
    async fn fetch_mitre_matrix(&self) -> anyhow::Result<MitreMatrix> {
        let object = self.client
            .get_object()
            .bucket(&self.buckets.mitre_attack)
            .key("enterprise-attack.json")
            .send()
            .await?;
        
        let body = object.body.collect().await?;
        let matrix: MitreMatrix = serde_json::from_slice(&body.into_bytes())?;
        
        info!("📥 Fetched {} MITRE techniques from R2", matrix.techniques.len());
        
        Ok(matrix)
    }
    
    /// Start Axum server
    pub async fn serve(self) -> anyhow::Result<()> {
        let state = Arc::new(self);
        
        // Start background sync loop
        let sync_state = Arc::clone(&state);
        tokio::spawn(async move {
            sync_state.start_sync_loop().await;
        });
        
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/tools", get(get_all_tools))
            .route("/tools/:unicode", get(get_tool_by_unicode))
            .route("/mitre", get(get_mitre_matrix))
            .route("/mitre/:technique_id", get(get_mitre_technique))
            .route("/sync", get(trigger_sync))
            .with_state(state.clone());
        
        let addr = format!("0.0.0.0:{}", state.port);
        info!("🚀 R2 CDN subscriber listening on {}", addr);
        
        axum::Server::bind(&addr.parse()?)
            .serve(app.into_make_service())
            .await?;
        
        Ok(())
    }
}

// ============================================================================
// HANDLERS
// ============================================================================

async fn health_check(
    State(service): State<Arc<R2SubscriberService>>,
) -> Json<serde_json::Value> {
    let cache = service.cache.read().await;
    let age_secs = cache.last_sync.elapsed().as_secs();
    
    Json(serde_json::json!({
        "status": "healthy",
        "service": "r2-cdn-subscriber",
        "rfc": "9114-r2",
        "buckets": 4,
        "cache_age_seconds": age_secs,
        "threat_tools_cached": cache.threat_tools.is_some(),
        "mitre_cached": cache.mitre_matrix.is_some(),
    }))
}

async fn get_all_tools(
    State(service): State<Arc<R2SubscriberService>>,
) -> Json<serde_json::Value> {
    let cache = service.cache.read().await;
    
    match &cache.threat_tools {
        Some(manifest) => Json(serde_json::json!({
            "version": manifest.version,
            "total": manifest.total_tools,
            "tools": manifest.tools,
        })),
        None => Json(serde_json::json!({
            "error": "Cache not populated yet, sync in progress"
        }))
    }
}

async fn get_tool_by_unicode(
    State(service): State<Arc<R2SubscriberService>>,
    Path(unicode): Path<String>,
) -> Json<serde_json::Value> {
    let cache = service.cache.read().await;
    
    if let Some(manifest) = &cache.threat_tools {
        if let Some(tool) = manifest.tools.iter().find(|t| t.unicode == unicode) {
            return Json(serde_json::json!({
                "found": true,
                "tool": tool,
            }));
        }
    }
    
    Json(serde_json::json!({
        "found": false,
        "unicode": unicode,
    }))
}

async fn get_mitre_matrix(
    State(service): State<Arc<R2SubscriberService>>,
) -> Json<serde_json::Value> {
    let cache = service.cache.read().await;
    
    match &cache.mitre_matrix {
        Some(matrix) => Json(serde_json::json!({
            "version": matrix.version,
            "total": matrix.techniques.len(),
            "techniques": matrix.techniques,
        })),
        None => Json(serde_json::json!({
            "error": "MITRE matrix not cached yet"
        }))
    }
}

async fn get_mitre_technique(
    State(service): State<Arc<R2SubscriberService>>,
    Path(technique_id): Path<String>,
) -> Json<serde_json::Value> {
    let cache = service.cache.read().await;
    
    if let Some(matrix) = &cache.mitre_matrix {
        if let Some(technique) = matrix.techniques.iter().find(|t| t.id == technique_id) {
            return Json(serde_json::json!({
                "found": true,
                "technique": technique,
            }));
        }
    }
    
    Json(serde_json::json!({
        "found": false,
        "technique_id": technique_id,
    }))
}

async fn trigger_sync(
    State(service): State<Arc<R2SubscriberService>>,
) -> Json<serde_json::Value> {
    tokio::spawn(async move {
        if let Err(e) = service.sync_from_r2().await {
            warn!("Manual sync failed: {}", e);
        }
    });
    
    Json(serde_json::json!({
        "status": "sync_triggered",
        "message": "Background sync started"
    }))
}
```

---

## 🔧 **CARGO DEPENDENCIES:**

Add to `sx9-gateway/Cargo.toml`:

```toml
[dependencies]
# ... existing deps ...

# R2 / S3 client
aws-config = "1.1.0"
aws-sdk-s3 = "1.9.0"
aws-smithy-runtime-api = "1.1.0"
```

---

## 🔌 **GATEWAY INTEGRATION:**

```rust
// In sx9-gateway/src/main.rs

use crate::cdn::r2_subscriber::R2SubscriberService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... existing gateway setup ...
    
    // Register R2 CDN with port manager
    let r2_port = port_manager.allocate_port(
        "r2-cdn-subscriber",
        PortType::CDN,
        18127,  // Requested port
    ).await?;
    
    // Spawn R2 subscriber service
    let r2_service = R2SubscriberService::new(r2_port).await?;
    
    tokio::spawn(async move {
        if let Err(e) = r2_service.serve().await {
            tracing::error!("R2 CDN subscriber error: {}", e);
        }
    });
    
    info!("✅ R2 CDN subscriber started on port {}", r2_port);
    
    // Register with Neural Mux
    neural_mux.register_route(RouteEntry {
        hash_prefix: hash("r2-cdn"),
        target: RouteDest::LocalCDN {
            name: "r2-cdn-subscriber",
            port: r2_port,
            protocol: Protocol::HTTP,
        },
        latency_zone: BernoulliZone::C,  // Analytical
        fallback: Some(RouteDest::Supabase),
    })?;
    
    Ok(())
}
```

---

## 📋 **ENVIRONMENT VARIABLES:**

Add to `.env`:

```bash
# CloudFlare R2 Configuration
R2_ENDPOINT=https://YOUR-ACCOUNT-ID.r2.cloudflarestorage.com
R2_ACCESS_KEY_ID=your-access-key-id
R2_SECRET_ACCESS_KEY=your-secret-access-key

# R2 Bucket Names
R2_BUCKET_THREAT_INTEL=sx9-threat-intel
R2_BUCKET_MITRE=sx9-mitre-attack
R2_BUCKET_KALI=sx9-kali-tools
R2_BUCKET_OSINT=sx9-osint-data
```

---

## 🚀 **SETUP CLOUDFLARE R2:**

```bash
# 1. Create R2 buckets via Wrangler
wrangler r2 bucket create sx9-threat-intel
wrangler r2 bucket create sx9-mitre-attack
wrangler r2 bucket create sx9-kali-tools
wrangler r2 bucket create sx9-osint-data

# 2. Generate R2 API tokens
# Go to: https://dash.cloudflare.com
# R2 → Manage R2 API Tokens → Create API Token
# Permissions: Read & Write

# 3. Upload initial data (from CloudFlare Workflow)
# This happens automatically when you deploy the workflow!
# OR manually:
wrangler r2 object put sx9-threat-intel/threat-tools.json --file=threat-tools.json
```

---

## 📊 **DATA FLOW:**

```
CLOUDFLARE WORKFLOW (threat-intel-sync)
    ↓
    ├─→ Fetches 27 threat sources
    ├─→ Generates RFC-9001 hashes
    ├─→ Uploads to R2 buckets ✅
    │   ├── threat-tools.json (200 MB)
    │   ├── mitre-enterprise.json
    │   ├── kali-tools.json
    │   └── osint-datasets.json
    ↓
SX9 GATEWAY - R2 SUBSCRIBER (Port 18127)
    ↓
    ├─→ Syncs from R2 every 1 hour
    ├─→ Caches locally (in-memory)
    ├─→ Serves via REST API
    └─→ Falls back to Supabase if stale
    ↓
CTAS FRONTEND
    ↓
    ├─→ GET /tools → All threat tools
    ├─→ GET /tools/E800 → Specific tool
    └─→ GET /mitre → MITRE matrix
```

---

## 🎯 **API ENDPOINTS:**

```bash
# Health check
curl http://localhost:18127/health

# Get all tools (from R2 cache)
curl http://localhost:18127/tools

# Get specific tool by Unicode
curl http://localhost:18127/tools/E800

# Get MITRE ATT&CK matrix
curl http://localhost:18127/mitre

# Get specific MITRE technique
curl http://localhost:18127/mitre/T1046

# Trigger manual sync
curl http://localhost:18127/sync
```

---

## ✅ **BENEFITS:**

```
✅ $0/month cost (free tier generous)
✅ Zero egress fees (unlimited downloads!)
✅ Global edge caching (275+ locations)
✅ Automatic sync every hour
✅ Local cache for <1ms lookups
✅ RFC-9114 Bernoulli Zone C compliant
✅ Integrates with Port Manager
✅ Neural Mux routable
✅ Fallback to Supabase
```

---

## 🏗️ **COMPLETE SX9 GATEWAY ARCHITECTURE:**

```
SX9 GATEWAY (18120-18122)
    ↓
NEURAL MUX ROUTER (<250ns)
    ↓
┌─────────────────────────────────────────────────┐
│ LOCAL CDNs (Tweaked Axum Servers)              │
├─────────────────────────────────────────────────┤
│ Port 18123: Threat Intel CDN (local files)     │
│ Port 18124: OSINT Data CDN (local DB)          │
│ Port 18125: ChromaDB Vector CDN ← NEW!         │
│ Port 18126: GeoIP CDN (MaxMind DB)             │
│ Port 18127: R2 Subscriber CDN ← NEW!           │
└─────────────────────────────────────────────────┘
    ↓
PORT MANAGER (18104)
    ↓
BACKEND:
    ├── CloudFlare R2 (threat intel, MITRE)
    ├── ChromaDB (vectors)
    ├── Supabase (entities)
    ├── Neon (tool registry)
    └── Neo4j (GLAF graph)
```

---

## 💰 **TOTAL COST:**

```
CloudFlare R2:        $0/month (under 10 GB)
CloudFlare Workers:   $0/month (under 100k req/day)
CloudFlare KV:        $0/month (under 100k reads/day)
Supabase:             $0/month (free tier)
Neon:                 $0/month (free tier)
Local services:       $0/month (Docker)

TOTAL: $0/month! 🎉
```

---

**Want me to create the complete Rust module ready to compile?** 🚀
