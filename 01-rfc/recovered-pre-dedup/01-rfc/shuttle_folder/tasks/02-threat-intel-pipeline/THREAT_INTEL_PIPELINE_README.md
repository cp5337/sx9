# SX9 Threat Intelligence Pipeline

**RFC-9005 Unified Schema | Neon PostgreSQL | UI-Driven**

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        THREAT INTEL PIPELINE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         UI (CTAS Dashboard)                          │   │
│  │  ThreatIntelPipeline.tsx → [Start Pipeline ▶] → Progress Dashboard  │   │
│  └────────────────────────────────┬────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Firefly IAC API (Port 18300)                      │   │
│  │  POST /api/firefly/threat-intel/start → Background Job               │   │
│  │  GET  /api/firefly/threat-intel/status/:id → Progress Polling        │   │
│  └────────────────────────────────┬────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     master_pipeline.sh                               │   │
│  ├─────────────────────────────────────────────────────────────────────┤   │
│  │                                                                       │   │
│  │  Phase 1: DOWNLOAD (30-60 min)                                       │   │
│  │  ├── MITRE ATT&CK (700 techniques)                                   │   │
│  │  ├── Atomic Red Team (1,200 tests)                                   │   │
│  │  ├── Caldera (500 abilities)                                         │   │
│  │  ├── Nuclei (8,000 templates)                                        │   │
│  │  ├── Sigma (2,500 rules)                                             │   │
│  │  ├── Kali Tools (600 tools)                                          │   │
│  │  ├── LOLBAS/GTFOBins (500 binaries)                                  │   │
│  │  └── 20+ more sources...                                             │   │
│  │           ↓                                                           │   │
│  │  Phase 2: NORMALIZE (normalize_threat_intel.py)                      │   │
│  │  ├── Parse each source format                                        │   │
│  │  ├── Generate RFC-9001 trivariate hashes                             │   │
│  │  │   └── h1_operational = SCH + CUID + UUID (48 hex chars)          │   │
│  │  │   └── h2_semantic = hash(h1) (16 hex chars)                       │   │
│  │  ├── Assign Unicode runes (E000-E0FF)                                │   │
│  │  ├── Map HD4 phases (Hunt/Detect/Disrupt/Disable/Dominate)          │   │
│  │  └── Map PTCC primitives (0-31)                                      │   │
│  │           ↓                                                           │   │
│  │  Phase 3: CONVERT (rfc9005_converter.py)                             │   │
│  │  ├── Transform to unified entities table                             │   │
│  │  ├── Generate relationships                                          │   │
│  │  └── Create neon_seed.sql                                            │   │
│  │           ↓                                                           │   │
│  │  Phase 4: LOAD TO NEON                                               │   │
│  │  └── psql $DATABASE_URL -f neon_seed.sql                             │   │
│  │                                                                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         NEON PostgreSQL (RFC-9005)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  entities                          relationships                            │
│  ┌──────────────────────────┐      ┌──────────────────────────┐            │
│  │ id (UUID)                │      │ source_entity_id         │            │
│  │ trivariate_hash          │◄────►│ target_entity_id         │            │
│  │ sch_hash                 │      │ relationship_type        │            │
│  │ cuid                     │      │ confidence               │            │
│  │ unicode_address (U+E0xx) │      │ mapping_source           │            │
│  │ entity_type (tool/tech)  │      └──────────────────────────┘            │
│  │ source                   │                                               │
│  │ hd4_phase                │      views                                    │
│  │ ptcc_primitive           │      ┌──────────────────────────┐            │
│  │ type_extensions (JSONB)  │      │ v_tools                  │            │
│  └──────────────────────────┘      │ v_techniques             │            │
│                                    │ v_tool_technique_map     │            │
│  playbooks          atlas_nodes    │ v_stats                  │            │
│  iac_manifolds      exec_sessions  └──────────────────────────┘            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    ▼              ▼              ▼
             ┌──────────┐   ┌──────────┐   ┌──────────┐
             │   Sled   │   │  Sledis  │   │ SlotGraph│
             │  (cache) │   │ (pubsub) │   │  (graph) │
             └──────────┘   └──────────┘   └──────────┘
```

## Files

| File | Purpose |
|------|---------|
| `neon/schema_rfc9005.sql` | PostgreSQL schema (RFC-9005 compliant) |
| `normalize_threat_intel.py` | Parse sources, generate RFC-9001 hashes |
| `rfc9005_converter.py` | Convert to unified entities format |
| `master_pipeline.sh` | Orchestrator script |
| `firefly_threat_intel_api.py` | FastAPI backend for UI |
| `ThreatIntelPipeline.tsx` | React dashboard component |

## Quick Start

### 1. Apply Schema to Neon

```bash
export DATABASE_URL="postgresql://user:pass@host/db?sslmode=require"
psql $DATABASE_URL -f neon/schema_rfc9005.sql
```

### 2. Run Pipeline (CLI)

```bash
cd /Users/cp5337/Developer/sx9/tools/abe/iac

# Full pipeline (download + normalize + convert + load)
./master_pipeline.sh

# Skip download, use existing data
./master_pipeline.sh --skip-download

# Normalize + convert only (no load)
./master_pipeline.sh --normalize-only

# Just convert existing normalized data to RFC-9005
./master_pipeline.sh --convert-only
```

### 3. Run Pipeline (UI)

```bash
# Start Firefly API
pip install fastapi uvicorn
python3 firefly_threat_intel_api.py
# → Running on port 18300

# Add ThreatIntelPipeline.tsx to your CTAS UI
# → Click [Start Pipeline ▶]
```

## RFC-9005 Schema

All data uses the **unified `entities` table**:

```sql
-- Tools, techniques, tasks are all entity_type values
SELECT * FROM entities WHERE entity_type = 'tool';
SELECT * FROM entities WHERE entity_type = 'technique';

-- Relationships link entities
SELECT * FROM relationships WHERE relationship_type = 'covers_technique';

-- Convenience views
SELECT * FROM v_tools;
SELECT * FROM v_techniques;
SELECT * FROM v_tool_technique_map;
SELECT * FROM v_stats;
```

## RFC-9001 Hashing

Every entity gets trivariate hashes:

```
trivariate_hash = SCH-CUID-UUID (48 hex chars with dashes)
                  │    │    │
                  │    │    └── Murmur3-64 of name|source|idx
                  │    └─────── Murmur3-64 of name|timestamp
                  └──────────── Murmur3-64 of name|category|source

h2_semantic = Murmur3-64(trivariate_hash)
unicode_address = U+E0xx (derived from SCH)
```

## Data Flow

```
27 Sources
    │
    ▼
threat_content_fetcher.py  →  raw JSON/YAML/CSV
    │
    ▼
normalize_threat_intel.py  →  tools.json, techniques.json, tool_technique_map.json
    │                          (with RFC-9001 hashes)
    ▼
rfc9005_converter.py       →  entities.json, relationships.json, neon_seed.sql
    │                          (unified RFC-9005 format)
    ▼
psql                       →  Neon PostgreSQL
```

## Expected Output

| Metric | Count |
|--------|-------|
| Total Entities | ~15,000 |
| - Tools | ~12,000 |
| - Techniques | ~700 |
| - Playbooks | ~200 |
| Relationships | ~5,000 |
| Sources | 27 |

## Environment Variables

```bash
# Required for Phase 4 (auto-load)
export DATABASE_URL="postgresql://..."
# or
export NEON_DATABASE_URL="postgresql://..."

# Optional (for Supabase fallback)
export SUPABASE_URL="https://xxx.supabase.co"
export SUPABASE_KEY="eyJ..."
```

## Integration Points

| Service | Port | Purpose |
|---------|------|---------|
| Firefly IAC API | 18300 | UI backend |
| Kali Daemon | 18200 | Tool execution |
| ATLAS Daemon | 18106 | Cognitive processing |
| Neural Mux | 18107 | Routing |

## Next Steps

1. **Run pipeline** → Populate Neon with threat intel
2. **Start Kali daemon** → Execute tools from UI
3. **Wire CTAS UI** → Connect dashboard to Firefly API
4. **Configure sync** → Sled/Sledis/SlotGraph replication
