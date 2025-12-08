# RFC-9005 — Unified Schema Specification

**Version:** 1.1
**Status:** Final
**Date:** November 26, 2025
**Applies To:** CTAS-7.3.1, ABE, PLASMA/ATLAS Infrastructure
**Author:** CTAS Core Engineering Group
**Supersedes:** RFC-9005-Unfuck-The-Schemas.md (v1.0)

---

## Abstract

This specification establishes a consolidated schema architecture for the CTAS-7 ecosystem, eliminating fragmentation across multiple database systems and providing a single source of truth compliant with RFC-9001 (Trivariate Hashing), RFC-9002 (Unicode Routing), and RFC-9003 (Operation Classification).

---

## 1. Problem Statement

The CTAS-7 system exhibits schema fragmentation across multiple storage backends:

| Issue | Impact |
|-------|--------|
| Multiple overlapping databases | Supabase + SurrealDB + Sledis + GLAF |
| Unicode range conflicts | Inconsistent allocations across services |
| Schema duplication | Same concepts modeled differently |
| Mixed data types | JSONB vs structured fields vs string arrays |
| No migration strategy | No versioning or evolution path |
| Performance degradation | Multiple sync operations across systems |

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

### 2.2 Unicode Allocation (RFC-9002 Extension)

Per RFC-9002, the base allocation is U+E000–E9FF. This specification extends the allocation to support additional operational requirements:

| Range | Class | Purpose | Count | RFC |
|-------|-------|---------|-------|-----|
| `U+E000–E1FF` | A | Core Components | 512 | 9002 |
| `U+E200–E2FF` | B | CUID Slot Mapping | 256 | 9002 |
| `U+E300–E3FF` | C | Semantic Routing | 256 | 9002 |
| `U+E400–E6FF` | D | Neural Mux Operations | 768 | 9002 |
| `U+E700–E7FF` | — | Reserved (Future) | 256 | 9002 |
| `U+E800–E9FF` | E | Experimental/Research | 512 | 9002 |
| `U+EA00–EAFF` | F | IAC Infrastructure Triggers | 256 | 9005/9103 |
| `U+EB00–EBFF` | G | Escalation Tier Markers | 256 | 9005 |
| `U+EC00–EFFF` | H | EEI & Intelligence Patterns | 1024 | 9005 |

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

| Service | Migration Action |
|---------|------------------|
| Lightning QA | Query unified `entities` table |
| ATLAS Daemon | Use `atlas_nodes` table |
| IAC Controller | Use `iac_manifolds` table |
| Health Dashboard | Single schema queries |

---

## 7. RFC Compliance Matrix

| Requirement | RFC | Implementation | Status |
|-------------|-----|----------------|--------|
| Trivariate hash on all entities | 9001 | `trivariate_hash`, `sch_hash`, `cuid` columns | ✅ |
| Unicode addressing | 9002 | `unicode_address`, `unicode_class` columns | ✅ |
| Operation classification | 9003 | `operation_class`, `escalation_tier` columns | ✅ |
| GLAF relationships | 9005 | `relationships` table with typed links | ✅ |
| ATLAS cognitive nodes | 9005 | `atlas_nodes` table with tick tracking | ✅ |
| IAC manifold spawning | 9005/9103 | `iac_manifolds` table with triggers | ✅ |
| Extended Unicode (EA00-EFFF) | 9005 | Optional ranges documented | ✅ |

---

## 8. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Schema consolidation | 5 → 1 databases | Primary store count |
| Unicode compliance | 100% RFC-9002 base | Allocation validation |
| Query performance | <10ms p99 | Entity lookup latency |
| ATLAS tick rate | 1ms ± 50μs | Cognitive tick timing |
| Neural routing | <250ns | Mux latency measurement |
| IAC spawn time | <30s | Manifold initialization |

---

## 9. References

- RFC-9001: Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9003: Operation Classifier & Escalation Logic
- RFC-9103: IAC Adaptive Infrastructure (planned)

---

## 10. Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-23 | Initial specification |
| 1.1 | 2025-11-26 | Professional retitle; clarified Unicode extensions; added SX9 entity types; enhanced compliance matrix |

---

**Status:** This specification is FINAL and supersedes all prior schema definitions.
