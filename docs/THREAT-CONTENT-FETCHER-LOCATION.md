# Threat Content Fetcher Script Location

**Date:** 2025-01-XX  
**Status:** ✅ **FOUND - Enhanced with SPIRES Ontology & DSL Conversion**

---

## 📍 Script Location

**File:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/threat_content_fetcher.py`

**Total Lines:** 1,609 lines  
**Purpose:** Comprehensive threat content fetcher for RFC-9011 and RFC-9023

---

## 🎯 What This Script Does

### 1. Downloads Threat Content

**MITRE Suite (All)**
- ✅ MITRE ATT&CK Enterprise (JSON)
- ✅ MITRE ATT&CK ICS (JSON)
- ✅ MITRE ATT&CK Mobile (JSON)
- ✅ MITRE D3FEND (JSON)
- ✅ MITRE CAR Analytics (YAML)
- ✅ MITRE ENGAGE (YAML)
- ✅ MITRE ATLAS (YAML)

### Adversary Emulation
- ✅ Atomic Red Team (YAML - thousands of tests)
- ✅ Caldera Adversaries & Abilities (YAML)

### Detection Rules
- ✅ Nuclei Templates (YAML - thousands)
- ✅ Sigma Rules (YAML - thousands)
- ✅ YARA Rules (YAR files)
- ✅ Wazuh Rules (XML)

### Reconnaissance
- ✅ Nmap NSE Scripts (Lua)

### Living Off The Land (LOLTL)
- ✅ LOLBAS (YAML)
- ✅ GTFOBins (Markdown)
- ✅ LOLDrivers (YAML)
- ✅ HijackLibs (YAML)
- ✅ WADComs (Markdown)

### Kali Tools
- ✅ Kali Tools Inventory (from embedded categories)

### OSINT Resources
- ✅ Awesome OSINT (Markdown)
- ✅ OSINT Framework (JSON)
- ✅ Sherlock Sites (JSON)

### 2. Generates SPIRES Ontology (RFC-9105)
- ✅ Extracts ontology from threat content
- ✅ Generates JSON, Cypher (Neo4j), and LinkML outputs
- ✅ Creates unified SX9 ontology with trivariate hashing

### 3. Converts YAMLs to DSL (RFC-9011-B)
- ✅ Validates YAML against LinkML schemas
- ✅ Converts to SX9 DSL format with PTCC primitive mapping
- ✅ Generates trivariate hashes (SCH-CUID-UUID)
- ✅ Maps to HD4 phases (HUNT, DETECT, DISABLE, DISRUPT, DOMINATE)

---

## 📂 Current Status

**Location:** `04-abe-iac/node-interview-generator/output/threat_content/`

**Status:**
- ✅ **Directories exist** (atlas, atomic-red-team, caldera, nuclei-templates, sigma, etc.)
- ❌ **Directories are EMPTY** (64 bytes each = just empty dirs)
- ✅ **Index files exist:**
  - `mitre_attack.json` (33MB)
  - `mitre_attack_ics.json` (2.2MB)
  - `mitre_attack_mobile.json` (3.1MB)
  - `d3fend.json` (3.0M)
  - `crosswalk_index.json` (192KB)
  - `kali_tools_inventory.json` (21KB)

**Conclusion:** The script was run and created indexes, but the actual YAML files were either:
1. Never downloaded (script may have failed partway)
2. Downloaded then deleted/cleaned up
3. Stored elsewhere (GCP, containers, etc.)

---

## 🐳 OrbStack Container Check

**Checked:**
- ✅ OrbStack containers running (neo4j, wazuh, surrealdb, etc.)
- ✅ Docker volumes exist (no threat-specific volumes found)
- ❌ No threat content found in OrbStack volumes

**Conclusion:** Files are not in OrbStack containers/volumes.

---

## 🚀 How to Re-run

### Basic Usage (Fetch All + Ontology + DSL)

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator

# Fetch ALL sources, generate ontology, and convert to DSL
python3 threat_content_fetcher.py --all
```

This will:
1. ✅ Download all threat content (MITRE, Nuclei, Caldera, etc.)
2. ✅ Generate SPIRES ontology (JSON, Cypher, LinkML)
3. ✅ Convert YAMLs to DSL format

### Advanced Usage

```bash
# Fetch specific sources only
python3 threat_content_fetcher.py --mitre --nuclei --sigma

# Fetch without ontology generation
python3 threat_content_fetcher.py --all --no-ontology

# Fetch without DSL conversion
python3 threat_content_fetcher.py --all --no-dsl

# Fetch without both
python3 threat_content_fetcher.py --all --no-ontology --no-dsl
```

### What It Does

1. **Clones Git Repositories:**
   - `git clone --depth 1` for all repos (Nuclei, Sigma, Caldera, Atomic Red Team, etc.)

2. **Downloads JSON Files:**
   - Direct HTTP downloads for MITRE ATT&CK JSON files

3. **Parses and Indexes:**
   - Extracts techniques, rules, templates
   - Builds crosswalk mappings (technique → Nuclei, technique → Sigma, etc.)
   - Saves indexes to JSON files

4. **Output Location:**
   - `output/threat_content/` - All downloaded repos and files
   - `output/threat_content/*.json` - Index files

5. **Generates SPIRES Ontology:**
   - `output/ontology/threat_ontology.json` - Full ontology
   - `output/ontology/threat_ontology.cypher` - Neo4j import
   - `output/ontology/threat_ontology.linkml.yaml` - LinkML schema

6. **Converts YAMLs to DSL:**
   - `output/sx9_dsl/sx9_entities.yaml` - DSL entities
   - `output/sx9_dsl/sx9_entities.json` - DSL entities (JSON)
   - `output/sx9_dsl/validation_errors.json` - Validation errors
   - `output/sx9_dsl/pipeline_summary.json` - Conversion summary

---

## 📊 Expected Output

After running, you should have:

```
output/
├── threat_content/            # Raw downloaded content
│   ├── nuclei-templates/      # Thousands of YAML files
│   ├── sigma/                 # Thousands of YAML files
│   ├── atomic-red-team/       # Hundreds of YAML files
│   ├── caldera/               # YAML files for adversaries/abilities
│   ├── mitre_attack.json      # ✅ Already exists (33MB)
│   ├── mitre_attack_ics.json  # ✅ Already exists (2.2MB)
│   ├── mitre_attack_mobile.json # ✅ Already exists (3.1MB)
│   ├── crosswalk_index.json   # ✅ Already exists (192KB)
│   └── threat_content_summary.json
│
├── ontology/                   # SPIRES ontology outputs
│   ├── threat_ontology.json
│   ├── threat_ontology.cypher
│   └── threat_ontology.linkml.yaml
│
└── sx9_dsl/                    # Converted DSL entities
    ├── sx9_entities.yaml
    ├── sx9_entities.json
    ├── validation_errors.json
    └── pipeline_summary.json
```

---

## ⚠️ Notes

1. **Large Downloads:** Nuclei templates alone can be 100MB+ of YAML files
2. **Git Clones:** Script uses `--depth 1` for faster cloning
3. **Time Required:** Full fetch can take 10-30 minutes depending on network
4. **Storage:** Expect 500MB-2GB total for all sources

---

## 🔧 Script Features

- **Comprehensive:** Covers all major threat intelligence sources
- **Crosswalk Mappings:** Links MITRE techniques to Nuclei, Sigma, Atomic, etc.
- **Incremental:** Can update existing repos with `git pull`
- **Indexed:** Creates searchable JSON indexes for quick lookup
- **SPIRES Ontology:** Generates unified ontology with trivariate hashing (RFC-9105)
- **DSL Conversion:** Converts YAMLs to SX9 DSL with PTCC mapping (RFC-9011-B)
- **RFC Compliant:** Follows RFC-9011, RFC-9011-B, RFC-9023, and RFC-9105 specifications

---

## ✅ Ready to Re-run

The script is ready to execute. Just run:

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac/node-interview-generator
python3 threat_content_fetcher.py --all
```

This will download and process all MITRE, Nuclei, Caldera, Atomic Red Team, Sigma, Kali tools, and OSINT resources.

