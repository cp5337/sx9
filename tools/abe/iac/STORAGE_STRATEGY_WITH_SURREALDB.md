# Updated Storage Strategy: SPIRES Ontology → Cypher + SurrealQL

**Updated:** Now includes SurrealDB alongside Neo4j  
**SPIRES generates ontology, then we generate Cypher (Neo4j) and SurrealQL (SurrealDB) FROM the ontology**

---

## 🎯 **Storage Flow**

```
┌─────────────────────────────────────────────────────────┐
│  Download Threat Content (Python)                      │
│  ├─ MITRE ATT&CK, ENGAGE, D3FEND, CAR                  │
│  ├─ Atomic Red Team, Nuclei, Sigma, OSSEC/Wazuh         │
│  └─ LOLBINs                                             │
└─────────────────────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│  SPIRES Ontology Extraction (Python, during download)  │
│  ├─ Extract terms, relationships                        │
│  ├─ Load existing ontology (deduplicate)                │
│  ├─ Merge new with existing                             │
│  └─ Generate ontology graph                             │
└─────────────────────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│  Generate Database Queries FROM Ontology                │
│  ├─ Cypher queries (Neo4j)                              │
│  ├─ SurrealQL queries (SurrealDB)                       │
│  └─ JSON export (general use)                           │
└─────────────────────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
┌───────────────┐      ┌──────────────────┐
│  Neo4j        │      │  SurrealDB       │
│  (Cypher)     │      │  (SurrealQL)     │
│  Port: 7687   │      │  Port: 8000      │
└───────────────┘      └──────────────────┘
```

---

## 📊 **Current SPIRES Status**

**Existing Ontology:**
- **Terms:** 1,730
- **Relations:** 1,089
- **File:** `04-abe-iac/output/ontology/ontology_raw.json` (873 KB)
- **Last Updated:** December 7, 2025

**Categories:**
- Technique: 709
- Detection: 933
- Tool: 74
- Tactic: 14

---

## 🔄 **Deduplication Strategy**

### **1. Load Existing Before Extraction**
```python
# In threat_content_fetcher.py generate_spires_ontology()
existing_ontology_path = "output/ontology/ontology_raw.json"
threat_graph = OntologyGraph()

if existing_ontology_path.exists():
    existing_count = threat_graph.load_from_json(existing_ontology_path)
    print(f"✅ Loaded {existing_count} existing terms")
```

### **2. Merge Logic (Built-in)**
- `OntologyGraph.add_term()` automatically merges duplicates:
  - Increments frequency
  - Merges aliases
  - Merges RFC sources
- `OntologyGraph.add_relation()` deduplicates relations:
  - Checks for existing (source, target, relation_type)
  - Updates weight if new weight is higher

### **3. Output Files**
- `ontology_raw.json` - Combined ontology (merged)
- `ontology.cypher` - Neo4j queries (generated FROM ontology)
- `ontology.surql` - SurrealDB queries (generated FROM ontology)
- `ontology_threats_raw.json` - New extraction only (for comparison)

---

## 💾 **Storage Locations**

### **1. Neo4j (Graph Relationships)**
- **What:** Task graph nodes, relationships, predecessors/successors
- **Size:** ~50-70 MB (graph structure)
- **Queries:** Cypher (generated FROM ontology)
- **Port:** 7687 (Bolt), 7474 (HTTP)
- **Database:** `sx9_threat_extraction`

```cypher
// Generated FROM SPIRES ontology
CREATE (t:Term {
  sch_id: 'SCH_...',
  canonical_name: 'Defense Evasion',
  category: 'tactic',
  ...
});
```

### **2. SurrealDB (Multi-Model Storage)** ⭐ **NEW**
- **What:** Terms, relationships, full ontology data
- **Size:** ~100-150 MB
- **Queries:** SurrealQL (generated FROM ontology)
- **Port:** 8000
- **Namespace:** `ctas7`
- **Database:** `threat_ontology`

```surql
-- Generated FROM SPIRES ontology
USE NS ctas7 DB threat_ontology;
CREATE term:SCH_... SET
    sch_id = 'SCH_...',
    canonical_name = 'Defense Evasion',
    category = 'tactic',
    ...
;
```

### **3. Supabase (Structured Data)**
- **What:** Processed threat items, hashes, metadata
- **Size:** ~500-800 MB
- **Access:** SQL queries, REST API
- **Port:** 5432 (Postgres), 3000 (PostgREST)

### **4. Sled KVS (Fast Lookups)**
- **What:** Hash → unicode mappings
- **Size:** ~20-30 MB
- **Access:** O(1) hash lookups

---

## 🚀 **Updated Pipeline**

### **During Download (Python)**
```bash
python threat_content_fetcher.py --all
```

**What happens:**
1. Downloads all threat content
2. **SPIRES extracts ontology** (loads existing, merges new)
3. **Generates Cypher** FROM ontology (for Neo4j)
4. **Generates SurrealQL** FROM ontology (for SurrealDB)
5. Saves all outputs

### **Output Files**
```
04-abe-iac/output/ontology/
├── ontology_raw.json          # Combined ontology (1,730+ terms)
├── ontology.cypher            # Neo4j queries (FROM ontology)
├── ontology.surql             # SurrealDB queries (FROM ontology) ⭐ NEW
├── ontology_threats_raw.json  # New extraction only
└── ontology_enriched.json     # Enriched with Gemini
```

### **Load to Databases**
```bash
# Load Cypher to Neo4j
cypher-shell -u neo4j -p ctas7_graph < output/ontology/ontology.cypher

# Load SurrealQL to SurrealDB
surreal import --conn http://localhost:8000 \
  --user root --pass root \
  --ns ctas7 --db threat_ontology \
  output/ontology/ontology.surql
```

---

## 📋 **Deduplication Summary**

**Current Status:**
- ✅ **1,730 terms** already in ontology
- ✅ **1,089 relations** already in ontology
- ✅ **Merge logic** built-in (add_term, add_relation)
- ✅ **Load existing** before new extraction
- ✅ **No duplicates** - existing terms merged, not duplicated

**Next Run:**
- Will load existing 1,730 terms
- Extract NEW terms from threat content
- Merge new with existing (deduplicate)
- Generate updated Cypher + SurrealQL

---

## 💰 **Storage Costs**

| Storage | Size | Cost/Month |
|---------|------|------------|
| **Neo4j** (self-hosted) | 70 MB | FREE |
| **SurrealDB** (self-hosted) | 150 MB | FREE |
| **Supabase** (subscription) | 800 MB | Already paid |
| **Sled KVS** (local) | 30 MB | FREE |
| **CDN** (R2/GCP) | 300 MB | ~$0.005 |
| **Total** | 1.35 GB | **~$0.005/month** |

---

## ✅ **Benefits**

1. **No Duplicates:** Existing ontology loaded and merged
2. **Dual Database:** Neo4j (graph) + SurrealDB (multi-model)
3. **Generated FROM Ontology:** Cypher and SurrealQL both from same source
4. **Incremental Updates:** Only new terms/relations added
5. **Cost Effective:** Mostly free (self-hosted) + existing Supabase subscription

---

**The lattice is watching.** 🔥


