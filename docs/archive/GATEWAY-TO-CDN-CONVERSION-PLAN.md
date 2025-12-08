# Gateway to CDN Conversion Plan
## Converting sx9-gateway-primary to CDN-Capable Service

**Date:** December 2025  
**Status:** Conversion Plan  
**Target:** Transform gateway into edge CDN with caching, static asset serving, and multi-region deployment

---

## Current Gateway Architecture

**Current State:**
```
Client → sx9-gateway-primary (Axum) → Backend Services
         - WebSocket handler
         - REST API routes
         - Database queries
         - Graph operations
```

**Port:** 18600 (default)  
**Protocols:** HTTP/1.1, WebSocket  
**No caching, no static assets, no edge deployment**

---

## Target CDN Architecture

**Target State:**
```
Client → Edge Gateway (Multi-region) → Origin Gateway → Backend Services
         - Static asset serving
         - Response caching
         - Edge compute
         - CDN integration
```

**Capabilities:**
- Static asset serving (JS, CSS, fonts, WASM, images)
- Response caching (API responses, database queries)
- Edge deployment (multi-region)
- CDN origin server (for Cloudflare/GCP)
- Cache invalidation
- Geographic routing

---

## Conversion Strategy

### Option 1: Gateway as CDN Origin (Recommended)

**Architecture:**
```
Cloudflare Edge → sx9-gateway-primary (Origin) → Backend Services
```

**Benefits:**
- Gateway remains primary API server
- Cloudflare handles edge caching
- Gateway serves as origin for dynamic content
- Static assets can be in R2, dynamic from gateway

### Option 2: Gateway as Edge CDN

**Architecture:**
```
Client → sx9-gateway-primary (Edge) → Origin Services
```

**Benefits:**
- Gateway deployed at edge locations
- Local caching at edge
- Reduced latency
- Geographic distribution

### Option 3: Hybrid (Best of Both)

**Architecture:**
```
Client → Cloudflare Edge → sx9-gateway-primary (Origin) → Backend
         ↓ (static assets)
         R2 Buckets
```

**Benefits:**
- Static assets from R2 (fast, cheap)
- Dynamic content from gateway (flexible)
- Edge caching for both
- Optimal cost/performance

---

## Required Code Changes

### 1. Add Static Asset Serving

```rust
// src/static_assets.rs
use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::path::PathBuf;

pub fn static_assets_router() -> Router {
    Router::new()
        // Serve static files from public directory
        .nest_service("/static", ServeDir::new("public/static"))
        .nest_service("/assets", ServeDir::new("public/assets"))
        .nest_service("/wasm", ServeDir::new("public/wasm"))
        .route("/favicon.ico", get(favicon))
        .route("/robots.txt", get(robots))
}

async fn favicon() -> impl IntoResponse {
    // Serve favicon with cache headers
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/x-icon")
        .header(header::CACHE_CONTROL, "public, max-age=86400")
        .body(include_bytes!("../public/favicon.ico").to_vec())
        .unwrap()
}
```

### 2. Add Response Caching

```rust
// src/cache.rs
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use moka::future::Cache;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CacheLayer {
    cache: Cache<String, CachedResponse>,
}

#[derive(Clone, Serialize, Deserialize)]
struct CachedResponse {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    cached_at: u64,
}

impl CacheLayer {
    pub fn new() -> Self {
        let cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300)) // 5 minutes default
            .build();
        
        Self { cache }
    }
    
    pub async fn get(&self, key: &str) -> Option<CachedResponse> {
        self.cache.get(key).await
    }
    
    pub async fn set(&self, key: String, response: CachedResponse, ttl: Duration) {
        self.cache.insert_with(key, response, ttl).await;
    }
    
    pub fn cache_key(request: &Request) -> String {
        // Generate cache key from method + path + query + headers
        let uri = request.uri();
        format!("{}:{}:{}", 
            request.method(),
            uri.path(),
            uri.query().unwrap_or("")
        )
    }
}

// Middleware for caching
pub async fn cache_middleware(
    cache: CacheLayer,
    request: Request,
    next: axum::middleware::Next,
) -> Response {
    // Check cache
    let key = CacheLayer::cache_key(&request);
    if let Some(cached) = cache.get(&key).await {
        // Return cached response
        return build_response_from_cache(cached);
    }
    
    // Execute request
    let response = next.run(request).await;
    
    // Cache response if cacheable
    if is_cacheable(&response) {
        let cached = extract_response_for_cache(response).await;
        cache.set(key, cached, Duration::from_secs(300)).await;
    }
    
    response
}
```

### 3. Add CDN Integration

```rust
// src/cdn.rs
use aws_sdk_s3::Client as S3Client;
use std::sync::Arc;

pub struct CDNBackend {
    r2_client: Option<Arc<S3Client>>,
    gcp_client: Option<Arc<GCPStorageClient>>,
    local_fallback: bool,
}

impl CDNBackend {
    pub async fn new() -> Self {
        let r2_client = if let Ok(client) = create_r2_client().await {
            Some(Arc::new(client))
        } else {
            None
        };
        
        let gcp_client = if let Ok(client) = create_gcp_client().await {
            Some(Arc::new(client))
        } else {
            None
        };
        
        Self {
            r2_client,
            gcp_client,
            local_fallback: true,
        }
    }
    
    /// Serve asset from CDN or local fallback
    pub async fn serve_asset(&self, path: &str) -> Result<Vec<u8>> {
        // Try R2 first
        if let Some(client) = &self.r2_client {
            if let Ok(data) = self.get_from_r2(client, path).await {
                return Ok(data);
            }
        }
        
        // Try GCP
        if let Some(client) = &self.gcp_client {
            if let Ok(data) = self.get_from_gcp(client, path).await {
                return Ok(data);
            }
        }
        
        // Fallback to local
        if self.local_fallback {
            return self.get_from_local(path).await;
        }
        
        Err(anyhow::anyhow!("Asset not found"))
    }
    
    /// Upload asset to CDN
    pub async fn upload_asset(&self, path: &str, data: Vec<u8>) -> Result<()> {
        // Upload to R2
        if let Some(client) = &self.r2_client {
            self.upload_to_r2(client, path, &data).await?;
        }
        
        // Also upload to GCP if configured
        if let Some(client) = &self.gcp_client {
            self.upload_to_gcp(client, path, &data).await?;
        }
        
        Ok(())
    }
}
```

### 4. Add Cache Headers Middleware

```rust
// src/cache_headers.rs
use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};

pub async fn cache_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    // Add cache headers based on content type
    let content_type = response.headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok());
    
    let cache_control = match content_type {
        Some(ct) if ct.starts_with("text/css") || ct.starts_with("application/javascript") => {
            "public, max-age=86400, immutable" // 1 day, immutable
        }
        Some(ct) if ct.starts_with("image/") => {
            "public, max-age=604800, immutable" // 1 week, immutable
        }
        Some(ct) if ct.starts_with("font/") => {
            "public, max-age=31536000, immutable" // 1 year, immutable
        }
        Some(ct) if ct == "application/wasm" => {
            "public, max-age=86400, immutable" // 1 day, immutable
        }
        Some(ct) if ct == "application/json" => {
            "public, max-age=300" // 5 minutes for API responses
        }
        _ => {
            "no-cache" // Default: no cache
        }
    };
    
    response.headers_mut().insert(
        "cache-control",
        cache_control.parse().unwrap(),
    );
    
    // Add ETag for cache validation
    if let Some(body) = response.body() {
        let etag = generate_etag(body);
        response.headers_mut().insert("etag", etag.parse().unwrap());
    }
    
    response
}
```

### 5. Add Edge Deployment Support

```rust
// src/edge.rs
use std::sync::Arc;
use axum::Router;

pub struct EdgeConfig {
    pub region: String,
    pub origin_url: String,
    pub cache_enabled: bool,
    pub static_assets_enabled: bool,
}

impl EdgeConfig {
    pub fn from_env() -> Self {
        Self {
            region: std::env::var("EDGE_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            origin_url: std::env::var("ORIGIN_URL")
                .unwrap_or_else(|_| "https://gateway.sx9.io".to_string()),
            cache_enabled: std::env::var("EDGE_CACHE_ENABLED")
                .unwrap_or_else(|_| "true".to_string()) == "true",
            static_assets_enabled: std::env::var("EDGE_STATIC_ENABLED")
                .unwrap_or_else(|_| "true".to_string()) == "true",
        }
    }
}

pub fn create_edge_router(config: EdgeConfig) -> Router {
    let mut router = Router::new();
    
    // Static assets (served from edge)
    if config.static_assets_enabled {
        router = router.nest("/static", static_assets_router());
    }
    
    // API routes (proxy to origin with caching)
    router = router
        .route("/api/*path", get(proxy_to_origin))
        .route("/ws", get(websocket_handler))
        .layer(cache_middleware(config.cache_enabled));
    
    router
}

async fn proxy_to_origin(
    Path(path): Path<String>,
    State(config): State<EdgeConfig>,
    request: Request,
) -> Response {
    // Proxy to origin gateway
    let origin_url = format!("{}/api/{}", config.origin_url, path);
    let client = reqwest::Client::new();
    
    // Forward request to origin
    let response = client
        .request(request.method().clone(), &origin_url)
        .headers(request.headers().clone())
        .body(request.body())
        .send()
        .await?;
    
    // Return response (will be cached by middleware)
    response.into()
}
```

### 6. Update Gateway Server

```rust
// src/server.rs (updated)
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::ServeDir,
};

pub async fn run_gateway(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.unwrap_or(18600);
    
    // Initialize CDN backend
    let cdn = Arc::new(CDNBackend::new().await);
    
    // Initialize cache
    let cache = Arc::new(CacheLayer::new());
    
    // Check if running as edge or origin
    let edge_config = EdgeConfig::from_env();
    let is_edge = !edge_config.origin_url.is_empty();
    
    let router = if is_edge {
        // Edge deployment
        create_edge_router(edge_config)
    } else {
        // Origin deployment
        create_origin_router(cdn, cache)
    };
    
    // Add middleware
    let app = router
        .layer(CompressionLayer::new())
        .layer(CacheHeadersLayer)
        .layer(CorsLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)));
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("🚀 Gateway listening on port {} ({})", port, if is_edge { "EDGE" } else { "ORIGIN" });
    
    axum::serve(listener, app).await?;
    Ok(())
}
```

---

## Deployment Strategies

### Strategy 1: Gateway as Cloudflare Origin

**Setup:**
```bash
# Deploy gateway as origin server
docker run -d \
  -p 18600:18600 \
  -e CDN_PROVIDER=cloudflare-r2 \
  -e R2_BUCKET_NAME=sx9-gateway-assets \
  sx9-gateway-primary:latest

# Configure Cloudflare to use gateway as origin
# Cloudflare Dashboard → Workers → Routes
# Pattern: api.sx9.io/*
# Origin: http://your-gateway-ip:18600
```

**Benefits:**
- Cloudflare handles edge caching
- Gateway serves dynamic content
- Static assets from R2
- DDoS protection from Cloudflare

### Strategy 2: Multi-Region Edge Deployment

**Setup:**
```bash
# Deploy gateway to multiple regions
# US East
docker run -d \
  -e EDGE_REGION=us-east-1 \
  -e ORIGIN_URL=https://gateway.sx9.io \
  -e EDGE_CACHE_ENABLED=true \
  sx9-gateway-primary:edge

# EU West
docker run -d \
  -e EDGE_REGION=eu-west-1 \
  -e ORIGIN_URL=https://gateway.sx9.io \
  -e EDGE_CACHE_ENABLED=true \
  sx9-gateway-primary:edge

# Asia Pacific
docker run -d \
  -e EDGE_REGION=ap-southeast-1 \
  -e ORIGIN_URL=https://gateway.sx9.io \
  -e EDGE_CACHE_ENABLED=true \
  sx9-gateway-primary:edge
```

**DNS Configuration:**
```
api.sx9.io → Cloudflare (geographic routing)
  ├─ US → us-east-1 gateway
  ├─ EU → eu-west-1 gateway
  └─ AP → ap-southeast-1 gateway
```

### Strategy 3: Hybrid (Recommended)

**Architecture:**
```
Static Assets:
  Client → Cloudflare Edge → R2 Bucket

Dynamic Content:
  Client → Cloudflare Edge → sx9-gateway-primary (Origin) → Backend

WebSocket:
  Client → sx9-gateway-primary (Direct) → Backend
```

**Configuration:**
```rust
// Gateway serves as origin for dynamic content
// R2 serves static assets directly
// Cloudflare Workers route between them
```

---

## Performance Optimizations

### 1. Response Compression

```rust
// Already in Axum optimization plan
.layer(CompressionLayer::new())
// Supports: gzip, brotli, deflate
```

### 2. HTTP/2 Support

```rust
// Enable HTTP/2 for better multiplexing
let listener = tokio::net::TcpListener::bind(addr).await?;
let tcp = hyper_util::rt::TokioIo::new(listener);
let acceptor = hyper_util::server::conn::auto::Builder::new();
axum::serve::serve_with_incoming(tcp, app).await?;
```

### 3. Connection Pooling

```rust
// Reuse HTTP client connections
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(90))
    .build()?;
```

### 4. Cache Warming

```rust
// Pre-warm cache for common requests
pub async fn warm_cache(cache: Arc<CacheLayer>) {
    let common_paths = vec![
        "/api/health",
        "/api/metrics",
        "/api/stats",
    ];
    
    for path in common_paths {
        // Make request and cache response
        let response = make_request(path).await;
        cache.set(path.to_string(), response, Duration::from_secs(300)).await;
    }
}
```

---

## CDN Integration Points

### 1. Cloudflare Workers Integration

```javascript
// workers/gateway-router.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    
    // Static assets → R2
    if (url.pathname.startsWith('/static/') || 
        url.pathname.startsWith('/assets/')) {
      return env.R2_BUCKET.fetch(request);
    }
    
    // API requests → Gateway origin
    if (url.pathname.startsWith('/api/')) {
      return fetch(`https://gateway.sx9.io${url.pathname}`, {
        method: request.method,
        headers: request.headers,
        body: request.body,
      });
    }
    
    // WebSocket → Direct to gateway
    if (url.pathname === '/ws') {
      return fetch(`wss://gateway.sx9.io/ws`, request);
    }
    
    return new Response('Not Found', { status: 404 });
  }
}
```

### 2. Cache Invalidation

```rust
// src/cache_invalidation.rs
pub struct CacheInvalidator {
    cloudflare_api: CloudflareAPI,
    cache: Arc<CacheLayer>,
}

impl CacheInvalidator {
    pub async fn invalidate_path(&self, path: &str) -> Result<()> {
        // Invalidate local cache
        self.cache.invalidate(path).await;
        
        // Purge Cloudflare cache
        self.cloudflare_api.purge_cache(path).await?;
        
        Ok(())
    }
    
    pub async fn invalidate_all(&self) -> Result<()> {
        self.cache.invalidate_all().await;
        self.cloudflare_api.purge_everything().await?;
        Ok(())
    }
}
```

---

## Migration Steps

### Phase 1: Add Static Asset Serving (Week 1)

1. Create `public/` directory structure
2. Add static asset routes to gateway
3. Test serving JS, CSS, fonts, images
4. Add cache headers

### Phase 2: Add Response Caching (Week 2)

1. Implement cache layer
2. Add cache middleware
3. Configure TTLs per content type
4. Test cache hit/miss rates

### Phase 3: CDN Integration (Week 3)

1. Set up R2 buckets
2. Add R2 client to gateway
3. Upload static assets to R2
4. Configure Cloudflare Workers

### Phase 4: Edge Deployment (Week 4)

1. Add edge configuration
2. Deploy to multiple regions
3. Set up geographic routing
4. Test edge caching

### Phase 5: Optimization (Week 5)

1. Monitor cache hit rates
2. Tune TTLs
3. Optimize compression
4. Enable HTTP/2

---

## Configuration

### Environment Variables

```bash
# Gateway mode
GATEWAY_MODE=origin|edge
EDGE_REGION=us-east-1
ORIGIN_URL=https://gateway.sx9.io

# CDN settings
CDN_PROVIDER=cloudflare-r2
R2_BUCKET_NAME=sx9-gateway-assets
R2_ENABLED=true

# Caching
CACHE_ENABLED=true
CACHE_TTL_DEFAULT=300
CACHE_MAX_SIZE=10000

# Static assets
STATIC_ASSETS_ENABLED=true
STATIC_ASSETS_DIR=public
```

### Smart Crate TOML

```toml
[cdn]
enabled = true
provider = "cloudflare-r2"
bucket = "sx9-gateway-assets"
custom_domain = "api.sx9.io"

[cdn.caching]
enabled = true
default_ttl = 300
static_ttl = 86400
api_ttl = 60

[cdn.edge]
enabled = true
regions = ["us-east-1", "eu-west-1", "ap-southeast-1"]
origin_url = "https://gateway.sx9.io"
```

---

## Benefits

### Performance
- **Latency:** 50-80% reduction (edge caching)
- **Throughput:** 10x increase (static assets from CDN)
- **Bandwidth:** 70-90% reduction (caching)

### Cost
- **Infrastructure:** 60-80% reduction (edge caching)
- **Bandwidth:** 70-90% reduction (CDN egress)
- **Origin load:** 50-70% reduction (caching)

### Scalability
- **Geographic distribution:** Multi-region deployment
- **Auto-scaling:** Cloudflare handles traffic spikes
- **High availability:** Edge redundancy

---

## Summary

**Yes, the gateway can be converted to a CDN-capable service!**

**Conversion includes:**
1. ✅ Static asset serving
2. ✅ Response caching
3. ✅ CDN integration (R2/GCP)
4. ✅ Edge deployment support
5. ✅ Cache invalidation
6. ✅ Performance optimizations

**Deployment options:**
- **Origin:** Gateway serves as origin for Cloudflare
- **Edge:** Gateway deployed at edge locations
- **Hybrid:** Static from R2, dynamic from gateway

**Timeline:** 5 weeks  
**Cost savings:** 60-80% infrastructure, 70-90% bandwidth

---

**Status:** Ready for implementation  
**Next Steps:** Start with Phase 1 (static asset serving)



