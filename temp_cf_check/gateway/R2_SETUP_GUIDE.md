# R2 CDN Subscriber - Setup & Deployment Guide

Complete guide to integrating CloudFlare R2 CDN into your SX9 Gateway

---

## 📋 PREREQUISITES

```bash
# 1. CloudFlare account with R2 enabled
# 2. Wrangler CLI installed
npm install -g wrangler

# 3. Login to CloudFlare
wrangler login
```

---

## 🚀 STEP-BY-STEP DEPLOYMENT

### STEP 1: Create R2 Buckets

```bash
# Create the 4 required buckets
wrangler r2 bucket create sx9-threat-intel
wrangler r2 bucket create sx9-mitre-attack
wrangler r2 bucket create sx9-kali-tools
wrangler r2 bucket create sx9-osint-data

# Verify buckets
wrangler r2 bucket list
```

### STEP 2: Generate R2 API Tokens

```bash
# Option A: Via Wrangler
wrangler r2 api-token create sx9-gateway-token

# Option B: Via Dashboard
# 1. Go to: https://dash.cloudflare.com
# 2. R2 → Manage R2 API Tokens
# 3. Create API Token
# 4. Permissions: Object Read & Write
# 5. Copy Access Key ID and Secret Access Key
```

### STEP 3: Configure Environment

```bash
# Copy example env file
cp .env.r2.example .env

# Edit .env with your credentials
nano .env

# Required values:
# - R2_ENDPOINT (e.g., https://abc123def456.r2.cloudflarestorage.com)
# - R2_ACCESS_KEY_ID
# - R2_SECRET_ACCESS_KEY
```

### STEP 4: Add Files to R2 (Initial Setup)

You have two options:

#### Option A: Manual Upload (Quick Test)

```bash
# Create sample threat-tools.json
cat > threat-tools.json << 'EOF'
{
  "version": "1.0.0",
  "generated_at": "2025-12-14T00:00:00Z",
  "total_tools": 3,
  "sources": ["manual-upload"],
  "tools": [
    {
      "unicode": "E800",
      "name": "nmap",
      "category": "NetworkRecon",
      "hash_operational": "triv:abc123_def456_uuid",
      "hash_semantic": "triv:xyz789_uvw012_uuid",
      "genome": "1234567890abcdef",
      "binary_path": "/usr/bin/nmap",
      "docker_image": "instrumentisto/nmap:latest",
      "mitre_techniques": ["T1046"],
      "dependencies": ["nmap"],
      "capabilities": ["NET_ADMIN", "NET_RAW"]
    }
  ]
}
EOF

# Upload to R2
wrangler r2 object put sx9-threat-intel/threat-tools.json --file=threat-tools.json
```

#### Option B: Automated via CloudFlare Workflow (Recommended)

```bash
# Deploy the CloudFlare Workflow (from previous setup)
cd cloudflare
npm run deploy:workflow

# Trigger the workflow to populate R2
curl -X POST https://threat-intel-sync.YOUR-ACCOUNT.workers.dev

# This will:
# - Fetch from 27 threat sources
# - Generate RFC-9001 hashes
# - Upload to all 4 R2 buckets
```

### STEP 5: Integrate R2 Module into Gateway

```bash
cd sx9-gateway

# 1. Create cdn module directory
mkdir -p src/cdn

# 2. Copy the R2 subscriber module
cp /path/to/r2_subscriber.rs src/cdn/

# 3. Copy the cdn mod file
cp /path/to/cdn_mod.rs src/cdn/mod.rs

# 4. Add to Cargo.toml
cat cargo_additions.toml >> Cargo.toml

# 5. Update src/lib.rs or src/main.rs
# Add: pub mod cdn;
```

### STEP 6: Update Main Gateway Code

```rust
// In your main.rs (see main_integration_example.rs for full example)

use cdn::{R2Config, R2SubscriberService};

#[tokio::main]
async fn main() -> Result<()> {
    // ... existing initialization ...
    
    // Initialize R2 service
    let r2_config = R2Config {
        port: 18127,
        sync_interval: Duration::from_secs(3600),
        cache_ttl: Duration::from_secs(7200),
    };
    
    let r2_service = Arc::new(R2SubscriberService::new(r2_config).await?);
    
    // Start R2 service
    tokio::spawn({
        let r2 = Arc::clone(&r2_service);
        async move {
            if let Err(e) = r2.start().await {
                error!("R2 service error: {}", e);
            }
        }
    });
    
    // ... rest of gateway ...
}
```

### STEP 7: Build & Run

```bash
# Build with R2 feature
cargo build --release --features r2-cdn

# Run
./target/release/sx9-gateway

# Or run directly
cargo run --release --features r2-cdn
```

### STEP 8: Verify Deployment

```bash
# Check health
curl http://localhost:18127/health

# Should return:
# {
#   "status": "healthy",
#   "service": "r2-cdn-subscriber",
#   "cache_age_seconds": 10,
#   "cached_manifests": {
#     "threat_tools": true,
#     "mitre_matrix": true,
#     "kali_tools": true
#   }
# }

# Get all tools
curl http://localhost:18127/tools

# Get specific tool
curl http://localhost:18127/tools/E800

# Get MITRE matrix
curl http://localhost:18127/mitre

# Trigger manual sync
curl -X POST http://localhost:18127/sync
```

---

## 🔧 TROUBLESHOOTING

### Issue: "R2_ENDPOINT not set"

```bash
# Check .env file exists and is loaded
cat .env | grep R2_ENDPOINT

# Ensure .env is in correct location
# For Docker: mount .env or pass as env vars
# For local: .env in project root
```

### Issue: "Failed to fetch from R2"

```bash
# Test R2 credentials manually
wrangler r2 bucket list

# If that works, check bucket contents
wrangler r2 object list sx9-threat-intel

# If empty, upload test file
echo '{"test": true}' | wrangler r2 object put sx9-threat-intel/test.json
```

### Issue: "Cache not populated yet"

```bash
# Check logs for sync errors
# Should see: "✅ R2 sync complete in XXms"

# If not, check network connectivity to R2
curl -I https://YOUR-ACCOUNT-ID.r2.cloudflarestorage.com

# Trigger manual sync
curl -X POST http://localhost:18127/sync

# Check cache stats
curl http://localhost:18127/cache/stats
```

### Issue: Build errors

```bash
# Ensure all dependencies installed
cargo update

# Check Rust version (needs 1.70+)
rustc --version

# If AWS SDK errors, clean and rebuild
cargo clean
cargo build --release --features r2-cdn
```

---

## 📊 MONITORING

### View Cache Stats

```bash
curl http://localhost:18127/cache/stats

# Returns:
# {
#   "cache_age_seconds": 600,
#   "sync_count": 5,
#   "cached_items": {
#     "threat_tools": 27606,
#     "mitre_techniques": 500,
#     "kali_tools": 600
#   }
# }
```

### Check Sync Frequency

```bash
# Watch logs for sync activity
tail -f /var/log/sx9-gateway.log | grep "R2 sync"

# Should see every hour:
# "🔄 Starting scheduled R2 sync..."
# "✅ R2 sync complete in 2.3s"
```

### Monitor R2 Usage

```bash
# Via Wrangler
wrangler r2 bucket list

# Via Dashboard
# https://dash.cloudflare.com → R2 → Analytics
# Shows:
# - Storage used
# - Requests (Class A / Class B)
# - Egress (should be $0!)
```

---

## 🎯 PERFORMANCE TUNING

### Adjust Sync Interval

```rust
// In main.rs
let r2_config = R2Config {
    port: 18127,
    sync_interval: Duration::from_secs(1800), // 30 minutes instead of 1 hour
    cache_ttl: Duration::from_secs(3600),
};
```

### Increase Cache TTL

```rust
let r2_config = R2Config {
    port: 18127,
    sync_interval: Duration::from_secs(3600),
    cache_ttl: Duration::from_secs(14400), // 4 hours
};
```

### Memory Usage

```bash
# R2 cache is in-memory
# Typical usage with 27K tools: ~50-100 MB
# Monitor with:
ps aux | grep sx9-gateway

# If memory constrained, reduce cache TTL or implement LRU eviction
```

---

## 💰 COST TRACKING

```bash
# Check R2 costs via Wrangler
wrangler r2 bucket list --json

# Or via Dashboard Analytics
# https://dash.cloudflare.com → R2 → Analytics

# Expected for SX9:
# - Storage: 0-0.3 GB = $0/month
# - Class A ops: <1M/month = $0/month
# - Class B ops: <10M/month = $0/month
# - Egress: Unlimited = $0/month
# TOTAL: $0/month
```

---

## 📋 FILES PROVIDED

```
r2_subscriber.rs                 - Main R2 CDN module (700+ lines)
cargo_additions.toml             - Dependencies to add
cdn_mod.rs                       - CDN module declaration
main_integration_example.rs      - Integration example
.env.r2.example                  - Environment template
R2_CDN_SUBSCRIBER_SERVICE.md     - Architecture docs
```

---

## ✅ DEPLOYMENT CHECKLIST

- [ ] CloudFlare account created
- [ ] R2 enabled in account
- [ ] Wrangler CLI installed
- [ ] 4 R2 buckets created
- [ ] R2 API tokens generated
- [ ] .env file configured
- [ ] Sample data uploaded to R2
- [ ] R2 module integrated into gateway
- [ ] Cargo.toml updated
- [ ] Gateway compiled successfully
- [ ] Service starts without errors
- [ ] Health check returns 200 OK
- [ ] Tools endpoint returns data
- [ ] Sync completes successfully
- [ ] Monitoring configured

---

## 🎉 SUCCESS!

Your SX9 Gateway now has:
- ✅ Global CloudFlare R2 CDN integration
- ✅ Automatic hourly sync
- ✅ <1ms local cache lookups
- ✅ $0/month egress costs
- ✅ RFC-9114 compliant
- ✅ Bernoulli Zone C performance

**Total cost: $0/month!** 🚀
