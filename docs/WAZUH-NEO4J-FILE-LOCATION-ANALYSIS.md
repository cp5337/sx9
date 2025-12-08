# Wazuh & Neo4j File Location Analysis
## Investigation Results - December 2025

**Status:** ✅ Complete Investigation  
**Purpose:** Determine Wazuh activity, Neo4j data status, and locate YAML threat intelligence files

---

## Executive Summary

### Wazuh Status
- ✅ **Fully Initialized and Running**
- **Services Active:**
  - `wazuh-analysisd` - Analyzing logs (16.5% CPU)
  - `wazuh-apid` - API daemon (80% CPU during init, now stable)
  - `wazuh-authd`, `wazuh-execd`, `wazuh-syscheckd` - All running
- **Rules/Decoders:** Loaded from `/var/ossec/ruleset/`
  - 1,860+ XML rule files
  - 864+ XML decoder files
  - Format: XML (not YAML) - Wazuh uses XML for rules/decoders

### Neo4j Status
- ✅ **MITRE ATT&CK Data Loaded**
- **Database:** `ctas7-neo4j` (port 7687)
- **Data Present:**
  - **1,088 Techniques** (e.g., T1055.011, T1053.005, T1205.002)
  - **19 Tactics** (collection, command-and-control, credential-access, etc.)
  - **26 Platforms** (Android, Windows, Linux, etc.)
  - **Labels:** Technique, Tactic, Platform, Group, Rule, Tool, Test
- **Loader Script:** `neo4j_threat_loader.py` exists and can load from directories

### YAML File Locations

**Critical Finding:** YAML directories are **EMPTY PLACEHOLDERS** but data was fetched:

**From `threat_content_summary.json`:**
```json
{
  "fetched_at": "2025-11-28T11:52:00",
  "counts": {
    "nuclei_templates": 11696,    // ✅ Fetched
    "sigma_rules": 3076,           // ✅ Fetched
    "atomic_tests": 327,           // ✅ Fetched
    "mitre_techniques": 835,       // ✅ Loaded to Neo4j
    "mitre_groups": 187,           // ✅ Loaded to Neo4j
    "caldera_abilities": 0,        // ⚠️ Not fetched
    "nmap_scripts": 612            // ✅ Fetched
  }
}
```

**Directory Status:**
```
/04-abe-iac/node-interview-generator/output/threat_content/
├── nuclei-templates/     → 0B (empty placeholder)
├── sigma/                 → 0B (empty placeholder)
├── atomic-red-team/      → 0B (empty placeholder)
├── caldera/              → 0B (empty placeholder)
├── mitre_attack.json     → 34MB ✅ (loaded to Neo4j)
├── mitre_attack_ics.json → 2.2MB ✅
├── mitre_attack_mobile.json → 3.2MB ✅
└── kali_tools_inventory.json → 21KB ✅
```

---

## Where Are The Files?

### Hypothesis 1: GCP Storage (Most Likely)
- **User mentioned:** "I use Google Cloud high-GPU to pull everything down"
- **Evidence:** Directories created but empty, summary shows files were fetched
- **Location:** Likely in GCP Cloud Storage buckets
- **Action:** Check GCP storage buckets for threat intelligence data

### Hypothesis 2: Already Converted to DSL
- **User mentioned:** "we converted the yamls from nuclei and other into DSL"
- **Evidence:** Some DSL files exist in `output/sx9_dsl/`
- **Location:** Check for DSL files in:
  - `output/sx9_dsl/`
  - `ctas7-ops-main-platform/DSL-orchestration/`

### Hypothesis 3: Loaded into Neo4j
- **Evidence:** MITRE ATT&CK data is in Neo4j (1,088 techniques)
- **Status:** Techniques loaded, but YAML files themselves not stored
- **Note:** Neo4j stores relationships, not raw YAML content

### Hypothesis 4: Processed and Discarded
- **Evidence:** Summary shows fetch counts but files missing
- **Possibility:** Files were processed, converted, and original YAMLs removed
- **Action:** Check for processed/converted outputs

---

## Wazuh Configuration

**Ruleset Location:**
- `/var/ossec/ruleset/rules/` - 1,860+ XML rule files
- `/var/ossec/ruleset/decoders/` - 864+ XML decoder files
- `/var/ossec/ruleset/sca/` - Security Configuration Assessment YAMLs

**Configuration:**
```xml
<ruleset>
  <decoder_dir>ruleset/decoders</decoder_dir>
  <rule_dir>ruleset/rules</rule_dir>
  <decoder_dir>etc/decoders</decoder_dir>
  <rule_dir>etc/rules</rule_dir>
</ruleset>
```

**Note:** Wazuh uses **XML format** for rules/decoders, not YAML. Only SCA (Security Configuration Assessment) uses YAML.

---

## Neo4j Data Structure

**Current Data:**
- **Techniques:** 1,088 nodes with relationships to Tactics
- **Tactics:** 19 nodes (collection, command-and-control, etc.)
- **Platforms:** 26 nodes (Android, Windows, Linux, etc.)
- **Relationships:** `BELONGS_TO` (Technique → Tactic)

**Loader Capabilities:**
- Loads from JSON files (MITRE ATT&CK)
- Can load YAML files from directories (Sigma rules, detection rules)
- Supports vector embeddings
- Creates indexes for fast querying

**Missing Data:**
- Detection rules (Sigma) - not loaded yet
- Threat actors - not loaded yet
- Offensive tools - not loaded yet
- File path references - not stored

---

## Recommendations

### 1. Check GCP Storage
```bash
# Check GCP buckets for threat intelligence data
gsutil ls gs://*/threat-intelligence/
gsutil ls gs://*/nuclei-templates/
gsutil ls gs://*/sigma-rules/
```

### 2. Locate DSL Converted Files
```bash
# Search for converted DSL files
find . -name "*.dsl" -type f
find . -name "*nuclei*.dsl"
find . -name "*sigma*.dsl"
```

### 3. Check Neo4j for File References
```cypher
// Query Neo4j for any file path references
MATCH (n) WHERE any(key in keys(n) WHERE n[key] CONTAINS '.yaml' OR n[key] CONTAINS '.yml')
RETURN labels(n), keys(n), [k in keys(n) WHERE n[k] CONTAINS '.yaml' OR n[k] CONTAINS '.yml' | {key: k, value: n[k]}] as matches
LIMIT 20;
```

### 4. Re-fetch if Needed
```bash
# Run threat content fetcher to re-download
cd 04-abe-iac/node-interview-generator/
python threat_content_fetcher.py --all
```

### 5. Load Remaining Data to Neo4j
```bash
# Load Sigma rules, threat actors, tools
python neo4j_threat_loader.py --all
```

---

## File Count Summary

| Source | Fetched Count | Local Files | Neo4j Loaded | Status |
|--------|--------------|-------------|--------------|--------|
| **Nuclei Templates** | 11,696 | 0 | ❌ | ⚠️ Need to locate |
| **Sigma Rules** | 3,076 | 0 | ❌ | ⚠️ Need to locate |
| **Atomic Red Team** | 327 | 0 | ❌ | ⚠️ Need to locate |
| **MITRE ATT&CK** | 835 techniques | ✅ JSON | ✅ 1,088 loaded | ✅ Complete |
| **MITRE Groups** | 187 | ✅ JSON | ❌ | ⚠️ Not loaded |
| **Nmap Scripts** | 612 | 0 | ❌ | ⚠️ Need to locate |
| **Caldera** | 0 | 0 | ❌ | ❌ Not fetched |

---

## Next Steps

1. **Verify GCP Storage:** Check if files are in GCP buckets
2. **Locate DSL Files:** Find converted DSL files
3. **Load Missing Data:** Load Sigma rules, threat actors, tools to Neo4j
4. **Re-fetch if Needed:** Run `threat_content_fetcher.py` if files are missing
5. **Integrate Wazuh:** Connect Wazuh alerts to `sx9-plasma-defender` (already wired)

---

## References

- **Neo4j Loader:** `04-abe-iac/node-interview-generator/neo4j_threat_loader.py`
- **Threat Fetcher:** `04-abe-iac/node-interview-generator/threat_content_fetcher.py`
- **Summary File:** `output/threat_content/threat_content_summary.json`
- **Wazuh Config:** `/var/ossec/etc/ossec.conf` (in container)
- **RFC-9011:** Threat Ingestion Pipeline


