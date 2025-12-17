# Threat Tools Scraping + SPIRES Ontology Plan

**Date:** 2025-01-27  
**Status:** 📋 **PLAN - Ready for Implementation**  
**Goal:** Scrape all threat tools and process through SPIRES ontology (same as Kali tools)

---

## 🎯 **OBJECTIVE**

Create a unified pipeline that:
1. **Scrapes/Fetches** all threat intelligence tools and sources
2. **Processes** all tools through SPIRES ontology extraction
3. **Generates** unified ontology (JSON, Cypher, LinkML) for all threat tools

---

## 📊 **CURRENT STATE**

### ✅ **What Already Exists**

1. **`threat_content_fetcher.py`** - Fetches all threat sources:
   - MITRE ATT&CK (Enterprise, ICS, Mobile)
   - MITRE Defense (D3FEND, CAR, ATLAS, ENGAGE)
   - Adversary Emulation (Atomic Red Team, Caldera)
   - Detection Rules (Nuclei, Sigma, YARA, Wazuh)
   - Reconnaissance (Nmap scripts)
   - LOLTL (LOLBAS, GTFOBins, LOLDrivers, HijackLibs, WADComs)
   - OSINT (Awesome OSINT, OSINT Framework, Sherlock)
   - Kali Tools (from embedded inventory)

2. **SPIRES Integration** - Partially implemented:
   - `spires_ontology_extractor.py` processes: MITRE, crosswalk, Kali tools
   - `threat_content_fetcher.py` has `generate_spires_ontology()` method
   - Currently processes only 3 categories

### ⚠️ **What's Missing**

1. **SPIRES processing for ALL threat tool categories** (not just MITRE, crosswalk, Kali)
2. **Unified scraper script** (like `scrape-all-kali-tools.sh`)
3. **Enhanced SPIRES extractor** to handle all tool types

---

## 🔧 **IMPLEMENTATION PLAN**

### **Phase 0: RFC Compliance - Dual-Trivariate + Unicode Assembly** (3-4 hours)

**RFC-9001 Compliance: Dual-Trivariate Hashing**
- **Primary:** `[SCH]_[CUID]_[UUID]` (48 chars Base96)
- **Secondary:** `[SCH*]_[CUID*]_[UUID*]` (48 chars Base96)
- **Algorithm:** Murmur3-64 with Base96 encoding
- **Seeds:** SCH=0xC7A5_0000, CUID=0xC7A5_0001, UUID=0xC7A5_0002

**RFC-9002 Compliance: Unicode Assembly**
- **Unicode Range:** U+E000-E9FF (2,560 operations)
- **Mapping:**
  - U+E000-E0FF: System Controller (UUID positions 33-48)
  - U+E100-E1FF: Trivariate Processor (SCH positions 1-16)
  - U+E200-E2FF: Context Processor (CUID positions 17-32)
  - U+E300-E3FF: Intelligence Processor (Semantic hash)
  - U+E400-E4FF: Environmental Processor (Auth/METOC)
  - U+E800-E8FF: Kali Tools (Tool identifiers)

**Task Graph Integration:**
- Each threat tool becomes a **task graph node**
- Nodes linked via **predecessors/successors** (hash relationships)
- HD4 phase classification (Hunt/Detect/Disrupt/Disable/Dominate)
- PTH metrics (Probability, Time, Hazard)

**Implementation:**
```python
# Generate dual-trivariate hash for each threat tool
def generate_dual_trivariate(tool_data: dict) -> dict:
    # Primary hash (RFC-9001)
    primary = generate_trivariate_hash(
        content=tool_data["name"] + tool_data.get("description", ""),
        context=tool_data.get("category", ""),
        primitive_type="tool"
    )
    
    # Secondary hash (RFC-9001 - for Synaptix9/ATLAS/PLASMA)
    secondary = generate_trivariate_hash(
        content=tool_data["name"] + tool_data.get("git_repo", ""),
        context=tool_data.get("version", ""),
        primitive_type="tool_secondary"
    )
    
    # Unicode Assembly mapping (RFC-9002)
    unicode_op = map_hash_to_unicode(primary["sch"], "SCH")  # U+E100-E1FF
    
    return {
        "primary_hash": primary,
        "secondary_hash": secondary,
        "unicode_operation": unicode_op,
        "canonical_format": f"triv:{primary['sch']}_{primary['cuid']}_{primary['uuid']}"
    }

# Create task graph node
def create_task_graph_node(tool_data: dict, hashes: dict) -> dict:
    return {
        "hash_id": hashes["primary_hash"]["full"],
        "task_name": tool_data["name"],
        "description": tool_data.get("description", ""),
        "category": tool_data.get("category", ""),
        "hd4_phase": classify_hd4_phase(tool_data),
        "primitive_type": "Object",  # Tools are objects
        "predecessors": [],  # Will be populated from relationships
        "successors": [],   # Will be populated from relationships
        "p_probability": 0.85,  # Default
        "t_time": 0.50,         # Default
        "h_hazard": 0.30,       # Default
        "sch_hash": hashes["primary_hash"]["sch"],
        "cuid_hash": hashes["primary_hash"]["cuid"],
        "uuid_hash": hashes["primary_hash"]["uuid"],
        "unicode_operation": hashes["unicode_operation"],
        "task_seq": 0  # Will be assigned
    }
```

### **Phase 1: Enhance SPIRES Extractor** (2-3 hours)

**File:** `04-abe-iac/spires_ontology_extractor.py`

**Add processing for:**

1. **Adversary Emulation Tools**
   - Atomic Red Team tests (`atomic-red-team/`)
   - Caldera abilities (`caldera/`)
   - Extract: tool names, techniques, commands, descriptions

2. **Detection Rules as Tools**
   - Nuclei templates (`nuclei-templates/`)
   - Sigma rules (`sigma/`)
   - YARA rules (`yara-rules/`)
   - Wazuh rules (`wazuh/`)
   - Extract: rule names, techniques, platforms, severity

3. **LOLTL Tools**
   - LOLBAS binaries (`lolbas/`)
   - GTFOBins (`gtfobins/`)
   - LOLDrivers (`loldrivers/`)
   - HijackLibs (`hijacklibs/`)
   - WADComs (`wadcoms/`)
   - Extract: binary names, commands, techniques, platforms

4. **OSINT Tools**
   - Awesome OSINT (`awesome-osint/`)
   - OSINT Framework (`osint-framework/`)
   - Sherlock sites (`sherlock/`)
   - Extract: tool names, categories, descriptions, URLs

5. **Reconnaissance Tools**
   - Nmap NSE scripts (`nmap/`)
   - Extract: script names, categories, ports, protocols

6. **MITRE Defense Tools**
   - D3FEND countermeasures (`d3fend.json`)
   - CAR analytics (`car/`)
   - ATLAS techniques (`atlas/`)
   - Extract: technique names, countermeasures, platforms

**Code Changes:**
```python
# In spires_ontology_extractor.py - extract_from_threats()

# Import RFC-compliant hash generation
from ctas7_foundation_core.hashing import (
    generate_trivariate_hash,
    generate_dual_trivariate,
    map_hash_to_unicode,
    murmur3_64_base96
)

# Add after Kali tools section (line 558):

# Atomic Red Team
atomic_dir = THREAT_CONTENT_PATH / "atomic-red-team"
if atomic_dir.exists():
    print(f"  Processing Atomic Red Team tests...")
    for yaml_file in atomic_dir.glob("**/*.yaml"):
        tool_data = parse_yaml(yaml_file)
        # Generate dual-trivariate hash (RFC-9001)
        hashes = generate_dual_trivariate(tool_data)
        # Create task graph node
        task_node = create_task_graph_node(tool_data, hashes)
        # Add to ontology with Unicode operation (RFC-9002)
        graph.add_term(OntologyTerm(
            canonical_name=tool_data["name"],
            category="tool",
            definition=tool_data.get("description", "")[:200],
            trivariate_hash=hashes["primary_hash"]["full"],
            secondary_hash=hashes["secondary_hash"]["full"],
            unicode_operation=hashes["unicode_operation"],
            task_graph_node=task_node,
            frequency=1
        ))

# Nuclei Templates
nuclei_dir = THREAT_CONTENT_PATH / "nuclei-templates"
if nuclei_dir.exists():
    print(f"  Processing Nuclei templates...")
    # Similar processing with dual-trivariate + Unicode + task graph

# Sigma Rules
sigma_dir = THREAT_CONTENT_PATH / "sigma"
if sigma_dir.exists():
    print(f"  Processing Sigma rules...")
    # Similar processing with dual-trivariate + Unicode + task graph

# LOLBAS
lolbas_dir = THREAT_CONTENT_PATH / "lolbas"
if lolbas_dir.exists():
    print(f"  Processing LOLBAS binaries...")
    # Similar processing with dual-trivariate + Unicode + task graph

# ... (repeat for all categories with RFC compliance)
```

---

### **Phase 2: Create Unified Scraper Script** (1 hour)

**File:** `04-abe-iac/scrape-all-threat-tools.sh`

**Script Flow:**
1. Run `threat_content_fetcher.py --all` (fetches all sources)
2. Copy scraped data to `threat_content/` (if needed)
3. Run SPIRES ontology extraction (`spires_ontology_extractor.py --threats`)
4. Generate outputs (JSON, Cypher, LinkML)

**Script Content:**
```bash
#!/bin/bash
# Scrape All Threat Tools + SPIRES Ontology
# Unified pipeline for all threat intelligence tools

set -e

cd "$(dirname "$0")/node-interview-generator"

echo "🔧 Scraping All Threat Tools + SPIRES Ontology"
echo "=============================================="
echo ""

# Step 1: Fetch all threat content
echo "📥 Step 1: Fetching all threat content..."
python3 threat_content_fetcher.py --all --no-training

# Step 2: Run SPIRES ontology extraction
echo ""
echo "🧠 Step 2: Running SPIRES ontology extraction..."
cd ..
python3 spires_ontology_extractor.py --threats

# Step 3: Generate outputs
echo ""
echo "📊 Step 3: Generating ontology outputs..."
# Outputs already generated by SPIRES extractor

echo ""
echo "✅ Complete!"
```

---

### **Phase 3: Enhance Threat Content Fetcher** (Optional - 1 hour)

**File:** `04-abe-iac/node-interview-generator/threat_content_fetcher.py`

**Enhancements:**
1. Ensure all sources are properly scraped (not just cloned)
2. Add metadata extraction for each tool type
3. Standardize output format for SPIRES processing

**Current Status:** ✅ Already fetches all sources via `fetch_all()`

---

## 📋 **TOOL CATEGORIES TO PROCESS**

| Category | Source | Count | SPIRES Status |
|----------|--------|-------|---------------|
| **MITRE ATT&CK** | `mitre_attack.json` | 835 techniques | ✅ Processed |
| **MITRE Defense** | `d3fend.json`, `car/`, `atlas/` | ~500+ | ❌ Not processed |
| **Atomic Red Team** | `atomic-red-team/` | 327 tests | ❌ Not processed |
| **Caldera** | `caldera/` | ~100+ abilities | ❌ Not processed |
| **Nuclei Templates** | `nuclei-templates/` | 11,696 templates | ❌ Not processed |
| **Sigma Rules** | `sigma/` | 3,076 rules | ❌ Not processed |
| **YARA Rules** | `yara-rules/` | ~500+ rules | ❌ Not processed |
| **Wazuh Rules** | `wazuh/` | ~1,000+ rules | ❌ Not processed |
| **Nmap Scripts** | `nmap/` | 612 scripts | ❌ Not processed |
| **LOLBAS** | `lolbas/` | ~400+ binaries | ❌ Not processed |
| **GTFOBins** | `gtfobins/` | ~1,000+ entries | ❌ Not processed |
| **LOLDrivers** | `loldrivers/` | ~200+ drivers | ❌ Not processed |
| **HijackLibs** | `hijacklibs/` | ~100+ libraries | ❌ Not processed |
| **WADComs** | `wadcoms/` | ~300+ commands | ❌ Not processed |
| **OSINT Tools** | `awesome-osint/`, `osint-framework/`, `sherlock/` | ~1,000+ tools | ❌ Not processed |
| **Kali Tools** | `kali_tools_inventory.json` | 600+ tools | ✅ Processed |

**Total:** ~20,000+ threat tools/techniques/rules to process

---

## 🔬 **TETH INTEGRATION (OPTIONAL)**

**TETH = Topological Entropy Threat Heuristics**

### **Is TETH Required?**

**NO** - TETH is **optional** for basic scraping and SPIRES ontology generation.

**TETH is useful for:**
- ✅ **Threat heuristic scoring** - Risk assessment of tools
- ✅ **Entropy analysis** - Topological/behavioral entropy of tool relationships
- ✅ **HD4 phase classification** - TETH has HD4 thresholds (Hunt: 0.80, Detect: 0.65, etc.)
- ✅ **Predictive modeling** - 30-day threat level predictions
- ✅ **Tool risk assessment** - Complexity scores, anomaly detection

### **When to Use TETH:**

**Use TETH if you want:**
1. **Risk scoring** for each tool (entropy_H, threat_heuristic_score)
2. **HD4 phase recommendations** based on TETH thresholds
3. **Predictive capabilities** (30-day threat forecasts)
4. **Anomaly detection** in tool patterns
5. **Enhanced threat intelligence** beyond basic ontology

**Skip TETH if you only need:**
- Basic tool metadata scraping
- SPIRES ontology generation
- Dual-trivariate hashing (RFC-9001)
- Unicode Assembly mapping (RFC-9002)
- Task graph node creation

### **TETH Integration (Optional Phase)**

If including TETH, add after SPIRES processing:

```python
# Optional: TETH analysis for threat tools
def apply_teth_analysis(tool_data: dict, hashes: dict) -> dict:
    """Apply TETH algorithms to threat tool"""
    from ctas7_ptcc_teth_database import TETHAnalyzer
    
    analyzer = TETHAnalyzer()
    
    # TETH-Topological: Analyze tool complexity
    topological_entropy = analyzer.calculate_topological_entropy(tool_data)
    
    # TETH-Heuristic: Score threat level
    threat_score = analyzer.calculate_threat_heuristic(
        tool=tool_data["name"],
        category=tool_data.get("category", ""),
        entropy_h=topological_entropy
    )
    
    # TETH-Predictive: 30-day forecast
    prediction = analyzer.predict_threat_level(
        tool_data, horizon_days=30
    )
    
    # HD4 phase recommendation (from TETH thresholds)
    hd4_phase = analyzer.recommend_hd4_phase(threat_score)
    
    return {
        "teth_analysis": {
            "topological_entropy": topological_entropy,
            "threat_heuristic_score": threat_score,
            "complexity_score": analyzer.complexity_score(tool_data),
            "anomaly_detected": analyzer.detect_anomaly(tool_data)
        },
        "teth_predictions": {
            "30_day_threat_level": prediction["threat_level"],
            "prediction_confidence": prediction["confidence"]
        },
        "recommended_hd4_phase": hd4_phase,
        "teth_validated": True
    }
```

**TETH Cost:** FREE (local computation, no API calls)

---

## 💰 **COST ESTIMATE**

| Component | Cost |
|-----------|------|
| **Data Fetching** | FREE (local git clones, API calls) |
| **SPIRES Processing** | ~$0.10-0.50 (Gemini API for 20K+ items) |
| **TETH Analysis (Optional)** | FREE (local computation) |
| **Total (without TETH)** | **~$0.10-0.50** |
| **Total (with TETH)** | **~$0.10-0.50** (TETH is free) |

**Note:** SPIRES uses Gemini API (via ontogpt) for zero-shot extraction. Cost is minimal due to efficient batching.

---

## 📊 **OUTPUT STRUCTURE**

After processing, you'll have:

```
output/
├── threat_content/          # All scraped threat data
│   ├── mitre_attack.json
│   ├── atomic-red-team/
│   ├── nuclei-templates/
│   ├── sigma/
│   ├── lolbas/
│   ├── kali_tools_inventory.json
│   └── ...
├── ontology/                 # SPIRES-generated ontology
│   ├── threat_ontology.json  # Unified ontology graph
│   ├── threat_ontology.cypher  # Neo4j import
│   ├── threat_ontology.linkml.yaml  # LinkML schema
│   └── threat_ontology_stats.json  # Statistics
├── task_graph/              # Task graph nodes (RFC-9001/9002 compliant)
│   ├── threat_tools_graph.json  # All tools as task nodes
│   ├── task_relationships.json  # Predecessor/successor links
│   ├── dual_trivariate_hashes.json  # Primary + secondary hashes
│   └── unicode_operations.json  # Unicode Assembly mappings
└── hashes/                  # Hash artifacts
    ├── primary_hashes.json   # Primary trivariate hashes
    ├── secondary_hashes.json # Secondary trivariate hashes
    └── unicode_mappings.json # Hash → Unicode operation mappings
```

## 🔐 **RFC COMPLIANCE REQUIREMENTS**

### **RFC-9001: Dual-Trivariate Hashing**

**MUST Implement:**
- ✅ Primary hash: `[SCH]_[CUID]_[UUID]` (48 chars Base96)
- ✅ Secondary hash: `[SCH*]_[CUID*]_[UUID*]` (48 chars Base96)
- ✅ Murmur3-64 algorithm with Base96 encoding
- ✅ Standard seeds: SCH=0xC7A5_0000, CUID=0xC7A5_0001, UUID=0xC7A5_0002
- ✅ Canonical format: `triv:[SCH]_[CUID]_[UUID]`

**For Each Threat Tool:**
```python
{
    "primary_hash": {
        "sch": "3kJ9mP4xQ7R8sN2m",  # 16 chars Base96
        "cuid": "K5fH9nL8vC3dF6gH",  # 16 chars Base96
        "uuid": "2jK9mP4xQ7R8sN2m",  # 16 chars Base96
        "full": "3kJ9mP4xQ7R8sN2mK5fH9nL8vC3dF6gH2jK9mP4xQ7R8sN2m"
    },
    "secondary_hash": {
        "sch": "...",  # Secondary SCH
        "cuid": "...", # Secondary CUID
        "uuid": "...", # Secondary UUID
        "full": "..."
    },
    "canonical": "triv:3kJ9mP4xQ7R8sN2m_K5fH9nL8vC3dF6gH_2jK9mP4xQ7R8sN2m"
}
```

### **RFC-9002: Unicode Assembly**

**MUST Implement:**
- ✅ Unicode range: U+E000-E9FF (2,560 operations)
- ✅ Hash component → Unicode mapping:
  - SCH (positions 1-16) → U+E100-E1FF (Trivariate Processor)
  - CUID (positions 17-32) → U+E200-E2FF (Context Processor)
  - UUID (positions 33-48) → U+E000-E0FF (System Controller)
- ✅ Tool-specific Unicode: U+E800-E8FF (Kali Tools)

**For Each Threat Tool:**
```python
{
    "unicode_operations": {
        "sch_unicode": "\uE100",  # Trivariate processor
        "cuid_unicode": "\uE200",  # Context processor
        "uuid_unicode": "\uE000",  # System controller
        "tool_unicode": "\uE800"   # Tool-specific (if Kali tool)
    }
}
```

### **Task Graph Integration**

**MUST Implement:**
- ✅ Each threat tool = task graph node
- ✅ Nodes linked via hash relationships (predecessors/successors)
- ✅ HD4 phase classification
- ✅ PTH metrics (Probability, Time, Hazard)

**Task Graph Node Structure:**
```python
{
    "hash_id": "3kJ9mP4xQ7R8sN2mK5fH9nL8vC3dF6gH2jK9mP4xQ7R8sN2m",
    "task_name": "nmap",
    "description": "Network discovery and security auditing",
    "category": "NetworkRecon",
    "hd4_phase": "Hunt",
    "primitive_type": "Object",
    "predecessors": [],  # Array of hash_ids
    "successors": [],   # Array of hash_ids
    "p_probability": 0.95,
    "t_time": 0.30,
    "h_hazard": 0.05,
    "sch_hash": "3kJ9mP4xQ7R8sN2m",
    "cuid_hash": "K5fH9nL8vC3dF6gH",
    "uuid_hash": "2jK9mP4xQ7R8sN2m",
    "unicode_operation": "\uE100",
    "task_seq": 1
}
```

---

## 🚀 **EXECUTION STEPS**

### **Step 1: Enhance SPIRES Extractor**
```bash
cd 04-abe-iac
# Edit spires_ontology_extractor.py
# Add processing for all tool categories (see Phase 1)
```

### **Step 2: Create Unified Script**
```bash
# Create scrape-all-threat-tools.sh (see Phase 2)
chmod +x scrape-all-threat-tools.sh
```

### **Step 3: Run Pipeline**
```bash
./scrape-all-threat-tools.sh
```

### **Step 4: Verify Outputs**
```bash
# Check ontology stats
cat output/ontology/threat_ontology_stats.json | jq

# Check tool counts
cat output/ontology/threat_ontology.json | jq '.terms | length'
```

---

## ✅ **SUCCESS CRITERIA**

### **Required (Core)**
1. ✅ All 20,000+ threat tools/techniques/rules scraped
2. ✅ All categories processed through SPIRES
3. ✅ Unified ontology generated (JSON, Cypher, LinkML)
4. ✅ Tool relationships extracted (techniques → tools, tools → categories)
5. ✅ **RFC-9001 compliant:** Dual-trivariate hashes (primary + secondary) for all tools
6. ✅ **RFC-9002 compliant:** Unicode Assembly operations mapped (U+E000-E9FF)
7. ✅ **Task graph integration:** All tools as graph nodes with hash relationships
8. ✅ Cost under $1.00

### **Optional (Enhanced)**
9. ⚪ **TETH analysis:** Threat heuristic scoring, entropy analysis (optional)
10. ⚪ **TETH predictions:** 30-day threat level forecasts (optional)
11. ⚪ **TETH HD4 recommendations:** HD4 phase classification via TETH thresholds (optional)

---

## 📝 **NEXT STEPS**

1. **Enhance `spires_ontology_extractor.py`** to process all tool categories
2. **Create `scrape-all-threat-tools.sh`** unified script
3. **Test** on small subset first (e.g., just Atomic Red Team)
4. **Run** full pipeline
5. **Verify** ontology outputs

---

## 🔗 **RELATED FILES**

- `04-abe-iac/node-interview-generator/threat_content_fetcher.py` - Main fetcher
- `04-abe-iac/spires_ontology_extractor.py` - SPIRES extractor (needs enhancement)
- `04-abe-iac/spires_threat_extractor.py` - Alternative SPIRES extractor
- `04-abe-iac/scrape-all-kali-tools.sh` - Reference implementation

---

**Status:** Ready to implement. Estimated time: 4-5 hours total.

