# RFC-9020 — Unified Interview Schema

**Version:** 1.0  
**Status:** Draft  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9000, RFC-9005, RFC-9011

---

## 1. Overview

This RFC defines the **Unified Interview Schema** for generating, storing, and querying structured interviews across CTAS components.

### 1.1 Interview Types

| Type | Purpose | Count Target |
|------|---------|--------------|
| **Node Interview** | Threat scenario analysis (CTAS tasks) | 164+ |
| **Crate Interview** | Rust crate capability documentation | 40+ |
| **Technique Interview** | MITRE ATT&CK technique deep-dives | 600+ |
| **Tool Interview** | Offensive/defensive tool profiles | 600+ |

---

## 2. Node Interview Schema

### 2.1 JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["interview_id", "node_number", "interview_type", "content"],
  "properties": {
    "interview_id": {
      "type": "string",
      "pattern": "^interview-node-[0-9]{3}-[0-9]{3}-[0-9]{3}$"
    },
    "node_number": {
      "type": "string",
      "pattern": "^[0-9]{3}-[0-9]{3}-[0-9]{3}$"
    },
    "interview_type": {
      "type": "string",
      "enum": ["node", "crate", "technique", "tool"]
    },
    "title": {
      "type": "string",
      "maxLength": 200
    },
    "content": {
      "type": "string",
      "minLength": 100,
      "maxLength": 5000
    },
    "mitre_techniques": {
      "type": "array",
      "items": {
        "type": "string",
        "pattern": "^T[0-9]{4}(\\.[0-9]{3})?$"
      }
    },
    "hd4_phase": {
      "type": "string",
      "enum": ["hunt", "detect", "disrupt", "disable", "dominate"]
    },
    "ptcc_primitives": {
      "type": "array",
      "items": {
        "type": "string",
        "pattern": "^0x[0-9A-F]{2}$"
      }
    },
    "generated_at": {
      "type": "string",
      "format": "date-time"
    },
    "model_used": {
      "type": "string"
    },
    "confidence_score": {
      "type": "number",
      "minimum": 0,
      "maximum": 1
    }
  }
}
```

### 2.2 Example Node Interview

```json
{
  "interview_id": "interview-node-001-001-001",
  "node_number": "001-001-001",
  "interview_type": "node",
  "title": "Initial Reconnaissance via Social Media OSINT",
  "content": "This task involves gathering intelligence from publicly available social media profiles to identify potential targets, their relationships, and behavioral patterns. The operator would use tools like Maltego, SpiderFoot, or manual analysis to collect data points including names, affiliations, travel patterns, and communication channels...",
  "mitre_techniques": ["T1592", "T1589", "T1591"],
  "hd4_phase": "hunt",
  "ptcc_primitives": ["0x00", "0x08", "0x10"],
  "generated_at": "2025-11-28T00:00:00Z",
  "model_used": "gemini-2.0-flash-exp",
  "confidence_score": 0.92
}
```

---

## 3. Crate Interview Schema

### 3.1 JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["interview_id", "crate_id", "interview_type", "content"],
  "properties": {
    "interview_id": {
      "type": "string",
      "pattern": "^interview-crate-[a-z0-9-]+$"
    },
    "crate_id": {
      "type": "string"
    },
    "crate_name": {
      "type": "string"
    },
    "interview_type": {
      "type": "string",
      "const": "crate"
    },
    "content": {
      "type": "string",
      "minLength": 100,
      "maxLength": 5000
    },
    "rfc_compliance": {
      "type": "array",
      "items": {
        "type": "string",
        "pattern": "^RFC-[0-9]{4}$"
      }
    },
    "bernoulli_zone": {
      "type": "string",
      "enum": ["A", "B", "C", "D"]
    },
    "capabilities": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "dependencies": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "port_allocation": {
      "type": "integer",
      "minimum": 1024,
      "maximum": 65535
    },
    "generated_at": {
      "type": "string",
      "format": "date-time"
    }
  }
}
```

### 3.2 Example Crate Interview

```json
{
  "interview_id": "interview-crate-ctas7-neural-mux",
  "crate_id": "ctas7-neural-mux",
  "crate_name": "ctas7-neural-mux",
  "interview_type": "crate",
  "content": "The Neural Mux crate provides ultra-low latency routing for CTAS cognitive operations. It implements the deterministic routing architecture specified in RFC-9004, targeting sub-250ns route lookups using lock-free DashMap structures...",
  "rfc_compliance": ["RFC-9004", "RFC-9002", "RFC-9001"],
  "bernoulli_zone": "A",
  "capabilities": [
    "Trivariate hash lookup",
    "Persona-aware routing",
    "HD4 phase selection",
    "Lock-free concurrent access"
  ],
  "dependencies": ["ctas7-foundation-core", "dashmap", "tokio"],
  "port_allocation": 18500,
  "generated_at": "2025-11-28T00:00:00Z"
}
```

---

## 4. Neo4j Graph Schema

### 4.1 Node Labels

```cypher
// Interview nodes
(:Interview:Node {
    id: STRING,           // interview-node-XXX-XXX-XXX
    node_number: STRING,  // XXX-XXX-XXX
    title: STRING,
    content: STRING,
    hd4_phase: STRING,
    generated_at: DATETIME,
    confidence_score: FLOAT
})

(:Interview:Crate {
    id: STRING,           // interview-crate-{crate_name}
    crate_id: STRING,
    crate_name: STRING,
    content: STRING,
    bernoulli_zone: STRING,
    generated_at: DATETIME
})

// Reference nodes
(:Technique {
    id: STRING,           // T1059, T1059.001
    name: STRING,
    tactic: STRING[],
    platforms: STRING[]
})

(:Component {
    id: STRING,           // ctas7-foundation-core
    name: STRING
})

(:RFC {
    id: STRING,           // RFC-9004
    title: STRING,
    status: STRING
})
```

### 4.2 Relationships

```cypher
// Interview relationships
(i:Interview:Node)-[:COVERS]->(t:Technique)
(i:Interview:Node)-[:USES_PRIMITIVE]->(p:Primitive)
(i:Interview:Node)-[:IN_PHASE]->(ph:HD4Phase)

(i:Interview:Crate)-[:DOCUMENTS]->(c:Component)
(i:Interview:Crate)-[:COMPLIES_WITH]->(r:RFC)
(i:Interview:Crate)-[:DEPENDS_ON]->(c2:Component)

// Cross-reference
(i:Interview:Node)-[:RELEVANT_TO]->(ic:Interview:Crate)
```

### 4.3 Index Creation

```cypher
// Performance indexes
CREATE INDEX interview_id FOR (i:Interview) ON (i.id);
CREATE INDEX interview_type FOR (i:Interview) ON (i.interview_type);
CREATE INDEX technique_id FOR (t:Technique) ON (t.id);
CREATE INDEX component_id FOR (c:Component) ON (c.id);

// Full-text search
CREATE FULLTEXT INDEX interview_content FOR (i:Interview) ON EACH [i.content, i.title];
```

---

## 5. Supabase Schema

### 5.1 Table: node_interviews

```sql
CREATE TABLE node_interviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    interview_id TEXT UNIQUE NOT NULL,
    node_number TEXT NOT NULL,
    title TEXT,
    content TEXT NOT NULL,
    mitre_techniques TEXT[],
    hd4_phase TEXT,
    ptcc_primitives TEXT[],
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    model_used TEXT,
    confidence_score NUMERIC(3,2),
    embedding_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_node_interviews_node_number ON node_interviews(node_number);
CREATE INDEX idx_node_interviews_hd4_phase ON node_interviews(hd4_phase);
CREATE INDEX idx_node_interviews_techniques ON node_interviews USING GIN(mitre_techniques);
```

### 5.2 Table: crate_interviews

```sql
CREATE TABLE crate_interviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    interview_id TEXT UNIQUE NOT NULL,
    crate_id TEXT NOT NULL,
    crate_name TEXT NOT NULL,
    content TEXT NOT NULL,
    rfc_compliance TEXT[],
    bernoulli_zone TEXT,
    capabilities TEXT[],
    dependencies TEXT[],
    port_allocation INTEGER,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    embedding_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_crate_interviews_crate_id ON crate_interviews(crate_id);
CREATE INDEX idx_crate_interviews_zone ON crate_interviews(bernoulli_zone);
CREATE INDEX idx_crate_interviews_rfcs ON crate_interviews USING GIN(rfc_compliance);
```

---

## 6. Generation Pipeline

### 6.1 Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    INTERVIEW GENERATION PIPELINE                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SOURCE DATA           GENERATION              STORAGE                      │
│  ───────────           ──────────              ───────                      │
│                                                                             │
│  ┌──────────┐         ┌──────────┐         ┌──────────┐                    │
│  │ Supabase │         │          │         │ Supabase │                    │
│  │ ctas_    │───────▶ │ Gemini   │───────▶ │ node_    │                    │
│  │ tasks    │         │ 2.0      │         │interviews│                    │
│  └──────────┘         │ Flash    │         └──────────┘                    │
│                       │          │                │                         │
│  ┌──────────┐         │ (or      │                ▼                         │
│  │ Cargo.   │───────▶ │ Vertex   │         ┌──────────┐                    │
│  │ toml     │         │ AI)      │───────▶ │ Neo4j    │                    │
│  │ files    │         │          │         │ Graph    │                    │
│  └──────────┘         └──────────┘         └──────────┘                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Generation Commands

```bash
# Node interviews
python generate_node_interviews.py --export-tasks
python generate_node_interviews.py --generate
python generate_node_interviews.py --upload

# Crate interviews
python crate_interview_generator.py --all
python neo4j_loader.py --crate-interviews output/crate_interviews/
```

---

## 7. Query Examples

### 7.1 Neo4j Queries

```cypher
// Find all interviews covering a specific technique
MATCH (i:Interview)-[:COVERS]->(t:Technique {id: "T1059"})
RETURN i.id, i.title, i.content

// Find crate interviews for a specific Bernoulli zone
MATCH (i:Interview:Crate {bernoulli_zone: "A"})
RETURN i.crate_name, i.capabilities

// Gap analysis: techniques without interview coverage
MATCH (t:Technique)
WHERE NOT EXISTS((t)<-[:COVERS]-(:Interview))
RETURN t.id, t.name AS "Uncovered Techniques"

// Cross-reference: which crates support which scenarios
MATCH (in:Interview:Node)-[:COVERS]->(t:Technique)
MATCH (ic:Interview:Crate)-[:DOCUMENTS]->(c:Component)
WHERE c.name CONTAINS "exploit"
RETURN in.title, ic.crate_name, COLLECT(t.id) AS techniques
```

### 7.2 Supabase Queries

```sql
-- Find interviews by technique
SELECT * FROM node_interviews
WHERE 'T1059' = ANY(mitre_techniques);

-- Find crates in Zone A that comply with RFC-9004
SELECT crate_name, capabilities
FROM crate_interviews
WHERE bernoulli_zone = 'A'
AND 'RFC-9004' = ANY(rfc_compliance);

-- Count interviews by HD4 phase
SELECT hd4_phase, COUNT(*) as count
FROM node_interviews
GROUP BY hd4_phase
ORDER BY count DESC;
```

---

## 8. Validation Requirements

### 8.1 Content Quality

| Metric | Threshold |
|--------|-----------|
| Minimum content length | 100 chars |
| Maximum content length | 5000 chars |
| MITRE technique coverage | >80% of tasks |
| Confidence score | >0.7 |

### 8.2 Schema Conformance

All interviews MUST:
- Have valid interview_id format
- Include required fields
- Reference valid MITRE technique IDs
- Specify HD4 phase for node interviews
- Specify Bernoulli zone for crate interviews

---

## 9. References

- RFC-9000: Synaptix9 Agnostic Core & Ontology
- RFC-9005: Unified Schema Specification
- RFC-9011: Threat Content Ingestion
- RFC-9100: Dual-Trivariate PTCC Integration

---

**End of RFC-9020**
