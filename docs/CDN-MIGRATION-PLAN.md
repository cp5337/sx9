# CDN Migration Plan: Local Axum → Cloudflare R2 / GCP CDN

## Current State

**Local CDN Services (Axum servers on ports 18108-18116):**
- `ctas7-cdn-statistical` (18108) - Statistical analysis CDN
- `ctas7-cdn-monitoring` (18109) - Monitoring CDN
- `ctas7-cdn-threat-intel` (18115) - Threat intelligence CDN
- `ctas7-cdn-threat-reaction` (18201) - Threat reaction CDN
- `ctas7-cdn-geospatial` (18111) - Geospatial CDN
- `ctas7-cdn-data-fabric` (18116) - Data fabric CDN
- `ctas7-smart-cdn-gateway` (18100) - Smart gateway CDN

**Current Architecture:**
```
Client → Axum Server (localhost:18108) → Local Storage/Processing
```

---

## Target State

**Three-Tier CDN Architecture:**

1. **Cloudflare R2** - Public/semi-public assets ($0 egress)
2. **GCP Cloud CDN** - IAM-gated assets ($0 idle LB)
3. **Internal** - Classified/air-gapped (existing infra)

**Target Architecture:**
```
Client → Cloudflare Edge → R2 Bucket / GCP CDN → Origin (if needed)
```

---

## Required Changes by Service

### 1. Statistical CDN (18108) → Cloudflare R2

**Current:** Axum server serving statistical analysis results  
**Target:** Cloudflare R2 bucket + Worker for processing

**Changes Required:**

#### A. Service Code Changes
```rust
// BEFORE: Direct Axum server
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/analyze", post(analyze_data));
    axum::serve(listener, app).await?;
}

// AFTER: Hybrid - Local processing + R2 upload
#[tokio::main]
async fn main() {
    // Keep local processing for real-time
    let app = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/analyze", post(analyze_data))
        .route("/api/upload-to-r2", post(upload_to_r2)); // New endpoint
    
    // Background job: Upload results to R2
    tokio::spawn(async {
        upload_processed_results_to_r2().await;
    });
    
    axum::serve(listener, app).await?;
}
```

#### B. Add R2 Client Integration
```rust
// Add to Cargo.toml
[dependencies]
aws-sdk-s3 = { version = "1.0", features = ["rustls"] } // R2 uses S3-compatible API
tokio = { version = "1.0", features = ["full"] }

// New module: src/r2_client.rs
use aws_sdk_s3::Client as S3Client;
use aws_config::BehaviorVersion;

pub struct R2Client {
    client: S3Client,
    bucket: String,
}

impl R2Client {
    pub async fn new(bucket: String) -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = S3Client::new(&config);
        
        Self { client, bucket }
    }
    
    pub async fn upload(&self, key: &str, data: Vec<u8>) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(data.into())
            .send()
            .await?;
        Ok(())
    }
    
    pub async fn get(&self, key: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        
        let data = response.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }
}
```

#### C. Update Health Endpoint
```rust
// Add R2 connectivity check
.route("/health", get(|| async {
    json!({
        "status": "ok",
        "r2_connected": check_r2_connection().await,
        "r2_bucket": env::var("R2_BUCKET_NAME").unwrap_or_default(),
    })
}))
```

#### D. Environment Variables
```bash
# .env
R2_ACCOUNT_ID=your-cloudflare-account-id
R2_ACCESS_KEY_ID=your-r2-access-key
R2_SECRET_ACCESS_KEY=your-r2-secret-key
R2_BUCKET_NAME=sx9-statistical-cdn
R2_ENDPOINT=https://your-account-id.r2.cloudflarestorage.com
```

---

### 2. Monitoring CDN (18109) → Cloudflare R2

**Current:** Axum server serving monitoring metrics  
**Target:** Cloudflare R2 + Workers for real-time aggregation

**Changes Required:**

#### A. Add Metrics Upload Job
```rust
// src/metrics_uploader.rs
pub struct MetricsUploader {
    r2: R2Client,
    interval: Duration,
}

impl MetricsUploader {
    pub async fn start(&self) {
        let mut interval = tokio::time::interval(self.interval);
        
        loop {
            interval.tick().await;
            
            // Collect metrics
            let metrics = collect_metrics().await;
            
            // Upload to R2 with timestamp key
            let key = format!("metrics/{}.json", Utc::now().timestamp());
            self.r2.upload(&key, serde_json::to_vec(&metrics)?).await?;
        }
    }
}
```

#### B. Cloudflare Worker for Aggregation
```javascript
// workers/monitoring-aggregator.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    
    // Aggregate metrics from R2
    if (url.pathname === '/api/metrics/aggregate') {
      const objects = await env.R2_BUCKET.list({
        prefix: 'metrics/',
        limit: 1000
      });
      
      // Aggregate and return
      return new Response(JSON.stringify(aggregated), {
        headers: { 'Content-Type': 'application/json' }
      });
    }
    
    // Serve cached metrics
    return env.R2_BUCKET.get(url.pathname);
  }
}
```

---

### 3. Threat Intel CDN (18115) → GCP Cloud CDN (IAM-gated)

**Current:** Axum server serving threat intelligence  
**Target:** GCP Cloud Storage + Cloud CDN with IAM authentication

**Changes Required:**

#### A. Add GCP Storage Client
```rust
// Add to Cargo.toml
[dependencies]
google-cloud-storage = "0.10"
google-cloud-auth = "0.10"

// src/gcp_client.rs
use google_cloud_storage::client::Client;
use google_cloud_storage::http::objects::upload::UploadObjectRequest;
use google_cloud_storage::http::objects::Object;

pub struct GCPStorageClient {
    client: Client,
    bucket: String,
}

impl GCPStorageClient {
    pub async fn new(bucket: String) -> Self {
        let client = Client::default().await.unwrap();
        Self { client, bucket }
    }
    
    pub async fn upload(&self, name: &str, data: Vec<u8>) -> Result<()> {
        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    name: name.to_string(),
                    ..Default::default()
                },
                data,
                &Default::default(),
            )
            .await?;
        Ok(())
    }
    
    pub async fn generate_signed_url(&self, name: &str, expires_in: Duration) -> Result<String> {
        // Generate IAM-gated signed URL
        let url = self.client
            .create_signed_url(
                &self.bucket,
                name,
                expires_in,
                &Default::default(),
            )
            .await?;
        Ok(url)
    }
}
```

#### B. Update Service to Generate Signed URLs
```rust
// Instead of serving directly, generate signed URLs
.route("/api/threat-intel/:id", get(async |Path(id): Path<String>| {
    let signed_url = gcp_client.generate_signed_url(
        &format!("threat-intel/{}", id),
        Duration::from_secs(3600), // 1 hour expiry
    ).await?;
    
    json!({
        "url": signed_url,
        "expires_in": 3600,
    })
}))
```

---

### 4. Geospatial CDN (18111) → Cloudflare R2

**Current:** Axum server serving Cesium tiles  
**Target:** Cloudflare R2 with custom domain

**Changes Required:**

#### A. Tile Upload Pipeline
```rust
// src/tile_uploader.rs
pub struct TileUploader {
    r2: R2Client,
    cesium_tiles_dir: PathBuf,
}

impl TileUploader {
    pub async fn upload_all_tiles(&self) -> Result<()> {
        // Walk directory and upload all tiles
        for entry in WalkDir::new(&self.cesium_tiles_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                let key = format!("tiles/{}", path.strip_prefix(&self.cesium_tiles_dir)?);
                let data = std::fs::read(path)?;
                self.r2.upload(&key, data).await?;
            }
        }
        Ok(())
    }
}
```

#### B. Update Cesium Viewer to Use CDN Domain
```typescript
// Frontend: Update Cesium asset base URL
const viewer = new Cesium.Viewer('cesiumContainer', {
  terrainProvider: new Cesium.CesiumTerrainProvider({
    url: 'https://geo.sx9.io/tiles/terrain', // Custom domain
  }),
  imageryProvider: new Cesium.UrlTemplateImageryProvider({
    url: 'https://geo.sx9.io/tiles/imagery/{z}/{x}/{y}.png',
  }),
});
```

---

### 5. Smart CDN Gateway (18100) → Cloudflare Workers

**Current:** Axum server routing requests  
**Target:** Cloudflare Worker for edge routing

**Changes Required:**

#### A. Create Cloudflare Worker
```javascript
// workers/smart-gateway.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    
    // Route based on path
    if (url.pathname.startsWith('/statistical')) {
      return env.R2_STATISTICAL.fetch(request);
    }
    if (url.pathname.startsWith('/geospatial')) {
      return env.R2_GEO.fetch(request);
    }
    if (url.pathname.startsWith('/threat-intel')) {
      // Forward to GCP CDN with IAM
      return forwardToGCP(request, env);
    }
    
    // C2 filtering
    if (isC2Signature(request)) {
      return new Response('Forbidden', { status: 403 });
    }
    
    return new Response('Not Found', { status: 404 });
  }
}

function isC2Signature(request) {
  const ua = request.headers.get('user-agent');
  // Block Cobalt Strike, Metasploit, Sliver signatures
  return ua?.includes('Mozilla/5.0.*MSIE') || 
         ua?.includes('CobaltStrike') ||
         request.headers.get('cookie')?.length > 4096;
}
```

#### B. Update Local Gateway to Proxy
```rust
// Keep local gateway as fallback/proxy
.route("/proxy/*path", get(async |Path(path): Path<String>| {
    // Proxy to Cloudflare Worker
    let worker_url = format!("https://gateway.sx9.io/{}", path);
    let response = reqwest::get(&worker_url).await?;
    Ok(response.text().await?)
}))
```

---

## Migration Steps

### Phase 1: Preparation (Week 1)

1. **Set up Cloudflare R2 buckets**
   ```bash
   terraform apply -target=cloudflare_r2_bucket.sx9_statistical
   terraform apply -target=cloudflare_r2_bucket.sx9_geo
   terraform apply -target=cloudflare_r2_bucket.sx9_assets
   ```

2. **Set up GCP Cloud Storage buckets**
   ```bash
   terraform apply -target=google_storage_bucket.sx9_threat_intel
   terraform apply -target=google_compute_backend_bucket.sx9_cdn
   ```

3. **Generate API keys**
   - Cloudflare R2: Access Key ID + Secret Access Key
   - GCP: Service Account JSON key

4. **Update environment variables**
   ```bash
   # Add to all CDN service .env files
   R2_ACCOUNT_ID=...
   R2_ACCESS_KEY_ID=...
   R2_SECRET_ACCESS_KEY=...
   R2_ENDPOINT=...
   GCP_PROJECT_ID=...
   GCP_SERVICE_ACCOUNT_KEY=...
   ```

### Phase 2: Code Changes (Week 2)

1. **Add R2/GCP clients to each service**
   - Create `src/r2_client.rs` or `src/gcp_client.rs`
   - Add upload/download methods
   - Add health check integration

2. **Update service endpoints**
   - Add `/api/upload-to-cdn` endpoints
   - Add `/api/cdn-status` endpoints
   - Keep local endpoints as fallback

3. **Create background upload jobs**
   - Upload processed results to CDN
   - Sync local cache with CDN
   - Handle upload failures gracefully

### Phase 3: Data Migration (Week 3)

1. **Upload existing data**
   ```rust
   // Migration script: scripts/migrate_to_r2.rs
   async fn migrate_statistical_cdn() {
       let r2 = R2Client::new("sx9-statistical-cdn").await;
       let local_data = load_local_statistical_data().await;
       
       for (key, data) in local_data {
           r2.upload(&key, data).await?;
       }
   }
   ```

2. **Verify data integrity**
   - Compare checksums
   - Test download/access
   - Verify signed URLs work

### Phase 4: DNS & Routing (Week 4)

1. **Set up custom domains**
   ```bash
   # Cloudflare DNS
   terraform apply -target=cloudflare_record.cdn
   terraform apply -target=cloudflare_record.crates
   terraform apply -target=cloudflare_record.geo
   ```

2. **Deploy Cloudflare Workers**
   ```bash
   wrangler publish --config workers/smart-gateway.toml
   wrangler publish --config workers/monitoring-aggregator.toml
   ```

3. **Update service discovery**
   - Update health dashboard to check CDN endpoints
   - Update client code to use CDN URLs
   - Keep local fallback for development

### Phase 5: Cutover (Week 5)

1. **Gradual traffic migration**
   - 10% → 50% → 100% traffic to CDN
   - Monitor error rates
   - Keep local servers as backup

2. **Update all clients**
   - Frontend: Update asset URLs
   - Backend: Update API endpoints
   - Documentation: Update URLs

3. **Decommission local servers** (optional)
   - Keep as fallback for 30 days
   - Monitor CDN performance
   - Remove local servers after validation

---

## Code Changes Summary

### Required Dependencies

```toml
# Add to each CDN service Cargo.toml

[dependencies]
# Cloudflare R2 (S3-compatible)
aws-sdk-s3 = { version = "1.0", features = ["rustls"] }
aws-config = { version = "1.0", features = ["rustls"] }

# GCP Cloud Storage
google-cloud-storage = "0.10"
google-cloud-auth = "0.10"

# HTTP client for CDN operations
reqwest = { version = "0.12", features = ["json"] }
```

### New Modules Required

1. **`src/cdn_client.rs`** - Unified CDN client interface
2. **`src/r2_client.rs`** - Cloudflare R2 client
3. **`src/gcp_client.rs`** - GCP Cloud Storage client
4. **`src/cdn_uploader.rs`** - Background upload jobs
5. **`src/cdn_config.rs`** - CDN configuration

### Updated Endpoints

Each service needs:
- `GET /health` - Include CDN connectivity status
- `POST /api/upload-to-cdn` - Manual upload trigger
- `GET /api/cdn-status` - CDN health and metrics
- `GET /api/cdn-url/:key` - Get CDN URL for a key

---

## Configuration Changes

### Environment Variables

```bash
# All CDN services need:
CDN_PROVIDER=cloudflare-r2|gcp-cdn|internal
CDN_ENABLED=true

# Cloudflare R2
R2_ACCOUNT_ID=...
R2_ACCESS_KEY_ID=...
R2_SECRET_ACCESS_KEY=...
R2_BUCKET_NAME=...
R2_ENDPOINT=...
R2_CUSTOM_DOMAIN=cdn.sx9.io

# GCP Cloud CDN
GCP_PROJECT_ID=...
GCP_SERVICE_ACCOUNT_KEY=...
GCP_BUCKET_NAME=...
GCP_CDN_ENABLED=true
GCP_SIGNED_URL_EXPIRY=3600

# Fallback
LOCAL_FALLBACK_ENABLED=true
```

### Smart Crate TOML Updates

```toml
[cdn]
provider = "cloudflare-r2"
bucket = "sx9-statistical-cdn"
custom_domain = "statistical.sx9.io"
enabled = true
fallback_local = true

[cdn.upload]
interval_seconds = 60
batch_size = 100
retry_attempts = 3
```

---

## Testing Checklist

- [ ] R2 bucket creation and access
- [ ] GCP bucket creation and IAM setup
- [ ] Upload/download operations work
- [ ] Signed URLs generate correctly
- [ ] Cloudflare Workers route correctly
- [ ] C2 filtering works in Workers
- [ ] Health checks include CDN status
- [ ] Fallback to local works if CDN fails
- [ ] Data integrity verified (checksums)
- [ ] Performance meets requirements (<50ms latency)
- [ ] Cost monitoring in place

---

## Rollback Plan

If migration fails:

1. **Immediate:** Switch DNS back to local servers
2. **Short-term:** Keep local servers running in parallel
3. **Long-term:** Fix issues and retry migration

**Rollback triggers:**
- Error rate > 1%
- Latency > 200ms
- Data integrity failures
- Cost overruns

---

## Cost Estimates

| Service | Current (Local) | Cloudflare R2 | GCP CDN | Savings |
|---------|----------------|---------------|---------|---------|
| Statistical CDN | $50/mo (server) | $5-10/mo | - | 80-90% |
| Monitoring CDN | $50/mo (server) | $5-10/mo | - | 80-90% |
| Threat Intel CDN | $50/mo (server) | - | $10-20/mo | 60-80% |
| Geospatial CDN | $100/mo (storage) | $10-20/mo | - | 80-90% |
| **Total** | **$250/mo** | **$25-50/mo** | **$10-20/mo** | **~75%** |

---

## Next Steps

1. **Review and approve this plan**
2. **Set up Cloudflare/GCP accounts**
3. **Create Terraform infrastructure**
4. **Implement code changes (Phase 2)**
5. **Test in staging environment**
6. **Execute migration (Phases 3-5)**

---

**Status:** Ready for implementation  
**Estimated Timeline:** 5 weeks  
**Risk Level:** Medium (with fallback plan)



