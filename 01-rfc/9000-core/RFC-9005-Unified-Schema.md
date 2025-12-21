# RFC-9005 — Unified Schema Specification

**Version:** 1.2
**Status:** Final
**Date:** December 14, 2025
**Applies To:** CTAS-7.3.1, ABE, PLASMA/ATLAS Infrastructure, SX9 Gateway
**Author:** CTAS Core Engineering Group
**Supersedes:** RFC-9005 v1.1 (November 2025)
**Dependencies:** RFC-9001, RFC-9002, RFC-9003, RFC-9114

---

## Abstract

This specification establishes a consolidated schema architecture for the CTAS-7 ecosystem, eliminating fragmentation across multiple database systems and providing a single source of truth compliant with RFC-9001 (Trivariate Hashing), RFC-9002 (Unicode Routing), and RFC-9003 (Operation Classification).

**Version 1.2 Updates:**

- Cloudflare R2 CDN integration for zero-egress threat intelligence distribution
- Waterfall data access pattern (KV → R2 → Supabase → Neon → Neo4j)
- R2 bucket schema for threat intel, embeddings, and OSINT data
- CDN service port allocations (18127, 18125, 18128)

---

## 1. Problem Statement

The CTAS-7 system exhibits schema fragmentation across multiple storage backends:

| Issue                          | Impact                                      |
| ------------------------------ | ------------------------------------------- |
| Multiple overlapping databases | Supabase + SurrealDB + Sledis + GLAF        |
| Unicode range conflicts        | Inconsistent allocations across services    |
| Schema duplication             | Same concepts modeled differently           |
| Mixed data types               | JSONB vs structured fields vs string arrays |
| No migration strategy          | No versioning or evolution path             |
| Performance degradation        | Multiple sync operations across systems     |

**Resolution:** This RFC establishes a unified schema architecture with PostgreSQL (Supabase) as the primary store.

---

## 2. Unified Schema Architecture

### 2.1 Single Source of Truth

**Primary Database:** PostgreSQL via Supabase

**Rationale:**

- Enterprise-grade reliability and ACID compliance
- Native JSONB for polymorphic data storage
- Row Level Security (RLS) for multi-tenant isolation
- Extensive ecosystem and tooling
- Real-time subscriptions via Supabase

**Secondary Systems (Derived/Cached):**

- Sledis: High-speed key-value cache
- SurrealDB: Graph query acceleration (read replicas)

**Cloudflare R2 CDN (Global Distribution):**

- R2 Buckets: Zero-egress threat intelligence storage
- Cloudflare KV: Edge-cached micro-objects (<100KB)
- Cloudflare Workers: Data processing and signed URL generation
- Local R2 Subscriber: 5-minute sync to Sled cache (port 18127)
- ChromaDB CDN: Vector embeddings distribution (port 18125)

### 2.2 Unicode Allocation (RFC-9002 Extension)

Per RFC-9002, the base allocation is U+E000–E9FF. This specification extends the allocation to support additional operational requirements:

| Range         | Class | Purpose                     | Count | RFC       |
| ------------- | ----- | --------------------------- | ----- | --------- |
| `U+E000–E1FF` | A     | Core Components             | 512   | 9002      |
| `U+E200–E2FF` | B     | CUID Slot Mapping           | 256   | 9002      |
| `U+E300–E3FF` | C     | Semantic Routing            | 256   | 9002      |
| `U+E400–E6FF` | D     | Neural Mux Operations       | 768   | 9002      |
| `U+E700–E7FF` | —     | Reserved (Future)           | 256   | 9002      |
| `U+E800–E9FF` | E     | Experimental/Research       | 512   | 9002      |
| `U+EA00–EAFF` | F     | IAC Infrastructure Triggers | 256   | 9005/9103 |
| `U+EB00–EBFF` | G     | Escalation Tier Markers     | 256   | 9005      |
| `U+EC00–EFFF` | H     | EEI & Intelligence Patterns | 1024  | 9005      |

**Total Allocation:** 4096 codepoints (U+E000–EFFF)

> **Note:** Ranges U+EA00–EFFF are extensions defined by this RFC and RFC-9103. Implementations MUST support RFC-9002 base range; extended ranges are OPTIONAL.

---

## 3. Core Schema Definition

### 3.1 Unified Entities Table

```sql
-- ============================================================================
-- RFC-9005 UNIFIED SCHEMA — PostgreSQL (Supabase)
-- ============================================================================
-- Version: 1.1
-- Compliance: RFC-9001, RFC-9002, RFC-9003, RFC-9005

CREATE TABLE entities (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- RFC-9001: Trivariate Hash Identity
    trivariate_hash TEXT NOT NULL,          -- Format: SCH-CUID-UUID
    sch_hash TEXT,                          -- Murmur3-128 (24 chars)
    cuid TEXT,                              -- Base96 (16 chars)

    -- RFC-9002: Unicode Addressing
    unicode_address TEXT,                   -- U+E000-EFFF
    unicode_class CHAR(1),                  -- A-H class designation

    -- RFC-9003: Operation Classification
    operation_class TEXT CHECK (operation_class IN (
        'intelligence', 'defensive', 'offensive', 'administrative'
    )),
    escalation_tier INTEGER DEFAULT 1 CHECK (escalation_tier BETWEEN 1 AND 7),

    -- Entity Identity
    name TEXT NOT NULL,
    entity_type TEXT NOT NULL CHECK (entity_type IN (
        'component', 'tool', 'escalation', 'eei', 'crate',
        'node', 'daemon', 'atlas_node', 'iac_manifold',
        'actor', 'object', 'event', 'concept', 'attribute',
        'function', 'module', 'header', 'footer', 'comment'
    )),
    description TEXT,

    -- Polymorphic Capabilities
    capabilities JSONB DEFAULT '{}',
    limitations JSONB DEFAULT '{}',

    -- Tactical Profile
    tactical JSONB DEFAULT '{
        "ttps": [],
        "toolchain_refs": [],
        "attack_vectors": []
    }',

    -- GLAF-Compatible Relationships
    relationships JSONB DEFAULT '{
        "dependencies": [],
        "provides_to": [],
        "coordinates_with": [],
        "escalates_to": []
    }',

    -- Type-Specific Extensions
    type_extensions JSONB DEFAULT '{}',

    -- State Management
    current_state TEXT DEFAULT 'draft' CHECK (current_state IN (
        'draft', 'active', 'deprecated', 'archived'
    )),
    compilation_status TEXT DEFAULT 'pending',
    last_compiled_at TIMESTAMPTZ,

    -- Cross-System Sync
    sled_key TEXT,
    surreal_node_id TEXT,
    hash_slot INTEGER,

    -- RFC Compliance Tracking
    rfc_version TEXT DEFAULT '9001-9002-9003-9005',
    schema_version TEXT DEFAULT '1.1',

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance Indexes
CREATE UNIQUE INDEX idx_entities_trivariate ON entities (trivariate_hash);
CREATE INDEX idx_entities_sch ON entities (sch_hash);
CREATE INDEX idx_entities_cuid ON entities (cuid);
CREATE INDEX idx_entities_unicode ON entities (unicode_address);
CREATE INDEX idx_entities_type ON entities (entity_type);
CREATE INDEX idx_entities_class ON entities (operation_class);
CREATE INDEX idx_entities_tier ON entities (escalation_tier);
CREATE INDEX idx_entities_state ON entities (current_state);
CREATE INDEX idx_entities_created ON entities (created_at DESC);
```

### 3.2 Relationships Table (GLAF + Neural Mux)

```sql
CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    target_entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,

    relationship_type TEXT NOT NULL CHECK (relationship_type IN (
        -- Standard relationships
        'depends_on', 'provides_to', 'coordinates_with', 'escalates_to',
        'implements', 'extends', 'uses', 'monitors', 'configures',
        -- ATLAS cognitive links
        'atlas_cognitive_link', 'neural_mux_routing',
        -- IAC relationships
        'iac_manifold_spawn', 'iac_manifold_teardown'
    )),

    -- Neural Mux Routing
    unicode_linkage TEXT,
    neural_weight DECIMAL(5,4) DEFAULT 1.0,
    routing_latency_ns INTEGER,

    -- Escalation Context
    escalation_tier INTEGER DEFAULT 1,
    escalation_approved BOOLEAN DEFAULT false,
    escalation_approver TEXT,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Prevent duplicate relationships
    UNIQUE(source_entity_id, target_entity_id, relationship_type)
);

CREATE INDEX idx_rel_source ON relationships (source_entity_id);
CREATE INDEX idx_rel_target ON relationships (target_entity_id);
CREATE INDEX idx_rel_type ON relationships (relationship_type);
CREATE INDEX idx_rel_unicode ON relationships (unicode_linkage);
```

### 3.3 ATLAS Cognitive Nodes Table

```sql
CREATE TABLE atlas_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,

    -- Cognitive Performance
    cognitive_tick_rate_us INTEGER DEFAULT 1000,   -- 1ms = 1000μs
    neural_mux_latency_ns INTEGER DEFAULT 250,     -- Target: <250ns

    -- Learning Capabilities
    l_star_learning_enabled BOOLEAN DEFAULT true,
    voice_orchestration_enabled BOOLEAN DEFAULT false,

    -- IAC Capabilities
    iac_manifold_capabilities TEXT[] DEFAULT '{}',
    cuda_compute_slots INTEGER DEFAULT 0,
    burst_compute_allocation JSONB DEFAULT '{}',

    -- Operational State
    last_cognitive_tick TIMESTAMPTZ,
    node_status TEXT DEFAULT 'dormant' CHECK (node_status IN (
        'dormant', 'initializing', 'active', 'burst', 'cooling', 'error'
    )),

    -- Metrics
    ticks_processed BIGINT DEFAULT 0,
    average_tick_latency_us DECIMAL(10,2),

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_atlas_entity ON atlas_nodes (entity_id);
CREATE INDEX idx_atlas_status ON atlas_nodes (node_status);
```

### 3.4 IAC Manifold Table

```sql
CREATE TABLE iac_manifolds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    manifold_id UUID NOT NULL UNIQUE,
    atlas_node_id UUID NOT NULL REFERENCES atlas_nodes(id) ON DELETE CASCADE,

    manifold_type TEXT NOT NULL CHECK (manifold_type IN (
        'abe_customer_env', 'plasma_burst_compute', 'cuda_parallel_cluster',
        'gee_paygo_instance', 'conda_scientific_env', 'kali_penetration_env',
        'cdn_edge_node', 'validation_cluster', 'monte_carlo_cluster'
    )),

    -- Infrastructure Paths
    terraform_module_path TEXT,
    kubernetes_manifest_path TEXT,

    -- Compute Configuration
    compute_requirements JSONB DEFAULT '{}',
    gpu_allocation INTEGER DEFAULT 0,
    memory_gb INTEGER DEFAULT 0,

    -- Unicode Trigger (RFC-9103)
    spawn_trigger_unicode TEXT,   -- U+EA00-EAFF range

    -- Lifecycle
    current_status TEXT DEFAULT 'dormant' CHECK (current_status IN (
        'dormant', 'spawning', 'active', 'scaling', 'teardown', 'error'
    )),
    spawn_time_ms INTEGER,
    teardown_time_ms INTEGER,
    last_spawn_at TIMESTAMPTZ,
    last_teardown_at TIMESTAMPTZ,

    -- Cost Tracking
    cost_per_minute DECIMAL(10,4),
    total_runtime_minutes INTEGER DEFAULT 0,
    total_cost DECIMAL(12,2) DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_manifold_atlas ON iac_manifolds (atlas_node_id);
CREATE INDEX idx_manifold_type ON iac_manifolds (manifold_type);
CREATE INDEX idx_manifold_status ON iac_manifolds (current_status);
CREATE INDEX idx_manifold_trigger ON iac_manifolds (spawn_trigger_unicode);
```

---

## 4. Row Level Security

```sql
-- Enable RLS on all tables
ALTER TABLE entities ENABLE ROW LEVEL SECURITY;
ALTER TABLE relationships ENABLE ROW LEVEL SECURITY;
ALTER TABLE atlas_nodes ENABLE ROW LEVEL SECURITY;
ALTER TABLE iac_manifolds ENABLE ROW LEVEL SECURITY;

-- Read policies (authenticated users)
CREATE POLICY "entities_read" ON entities
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "relationships_read" ON relationships
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "atlas_nodes_read" ON atlas_nodes
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "iac_manifolds_read" ON iac_manifolds
    FOR SELECT USING (auth.role() = 'authenticated');

-- Write policies (service role only for critical tables)
CREATE POLICY "atlas_nodes_write" ON atlas_nodes
    FOR ALL USING (auth.role() = 'service_role');

CREATE POLICY "iac_manifolds_write" ON iac_manifolds
    FOR ALL USING (auth.role() = 'service_role');
```

---

## 5. Automatic Triggers

```sql
-- Updated timestamp trigger
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER entities_updated
    BEFORE UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- ATLAS tick counter
CREATE OR REPLACE FUNCTION increment_tick_counter()
RETURNS TRIGGER AS $$
BEGIN
    NEW.ticks_processed = OLD.ticks_processed + 1;
    NEW.last_cognitive_tick = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

---

## 6. Migration Strategy

### 6.1 Phase 1: Schema Deployment

```bash
# 1. Deploy unified schema to Supabase
supabase db push

# 2. Verify table creation
supabase db diff
```

### 6.2 Phase 2: Data Migration

```sql
-- Migrate from legacy tables
INSERT INTO entities (name, entity_type, trivariate_hash, ...)
SELECT
    name,
    COALESCE(type, 'component') as entity_type,
    generate_trivariate_hash(id) as trivariate_hash,
    ...
FROM legacy_components;
```

### 6.3 Phase 3: Service Updates

| Service          | Migration Action               |
| ---------------- | ------------------------------ |
| Lightning QA     | Query unified `entities` table |
| ATLAS Daemon     | Use `atlas_nodes` table        |
| IAC Controller   | Use `iac_manifolds` table      |
| Health Dashboard | Single schema queries          |

---

## 7. RFC Compliance Matrix

| Requirement                     | RFC       | Implementation                                | Status |
| ------------------------------- | --------- | --------------------------------------------- | ------ |
| Trivariate hash on all entities | 9001      | `trivariate_hash`, `sch_hash`, `cuid` columns | ✅     |
| Unicode addressing              | 9002      | `unicode_address`, `unicode_class` columns    | ✅     |
| Operation classification        | 9003      | `operation_class`, `escalation_tier` columns  | ✅     |
| GLAF relationships              | 9005      | `relationships` table with typed links        | ✅     |
| ATLAS cognitive nodes           | 9005      | `atlas_nodes` table with tick tracking        | ✅     |
| IAC manifold spawning           | 9005/9103 | `iac_manifolds` table with triggers           | ✅     |
| Extended Unicode (EA00-EFFF)    | 9005      | Optional ranges documented                    | ✅     |

---

## 8. Success Metrics

| Metric               | Target             | Measurement             |
| -------------------- | ------------------ | ----------------------- |
| Schema consolidation | 5 → 1 databases    | Primary store count     |
| Unicode compliance   | 100% RFC-9002 base | Allocation validation   |
| Query performance    | <10ms p99          | Entity lookup latency   |
| ATLAS tick rate      | 1ms ± 50μs         | Cognitive tick timing   |
| Neural routing       | <250ns             | Mux latency measurement |
| IAC spawn time       | <30s               | Manifold initialization |

---

## 10. Cloudflare R2 CDN Integration

### 10.1 Architecture Overview

Cloudflare R2 provides zero-egress cost global distribution for threat intelligence data:

```
┌─────────────────────────────────────────────────────────────────┐
│                    CLOUDFLARE EDGE                              │
├─────────────────────────────────────────────────────────────────┤
│  Cloudflare KV (Edge Cache)                                     │
│  ├─ threat/{trivariate_hash}.json  (<10ms access)              │
│  └─ embeddings/{id}.bin            (<10ms access)              │
│                                                                 │
│  Cloudflare R2 (Bulk Storage, $0 Egress)                       │
│  ├─ threat-intel/                  (<50ms access)              │
│  ├─ mitre-attack/                                              │
│  ├─ kali-tools/                                                │
│  └─ osint-feeds/                                               │
│                                                                 │
│  Cloudflare Workers                                            │
│  ├─ Data processing & validation                               │
│  ├─ Trivariate hash generation (RFC-9001)                      │
│  └─ Signed URL generation                                      │
└─────────────────────────────────────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 GATEWAY (Local)                          │
├─────────────────────────────────────────────────────────────────┤
│  R2 Subscriber Service (Port 18127)                            │
│  ├─ 5-minute sync interval                                     │
│  ├─ Validates trivariate hashes                                │
│  └─ Stores in Sled cache                                       │
│                                                                 │
│  ChromaDB CDN Service (Port 18125)                             │
│  ├─ Vector embedding distribution                              │
│  └─ 768-dim embeddings from R2                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 10.2 R2 Bucket Schema

**Bucket: `sx9-threat-intel`**

```
threat-intel/
├─ {trivariate_hash}.toml          # Threat intelligence records
├─ {trivariate_hash}.json          # JSON format
└─ metadata/
   └─ index.json                   # Bucket index

mitre-attack/
├─ techniques/
│  └─ {technique_id}.json          # T1059.001, etc.
├─ tactics/
│  └─ {tactic_id}.json
└─ matrices/
   └─ enterprise.json

kali-tools/
├─ manifests/
│  └─ {tool_name}.json             # Tool metadata
└─ binaries/
   └─ {tool_name}.tar.gz           # Compressed binaries

osint-feeds/
├─ daily/
│  └─ {YYYY-MM-DD}.json            # Daily OSINT aggregation
└─ sources/
   └─ {source_id}/
      └─ latest.json

embeddings/
├─ threat/
│  └─ {id}.bin                     # 768-dim vectors
└─ technique/
   └─ {technique_id}.bin
```

### 10.3 Waterfall Data Access Pattern

Client requests follow this priority order:

```
1. Cloudflare KV (Edge)        →  <10ms   (80%+ hit rate target)
2. Cloudflare R2 (Bulk)        →  <50ms   (15% of requests)
3. Supabase (Primary DB)       →  100-200ms (4% of requests)
4. Neon (PostgreSQL)           →  200-300ms (0.9% of requests)
5. Neo4j (Graph DB)            →  300-500ms (0.1% of requests)
```

**Implementation:**

```python
async def fetch_threat_intel(trivariate_hash: str) -> ThreatIntel:
    # 1. Check Cloudflare KV (fastest)
    kv_result = await cloudflare_kv.get(f"threat/{trivariate_hash}")
    if kv_result:
        metrics.record("cdn.kv.hit")
        return ThreatIntel.parse(kv_result)

    # 2. Check R2 bucket
    r2_result = await r2_client.fetch(f"threat-intel/{trivariate_hash}.json")
    if r2_result:
        metrics.record("cdn.r2.hit")
        # Populate KV for next request
        await cloudflare_kv.put(f"threat/{trivariate_hash}", r2_result, ttl=300)
        return ThreatIntel.parse(r2_result)

    # 3. Fallback to Supabase
    supabase_result = await supabase.from_("entities").select("*").eq("trivariate_hash", trivariate_hash).single()
    if supabase_result:
        metrics.record("cdn.supabase.fallback")
        return ThreatIntel.from_db(supabase_result)

    # 4. Fallback to Neon
    # 5. Fallback to Neo4j
    # ... (similar pattern)
```

### 10.4 Cloudflare Worker Configuration

**`wrangler.toml`:**

```toml
name = "sx9-threat-intel-worker"
main = "src/index.ts"
compatibility_date = "2025-12-14"

[[r2_buckets]]
binding = "SX9_THREAT_INTEL"
bucket_name = "sx9-threat-intel"

[[kv_namespaces]]
binding = "SX9_KV_CACHE"
id = "YOUR_KV_NAMESPACE_ID"

[env.production]
vars = { ENVIRONMENT = "production" }
```

**Worker Logic:**

```typescript
export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const trivariateHash = url.searchParams.get("hash");

    // 1. Check KV cache
    const cached = await env.SX9_KV_CACHE.get(`threat/${trivariateHash}`);
    if (cached) {
      return new Response(cached, {
        headers: { "Content-Type": "application/json", "X-Cache": "HIT" },
      });
    }

    // 2. Fetch from R2
    const object = await env.SX9_THREAT_INTEL.get(
      `threat-intel/${trivariateHash}.json`
    );
    if (object) {
      const data = await object.text();

      // 3. Populate KV cache
      await env.SX9_KV_CACHE.put(`threat/${trivariateHash}`, data, {
        expirationTtl: 300, // 5 minutes
      });

      return new Response(data, {
        headers: { "Content-Type": "application/json", "X-Cache": "MISS" },
      });
    }

    return new Response("Not Found", { status: 404 });
  },
};
```

### 10.5 R2 Subscriber Service (Port 18127)

**Rust Implementation:**

```rust
pub struct R2SubscriberService {
    r2_client: R2Client,
    sled_cache: sled::Db,
    sync_interval: Duration,
    hash_engine: Arc<TrivariateHashEngine>,
}

impl R2SubscriberService {
    pub async fn sync_loop(&self) -> Result<()> {
        let mut interval = tokio::time::interval(self.sync_interval);

        loop {
            interval.tick().await;

            // 1. List changed objects since last sync
            let changed = self.r2_client.list_changed().await?;

            for obj in changed {
                // 2. Download object
                let bytes = self.r2_client.fetch(&obj.key).await?;

                // 3. Validate trivariate hash (RFC-9001)
                let hash = self.hash_engine.validate(&bytes)?;

                // 4. Store in Sled cache
                self.sled_cache.insert(obj.key.as_bytes(), bytes)?;

                tracing::info!("Synced: {} (hash: {})", obj.key, hash);
            }

            // 5. Publish sync completion to NATS
            self.nats.publish("sx9.cdn.r2.sync.completed", "ok").await?;
        }
    }
}
```

### 10.6 Performance Targets

| Metric             | Target     | Measurement              |
| ------------------ | ---------- | ------------------------ |
| KV Cache Hit Rate  | >80%       | Cloudflare Analytics     |
| R2 Fetch Latency   | <50ms      | ATLAS Daemon (18106)     |
| Sync Interval      | 5 min      | Configurable             |
| Waterfall Fallback | <20%       | Neural Mux (18107) stats |
| Hash Validation    | 100%       | Hash Engine (18105)      |
| Data Freshness     | <5 min lag | R2 Subscriber telemetry  |

### 10.7 NATS Integration

All CDN operations publish to NATS:

```
sx9.cdn.r2.sync.started      → R2 sync initiated
sx9.cdn.r2.sync.completed    → R2 sync finished (latency_ms)
sx9.cdn.r2.fetch.kv          → KV cache hit (key)
sx9.cdn.r2.fetch.r2          → R2 bucket fetch (key)
sx9.cdn.r2.fetch.fallback    → Waterfall fallback triggered (target)
sx9.cdn.chromadb.query       → Vector embedding query
sx9.cdn.error                → CDN error occurred
```

### 10.8 Security

- **Mutual TLS:** Between gateway services (18104/18127/18106)
- **R2 API Keys:** Rotated via Vault every 24h
- **Port Manager:** Crystal gate authorization before bind
- **Hash Validation:** Lightning QA (18109) verifies integrity
- **Signed URLs:** Cloudflare Worker generates time-limited URLs

---

## 11. References

- RFC-9001: Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9003: Operation Classifier & Escalation Logic
- RFC-9103: IAC Adaptive Infrastructure (planned)

---

## 10. Revision History

| Version | Date       | Changes                                                                                                                      |
| ------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------- |
| 1.0     | 2025-11-23 | Initial specification                                                                                                        |
| 1.1     | 2025-11-26 | Professional retitle; clarified Unicode extensions; added SX9 entity types; enhanced compliance matrix                       |
| 1.2     | 2025-12-14 | Added Cloudflare R2 CDN integration; waterfall data access pattern; R2 bucket schema; CDN service ports; performance targets |

---

**Status:** This specification is FINAL and supersedes all prior schema definitions.
