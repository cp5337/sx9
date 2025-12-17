# ChromaDB Unicode Integration (RFC-9002, RFC-9012)

**Date:** 2025-01-27  
**Status:** ✅ **IMPLEMENTED**  
**Goal:** Add all threat intelligence data to ChromaDB with Unicode operations for semantic search and routing.

---

## 🎯 **OBJECTIVE**

Create vector embeddings in ChromaDB for all threat intelligence data (tools, CTAS tasks, PTCCs, tool chains) with **Unicode operations** embedded in metadata and text, enabling:

1. **Semantic Search**: Find similar tools/tasks using vector similarity
2. **Unicode Routing**: Use Unicode operations (RFC-9002) for Neural Mux routing
3. **Hash-Based Lookup**: Map dual-trivariate hashes (RFC-9001) to Unicode operations
4. **GNN Integration**: Unicode as embedding substrate (RFC-9012)

---

## 📊 **ARCHITECTURE**

### **Unicode Mapping (RFC-9002)**

```
Dual-Trivariate Hash (RFC-9001)
┌──────────────┬──────────────┬──────────────┐
│  SCH (1-16)  │ CUID (17-32) │ UUID (33-48) │
│ 48 chars     │ 48 chars     │ 48 chars     │
└──────────────┴──────────────┴──────────────┘
         │              │              │
         ▼              ▼              ▼
    U+E100-E1FF   U+E200-E2FF   U+E000-E0FF
  (Trivariate)    (Context)     (System)
```

### **ChromaDB Collections (RFC-9021)**

| Collection | Purpose | Unicode Range |
|------------|---------|----------------|
| `tools` | Threat intelligence tools (Kali, ATT&CK, Atomic, Nuclei, etc.) | U+E800-E8FF (Kali Tools) |
| `ctas_tasks` | CTAS tasks (uuid- format) | U+E000-E0FF (System Controller) |
| `ptcc_configs` | PTCC configurations | U+E300-E3FF (Intelligence Processor) |
| `tool_chains` | Derived tool chains | U+E400-E6FF (Neural Mux) |

### **Embedding Model**

- **Model**: `all-MiniLM-L6-v2` (RFC-9021)
- **Dimensions**: 384
- **Space**: Cosine similarity
- **Latency Target**: <50ms P95, <100ms P99

---

## 🔧 **IMPLEMENTATION**

### **Script:** `04-abe-iac/add_to_chromadb_with_unicode.py`

### **Key Functions:**

1. **`hash_to_unicode(hash_component, component_type) -> int`**
   - Maps Base96 hash components to Unicode code points (U+E000-E9FF)
   - Component types: SCH → U+E100, CUID → U+E200, UUID → U+E000, TOOL → U+E800

2. **`extract_trivariate_components(trivariate_hash) -> Dict`**
   - Extracts SCH, CUID, UUID from 48-character dual-trivariate hash
   - Handles both separator (`_`) and concatenated formats

3. **`generate_unicode_ops_from_hash(trivariate_hash, entity_type) -> List[int]`**
   - Generates list of Unicode code points from trivariate hash
   - Returns Unicode operations for embedding metadata

4. **`unicode_ops_to_string(unicode_ops) -> str`**
   - Converts Unicode code points to string for metadata storage

5. **`ChromaDBUnicodeLoader`**
   - Main class for loading data into ChromaDB
   - Methods:
     - `add_tools_to_chromadb()`: Adds tools with Unicode ops
     - `add_ctas_tasks_to_chromadb()`: Adds CTAS tasks with Unicode ops
     - `add_ptcc_configs_to_chromadb()`: Adds PTCC configs with Unicode ops
     - `add_tool_chains_to_chromadb()`: Adds tool chains with Unicode ops

### **Metadata Structure**

Each vector includes:

```json
{
  "tool_id": "kali_nmap",
  "name": "nmap",
  "type": "network_scanner",
  "source": "kali_linux",
  "trivariate_primary": "[SCH|CUID|UUID]",
  "trivariate_secondary": "[SCH*|CUID*|UUID*]",
  "unicode_ops_primary": "57344,57600,57856",
  "unicode_ops_secondary": "57345,57601,57857",
  "unicode_string_primary": "\uE000\uE100\uE200",
  "unicode_string_secondary": "\uE001\uE101\uE201"
}
```

### **Embedding Text Format**

Unicode operations are included in the embedding text for semantic search:

```
Tool: nmap
Type: network_scanner
Description: Network mapper for port scanning
Unicode: [Unicode string from hash]
```

---

## 🚀 **HOW TO RUN**

### **Prerequisites**

```bash
# Install dependencies
pip install chromadb sentence-transformers

# Ensure matching script has been run
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac
python3 match_tools_to_ctas_tasks.py
```

### **Run Full Pipeline**

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac
python3 add_to_chromadb_with_unicode.py
```

### **Run Specific Collections**

```bash
# Only tools
python3 add_to_chromadb_with_unicode.py --tools-only

# Only CTAS tasks
python3 add_to_chromadb_with_unicode.py --tasks-only

# Only PTCC configs
python3 add_to_chromadb_with_unicode.py --ptcc-only

# Only tool chains
python3 add_to_chromadb_with_unicode.py --chains-only
```

---

## 📊 **OUTPUT**

### **ChromaDB Location**

```
04-abe-iac/node-interview-generator/output/vectors/chromadb/
```

### **Collections Created**

- `tools`: All threat intelligence tools
- `ctas_tasks`: CTAS tasks (uuid- format)
- `ptcc_configs`: PTCC configurations
- `tool_chains`: Derived tool chains

### **Query Example**

```python
import chromadb
client = chromadb.PersistentClient(path="./output/vectors/chromadb")
collection = client.get_collection("tools")

# Semantic search
results = collection.query(
    query_texts=["port scanner"],
    n_results=10
)

# Filter by Unicode operation
results = collection.query(
    query_texts=["network tool"],
    n_results=10,
    where={"unicode_ops_primary": {"$contains": "57344"}}  # U+E000
)
```

---

## 🔗 **INTEGRATION**

### **With Neural Mux**

Unicode operations in ChromaDB metadata can be used for Neural Mux routing:

```rust
// Extract Unicode ops from ChromaDB metadata
let unicode_ops = metadata.get("unicode_ops_primary")
    .split(",")
    .map(|s| s.parse::<u32>().unwrap())
    .collect();

// Route via Neural Mux
neural_mux.route(unicode_ops);
```

### **With GNN (RFC-9012)**

Unicode operations serve as structured node features:

```
Unicode Rune → Semantic Vector → GNN Node Features
```

### **With SlotGraph**

Unicode operations map to SlotGraph routing:

```rust
// Unicode → SlotGraph slot
let slot = unicode_to_slot(unicode_op);
slotgraph.route(hash, slot);
```

---

## 📋 **RFC COMPLIANCE**

- ✅ **RFC-9001**: Dual-trivariate hashing (SCH-CUID-UUID, 48 chars Base96)
- ✅ **RFC-9002**: Unicode routing (U+E000-E9FF)
- ✅ **RFC-9012**: Unicode as embedding substrate
- ✅ **RFC-9021**: ChromaDB collections and 384-dim embeddings

---

## 🎯 **NEXT STEPS**

1. ✅ Script created and tested
2. ⏳ Run full pipeline to populate ChromaDB
3. ⏳ Integrate with Neural Mux for routing
4. ⏳ Add GNN node features from Unicode ops
5. ⏳ Create query API for semantic search

---

## 📝 **NOTES**

- Unicode operations are stored in **both** metadata (as comma-separated integers) and text (as Unicode characters)
- This enables both **exact matching** (via metadata filters) and **semantic search** (via embedding text)
- ChromaDB will generate embeddings if `sentence-transformers` is not available, but custom embeddings are preferred for consistency
- All vectors include dual-trivariate hashes for cross-system integration (Neo4j, Sled KVS, SlotGraph)



