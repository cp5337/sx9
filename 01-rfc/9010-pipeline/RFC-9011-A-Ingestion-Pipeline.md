# RFC-9011-A — Canonical Intelligence Ingestion Pipeline

**Version:** 1.0  
**Status:** Architecture Definition  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9011, RFC-9010, RFC-9012, RFC-9021, RFC-9023, RFC-9024  

---

## 1. Abstract

This document defines the **Canonical Intelligence Ingestion Pipeline**, which converts heterogeneous external threat data (MITRE ATT&CK, Nuclei templates, Sigma rules, etc.) into the standardized **SX9 DSL** (Domain Specific Language).

This process is critical for:
1. Generating the **Secondary Trivariate Hash (SCH-S)**
2. Populating the **Cognigraph GNN Fabric** for semantic convergence
3. Enabling **H2 scoring** via the GLAF Matroid Core

---

## 2. Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              CANONICAL INTELLIGENCE INGESTION PIPELINE                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  EXTERNAL SOURCES                                                           │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐              │
│  │ MITRE   │ │ Nuclei  │ │ Sigma   │ │ Caldera │ │ YARA    │              │
│  │ ATT&CK  │ │ Templ.  │ │ Rules   │ │ Ability │ │ Rules   │              │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘              │
│       │           │           │           │           │                    │
│       └───────────┴───────────┴─────┬─────┴───────────┘                    │
│                                     │                                       │
│                                     ▼                                       │
│  ════════════════════════════════════════════════════════════════════════  │
│  ║                    STAGE 1: THALAMIC FILTER                          ║  │
│  ║                    (Pre-processing / Zone A-B)                       ║  │
│  ════════════════════════════════════════════════════════════════════════  │
│                                     │                                       │
│       ┌─────────────────────────────┼─────────────────────────────┐        │
│       │                             │                             │        │
│       ▼                             ▼                             ▼        │
│  ┌─────────────┐           ┌─────────────┐           ┌─────────────┐      │
│  │ 1. FETCH    │           │ 2. NORMALIZE│           │ 3. CROSSWALK│      │
│  │ ─────────── │           │ ─────────── │           │ ─────────── │      │
│  │ Registry    │──────────▶│ Parser      │──────────▶│ TTP→PTCC    │      │
│  │ Client      │           │ Engine      │           │ Mapper      │      │
│  │             │           │             │           │             │      │
│  │ Raw YAML/   │           │ Canonical   │           │ Partial     │      │
│  │ JSON/STIX   │           │ Flat JSON   │           │ SX9 DSL     │      │
│  └─────────────┘           └─────────────┘           └──────┬──────┘      │
│                                                             │              │
│                                                             ▼              │
│                                                  ┌─────────────┐          │
│                                                  │ 4. VALIDATE │          │
│                                                  │ ─────────── │          │
│                                                  │ SCR/Entropy │          │
│                                                  │ Gate        │          │
│                                                  │             │          │
│                                                  │ Validated   │          │
│                                                  │ DSL         │          │
│                                                  └──────┬──────┘          │
│                                                         │                  │
│  ════════════════════════════════════════════════════════════════════════  │
│  ║                    STAGE 2: COGNIGRAPH ENGINE                        ║  │
│  ║                    (Synthesis / Zone C)                              ║  │
│  ════════════════════════════════════════════════════════════════════════  │
│                                                         │                  │
│       ┌─────────────────────────────┬─────────────────┬┴────────────┐     │
│       │                             │                 │             │     │
│       ▼                             ▼                 ▼             ▼     │
│  ┌─────────────┐           ┌─────────────┐   ┌─────────────┐ ┌──────────┐│
│  │ 5. IMPUTE   │           │ 6. HASH &   │   │ 7. EMBED    │ │8. PERSIST││
│  │ ─────────── │           │    ASSIGN   │   │ ─────────── │ │──────────││
│  │ Semantic    │──────────▶│ ─────────── │──▶│ GNN Fabric  │▶│ Storage  ││
│  │ Filler (SFE)│           │ Trivariate  │   │             │ │ Writer   ││
│  │             │           │ Engine      │   │ 768-dim     │ │          ││
│  │ Complete    │           │             │   │ Vector      │ │ Supabase/││
│  │ SX9 DSL     │           │ SCH-S Hash  │   │             │ │ SurrealDB││
│  └─────────────┘           └─────────────┘   └─────────────┘ └──────────┘│
│                                                                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Stage 1: Thalamic Filter (Pre-processing)

This stage normalizes and quality-gates raw data before expensive AI processing.

### 3.1 Step 1: Fetch (Registry Client)

| Property | Value |
|----------|-------|
| **Input** | External API / GitHub URLs |
| **Output** | Raw YAML/JSON/STIX |
| **Purpose** | Download threat catalogs |

**Supported Sources:**

| Source | URL Pattern | Format |
|--------|-------------|--------|
| MITRE ATT&CK | `cti-taxii.mitre.org` | STIX 2.1 |
| Nuclei Templates | `github.com/projectdiscovery/nuclei-templates` | YAML |
| Sigma Rules | `github.com/SigmaHQ/sigma` | YAML |
| Caldera Abilities | `github.com/mitre/caldera` | YAML |
| Atomic Red Team | `github.com/redcanaryco/atomic-red-team` | YAML |
| YARA Rules | `github.com/Yara-Rules/rules` | YARA |

### 3.2 Step 2: Normalize (Parser Engine)

| Property | Value |
|----------|-------|
| **Input** | Raw heterogeneous data |
| **Output** | Canonical Flat JSON |
| **Purpose** | Uniform structure for all formats |

**Canonical JSON Schema:**

```json
{
  "source_id": "T1003.001",
  "source_type": "MITRE_ATTACK",
  "name": "LSASS Memory",
  "description": "Adversaries may attempt to access credential material...",
  "tactics": ["credential-access"],
  "platforms": ["windows"],
  "data_sources": ["Process: Process Access"],
  "detection": "Monitor for unexpected processes...",
  "references": ["https://attack.mitre.org/techniques/T1003/001"],
  "raw_content": "..."
}
```

### 3.3 Step 3: Crosswalk (TTP→PTCC Mapper)

| Property | Value |
|----------|-------|
| **Input** | Canonical JSON |
| **Output** | Partial SX9 DSL |
| **Purpose** | Map external TTPs to universal primitives |

**Crosswalk Mapping Table:**

| Source TTP (External) | PTCC Primitive (Universal) | HD4 Phase |
|-----------------------|----------------------------|-----------|
| MITRE T1003 (Credential Access) | `AUTHENTICATE` (0x10) | Disable |
| MITRE T1059 (Command Execution) | `EXECUTE` (0x08) | Disrupt |
| MITRE T1041 (Exfiltration) | `SEND` (0x04) | Dominate |
| Nuclei CVE-2023-XXXX | `CONNECT` (0x0C) | Detect |
| Caldera Exfil Data | `SEND` (0x04) | Dominate |
| Industrial Override Valve | `UPDATE` (0x02) | Disable |
| Sigma Process Creation | `CREATE` (0x01) | Hunt |
| YARA Malware Match | `READ` (0x03) | Detect |

**Partial SX9 DSL Output:**

```yaml
sx9_partial:
  source_ref: "T1003.001"
  ptcc_primitive: 0x10  # AUTHENTICATE
  hd4_phase: "DISABLE"
  confidence: 0.85
  domains:
    - CYBER
    - KINETIC
  requires_imputation:
    - entity_type
    - analysis_algorithm
    - semantic_context_tokens
```

### 3.4 Step 4: Validate (SCR/Entropy Gate)

| Property | Value |
|----------|-------|
| **Input** | Partial DSL |
| **Output** | Validated DSL |
| **Purpose** | Semantic conflict resolution, entropy bounds check |

**Validation Rules:**

```rust
pub struct SemanticConflictResolver {
    existing_definitions: HashSet<u64>,
}

impl SemanticConflictResolver {
    /// Check for semantic overlap with existing definitions
    pub fn validate(&self, partial_dsl: &PartialSx9Dsl) -> ValidationResult {
        // 1. Check PTCC entropy bounds (RFC-9100)
        if !self.check_ptcc_entropy_bounds(&partial_dsl.ptcc_primitive) {
            return ValidationResult::Rejected("PTCC entropy out of bounds");
        }
        
        // 2. Check for duplicate definitions
        let semantic_hash = partial_dsl.compute_semantic_hash();
        if self.existing_definitions.contains(&semantic_hash) {
            return ValidationResult::Duplicate(semantic_hash);
        }
        
        // 3. Check HD4 phase consistency
        if !self.validate_hd4_consistency(&partial_dsl) {
            return ValidationResult::Rejected("HD4 phase inconsistent with PTCC");
        }
        
        ValidationResult::Valid
    }
}
```

---

## 4. Stage 2: Cognigraph Engine (Synthesis)

This stage performs deep semantic analysis and persistence.

### 4.1 Step 5: Impute (Semantic Filler/SFE)

| Property | Value |
|----------|-------|
| **Input** | Validated DSL |
| **Output** | Complete SX9 DSL |
| **Purpose** | Fill missing relationships using GNN + Phi-3 |

**Required Fields for SCH-S Generation:**

| Field | Source/Derivation | Purpose |
|-------|-------------------|---------|
| `entity_type` | Imputed by SFE | Primary filter (THREAT_ACTOR, TOOL, etc.) |
| `analysis_algorithm` | Assigned by SFE | Directs GLAF processor (TETH-ENTROPY, k-NN) |
| `semantic_context_tokens` | GNN embedding clustering | Forms bulk of SCH-S hash material |
| `unicode_tail` | Reserved PUA space (RFC-9002) | Behavioral context annotation |

**SFE Implementation:**

```rust
pub struct SemanticFillerEngine {
    gnn_client: GnnFabricClient,
    phi3_client: Phi3InferenceClient,
}

impl SemanticFillerEngine {
    pub async fn impute(&self, validated_dsl: ValidatedSx9Dsl) -> Result<CompleteSx9Dsl> {
        // 1. Get GNN embedding for context
        let embedding = self.gnn_client.embed(&validated_dsl.content).await?;
        
        // 2. Cluster to find semantic context tokens
        let context_tokens = self.gnn_client.cluster_tokens(&embedding).await?;
        
        // 3. Use Phi-3 to infer missing fields
        let inference = self.phi3_client.infer_missing_fields(&validated_dsl).await?;
        
        // 4. Assign analysis algorithm based on entity type
        let analysis_algo = match inference.entity_type {
            EntityType::ThreatActor => AnalysisAlgorithm::TethEntropy,
            EntityType::Tool => AnalysisAlgorithm::KNearestNeighbor,
            EntityType::Technique => AnalysisAlgorithm::MatroidRank,
            _ => AnalysisAlgorithm::Default,
        };
        
        // 5. Assign Unicode tail from PUA space
        let unicode_tail = self.assign_unicode_tail(&inference)?;
        
        Ok(CompleteSx9Dsl {
            base: validated_dsl,
            entity_type: inference.entity_type,
            analysis_algorithm: analysis_algo,
            semantic_context_tokens: context_tokens,
            unicode_tail,
            confidence: inference.confidence,
        })
    }
}
```

### 4.2 Step 6: Hash & Assign (Trivariate Engine)

| Property | Value |
|----------|-------|
| **Input** | Complete DSL |
| **Output** | Dual-Trivariate Hash (SCH-T + SCH-S) |
| **Purpose** | Generate canonical hash identifiers |

**Hash Generation:**

```rust
impl TrivariateEngine {
    pub fn generate_dual_hash(&self, dsl: &CompleteSx9Dsl) -> DualTrivariateHash {
        // Primary Hash (SCH-T): Tactical identifier
        let sch_t = self.compute_primary_hash(
            &dsl.ptcc_primitive,
            &dsl.hd4_phase,
            &dsl.source_ref,
        );
        
        // Secondary Hash (SCH-S): Semantic context
        let sch_s = self.compute_secondary_hash(
            &dsl.entity_type,
            &dsl.semantic_context_tokens,
            &dsl.analysis_algorithm,
        );
        
        DualTrivariateHash {
            sch_t,
            sch_s,
            unicode_tail: dsl.unicode_tail,
            generated_at: Utc::now(),
        }
    }
}
```

### 4.3 Step 7: Embed (GNN Fabric)

| Property | Value |
|----------|-------|
| **Input** | Complete DSL |
| **Output** | 768-dimensional embedding vector |
| **Purpose** | Semantic vector for Matroid H2 analysis |

**Embedding Specification:**

```rust
pub struct GnnEmbedding {
    /// 768-dimensional vector (RFC-9012)
    pub vector: [f64; 768],
    
    /// Model version for reproducibility
    pub model_version: String,
    
    /// Timestamp for cache invalidation
    pub generated_at: DateTime<Utc>,
}

impl GnnFabricClient {
    pub async fn embed(&self, content: &str) -> Result<GnnEmbedding> {
        // Uses sentence-transformers model
        let vector = self.model.encode(content).await?;
        
        Ok(GnnEmbedding {
            vector,
            model_version: self.model_version.clone(),
            generated_at: Utc::now(),
        })
    }
}
```

### 4.4 Step 8: Persist (Storage Writer)

| Property | Value |
|----------|-------|
| **Input** | Dual-Trivariate + Embedding |
| **Output** | Stored artifact |
| **Storage** | Supabase (PostgreSQL) / SurrealDB |

**Storage Schema (RFC-9005 Unified Schema):**

```sql
-- Supabase table for ingested artifacts
CREATE TABLE sx9_artifacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Trivariate Hashes
    sch_t_prefix VARCHAR(32) NOT NULL,  -- Retrieval key
    sch_t_full BYTEA NOT NULL,
    sch_s_full BYTEA NOT NULL,
    unicode_tail VARCHAR(4),
    
    -- DSL Content
    source_ref VARCHAR(64) NOT NULL,
    source_type VARCHAR(32) NOT NULL,
    ptcc_primitive SMALLINT NOT NULL,
    hd4_phase VARCHAR(16) NOT NULL,
    entity_type VARCHAR(32) NOT NULL,
    analysis_algorithm VARCHAR(32) NOT NULL,
    
    -- Embedding (stored as vector type for pgvector)
    embedding vector(768) NOT NULL,
    
    -- Metadata
    confidence REAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Indexes
    CONSTRAINT unique_sch_t UNIQUE (sch_t_prefix)
);

-- Index for fast H2 retrieval by SCH prefix
CREATE INDEX idx_sch_t_prefix ON sx9_artifacts(sch_t_prefix);

-- Vector similarity index for semantic search
CREATE INDEX idx_embedding ON sx9_artifacts 
    USING ivfflat (embedding vector_cosine_ops);
```

---

## 5. H2 Convergence Payload Generation

The pipeline output must contain all data points required for H2ConvergencePayload (RFC-9024 §2):

```rust
impl From<StoredArtifact> for H2ConvergencePayload {
    fn from(artifact: StoredArtifact) -> Self {
        H2ConvergencePayload {
            semantic_score: artifact.confidence as f64,
            matroid_rank_current: 0,  // Set by GLAF Matroid Core
            matroid_rank_delta: 0,     // Set by GLAF Matroid Core
            estimated_adversary_phase: artifact.infer_adversary_phase(),
            secondary_hash_sch: hex::encode(&artifact.sch_s_full),
            generated_at_utc: artifact.created_at,
            recommended_hd4_phase: artifact.hd4_phase.into(),
        }
    }
}
```

---

## 6. Retrieval Path

The ATLAS Daemon retrieves artifacts using the **SCH Prefix** as the lookup key:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         H2 RETRIEVAL PATH                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ATLAS Daemon                                                               │
│  (Zone B)                                                                   │
│      │                                                                      │
│      │  1. Extract SCH Prefix from Primary Hash                            │
│      │     sch_prefix = primary_hash.sch_t >> 64                           │
│      │                                                                      │
│      ▼                                                                      │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  GET /api/v7/convergence/semantic/{sch_prefix}                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│      │                                                                      │
│      ▼                                                                      │
│  GLAF Matroid Core                                                          │
│  (Zone C)                                                                   │
│      │                                                                      │
│      │  2. Lookup artifact by SCH Prefix                                   │
│      │     SELECT * FROM sx9_artifacts WHERE sch_t_prefix = $1             │
│      │                                                                      │
│      │  3. Calculate matroid rank against current context                  │
│      │     rank = matroid.calculate_rank(context_indices)                  │
│      │                                                                      │
│      │  4. Return H2ConvergencePayload                                     │
│      │                                                                      │
│      ▼                                                                      │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  H2ConvergencePayload {                                              │  │
│  │    semantic_score: 0.87,                                             │  │
│  │    matroid_rank_current: 5,                                          │  │
│  │    matroid_rank_delta: 1,                                            │  │
│  │    estimated_adversary_phase: "EXECUTION",                           │  │
│  │    recommended_hd4_phase: "DISRUPT"                                  │  │
│  │  }                                                                   │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Implementation Status

| Component | Status | Crate |
|-----------|--------|-------|
| Registry Client (Fetch) | 🟡 80% | `ctas7-threat-registry` |
| Parser Engine (Normalize) | 🟡 60% | `ctas7-parser-engine` |
| Crosswalk Engine | 🔴 20% | `ctas7-crosswalk` |
| SCR Validator | 🔴 30% | `ctas7-scr-gate` |
| Semantic Filler (SFE) | 🔴 10% | `ctas7-sfe` |
| Trivariate Engine | 🟢 100% | `ctas7-foundation-core` |
| GNN Fabric | 🟡 50% | `ctas7-gnn-fabric` |
| Storage Writer | 🟢 90% | `ctas7-foundation-data` |

---

## 8. References

- RFC-9010: Enterprise Extraction (SPIRES/OntoGPT)
- RFC-9011: Threat Ingestion (source catalog)
- RFC-9012: GNN Embeddings & Training Fabric
- RFC-9021: Cognitive Inference Engine
- RFC-9023: GLAF Matroid Convergence Mathematics
- RFC-9024: H2 Convergence Service Contract

---

## 9. ATL-Physical Domain Integration

### 9.1 Physical Threat Domain Overview

In addition to cyber threat intelligence, the pipeline supports **ATL-Physical** (Adversary Task List - Physical Domain) for kinetic threat modeling. This enables cross-domain pattern recognition between cyber and physical attack chains.

**Source Data:** TTL (Terrorist IED Task List) decomposition from NCTC/DHS publications.

### 9.2 ATL-Physical Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              ATL-PHYSICAL INGESTION PIPELINE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SOURCE DATA                                                                │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ctas-dir/20-atl/physical/data/atl_physical_ied.yaml                │   │
│  │  - 114 adversary tasks                                               │   │
│  │  - 141 relationships                                                 │   │
│  │  - HD4 phase mappings                                                │   │
│  │  - Interdiction points                                               │   │
│  │  - Mundanity scores                                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                          │                                                  │
│                          ▼                                                  │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  STAGE 1: NEO4J CONTAINER (neo4j-atl-physical)                       │  │
│  │  ─────────────────────────────────────────────────────────────────── │  │
│  │  Port 7475 (browser) / Port 7688 (bolt)                              │  │
│  │  Password: atl_physical_graph                                         │  │
│  │                                                                       │  │
│  │  Nodes (all labeled :ATLPhysical for future merge):                  │  │
│  │  - :AdversaryTask (157)                                              │  │
│  │  - :HD4Phase (5: HUNT, DETECT, DISABLE, DISRUPT, DOMINATE)          │  │
│  │  - :ThreatModality (IED, ActiveShooter, Intrusion, etc.)            │  │
│  │                                                                       │  │
│  │  Relationships:                                                       │  │
│  │  - :MAPS_TO_HD4 (task → phase mapping)                               │  │
│  │  - :SUBTASK_OF (task hierarchy)                                      │  │
│  │  - :RELATED_TO (cross-references)                                    │  │
│  │  - :HAS_MODALITY (threat type classification)                        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                          │                                                  │
│                          ▼                                                  │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  STAGE 2: INTERVIEW GENERATION                                       │  │
│  │  ─────────────────────────────────────────────────────────────────── │  │
│  │  atl_physical_interview_generator.py                                 │  │
│  │                                                                       │  │
│  │  Output:                                                              │  │
│  │  - 156 RFC-9025 compliant interviews                                 │  │
│  │  - :ATLPhysical:Interview nodes linked via :HAS_INTERVIEW            │  │
│  │  - :ATLPhysical:Indicator nodes (2,285 extracted indicators)         │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                          │                                                  │
│                          ▼                                                  │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  STAGE 3: TRAINING DATA GENERATION                                   │  │
│  │  ─────────────────────────────────────────────────────────────────── │  │
│  │  leptose_training_prep.py + threat_vector_pipeline.py                │  │
│  │                                                                       │  │
│  │  Output:                                                              │  │
│  │  - Phi-3 LoRA samples (227 ATL-Physical specific)                    │  │
│  │  - HD4 classification samples (171)                                   │  │
│  │  - Alpaca format samples (114)                                        │  │
│  │  - ChromaDB embeddings (255 documents)                               │  │
│  │                                                                       │  │
│  │  NOTE: ATL-Physical is INVISIBLE to operational queries but          │  │
│  │        INCLUDED in training for cross-domain pattern recognition     │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.3 ATL-Physical Specific Fields

| Field | Description | Values |
|-------|-------------|--------|
| `phase` | Attack chain phase (1-6) | 1=Planning, 2=Acquisition, 3=Assembly, 4=Movement, 5=Emplacement, 6=Execution |
| `classification` | Task criticality | MANDATORY, DESIRABLE, OPTIONAL |
| `is_interdiction_point` | Left-of-Bang opportunity | true/false |
| `is_key_indicator` | Observable behavior | true/false |
| `mundanity_score` | How "normal" activity appears | 0.0 (suspicious) - 1.0 (mundane) |
| `node_form` | Actor configuration | 1n (lone), 2n (cell), 3n (network) |
| `modality` | Threat type | IED, ActiveShooter, Intrusion, CBRN |

### 9.4 Merge Strategy

The `:ATLPhysical` label enables future merge into the operational graph:

```cypher
-- Export from ATL-Physical container
CALL apoc.export.cypher.query(
  "MATCH (n:ATLPhysical) RETURN n",
  "atl_physical_export.cypher",
  {}
)

-- Import to operational with label preserved
CALL apoc.cypher.runFile("atl_physical_export.cypher")

-- Query cross-domain patterns
MATCH (cyber:Technique)-[:MAPS_TO_HD4]->(phase:HD4Phase)
MATCH (physical:ATLPhysical:AdversaryTask)-[:MAPS_TO_HD4]->(phase)
RETURN phase.phase_name, collect(cyber.name), collect(physical.title)
```

### 9.5 Implementation Files

| File | Purpose |
|------|---------|
| `neo4j_atl_physical_loader.py` | Load YAML to Neo4j with :ATLPhysical labels |
| `atl_physical_interview_generator.py` | Generate RFC-9025 interviews for physical domain |
| `leptose_training_prep.py` | Include ATL-Physical in training data |
| `threat_vector_pipeline.py` | Embed ATL-Physical to ChromaDB |

---

**End of RFC-9011-A**
