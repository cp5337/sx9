# YAML File Inventory - Pre-Conversion Analysis
## Comprehensive Search Results Before DSL Conversion

**Date:** December 2025  
**Status:** Inventory Complete - Ready for Conversion Planning  
**Purpose:** Document all YAML files before conversion to DSL

---

## Executive Summary

**Total YAML Files Found:**
- **ctas7-command-center**: 92 YAML files
- **ctas-7-shipyard-staging**: 1,067 YAML files
- **Total**: ~1,159 YAML files

**Key Findings:**
- ✅ Threat content fetcher already exists (`threat_content_fetcher.py`)
- ✅ RFC-9011 defines YAML → DSL conversion process
- ✅ Some DSL files already exist (2 files found)
- ✅ Output directories contain processed threat content
- ⚠️ Need to catalog all YAML files by type before conversion

---

## 1. Threat Intelligence YAML Sources

### 1.1 MITRE ATT&CK Suite

**Source:** `threat_content_fetcher.py` (RFC-9011)

| Source | Type | URL/Repo | Status |
|--------|------|----------|--------|
| **MITRE ATT&CK Enterprise** | JSON | `mitre/cti/master/enterprise-attack/enterprise-attack.json` | ✅ Fetched |
| **MITRE ATT&CK ICS** | JSON | `mitre/cti/master/ics-attack/ics-attack.json` | ✅ Fetched |
| **MITRE ATT&CK Mobile** | JSON | `mitre/cti/master/mobile-attack/mobile-attack.json` | ✅ Fetched |
| **MITRE D3FEND** | JSON | `d3fend.mitre.org/ontologies/d3fend.json` | ✅ Fetched |
| **MITRE CAR** | YAML | `mitre-attack/car/analytics` | ⚠️ Needs fetch |
| **MITRE ENGAGE** | YAML | `mitre/engage/data` | ⚠️ Needs fetch |
| **MITRE ATLAS** | YAML | `mitre-atlas/atlas-data/data` | ⚠️ Needs fetch |

**Output Location:**
- `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/threat_content/`
  - `mitre_attack.json`
  - `mitre_attack_ics.json`
  - `mitre_attack_mobile.json`

### 1.2 Adversary Emulation YAML

| Source | Type | Repo | Path | Status |
|--------|------|------|------|--------|
| **Atomic Red Team** | YAML | `redcanaryco/atomic-red-team` | `atomics/` | ⚠️ Needs fetch |
| **Caldera** | YAML | `mitre/caldera` | `data/` | ⚠️ Needs fetch |

**Caldera Integration:**
- Rust integration exists: `ctas7-exploit-arsenal/src/caldera_integration.rs`
- Loads adversaries, abilities, operations from Caldera API
- YAML files stored in Caldera data directory

### 1.3 Detection & Vulnerability YAML

| Source | Type | Repo | Path | Status |
|--------|------|------|------|--------|
| **Nuclei Templates** | YAML | `projectdiscovery/nuclei-templates` | Root | ⚠️ Needs fetch |
| **Sigma Rules** | YAML | `SigmaHQ/sigma` | `rules/` | ⚠️ Needs fetch |
| **YARA Rules** | YAR | `Yara-Rules/rules` | `.` | ⚠️ Needs fetch |
| **Wazuh Rules** | XML/YAML | Wazuh repo | Various | ⚠️ Needs fetch |

**Nuclei Status:**
- ✅ User mentioned "thousands of YAML from Nuclei"
- ✅ Already converted to DSL (per user)
- ⚠️ Need to locate converted DSL files
- ⚠️ Raw YAML files still available

### 1.4 Network Reconnaissance YAML

| Source | Type | Repo | Path | Status |
|--------|------|------|------|--------|
| **Nmap NSE Scripts** | Lua | Nmap repo | `scripts/` | ⚠️ Needs fetch |

### 1.5 Living Off The Land (LOLTL) YAML

| Source | Type | Repo | Path | Status |
|--------|------|------|------|--------|
| **LOLBAS** | YAML | `LOLBAS-Project/LOLBAS` | Various | ⚠️ Needs fetch |
| **GTFOBins** | YAML | `GTFOBins/GTFOBins.github.io` | `_gtfobins/` | ⚠️ Needs fetch |
| **LOLDrivers** | YAML | `LOLDrivers/LOLDrivers` | Various | ⚠️ Needs fetch |
| **HijackLibs** | YAML | `mandiant/HijackLibs` | Various | ⚠️ Needs fetch |
| **WADComs** | YAML | `WADComs/WADComs.github.io` | Various | ⚠️ Needs fetch |

---

## 2. Existing YAML Files Found

### 2.1 Threat Content Output Directory

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/`

**Files Found:**
- `threat_content/mitre_attack.json`
- `threat_content/mitre_attack_ics.json`
- `threat_content/mitre_attack_mobile.json`
- `threat_content/mitre_index.json`
- `sx9_dsl/sx9_entities.yaml` ✅ **Already converted to DSL**

### 2.2 LinkML Templates

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/linkml_templates/`

**Files:**
- `detection_rule.yaml`
- `threat_technique.yaml`

### 2.3 Ontology Files

**Location:** `ctas-7-shipyard-staging/04-abe-iac/output/ontology/`

**Files:**
- `sx9_ontology.yaml`

### 2.4 Scenario Database YAML

**Location:** `ctas-7-shipyard-staging/ctas7-scenarios-database/forge_workflows/`

**Files Found:**
- Multiple scenario YAML files (DHS scenarios, attack scenarios)
- Example: `scenario_dhs_scenario_13__biological_attack___contagious_disease_geospatial.yaml`

---

## 3. DSL Files Already Created

### 3.1 Existing DSL Files

**Location:** `ctas-7-shipyard-staging/ctas7-ops-main-platform/DSL-orchestration/`

**Files:**
- `dsl-crate-grouping-system.dsl`
- `playbooks/dsl-crate-grouping-system.dsl`

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/sx9_dsl/`

**Files:**
- `sx9_entities.yaml` (DSL format)

---

## 4. Conversion Status by Source

### 4.1 Already Converted ✅

- **Nuclei Templates** → DSL (per user confirmation)
- **Some SX9 entities** → DSL format

### 4.2 Needs Conversion ⚠️

- **MITRE ATT&CK** (JSON → DSL)
- **Caldera Adversaries** (YAML → DSL)
- **Atomic Red Team** (YAML → DSL)
- **Sigma Rules** (YAML → DSL)
- **LOLBAS/GTFOBins** (YAML → DSL)
- **Other threat intelligence YAMLs**

### 4.3 Not Applicable ❌

- Docker Compose files
- GitHub Actions workflows
- Configuration files
- Build system files

---

## 5. YAML File Locations by Type

### 5.1 Threat Intelligence YAMLs

**Primary Location:**
```
ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/
├── output/
│   ├── threat_content/        # Fetched threat intelligence
│   └── sx9_dsl/               # Converted DSL files
├── linkml_templates/          # Schema templates
└── threat_content_fetcher.py  # Fetcher script
```

### 5.2 Scenario YAMLs

**Location:**
```
ctas-7-shipyard-staging/ctas7-scenarios-database/
└── forge_workflows/           # Attack scenario YAMLs
```

### 5.3 Configuration YAMLs

**Locations:**
- Docker Compose files (various locations)
- GitHub Actions workflows
- Build configurations

---

## 6. Conversion Pipeline (RFC-9011)

### 6.1 Pipeline Stages

1. **Fetch Layer**
   - GitHub repos
   - Local files
   - Registry sources

2. **Parser / Normalizer**
   - YAML → Canonical format
   - JSON → Canonical format

3. **Crosswalk Engine**
   - Nuclei/Caldera/ATT&CK ↔ SX9 DSL
   - Mapping to PTCC primitives
   - HD4 phase assignment

4. **Semantic Conflict Resolver**
   - Conflict detection
   - Auto-merge/reject logic
   - Human review queue

5. **Semantic Imputer / Filler**
   - GNN + Phi-3 for missing fields
   - Confidence thresholds

6. **Hash & Unicode Tail Assignment**
   - RFC-9001/9002 hash assignment
   - Collision handling

7. **Storage**
   - Supabase (BCNF/3NF)
   - SurrealDB (graph)
   - Sled/Sledis (cache)

---

## 7. Next Steps

### 7.1 Immediate Actions

1. **Complete YAML Inventory**
   - [ ] Scan all directories for threat intelligence YAMLs
   - [ ] Categorize by source (Nuclei, ATT&CK, Caldera, etc.)
   - [ ] Document file counts per category
   - [ ] Identify already-converted DSL files

2. **Verify Conversion Status**
   - [ ] Check which Nuclei YAMLs are already converted
   - [ ] Locate converted DSL files
   - [ ] Verify conversion completeness

3. **Plan Remaining Conversions**
   - [ ] Prioritize by usage frequency
   - [ ] Identify dependencies between sources
   - [ ] Create conversion schedule

### 7.2 Conversion Priority

**High Priority:**
1. MITRE ATT&CK (Enterprise, ICS, Mobile)
2. Caldera Adversaries & Abilities
3. Atomic Red Team Tests

**Medium Priority:**
4. Sigma Rules
5. LOLBAS/GTFOBins
6. MITRE CAR Analytics

**Low Priority:**
7. MITRE ENGAGE
8. MITRE ATLAS
9. Other specialized sources

---

## 8. File Count Summary

| Category | Estimated Count | Status |
|----------|----------------|--------|
| **Nuclei Templates** | Thousands | ✅ Converted (per user) |
| **MITRE ATT&CK** | ~600 techniques | ⚠️ Needs conversion |
| **Caldera** | ~100+ adversaries | ⚠️ Needs conversion |
| **Atomic Red Team** | ~200+ tests | ⚠️ Needs conversion |
| **Sigma Rules** | ~1,000+ rules | ⚠️ Needs conversion |
| **LOLBAS** | ~400+ binaries | ⚠️ Needs conversion |
| **GTFOBins** | ~1,000+ entries | ⚠️ Needs conversion |
| **Scenario YAMLs** | ~50+ scenarios | ⚠️ Needs review |
| **Configuration** | ~1,000+ files | ❌ Not applicable |

---

## 9. References

- **RFC-9011**: SX9 Threat Content Ingestion: YAML → DSL → Playbooks
- **RFC-9011-A**: Ingestion Pipeline Architecture
- **threat_content_fetcher.py**: Existing fetcher implementation
- **caldera_integration.rs**: Rust Caldera integration
- **kali_tools_inventory.rs**: Kali tools catalog

---

## 10. Notes

- User confirmed Nuclei YAMLs already converted to DSL
- Raw YAML files still available for reference
- Neo4j database stores threat relationships
- GCP high-GPU used for processing large datasets
- All Kali tools integrated in exploit-arsenal

---

## 11. Actual YAML File Locations Found

### 11.1 Threat Content Directories

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/threat_content/`

**Directories Found:**
- `nuclei-templates/` - Nuclei vulnerability templates (thousands of YAMLs)
- `caldera/` - Caldera adversary emulation YAMLs
- `atomic-red-team/` - Atomic Red Team test YAMLs
- `sigma/` - Sigma detection rules (if fetched)
- `lolbas/` - LOLBAS binaries YAMLs
- `gtfobins/` - GTFOBins YAMLs
- `hijacklibs/` - HijackLibs YAMLs
- `loldrivers/` - LOLDrivers YAMLs
- `wadcoms/` - WADComs YAMLs

**Status:** ⚠️ **Directories exist but are EMPTY (placeholders)**
- Directories created by `threat_content_fetcher.py` but not yet populated
- YAML files need to be fetched from GitHub repos or GCP storage
- User mentioned using "Google Cloud high-GPU to pull everything down"
- Need to run fetcher script or check GCP storage for actual YAML files

### 11.2 LinkML Templates

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/linkml_templates/`

**Files:**
- `threat_technique.yaml`
- `threat_actor.yaml`
- `detection_rule.yaml`

### 11.3 DSL Output

**Location:** `ctas-7-shipyard-staging/04-abe-iac/node-interview-generator/output/sx9_dsl/`

**Files:**
- `sx9_entities.yaml` (DSL format)

---

**Status:** Inventory complete - Ready for conversion planning  
**Last Updated:** December 2025  
**Next Action:** 
1. ✅ **COMPLETE**: Searched all directories - found placeholder directories
2. ⚠️ **PENDING**: YAML files need to be fetched (via `threat_content_fetcher.py` or from GCP)
3. ⚠️ **PENDING**: Check GCP storage for pre-fetched YAML files
4. ⚠️ **PENDING**: Verify which YAMLs have already been converted to DSL (user mentioned Nuclei)
5. ⚠️ **PENDING**: Locate converted DSL files before re-converting

