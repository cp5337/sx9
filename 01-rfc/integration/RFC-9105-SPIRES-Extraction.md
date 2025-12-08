# RFC-9105 — SPIRES/OntoGPT Semantic Extraction

**Version:** 1.0  
**Status:** Implementation Blueprint  
**Date:** November 2025  
**Applies To:** ABE Core, Synaptix9  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9011, RFC-9012, RFC-9200

---

## 1. Abstract

This RFC defines the **SPIRES** (Structured Prompt Interrogation and Recursive Extraction of Semantics) pipeline for zero-shot knowledge extraction using Vertex AI (Gemini 2.5 Flash).

The pipeline transforms unstructured documents and raw threat feeds into **LinkML-conformant JSON**, generating Nodes, Edges, and Features for the Neo4j/SurrealDB knowledge graph.

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    SPIRES EXTRACTION PIPELINE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SOURCE DOCUMENTS                                                           │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ Threat  │ │ ATT&CK  │ │ Sigma   │ │ Raw     │                           │
│  │ Reports │ │ Docs    │ │ Rules   │ │ Intel   │                           │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘                           │
│       └───────────┴───────────┴─────┬─────┘                                │
│                                     │                                       │
│                              GCS Upload                                     │
│                                     │                                       │
│                                     ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    STEP 1: TRIGGER                                  │   │
│  │  Pub/Sub Topic: doc-upload-topic                                    │   │
│  │  Input: GCS Path (source-documents/)                                │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                                 ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    STEP 2: SPIRES EXTRACTION                        │   │
│  │  ┌───────────────────────────────────────────────────────────────┐ │   │
│  │  │ ABE Summarization Service (Cloud Run)                         │ │   │
│  │  │                                                               │ │   │
│  │  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐      │ │   │
│  │  │  │ Unstructured│    │ LinkML      │    │ Gemini 2.5  │      │ │   │
│  │  │  │ Text        │───▶│ Schema      │───▶│ Flash       │      │ │   │
│  │  │  └─────────────┘    └─────────────┘    └──────┬──────┘      │ │   │
│  │  │                                               │              │ │   │
│  │  │                                               ▼              │ │   │
│  │  │                                      LinkML-JSON Output      │ │   │
│  │  └───────────────────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                                 ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    STEP 3: VECTOR PREPARATION                       │   │
│  │  Threat Vector Pipeline                                             │   │
│  │  Input: LinkML-JSON                                                 │   │
│  │  Output: 768-dim Embedding (Sentence Transformers)                  │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                                 ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    STEP 4: GRAPH PERSISTENCE                        │   │
│  │  neo4j_threat_loader.py                                             │   │
│  │  Output: Cypher/SurrealQL transactions                              │   │
│  │  Target: Neo4j / SurrealDB                                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. LinkML Schemas

### 3.1 ThreatTechnique Schema

Primary entity for H2 semantic graph:

```yaml
id: https://w3id.org/ctas7/threat_technique
name: ThreatTechnique

classes:
  ThreatTechnique:
    attributes:
      technique_id:
        description: MITRE ATT&CK technique ID (e.g., T1059)
        identifier: true
      name:
        range: string
      tactic:
        description: Associated tactic (e.g., execution)
        multivalued: true
      detection_methods:
        range: DetectionMethod
        multivalued: true
      mitigations:
        range: Mitigation
        multivalued: true

  DetectionMethod:
    attributes:
      name:
        range: string
      rule_type:
        range: RuleType  # sigma, yara, nuclei, wazuh
```

### 3.2 DetectionRule Schema

For Sigma/YARA rule ingestion:

```yaml
id: https://w3id.org/ctas7/detection_rule
name: DetectionRule

classes:
  DetectionRule:
    attributes:
      rule_id:
        range: string
        identifier: true
      title:
        range: string
      logsource:
        description: Category and product of log
      mitre_attack_refs:
        range: string
        multivalued: true
      detection_logic:
        description: Condition and selection fields
```

### 3.3 Tool Schema

For Kali/offensive tools:

```yaml
id: https://w3id.org/ctas7/tool
name: Tool

classes:
  Tool:
    attributes:
      name:
        range: string
        identifier: true
      package_name:
        range: string
      categories:
        range: string
        multivalued: true
      commands:
        range: string
        multivalued: true
      dependencies:
        range: string
        multivalued: true
```

---

## 4. Vertex AI Implementation

### 4.1 LLM Prompt Structure

```
SYSTEM INSTRUCTION:
You are the CTAS7 Semantic Extraction Agent. Your output MUST 
strictly adhere to the provided LinkML schema. Do not add 
narrative or explanation. The user will provide a raw threat report.

[LINKML SCHEMA]
... (YAML content of ThreatTechnique schema)

USER QUERY:
Based on the following raw document text, extract a ThreatTechnique entity.

[RAW DOCUMENT TEXT]
... (e.g., A paragraph describing T1059.001)
```

### 4.2 Output Processing

```python
# neo4j_threat_loader.py (excerpt)

for technique in extracted_data["techniques"]:
    # 1. Create Node (Technique)
    cypher_create_node = f"""
    CREATE (t:Technique {{
        id: '{technique['technique_id']}',
        name: '{technique['name']}',
        platforms: {technique['platforms']}
    }});
    """
    
    # 2. Create Edges (Relationships)
    for method in technique.get("detection_methods", []):
        cypher_create_edge = f"""
        MATCH (t:Technique {{id: '{technique['technique_id']}'}})
        MERGE (d:Rule {{name: '{method['name']}', type: '{method['rule_type']}'}})
        MERGE (t)-[:DETECTED_BY]->(d);
        """
```

---

## 5. GNN Embedding Generation

### 5.1 Vector Pipeline

1. **768-dim Vector**: Structured JSON → Sentence Transformer → embedding
2. **Graph Node Feature**: Embedding stored on Neo4j/SurrealDB node
3. **Matroid Input**: `ctas7-glaf-matroid-core` uses embeddings for rank calculation

### 5.2 Integration with H2

```
LinkML-JSON → Embedding → Neo4j Node Feature
                              │
                              ▼
                    Matroid Rank Calculation
                              │
                              ▼
                    H2 Semantic Score → OODA Orient
```

---

## 6. Deployment

### 6.1 Trigger Flow

```
GCS object finalization
        │
        ▼
Pub/Sub (doc-upload-topic)
        │
        ▼
Cloud Run (ABE Summarization Service)
        │
        ▼
Gemini 2.5 Flash API
```

### 6.2 Authentication

| Component | Service Account | Permissions |
|-----------|-----------------|-------------|
| Cloud Run | `cognetix-abe-summarization-sa` | `roles/aiplatform.user` |
| GCS Access | Same SA | `roles/storage.objectViewer` |
| Neo4j Write | Application credentials | Bolt connection |

---

## 7. References

- RFC-9011: Threat Ingestion
- RFC-9012: GNN Embeddings & Training Fabric
- RFC-9200: SX9 Dev Center
- OntoGPT: https://github.com/monarch-initiative/ontogpt
- LinkML: https://linkml.io/

---

**End of RFC-9105**
