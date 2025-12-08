# Axum Performance Optimization Plan
## Aligning CDN and Gateway Services for Maximum Throughput

**Date:** December 2025  
**Status:** Optimization Plan  
**Target:** 10x throughput improvement, <1ms p99 latency

---

## Current State Analysis

### Issues Identified

1. **Basic Server Setup**
   - Using `axum::serve()` without optimizations
   - No connection pooling
   - No keep-alive configuration
   - No compression
   - No HTTP/2 support

2. **No Performance Tuning**
   - Default buffer sizes
   - No worker thread configuration
   - No timeout settings
   - No rate limiting

3. **Database Connections**
   - No connection pooling
   - Synchronous database calls
   - No query caching

4. **WebSocket**
   - Basic implementation
   - No backpressure handling
   - No connection limits

---

## Optimization Strategy

### 1. Axum Server Optimizations

#### 1.1 Enhanced Server Configuration

```rust
use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
    cors::CorsLayer,
};
use std::time::Duration;

pub fn create_optimized_router() -> Router {
    Router::new()
        // Routes...
        .layer(
            ServiceBuilder::new()
                // Compression (gzip, brotli)
                .layer(CompressionLayer::new())
                
                // Request body size limit (prevent DoS)
                .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB
                
                // Timeout (prevent hanging requests)
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                
                // Tracing (for observability)
                .layer(TraceLayer::new_for_http())
                
                // CORS
                .layer(CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
                    .max_age(Duration::from_secs(3600)) // Cache preflight
                )
        )
}
```

#### 1.2 Optimized Server Startup

```rust
use axum::serve;
use hyper::server::conn::Http;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure Tokio runtime for maximum performance
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get()) // Use all CPU cores
        .thread_name("axum-worker")
        .thread_stack_size(3 * 1024 * 1024) // 3MB stack
        .enable_all()
        .build()?;
    
    let app = create_optimized_router();
    
    // Bind with SO_REUSEPORT for load balancing
    let listener = TcpListener::bind("0.0.0.0:18100").await?;
    
    // Enable TCP optimizations
    let tcp = listener.local_addr()?;
    let listener = TcpListener::bind(tcp).await?;
    
    // Use hyper with HTTP/2 support
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal());
    
    info!("🚀 Optimized server listening on {}", tcp);
    server.await?;
    
    Ok(())
}
```

---

### 2. Connection Pooling & Keep-Alive

#### 2.1 Database Connection Pools

```rust
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(100) // Max connections per pool
        .min_connections(10)  // Keep minimum connections warm
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .test_before_acquire(true) // Test connections before use
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}

// SurrealDB connection pool
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub async fn create_surreal_pool() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("ws://localhost:18019").await?;
    db.use_ns("sx9").use_db("glaf").await?;
    db
}
```

#### 2.2 HTTP Client Connection Pooling

```rust
use reqwest::Client;

pub fn create_http_client() -> Client {
    Client::builder()
        .pool_max_idle_per_host(10) // Keep connections alive
        .pool_idle_timeout(Duration::from_secs(90))
        .timeout(Duration::from_secs(30))
        .tcp_keepalive(Duration::from_secs(60))
        .tcp_nodelay(true) // Disable Nagle's algorithm
        .http2_prior_knowledge() // Use HTTP/2
        .build()
        .expect("Failed to create HTTP client")
}
```

---

### 3. Caching Strategy

#### 3.1 Response Caching

```rust
use tower_http::cache::CacheLayer;
use tower_http::cache_control::CacheControlLayer;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, HttpCacheOptions};
use moka::future::Cache;

// In-memory cache for hot data
let cache = Cache::builder()
    .max_capacity(10_000) // 10k entries
    .time_to_live(Duration::from_secs(300)) // 5 minutes
    .time_to_idle(Duration::from_secs(60)) // 1 minute idle
    .build();

// Add to router
.layer(CacheControlLayer::new())
.layer(CacheLayer::new(cache))
```

#### 3.2 Query Result Caching

```rust
use dashmap::DashMap;
use std::sync::Arc;

pub struct QueryCache {
    cache: Arc<DashMap<String, (serde_json::Value, Instant)>>,
    ttl: Duration,
}

impl QueryCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            ttl,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.cache.get(key).and_then(|entry| {
            if entry.1.elapsed() < self.ttl {
                Some(entry.0.clone())
            } else {
                self.cache.remove(key);
                None
            }
        })
    }
    
    pub fn set(&self, key: String, value: serde_json::Value) {
        self.cache.insert(key, (value, Instant::now()));
    }
}
```

---

### 4. Rate Limiting

#### 4.1 Per-IP Rate Limiting

```rust
use tower_http::limit::RateLimitLayer;
use tower::limit::RateLimit;
use std::num::NonZeroU32;

// Rate limit: 100 requests per minute per IP
.layer(RateLimitLayer::new(
    NonZeroU32::new(100).unwrap(),
    Duration::from_secs(60)
))
```

#### 4.2 Token Bucket Rate Limiting

```rust
use governor::{Quota, RateLimiter};
use nonzero::NonZeroU32;
use std::sync::Arc;

pub struct RateLimiterMiddleware {
    limiter: Arc<RateLimiter<String>>,
}

impl RateLimiterMiddleware {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
        let limiter = Arc::new(RateLimiter::keyed(quota));
        
        Self { limiter }
    }
}
```

---

### 5. WebSocket Optimizations

#### 5.1 WebSocket Connection Limits

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct WebSocketManager {
    active_connections: Arc<AtomicUsize>,
    max_connections: usize,
}

impl WebSocketManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            active_connections: Arc::new(AtomicUsize::new(0)),
            max_connections,
        }
    }
    
    pub fn try_acquire(&self) -> Result<(), String> {
        let current = self.active_connections.fetch_add(1, Ordering::SeqCst);
        if current >= self.max_connections {
            self.active_connections.fetch_sub(1, Ordering::SeqCst);
            Err("Connection limit reached".to_string())
        } else {
            Ok(())
        }
    }
    
    pub fn release(&self) {
        self.active_connections.fetch_sub(1, Ordering::SeqCst);
    }
}
```

#### 5.2 WebSocket Backpressure

```rust
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

async fn handle_websocket_with_backpressure(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    
    // Channel for backpressure
    let (tx, mut rx) = mpsc::channel::<Message>(1000);
    
    // Spawn sender task
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });
    
    // Process incoming messages
    while let Some(msg) = receiver.next().await {
        // Handle message...
        // Send via channel (will backpressure if full)
        if tx.send(response).await.is_err() {
            break;
        }
    }
}
```

---

### 6. HTTP/2 & HTTP/3 Support

#### 6.1 HTTP/2 Configuration

```rust
use hyper::server::conn::Http;
use hyper_util::rt::TokioExecutor;

let http = Http::new()
    .http2_only(true) // Force HTTP/2
    .http2_keep_alive_interval(Duration::from_secs(30))
    .http2_keep_alive_timeout(Duration::from_secs(10))
    .http2_keep_alive_while_idle(true)
    .http2_max_concurrent_streams(1000)
    .http2_initial_stream_window_size(1_000_000) // 1MB
    .http2_initial_connection_window_size(10_000_000); // 10MB
```

#### 6.2 HTTP/3 Support (Future)

```rust
// HTTP/3 requires quinn
use quinn::ServerConfig;
use quinn::Endpoint;

// HTTP/3 provides:
// - Multiplexing without head-of-line blocking
// - Better performance on high-latency networks
// - Built-in encryption (TLS 1.3)
```

---

### 7. Buffer & Memory Optimizations

#### 7.1 Tuned Buffer Sizes

```rust
use tokio::net::TcpStream;

// Set TCP buffer sizes
let stream = TcpStream::connect(addr).await?;
stream.set_send_buffer_size(1_000_000)?; // 1MB send buffer
stream.set_recv_buffer_size(1_000_000)?; // 1MB recv buffer
```

#### 7.2 Memory Pool for Allocations

```rust
use bumpalo::Bump;

// Use bump allocator for short-lived allocations
let bump = Bump::new();
let data = bump.alloc(data);
// Automatically freed when bump goes out of scope
```

---

### 8. Load Balancing & Horizontal Scaling

#### 8.1 SO_REUSEPORT for Load Balancing

```rust
use socket2::{Socket, Domain, Type, Protocol};

fn create_reusable_listener(addr: SocketAddr) -> TcpListener {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_port(true)?; // Enable SO_REUSEPORT
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?; // Backlog
    TcpListener::from_std(socket.into())?
}
```

#### 8.2 Multiple Server Instances

```bash
# Run multiple instances on same port (SO_REUSEPORT)
for i in {1..4}; do
    RUST_LOG=info ./sx9-gateway-primary --port 18100 &
done
```

---

### 9. Monitoring & Observability

#### 9.1 Metrics Collection

```rust
use prometheus::{Counter, Histogram, Registry};

lazy_static! {
    static ref REQUEST_COUNT: Counter = Counter::new(
        "http_requests_total",
        "Total HTTP requests"
    ).unwrap();
    
    static ref REQUEST_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("http_request_duration_seconds", "Request duration")
            .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0])
    ).unwrap();
}

// In handler
let timer = REQUEST_DURATION.start_timer();
REQUEST_COUNT.inc();
// ... handle request ...
drop(timer);
```

#### 9.2 Performance Tracing

```rust
use tracing::{info_span, Instrument};

async fn handle_request() {
    let span = info_span!("handle_request");
    async move {
        // Request handling
    }
    .instrument(span)
    .await
}
```

---

## Implementation Plan

### Phase 1: Core Optimizations (Week 1)

1. **Add Compression Layer**
   - Gzip/Brotli compression
   - Target: 50% size reduction

2. **Connection Pooling**
   - Database connection pools
   - HTTP client pools
   - Target: 10x connection reuse

3. **Response Caching**
   - In-memory cache for hot data
   - Target: 80% cache hit rate

### Phase 2: Advanced Optimizations (Week 2)

4. **HTTP/2 Support**
   - Enable HTTP/2
   - Multiplexing
   - Target: 2x throughput

5. **Rate Limiting**
   - Per-IP rate limits
   - Token bucket algorithm
   - Target: Prevent DoS

6. **WebSocket Optimizations**
   - Connection limits
   - Backpressure handling
   - Target: 10k concurrent connections

### Phase 3: Scaling (Week 3)

7. **Load Balancing**
   - SO_REUSEPORT
   - Multiple instances
   - Target: Horizontal scaling

8. **Monitoring**
   - Prometheus metrics
   - Performance tracing
   - Target: Full observability

---

## Expected Performance Gains

| Optimization | Throughput Gain | Latency Reduction |
|--------------|----------------|-------------------|
| Compression | 2x (bandwidth) | - |
| Connection Pooling | 5x | 50% |
| Response Caching | 10x (cache hits) | 90% |
| HTTP/2 | 2x | 30% |
| Rate Limiting | - | - (DoS protection) |
| WebSocket Opts | 3x | 40% |
| Load Balancing | 4x (4 instances) | - |
| **Total** | **~100x** | **~70%** |

---

## Unified Configuration

### Shared Axum Configuration

```rust
// crates/sx9-gateway-primary/src/config.rs

pub struct ServerConfig {
    pub port: u16,
    pub worker_threads: usize,
    pub max_connections: usize,
    pub request_timeout: Duration,
    pub body_size_limit: usize,
    pub enable_compression: bool,
    pub enable_http2: bool,
    pub cache_ttl: Duration,
    pub rate_limit_per_minute: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 18100,
            worker_threads: num_cpus::get(),
            max_connections: 10_000,
            request_timeout: Duration::from_secs(30),
            body_size_limit: 10 * 1024 * 1024, // 10MB
            enable_compression: true,
            enable_http2: true,
            cache_ttl: Duration::from_secs(300),
            rate_limit_per_minute: 1000,
        }
    }
}
```

---

## Summary

**Optimizations:**
1. ✅ Compression (gzip/brotli)
2. ✅ Connection pooling (DB + HTTP)
3. ✅ Response caching
4. ✅ HTTP/2 support
5. ✅ Rate limiting
6. ✅ WebSocket optimizations
7. ✅ Load balancing (SO_REUSEPORT)
8. ✅ Monitoring & metrics

**Expected Results:**
- **Throughput:** 100x improvement
- **Latency:** 70% reduction
- **Concurrent Connections:** 10k+ per instance
- **Cache Hit Rate:** 80%+

---

**Status:** Ready for implementation. Start with Phase 1 optimizations.

