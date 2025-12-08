# ML Components Location

**Date:** 2025-01-XX  
**Status:** ✅ **ALL FOUND**

---

## 📍 Component Locations

### 1. DistilBERT ✅

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-hash-fingerprint-engine/ctas_hash/embeddings/distilbert.py`

**Status:** ✅ **Fully Implemented**

**Features:**
- Uses `distilbert-base-uncased` from HuggingFace
- 768-dimensional embeddings
- Batch processing support
- GPU/CPU auto-detection
- Integration with CTAS-HASH fingerprint engine

**Usage:**
```python
from ctas_hash.embeddings.distilbert import DistilBERTEmbedder

embedder = DistilBERTEmbedder()
embeddings = embedder.embed("threat intelligence text")
```

**Dependencies:**
- `transformers`
- `torch`
- `numpy`

**References:**
- RFC-9021: Cognitive Inference (Thalamic Filter)
- RFC-9108: Thalmic Filter Model Registry
- Used for MITRE technique classification

---

### 2. Phi (Microsoft Phi-2/Phi-3) ✅

**Locations:**
1. `/Users/cp5337/Developer/ctas-7-shipyard-staging/sx9-quarantine-blake/blake3_sliding_phi.py`
2. `/Users/cp5337/Developer/ctas-7-shipyard-staging/sx9-quarantine-blake/blake3_sliding_phi_gnn.py`

**Status:** ✅ **Fully Implemented**

**Features:**
- Uses `microsoft/phi-2` model
- Phi-3 integration available
- Cognitive reasoning for node analysis
- GNN integration variant available
- GPU/CPU support with auto device mapping

**Usage:**
```python
from blake3_sliding_phi_gnn import PhiOracle

oracle = PhiOracle()
oracle.initialize()
reasoning = oracle.reason_about_node(node_profile)
```

**Dependencies:**
- `transformers`
- `torch`
- `torch-geometric` (for GNN variant)

**References:**
- RFC-9021: Cognitive Inference (Generative Inference)
- RFC-9112: Deterministic Prompt Engineering
- Used for threat intelligence summarization and reasoning

**Model Registry:**
- `sx9_phi3` in `unified-registry.json`
- Base: `microsoft/phi-3-mini-4k-instruct`
- LoRA: `phi3-mitre-explainer-lora-v1`

---

### 3. GNN (Graph Neural Network) ✅

**Locations:**
1. `/Users/cp5337/Developer/ctas7-command-center/ctas7-intelligence-generator/gnn_osint_intelligence.py`
2. `/Users/cp5337/Developer/ctas-7-shipyard-staging/sx9-quarantine-blake/blake3_sliding_phi_gnn.py`

**Status:** ✅ **Fully Implemented**

**Features:**

**GNN OSINT Intelligence:**
- Multi-layer GCN with attention (GAT)
- TransformerConv layers
- OSINT graph node/edge types
- High-GPU remote processing support
- Integration with ABE Drop Zone and SlotGraph

**CognetixGNN:**
- GCN layers for cognitive network analysis
- Graph pooling (global_mean_pool)
- Node embeddings and graph embeddings
- Centrality calculations

**Usage:**
```python
from gnn_osint_intelligence import GNNOSINTIntelligence

gnn = GNNOSINTIntelligence()
analysis = gnn.analyze_threat_graph(osint_graph)
```

**Dependencies:**
- `torch`
- `torch-geometric`
- `networkx`
- `numpy`

**References:**
- RFC-9012: Embeddings & GNN Training Fabric
- RFC-9021: Cognitive Inference
- Used for threat actor relationship analysis and attack path prediction

**Processing Modes:**
- `LOCAL_CPU` - Local CPU-based processing
- `LOCAL_GPU` - Local GPU if available
- `HIGH_GPU_REMOTE` - Marcus GCP high-GPU system
- `HYBRID` - Local + Remote combination

---

### 4. ChromaDB ✅

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-ops-main-platform/src/lib/services/chromadb-client.ts`

**Status:** ✅ **Fully Implemented**

**Features:**
- TypeScript client for ChromaDB vector database
- 384-dimensional embeddings (all-MiniLM-L6-v2)
- Multiple collections:
  - `plasma_threats` - Threat descriptions
  - `plasma_indicators` - IOC embeddings
  - `plasma_scenarios` - Attack scenarios
  - `techniques` - MITRE ATT&CK techniques
  - `detection_rules` - Sigma/YARA/Wazuh rules
  - `tools` - Security tools
  - `interviews` - Node interviews
- Semantic similarity search
- Document indexing and querying

**Usage:**
```typescript
import { findSimilarThreats, indexThreat } from "./chromadb-client"

// Query similar threats
const results = await findSimilarThreats("APT29 spear phishing", {
  n_results: 10
})

// Index new threat
await indexThreat({
  threat_id: "threat_001",
  description: "Advanced persistent threat...",
  domain: "cyber"
})
```

**Configuration:**
- Endpoint: `CHROMADB_URL` env var (default: `http://localhost:8000`)
- Embedding endpoint: `EMBEDDING_URL` env var (default: `http://localhost:18117`)
- Target latency: <50ms P95, <100ms P99

**References:**
- RFC-9021: Cognitive Inference (Layer 2a: Vector Search)
- Used for semantic similarity search in Plasma Defender

---

## 🔗 Integration Points

### Threat Content Fetcher Integration

All four components can be integrated into the threat content fetcher pipeline:

1. **DistilBERT** → Classify threat content (MITRE techniques, detection rules)
2. **Phi** → Summarize and reason about threat intelligence
3. **GNN** → Analyze threat actor relationships and attack paths
4. **ChromaDB** → Store and search threat embeddings

### Recommended Integration Flow

```
Threat Content (YAML/JSON)
    ↓
[DistilBERT] → Classification & Embeddings
    ↓
[ChromaDB] → Vector Storage
    ↓
[GNN] → Graph Analysis
    ↓
[Phi] → Summarization & Reasoning
    ↓
DSL Output with Dual Hashes
```

---

## 📦 Dependencies Summary

### Python Dependencies
```bash
# DistilBERT
pip install transformers torch numpy

# Phi
pip install transformers torch

# GNN
pip install torch torch-geometric networkx numpy

# ChromaDB (Python client)
pip install chromadb
```

### TypeScript/Node Dependencies
```bash
# ChromaDB (TypeScript)
npm install chromadb
```

---

## 🚀 Next Steps

1. **Integrate into Threat Content Fetcher:**
   - Add DistilBERT classification step
   - Add ChromaDB indexing step
   - Add GNN analysis step
   - Add Phi summarization step

2. **Model Loading:**
   - Ensure models are downloaded/cached
   - Set up model registry paths
   - Configure GPU/CPU fallback

3. **ChromaDB Setup:**
   - Start ChromaDB service
   - Create collections
   - Configure embedding service

---

## 📊 Component Status Matrix

| Component | Location | Status | Language | Dependencies |
|-----------|----------|--------|----------|--------------|
| DistilBERT | `ctas7-hash-fingerprint-engine` | ✅ Ready | Python | transformers, torch |
| Phi | `sx9-quarantine-blake` | ✅ Ready | Python | transformers, torch |
| GNN | `ctas7-intelligence-generator` | ✅ Ready | Python | torch-geometric |
| ChromaDB | `ctas7-ops-main-platform` | ✅ Ready | TypeScript | chromadb npm |

---

## ✅ All Components Found and Ready

All four ML components are implemented and ready for integration into the threat content fetcher pipeline.


