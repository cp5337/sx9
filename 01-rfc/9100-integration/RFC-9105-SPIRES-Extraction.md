# RFC-9105 — SPIRES/OntoGPT Semantic Extraction

**Version:** 1.1  
**Status:** Implementation Blueprint  
**Date:** December 14, 2025  
**Applies To:** ABE Core, Synaptix9, SX9 Gateway  
**Author:** CTAS Core Engineering Group  
**Supersedes:** RFC-9105 v1.0 (November 2025)  
**Dependencies:** RFC-9001, RFC-9005, RFC-9011, RFC-9012, RFC-9114, RFC-9200

---

## 1. Abstract

This RFC defines the **SPIRES** (Structured Prompt Interrogation and Recursive Extraction of Semantics) pipeline for zero-shot knowledge extraction using Vertex AI (Gemini 2.5 Flash).

The pipeline transforms unstructured documents and raw threat feeds into **LinkML-conformant JSON**, generating Nodes, Edges, and Features for the Neo4j/SurrealDB knowledge graph.

**Version 1.1 Updates:**

- Cloudflare R2 integration for extracted threat intelligence distribution
- Automatic upload of SPIRES outputs to R2 buckets
- RFC-9001 trivariate hash generation for all extracted entities
- Integration with SX9 Gateway R2 Subscriber (port 18127)

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
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                                 ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    STEP 5: R2 CDN DISTRIBUTION                      │   │
│  │  r2_uploader.py                                                     │   │
│  │  ┌───────────────────────────────────────────────────────────────┐ │   │
│  │  │ 1. Generate trivariate hash (RFC-9001)                        │ │   │
│  │  │ 2. Upload to R2: threat-intel/{hash}.json                     │ │   │
│  │  │ 3. Upload embeddings to R2: embeddings/{id}.bin               │ │   │
│  │  │ 4. Populate Cloudflare KV cache                               │ │   │
│  │  │ 5. Notify SX9 Gateway R2 Subscriber (NATS)                    │ │   │
│  │  └───────────────────────────────────────────────────────────────┘ │   │
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
        range: RuleType # sigma, yara, nuclei, wazuh
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

| Component   | Service Account                 | Permissions                  |
| ----------- | ------------------------------- | ---------------------------- |
| Cloud Run   | `cognetix-abe-summarization-sa` | `roles/aiplatform.user`      |
| GCS Access  | Same SA                         | `roles/storage.objectViewer` |
| Neo4j Write | Application credentials         | Bolt connection              |

---

## 8. R2 CDN Integration

### 8.1 R2 Uploader Implementation

```python
# r2_uploader.py

import boto3
from ctas7_foundation_core import trivariate_hash
import json

class R2Uploader:
    def __init__(self, r2_endpoint: str, access_key: str, secret_key: str):
        self.s3_client = boto3.client(
            's3',
            endpoint_url=r2_endpoint,
            aws_access_key_id=access_key,
            aws_secret_access_key=secret_key
        )
        self.bucket = "sx9-threat-intel"
        self.kv_client = CloudflareKVClient()
        self.nats_client = NATSClient()

    async def upload_spires_output(self, linkml_json: dict, embedding: np.ndarray):
        # 1. Generate trivariate hash (RFC-9001)
        triv_hash = trivariate_hash.generate(
            operation=linkml_json.get("technique_id", ""),
            context=json.dumps(linkml_json).encode(),
            nonce=os.urandom(16)
        )

        # 2. Upload JSON to R2
        json_key = f"threat-intel/{triv_hash}.json"
        self.s3_client.put_object(
            Bucket=self.bucket,
            Key=json_key,
            Body=json.dumps(linkml_json),
            ContentType="application/json",
            Metadata={
                "trivariate-hash": triv_hash,
                "rfc-compliance": "9001,9005,9105"
            }
        )

        # 3. Upload embedding to R2
        embedding_key = f"embeddings/threat/{triv_hash}.bin"
        self.s3_client.put_object(
            Bucket=self.bucket,
            Key=embedding_key,
            Body=embedding.tobytes(),
            ContentType="application/octet-stream"
        )

        # 4. Populate Cloudflare KV cache
        await self.kv_client.put(
            f"threat/{triv_hash}",
            json.dumps(linkml_json),
            ttl=300  # 5 minutes
        )

        # 5. Notify SX9 Gateway via NATS
        await self.nats_client.publish(
            "sx9.cdn.r2.upload",
            json.dumps({
                "trivariate_hash": triv_hash,
                "json_key": json_key,
                "embedding_key": embedding_key,
                "timestamp": datetime.utcnow().isoformat()
            })
        )

        return triv_hash
```

### 8.2 Integration with SPIRES Pipeline

Modified `neo4j_threat_loader.py`:

```python
async def process_extraction(linkml_json: dict, embedding: np.ndarray):
    # 1. Load to Neo4j (existing)
    await load_to_neo4j(linkml_json)

    # 2. Load to SurrealDB (existing)
    await load_to_surrealdb(linkml_json)

    # 3. Upload to R2 CDN (NEW)
    r2_uploader = R2Uploader(
        r2_endpoint=os.getenv("R2_ENDPOINT"),
        access_key=os.getenv("R2_ACCESS_KEY_ID"),
        secret_key=os.getenv("R2_SECRET_ACCESS_KEY")
    )

    triv_hash = await r2_uploader.upload_spires_output(linkml_json, embedding)

    logger.info(f"Uploaded to R2 CDN: {triv_hash}")
```

### 8.3 SX9 Gateway Integration

The R2 Subscriber Service (port 18127) automatically syncs SPIRES outputs:

```
SPIRES Pipeline → R2 Upload → NATS Notification
                                    ↓
                        SX9 Gateway R2 Subscriber
                                    ↓
                        Local Sled Cache
                                    ↓
                        Available for queries via Neural Mux
```

### 8.4 Performance Benefits

| Metric                | Without R2 | With R2    | Improvement    |
| --------------------- | ---------- | ---------- | -------------- |
| Global Access Latency | 200-500ms  | <50ms      | 4-10x faster   |
| Egress Costs          | $0.09/GB   | $0.00/GB   | 100% savings   |
| Cache Hit Rate        | N/A        | >80%       | New capability |
| Data Freshness        | Real-time  | <5 min lag | Acceptable     |

---

## 9. References

- RFC-9011: Threat Ingestion
- RFC-9012: GNN Embeddings & Training Fabric
- RFC-9200: SX9 Dev Center
- OntoGPT: https://github.com/monarch-initiative/ontogpt
- LinkML: https://linkml.io/

---

**End of RFC-9105**
