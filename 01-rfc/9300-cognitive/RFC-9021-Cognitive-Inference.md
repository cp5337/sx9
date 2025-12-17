# RFC-9021 — Cognitive Inference Engine

**Version:** 1.0  
**Status:** Draft  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9004, RFC-9012, RFC-9020

---

## 1. Overview

This RFC defines the **Cognitive Inference Engine** architecture for CTAS, implementing a biologically-inspired thinking system with:

1. **Thalamic Filter** - Rapid pre-classification (<10ms) for signal gating
2. **Vector Search** - Semantic similarity via ChromaDB/pgvector
3. **Graph Reasoning** - Relationship traversal via Neo4j
4. **Generative Inference** - LoRA-tuned Phi-3 for response generation

### 1.1 Design Philosophy

The system mimics mammalian cognitive architecture:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    BIOLOGICAL ANALOGY                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BIOLOGICAL                           CTAS IMPLEMENTATION                   │
│  ──────────                           ───────────────────                   │
│                                                                             │
│  Thalamus (sensory gating)    ───▶    DistilBERT Thalamic Filter           │
│  Hippocampus (memory)         ───▶    ChromaDB Vector Store                │
│  Prefrontal Cortex (reasoning)───▶    Neo4j Graph Traversal                │
│  Neocortex (generation)       ───▶    Phi-3 LoRA Model                     │
│                                                                             │
│  Reflexive Response           ───▶    Fast path (bypass full processing)   │
│  Deliberate Thought           ───▶    Full cognitive pipeline              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. System Architecture

### 2.1 Component Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    COGNITIVE INFERENCE ENGINE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  INPUT                                                                      │
│  ──────                                                                     │
│  ┌──────────────┐                                                          │
│  │ User Query   │                                                          │
│  │ "How would   │                                                          │
│  │  APT29 move  │                                                          │
│  │  laterally?" │                                                          │
│  └──────┬───────┘                                                          │
│         │                                                                   │
│         ▼                                                                   │
│  LAYER 1: THALAMIC FILTER (DistilBERT LoRA)                                │
│  ───────────────────────────────────────────                               │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │  Input ──▶ DistilBERT ──▶ ┌─────────────────────────────────────┐   │  │
│  │                    (<10ms) │ gate_decision: full_processing     │   │  │
│  │                            │ pathway: threat_analysis           │   │  │
│  │                            │ priority: high                     │   │  │
│  │                            │ activated_domains: [apt, lateral]  │   │  │
│  │                            │ pre_fetch: ["G0016", "T1021"]      │   │  │
│  │                            └─────────────────────────────────────┘   │  │
│  │                                                                      │  │
│  │  Gate Decision:                                                      │  │
│  │  ┌────────────────┐    ┌────────────────┐                           │  │
│  │  │   reflexive    │    │ full_processing│                           │  │
│  │  │   (fast path)  │    │ (continue)     │                           │  │
│  │  └───────┬────────┘    └───────┬────────┘                           │  │
│  │          │                     │                                     │  │
│  │          ▼                     ▼                                     │  │
│  │    Direct Response       Continue Pipeline                           │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                      │                                      │
│                                      ▼                                      │
│  LAYER 2: MEMORY RETRIEVAL (Parallel)                                      │
│  ─────────────────────────────────────                                     │
│  ┌────────────────────────┐    ┌────────────────────────┐                 │
│  │ VECTOR SEARCH          │    │ GRAPH TRAVERSAL        │                 │
│  │ (ChromaDB)             │    │ (Neo4j)                │                 │
│  │ ────────────────       │    │ ────────────────       │                 │
│  │                        │    │                        │                 │
│  │ Query embedding ──▶    │    │ Pre-fetched IDs ──▶    │                 │
│  │ Semantic similarity    │    │ Relationship walk      │                 │
│  │ Top-K results          │    │ Context expansion      │                 │
│  │                        │    │                        │                 │
│  │ Latency: <50ms         │    │ Latency: <100ms        │                 │
│  └───────────┬────────────┘    └───────────┬────────────┘                 │
│              │                              │                              │
│              └──────────────┬───────────────┘                              │
│                             │                                              │
│                             ▼                                              │
│  LAYER 3: CONTEXT ASSEMBLY                                                 │
│  ─────────────────────────                                                 │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │  Assembled Context:                                                  │  │
│  │  ┌─────────────────────────────────────────────────────────────┐    │  │
│  │  │ Relevant Techniques: T1021.001, T1021.002, T1570, T1563     │    │  │
│  │  │ Detection Rules: sigma/lateral_movement_*, wazuh/rdp_*      │    │  │
│  │  │ Known Actors: APT29 (G0016), Cozy Bear                      │    │  │
│  │  │ Related Interviews: interview-node-003-002-001, ...         │    │  │
│  │  │ Tools: PsExec, WinRM, RDP, SMB                              │    │  │
│  │  └─────────────────────────────────────────────────────────────┘    │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                      │                                      │
│                                      ▼                                      │
│  LAYER 4: GENERATIVE INFERENCE (Phi-3 LoRA)                                │
│  ──────────────────────────────────────────                                │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │  System Prompt + Context + Query ──▶ Phi-3 LoRA ──▶ Response        │  │
│  │                                                                      │  │
│  │  Latency: <500ms                                                    │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                      │                                      │
│                                      ▼                                      │
│  OUTPUT                                                                    │
│  ──────                                                                    │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │ "APT29 typically uses several lateral movement techniques:           │  │
│  │  1. Remote Desktop Protocol (T1021.001) - often with stolen creds   │  │
│  │  2. Windows Remote Management (T1021.006) - PowerShell remoting     │  │
│  │  3. SMB/Windows Admin Shares (T1021.002) - PsExec variants          │  │
│  │                                                                      │  │
│  │  Detection: Monitor for Event IDs 4624/4625 (logon), Sysmon 3..."   │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Latency Budget

| Layer | Component | Target | P95 |
|-------|-----------|--------|-----|
| 1 | Thalamic Filter | <10ms | <15ms |
| 2a | Vector Search | <50ms | <75ms |
| 2b | Graph Traversal | <100ms | <150ms |
| 3 | Context Assembly | <10ms | <15ms |
| 4 | Phi-3 Inference | <400ms | <500ms |
| **Total** | End-to-end | <570ms | <700ms |

---

## 3. Thalamic Filter (Layer 1)

### 3.1 Purpose

The thalamic filter performs rapid pre-classification to:

1. **Gate signals** - Determine if full cognitive processing is needed
2. **Select pathways** - Route to appropriate processing pipeline
3. **Score priority** - Urgency classification for attention allocation
4. **Prime context** - Pre-activate relevant knowledge domains

### 3.2 Model Architecture

```
Model: DistilBERT + LoRA adapter
Base: distilbert-base-uncased
LoRA config:
  r: 8
  alpha: 16
  target_modules: [q_lin, k_lin, v_lin]
  
Output heads:
  - gate_decision: binary (reflexive | full_processing)
  - pathway: multi-class (threat_analysis | operational | informational | creative)
  - priority: ordinal (low | medium | high | critical)
  - domains: multi-label (apt_attribution, technique_mapping, detection, ...)
```

### 3.3 Training Data Format

```json
{
  "text": "APT29 used PowerShell encoded commands for lateral movement",
  "thalamic_output": {
    "gate_decision": "full_processing",
    "pathway": "threat_analysis",
    "priority": "high",
    "activated_domains": ["apt_attribution", "technique_mapping", "lateral_movement"],
    "pre_fetched_context": ["G0016", "T1059.001", "T1021"]
  }
}
```

### 3.4 Gating Logic

```python
class ThalamicFilter:
    def process(self, query: str) -> ThalamicOutput:
        # Fast inference (<10ms)
        output = self.model(query)
        
        if output.gate_decision == "reflexive":
            # Bypass full pipeline
            return self.fast_path_response(query, output)
        else:
            # Continue to full cognitive processing
            return ThalamicOutput(
                gate="full_processing",
                pathway=output.pathway,
                priority=output.priority,
                activated_domains=output.domains,
                pre_fetch_ids=self.extract_entity_ids(query, output.domains)
            )
```

---

## 4. Memory Retrieval (Layer 2)

### 4.1 Vector Search (ChromaDB)

**Collections:**

| Collection | Content | Embedding Model |
|------------|---------|-----------------|
| `techniques` | MITRE ATT&CK descriptions | all-MiniLM-L6-v2 |
| `detection_rules` | Sigma/YARA/Wazuh rules | all-MiniLM-L6-v2 |
| `tools` | Kali/LOLTL tool docs | all-MiniLM-L6-v2 |
| `interviews` | Node/Crate interviews | all-MiniLM-L6-v2 |
| `threat_reports` | CTI reports | all-MiniLM-L6-v2 |

**Query Example:**

```python
results = chroma_client.query(
    collection_name="techniques",
    query_texts=[user_query],
    n_results=10,
    where={"tactic": {"$in": activated_domains}}  # Filter by thalamic output
)
```

### 4.2 Graph Traversal (Neo4j)

**Query Templates:**

```cypher
// Technique expansion
MATCH (t:Technique {id: $tech_id})
OPTIONAL MATCH (t)-[:DETECTED_BY]->(r:DetectionRule)
OPTIONAL MATCH (t)<-[:USES]-(a:ThreatActor)
OPTIONAL MATCH (t)<-[:COVERS]-(i:Interview)
OPTIONAL MATCH (t)-[:SUBTECHNIQUE_OF]->(parent:Technique)
RETURN t, COLLECT(DISTINCT r) as rules, 
       COLLECT(DISTINCT a) as actors,
       COLLECT(DISTINCT i) as interviews,
       parent

// Actor profile
MATCH (a:ThreatActor {id: $actor_id})-[:USES]->(t:Technique)
OPTIONAL MATCH (t)-[:DETECTED_BY]->(r:DetectionRule)
RETURN a, COLLECT({technique: t, rules: COLLECT(r)}) as ttps

// Attack path
MATCH path = shortestPath(
    (start:Technique {tactic: "initial-access"})-[:FOLLOWED_BY*..5]->(end:Technique {tactic: "impact"})
)
WHERE all(n IN nodes(path) WHERE n.platforms CONTAINS "windows")
RETURN path
```

---

## 5. Generative Inference (Layer 4)

### 5.1 Phi-3 LoRA Configuration

```yaml
base_model: microsoft/Phi-3-mini-4k-instruct
lora:
  r: 16
  alpha: 32
  dropout: 0.05
  target_modules: [q_proj, k_proj, v_proj, o_proj, gate_proj, up_proj, down_proj]

training:
  epochs: 3
  batch_size: 4
  learning_rate: 2e-4
  gradient_accumulation_steps: 4
  warmup_ratio: 0.03
  max_seq_length: 4096

quantization:
  load_in_4bit: true
  bnb_4bit_compute_dtype: float16
  bnb_4bit_quant_type: nf4
```

### 5.2 Training Data Format (Alpaca)

```json
{
  "instruction": "Explain how an attacker would use T1021.002 (SMB/Windows Admin Shares) for lateral movement",
  "input": "Context: Windows domain environment, compromised service account credentials available",
  "output": "An attacker with compromised service account credentials would leverage SMB/Windows Admin Shares (T1021.002) for lateral movement through several methods:\n\n1. **PsExec or similar tools** - Using the admin$ share to copy and execute a service binary on remote systems\n2. **Direct SMB file copy** - Placing malicious payloads in accessible shares\n3. **WMI over SMB** - Executing commands via Windows Management Instrumentation\n\n**Detection indicators:**\n- Sysmon Event ID 1 (Process Creation) for PsExec variants\n- Windows Security Event ID 5145 (network share access)\n- Unusual lateral connections from non-admin workstations\n\n**Sigma rule:** `win_susp_psexec_usage.yml`\n\n**Mitigation:** Implement network segmentation, restrict admin share access, enable Windows Firewall rules to block SMB between workstations."
}
```

### 5.3 Inference Prompt Template

```python
SYSTEM_PROMPT = """You are a CTAS threat analyst assistant. You have access to:
- MITRE ATT&CK framework knowledge
- Detection rules (Sigma, YARA, Wazuh)
- Threat actor profiles and TTPs
- CTAS interview knowledge base

Provide accurate, actionable threat analysis. Always cite specific:
- Technique IDs (T1xxx)
- Detection rule references
- Threat actor identifiers (Gxxxx)
"""

def build_prompt(query: str, context: AssembledContext) -> str:
    return f"""{SYSTEM_PROMPT}

## Relevant Context

### Techniques
{context.techniques_summary}

### Detection Coverage
{context.detection_rules_summary}

### Related Threat Actors
{context.actors_summary}

### Interview Excerpts
{context.interview_excerpts}

## User Query
{query}

## Response
"""
```

---

## 6. Leptose Integration

### 6.1 Service Configuration

```toml
# leptose.toml
[server]
port = 18114
host = "0.0.0.0"
workers = 4

[models]
thalamic = "models/distilbert-thalamic-lora"
generative = "models/phi3-ctas-lora"

[vector_store]
type = "chromadb"
persist_directory = "./chroma_data"

[graph_store]
type = "neo4j"
uri = "bolt://localhost:7687"
user = "neo4j"
password_env = "NEO4J_PASSWORD"

[performance]
thalamic_timeout_ms = 15
vector_timeout_ms = 75
graph_timeout_ms = 150
inference_timeout_ms = 500
```

### 6.2 Rust Client (existing)

```rust
// ctas7-foundation-data/src/leptose_integration.rs
pub struct LeptoseClient {
    endpoint: String,
    timeout: Duration,
}

impl LeptoseClient {
    pub async fn infer(&self, request: InferenceRequest) -> Result<InferenceResponse> {
        // ... existing implementation
    }
}
```

### 6.3 API Endpoints

```
POST /v1/infer
{
  "query": "string",
  "inference_type": "threat_analysis" | "tactical" | "informational",
  "max_tokens": 1024,
  "temperature": 0.7
}

Response:
{
  "response": "string",
  "metadata": {
    "thalamic_gate": "full_processing",
    "pathway": "threat_analysis",
    "priority": "high",
    "latency_ms": 523,
    "sources_used": ["T1021.002", "G0016", "interview-node-003-002-001"]
  }
}

GET /v1/health
{
  "status": "healthy",
  "models_loaded": ["thalamic", "generative"],
  "vector_store": "connected",
  "graph_store": "connected"
}
```

---

## 7. Training Pipeline

### 7.1 Data Generation

```bash
# Generate thalamic filter training data
python generate_thalamic_training.py \
  --interviews output/interviews_for_upload.json \
  --threat-content output/threat_content/ \
  --output output/training_data/distilbert_thalamic.jsonl

# Generate Phi-3 instruction tuning data  
python generate_phi3_training.py \
  --interviews output/interviews_for_upload.json \
  --threat-content output/threat_content/ \
  --graph-export output/neo4j_export.json \
  --output output/training_data/phi3_lora_training.jsonl
```

### 7.2 Training Commands

```bash
# Thalamic filter (DistilBERT)
python train_thalamic.py \
  --base-model distilbert-base-uncased \
  --train-data output/training_data/distilbert_thalamic.jsonl \
  --output-dir models/distilbert-thalamic-lora \
  --epochs 5 \
  --batch-size 16

# Generative model (Phi-3)
python train_phi3.py \
  --base-model microsoft/Phi-3-mini-4k-instruct \
  --train-data output/training_data/phi3_lora_training.jsonl \
  --output-dir models/phi3-ctas-lora \
  --epochs 3 \
  --batch-size 4 \
  --gradient-accumulation 4
```

---

## 8. Performance Requirements

### 8.1 Latency SLOs

| Metric | Target | Critical |
|--------|--------|----------|
| P50 end-to-end | <400ms | <600ms |
| P95 end-to-end | <700ms | <1000ms |
| P99 end-to-end | <1000ms | <1500ms |
| Thalamic P99 | <15ms | <25ms |

### 8.2 Throughput

| Metric | Target |
|--------|--------|
| Queries/second | >50 |
| Concurrent users | >100 |
| Model memory | <8GB (4-bit quantized) |

### 8.3 Quality Metrics

| Metric | Target |
|--------|--------|
| Thalamic accuracy | >95% |
| Response relevance | >90% (human eval) |
| Technique citation accuracy | >98% |
| Hallucination rate | <2% |

---

## 9. Deployment Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    DEPLOYMENT ARCHITECTURE                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    KUBERNETES CLUSTER                                │   │
│  │                                                                     │   │
│  │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐  │   │
│  │  │ Leptose Pod      │  │ ChromaDB Pod     │  │ Neo4j Pod        │  │   │
│  │  │ ────────────     │  │ ────────────     │  │ ────────────     │  │   │
│  │  │ Port: 18114      │  │ Port: 8000       │  │ Ports: 7474,7687 │  │   │
│  │  │ Replicas: 3      │  │ Replicas: 1      │  │ Replicas: 1      │  │   │
│  │  │ GPU: T4 (opt)    │  │ Storage: 50GB    │  │ Storage: 100GB   │  │   │
│  │  └──────────────────┘  └──────────────────┘  └──────────────────┘  │   │
│  │                                                                     │   │
│  │  ┌──────────────────────────────────────────────────────────────┐  │   │
│  │  │ Model Storage (PVC)                                          │  │   │
│  │  │ ───────────────────                                          │  │   │
│  │  │ /models/distilbert-thalamic-lora (~500MB)                    │  │   │
│  │  │ /models/phi3-ctas-lora (~2GB 4-bit)                          │  │   │
│  │  └──────────────────────────────────────────────────────────────┘  │   │
│  │                                                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 10. References

- RFC-9004: Deterministic Routing Architecture
- RFC-9012: GNN Embeddings & Training Fabric
- RFC-9020: Unified Interview Schema
- Existing: `ctas7-foundation-data/src/leptose_integration.rs`
- Existing: `ctas7-linear/docker-compose.leptose-integrated.yml`

---

**End of RFC-9021**
