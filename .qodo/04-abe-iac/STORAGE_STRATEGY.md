# Storage Strategy for 8.1M Lines / 1.3 GB Threat Data

**Problem:** Where to store 8.1 million lines (~1.3 GB uncompressed, ~400-500 MB compressed)  
**Requirement:** Must be performant and CDN-distributed for fast global access

---

## 🎯 **RECOMMENDED APPROACH: Multi-Tier Storage + CDN**

### **Tier 0: CDN Distribution (Edge Performance)**
Serve processed data from CDN for global performance:

#### **1. Cloudflare R2 + Workers (Public/Semi-public)**
- **What:** Processed JSON/DSL files, task graph, hashes
- **Size:** ~200-300 MB (compressed, optimized)
- **Cost:** $0.015/GB storage + **$0 egress** ⭐
- **Performance:** <50ms latency, 300+ PoPs worldwide
- **Integration:** Use existing `ctas7-cdn-threat-intel` service

```bash
# Upload to R2 bucket
rclone copy output/ r2:ctas7-threat-intel/ --progress

# Serve via Cloudflare Workers
# https://threat-intel.sx9.io/api/tools/{hash_id}
# https://threat-intel.sx9.io/api/ontology/{term}
```

#### **2. GCP Cloud CDN (IAM-gated, Private)**
- **What:** Sensitive threat data, operator-specific configs
- **Size:** ~100-150 MB
- **Cost:** $0.020/GB storage + $0 idle LB
- **Performance:** <100ms latency, IAM authentication
- **Integration:** Behind GCP Load Balancer

```bash
# Upload to GCS bucket
gsutil -m cp -r output/ gs://ctas7-threat-intel-private/

# Configure Cloud CDN
gcloud compute backend-buckets create threat-intel-backend \
  --gcs-bucket-name=ctas7-threat-intel-private
```

#### **3. ctas7-cdn-threat-intel Service (Local Edge)**
- **What:** Real-time threat intel API, caching layer
- **Port:** 18115 (existing service)
- **Performance:** Sub-millisecond local access
- **Integration:** Acts as cache between CDN and databases

---

### **Tier 1: Active Databases (Keep Hot)**
Store processed, queryable data in your existing databases:

#### **1. Supabase (Structured Data) - PRIMARY STORAGE** ⭐
- **What:** Processed threat items, hashes, metadata, task graph nodes
- **Tables:** Use existing `plasma_threats`, `plasma_entities` + new tables
- **Size:** ~500-800 MB (with subscription, can store more)
- **Cost:** **Already paid** (subscription includes storage)
- **Access:** Fast SQL queries, REST API, real-time subscriptions
- **Benefits:** 
  - Postgres performance (indexed queries)
  - Row-level security (RLS) for access control
  - Real-time subscriptions for live updates
  - Automatic backups and point-in-time recovery

```sql
-- New tables for threat content
CREATE TABLE threat_tools (
  id UUID PRIMARY KEY,
  name TEXT,
  category TEXT,
  trivariate_hash TEXT,
  unicode_operation TEXT,
  hd4_phase TEXT,
  task_graph_node JSONB,
  created_at TIMESTAMPTZ
);

CREATE TABLE threat_ontology (
  id UUID PRIMARY KEY,
  term TEXT,
  category TEXT,
  relationships JSONB,
  spires_metadata JSONB,
  created_at TIMESTAMPTZ
);
```

#### **2. Neo4j (Graph Relationships)**
- **What:** Task graph nodes, relationships, predecessors/successors
- **Size:** ~50-70 MB (graph structure)
- **Cost:** FREE (self-hosted) or Neo4j Aura: $65/mo
- **Access:** Cypher queries, graph traversals

```cypher
// Load task graph nodes
LOAD CSV WITH HEADERS FROM 'file:///task_graph.json' AS row
CREATE (t:ThreatTool {
  hash_id: row.hash_id,
  name: row.task_name,
  hd4_phase: row.hd4_phase,
  trivariate_hash: row.sch_hash + row.cuid_hash + row.sx9_uuid
})
```

#### **3. Sled KVS (Fast Lookups)**
- **What:** Hash lookups, Unicode mappings, quick IOC checks
- **Size:** ~20-30 MB
- **Cost:** FREE (local file-based)
- **Access:** O(1) hash lookups

---

### **Tier 2: GCP Cloud Storage (Archive)**
Store compressed archives for backup and long-term storage:

#### **GCS Bucket: `gs://ctas7-threat-intel/`**
- **What:** Compressed tar.gz archives of raw + processed data
- **Size:** ~400-500 MB compressed
- **Cost:** $0.020/GB/month = **~$0.01/month** (practically free)
- **Access:** Download when needed

```bash
# Upload compressed archive
gsutil cp threat_data_$(date +%Y%m%d).tar.gz gs://ctas7-threat-intel/

# Download when needed
gsutil cp gs://ctas7-threat-intel/threat_data_*.tar.gz ./
```

**Storage Classes:**
- **Standard:** $0.020/GB (frequent access)
- **Nearline:** $0.010/GB (monthly access) ⭐ **RECOMMENDED**
- **Coldline:** $0.004/GB (quarterly access)
- **Archive:** $0.0012/GB (yearly access)

---

### **Tier 3: Local Processing (Temporary)**
Keep only what you're actively processing:

- **Raw downloads:** Delete after processing (save ~800 MB)
- **Processing temp files:** Clean up after each stage
- **Keep only:** Final processed outputs (JSON/DSL)

---

## 📊 **STORAGE BREAKDOWN**

| Data Type | Size | Storage Location | CDN | Cost/Month |
|-----------|------|------------------|-----|------------|
| **CDN: Processed JSON (R2)** | 200-300 MB | Cloudflare R2 | ✅ <50ms | **~$0.003** |
| **CDN: Private Data (GCP CDN)** | 100-150 MB | GCP Cloud CDN | ✅ <100ms | **~$0.002** |
| **Processed JSON (Supabase)** | 500-800 MB | Supabase DB | ✅ | **Already paid** ⭐ |
| **Graph Data (Neo4j)** | 50-70 MB | Neo4j | ❌ | FREE (self-hosted) |
| **Hash Lookups (Sled)** | 20-30 MB | Local KVS | ❌ | FREE |
| **Compressed Archive (GCS)** | 400-500 MB | GCP Storage (backup) | ❌ | **~$0.01** |
| **Raw Repos (Delete)** | 800 MB | ❌ Delete after processing | ❌ | $0 |
| **Total Active** | **1.0-1.2 GB** | CDN + Databases | ✅ | **~$0.008/month** (CDN only) |

---

## 🚀 **IMPLEMENTATION PLAN**

### **Phase 1: Process & Store in Databases**
1. Process threat content → Supabase tables
2. Load task graph → Neo4j
3. Generate hash index → Sled KVS
4. **Delete raw repos** (save 800 MB)

### **Phase 2: Upload to CDN (Performance)**
1. **Optimize JSON files** (compress, minify)
2. **Upload to Cloudflare R2:**
   ```bash
   # Install rclone
   rclone config  # Configure R2
   rclone copy output/ontology/ r2:ctas7-threat-intel/ontology/ --progress
   rclone copy output/sx9_dsl/ r2:ctas7-threat-intel/dsl/ --progress
   rclone copy output/task_graph/ r2:ctas7-threat-intel/graph/ --progress
   ```
3. **Upload to GCP Cloud CDN:**
   ```bash
   gsutil -m cp -r output/hashes/ gs://ctas7-threat-intel-private/hashes/
   gcloud compute backend-buckets update threat-intel-backend \
     --gcs-bucket-name=ctas7-threat-intel-private
   ```
4. **Configure ctas7-cdn-threat-intel service:**
   - Point to R2/GCP CDN as origin
   - Add local caching layer
   - Expose API endpoints

### **Phase 3: Archive to GCS (Backup)**
1. Compress processed outputs: `tar -czf threat_data.tar.gz output/`
2. Upload to GCS: `gsutil cp threat_data.tar.gz gs://ctas7-threat-intel-archive/`
3. **Delete local archive** (keep only in CDN + GCS backup)

### **Phase 4: Cleanup**
1. Keep only active database data locally
2. Everything else in CDN (R2/GCP) + GCS backup
3. **Local footprint: < 500 MB**

---

## 💰 **COST COMPARISON**

| Storage Option | Size | CDN | Monthly Cost |
|----------------|------|-----|--------------|
| **Cloudflare R2** | 300 MB | ✅ <50ms | **$0.005** ⭐ |
| **GCP Cloud CDN** | 150 MB | ✅ <100ms | **$0.003** |
| **Supabase (Subscription)** | 500-800 MB | ✅ | **Already paid** ⭐ |
| **Neo4j Self-hosted** | 70 MB | ❌ | **FREE** |
| **Sled KVS** | 30 MB | ❌ | **FREE** |
| **GCS Archive (backup)** | 500 MB | ❌ | **$0.01** |
| **Total** | 1.5-1.8 GB | ✅ | **~$0.008/month** (CDN only, Supabase already paid) |

**Performance Benefits:**
- **CDN Access:** <50ms latency globally (300+ PoPs)
- **Zero Egress:** Cloudflare R2 has $0 egress fees
- **Cached:** ctas7-cdn-threat-intel service caches hot data
- **Scalable:** Handles millions of requests

**vs. Keeping Everything Local:**
- 1.3 GB local disk = $0 (but slow, not accessible remotely)
- No CDN = High latency for global users
- No caching = Repeated database queries

---

## ✅ **RECOMMENDED STRATEGY (CDN-Optimized + Supabase Subscription)**

1. **Process → Supabase** (Primary storage, already paid) ⭐
   - Store all processed threat items in Supabase tables
   - Use existing `plasma_threats`, `plasma_entities` + new tables
   - Leverage subscription storage capacity
2. **Upload → CDN** (Cloudflare R2 + GCP Cloud CDN) ⭐ **PERFORMANT**
   - Serve frequently accessed data from CDN
   - Supabase as authoritative source, CDN as cache
3. **Graph → Neo4j** (Relationships only, ~50-70 MB)
4. **Cache → ctas7-cdn-threat-intel** (local edge cache)
5. **Archive → GCS** (compressed tar.gz backup)
6. **Delete raw repos** (save 800 MB)
7. **Keep local:** Only active processing files (< 100 MB)

**Result:**
- **Performant:** <50ms CDN latency globally
- **Queryable:** Fast SQL queries (Supabase) + CDN API
- **Accessible:** Available from anywhere via CDN + Supabase REST API
- **Cheap:** ~$0.008/month (CDN only, Supabase already paid)
- **Scalable:** Handles millions of requests
- **Cached:** Hot data in edge cache, Supabase as source of truth
- **Real-time:** Supabase subscriptions for live updates

---

## 🔧 **QUICK SETUP (CDN-Optimized)**

```bash
# 1. Process and load to databases
python threat_content_fetcher.py --all
python load_to_supabase.py
python load_to_neo4j.py

# 2. Optimize for CDN (compress JSON, minify)
python optimize_for_cdn.py --input output/ --output output_cdn/

# 3. Upload to Cloudflare R2 (public/semi-public)
rclone config  # Configure R2 bucket
rclone copy output_cdn/ontology/ r2:ctas7-threat-intel/ontology/ --progress
rclone copy output_cdn/sx9_dsl/ r2:ctas7-threat-intel/dsl/ --progress
rclone copy output_cdn/task_graph/ r2:ctas7-threat-intel/graph/ --progress

# 4. Upload to GCP Cloud CDN (private/IAM-gated)
gsutil mb -l us-central1 gs://ctas7-threat-intel-private
gsutil -m cp -r output_cdn/hashes/ gs://ctas7-threat-intel-private/hashes/
gcloud compute backend-buckets create threat-intel-backend \
  --gcs-bucket-name=ctas7-threat-intel-private

# 5. Configure ctas7-cdn-threat-intel service
# Update service config to point to R2/GCP CDN origins
# Add caching layer for hot data

# 6. Archive backup to GCS
tar -czf threat_data_$(date +%Y%m%d).tar.gz output/
gsutil cp threat_data_*.tar.gz gs://ctas7-threat-intel-archive/

# 7. Cleanup
rm -rf output/threat_content/*/  # Delete raw repos
rm -rf output_cdn/  # Delete optimized copies (now in CDN)
rm threat_data_*.tar.gz  # Delete local archive
```

## 🌐 **CDN API ENDPOINTS**

After setup, access via:

```bash
# Public CDN (Cloudflare R2)
GET https://threat-intel.sx9.io/api/tools/{hash_id}
GET https://threat-intel.sx9.io/api/ontology/{term}
GET https://threat-intel.sx9.io/api/graph/{node_id}

# Private CDN (GCP Cloud CDN, IAM-gated)
GET https://threat-intel-private.sx9.io/api/hashes/{hash}
GET https://threat-intel-private.sx9.io/api/unicode/{code_point}

# Local Edge Cache (ctas7-cdn-threat-intel)
GET http://localhost:18115/api/tools/{hash_id}  # Cached, <1ms
```

---

## 📝 **NOTES**

- **Supabase free tier:** 500 MB database, 1 GB file storage
- **Neo4j:** Self-hosted is free, Aura starts at $65/mo
- **GCS:** First 5 GB free, then $0.01/GB/month (Nearline)
- **Total cost:** Practically free for this dataset size

**You're storing 8.1M lines for ~$0.01/month. That's pretty good.**

