# File Size Estimates - Threat Content Downloads & Conversions

**Date:** December 1, 2025  
**Based on:** Current partial downloads + full pipeline estimates

---

## 📊 **CURRENT STATE (Partial Downloads)**

### **Threat Content (42 MB currently)**
```
output/threat_content/
├── mitre_attack.json              33 MB
├── mitre_attack_ics.json         2.2 MB
├── mitre_attack_mobile.json       3.1 MB
├── d3fend.json                    3.0 MB
├── crosswalk_index.json           192 KB
├── exploitdb_index.json           220 KB
├── mitre_index.json               640 KB
├── kali_tools_inventory.json      24 KB
└── [Empty directories - not cloned yet]
    ├── atomic-red-team/           0 B (will be ~500 MB)
    ├── nuclei-templates/          0 B (will be ~200 MB)
    ├── sigma/                     0 B (will be ~50 MB)
    ├── caldera/                   0 B (will be ~100 MB)
    ├── lolbas/                    0 B (will be ~20 MB)
    ├── gtfobins/                  0 B (will be ~10 MB)
    └── ... (other repos)

Total Current: 42 MB
```

---

## 📥 **FULL DOWNLOAD ESTIMATES**

### **1. Raw Threat Content (Git Clones + JSON)**

| Source | Estimated Size | Notes |
|--------|---------------|-------|
| **MITRE ATT&CK** | 40 MB | Enterprise + ICS + Mobile (already downloaded) |
| **MITRE D3FEND** | 3 MB | Already downloaded |
| **Atomic Red Team** | 500 MB | Large repo with YAML tests |
| **Nuclei Templates** | 200 MB | 11,696+ templates |
| **Sigma Rules** | 50 MB | 3,076+ rules |
| **Caldera Abilities** | 100 MB | Adversary emulation |
| **Nmap Scripts** | 20 MB | 612+ scripts |
| **LOLBAS** | 20 MB | Living Off The Land binaries |
| **GTFOBins** | 10 MB | Unix binaries |
| **LOLDrivers** | 15 MB | Driver abuse |
| **HijackLibs** | 10 MB | DLL hijacking |
| **WADComs** | 5 MB | Windows abuse commands |
| **OSINT Tools** | 30 MB | Awesome OSINT, frameworks |
| **Kali Tools** | 1 MB | Inventory JSON (already downloaded) |
| **ExploitDB** | 50 MB | Exploit database |
| **YARA Rules** | 30 MB | Malware detection |
| **Wazuh Rules** | 20 MB | Security rules |

**Total Raw Downloads: ~1.1 GB** (uncompressed)

**Compressed (tar.gz): ~300-400 MB**

---

## 🔄 **PROCESSED OUTPUTS**

### **2. SPIRES Ontology (Current: 6.3 MB)**

| File | Current Size | Estimated Full Size |
|------|-------------|-------------------|
| `ontology.json` | 1.1 MB | **15-20 MB** (20K+ items) |
| `ontology.cypher` | 920 KB | **12-15 MB** (Neo4j import) |
| `ontology.linkml.yaml` | - | **2-3 MB** (LinkML schema) |
| `ontology_enriched.json` | 1.1 MB | **18-25 MB** (with relationships) |
| `ptcc_rules.json` | 492 KB | **5-8 MB** (all rules) |

**Total SPIRES Output: ~50-70 MB**

---

### **3. DSL Conversion (YAML → SX9 DSL)**

| Output | Estimated Size | Notes |
|--------|---------------|-------|
| `sx9_dsl/techniques/` | 10-15 MB | MITRE techniques as DSL |
| `sx9_dsl/rules/` | 8-12 MB | Sigma/YARA/Nuclei as DSL |
| `sx9_dsl/tools/` | 5-8 MB | Tools as DSL entities |
| `sx9_dsl/atomic_tests/` | 15-20 MB | Atomic Red Team tests |
| `sx9_dsl/index.json` | 2-3 MB | Cross-reference index |

**Total DSL Output: ~40-60 MB**

---

### **4. Task Graph Nodes (RFC-9001/9002 Compliant)**

| File | Estimated Size | Structure |
|------|---------------|-----------|
| `task_graph/threat_tools_graph.json` | 25-35 MB | All tools as nodes with hashes |
| `task_graph/task_relationships.json` | 10-15 MB | Predecessor/successor links |
| `task_graph/dual_trivariate_hashes.json` | 8-12 MB | Primary + secondary hashes |
| `task_graph/unicode_operations.json` | 2-3 MB | Unicode Assembly mappings |
| `task_graph/hd4_phase_mapping.json` | 1-2 MB | HD4 phase classifications |

**Total Task Graph: ~45-65 MB**

---

### **5. Hash Artifacts (RFC-9001)**

| File | Estimated Size | Content |
|------|---------------|---------|
| `hashes/primary_hashes.json` | 5-8 MB | Primary trivariate hashes (48 chars each) |
| `hashes/secondary_hashes.json` | 5-8 MB | Secondary trivariate hashes |
| `hashes/unicode_mappings.json` | 2-3 MB | Hash → Unicode (U+E000-E9FF) |
| `hashes/hash_index.json` | 1-2 MB | Fast lookup index |

**Total Hashes: ~13-21 MB**

---

## 📦 **TOTAL STORAGE REQUIREMENTS**

### **By Stage:**

| Stage | Size | Notes |
|-------|------|-------|
| **1. Raw Downloads** | 1.1 GB | Git clones + JSON files |
| **2. SPIRES Ontology** | 50-70 MB | Processed ontology |
| **3. DSL Conversion** | 40-60 MB | YAML → SX9 DSL |
| **4. Task Graph** | 45-65 MB | Graph nodes + relationships |
| **5. Hash Artifacts** | 13-21 MB | RFC-9001/9002 hashes |

### **Total Uncompressed: ~1.25-1.3 GB**

### **Compressed (tar.gz): ~400-500 MB**

---

## 💾 **DISK SPACE BREAKDOWN**

```
output/
├── threat_content/         1.1 GB   (raw downloads)
├── ontology/                50-70 MB (SPIRES)
├── sx9_dsl/                 40-60 MB (DSL conversion)
├── task_graph/              45-65 MB (graph nodes)
└── hashes/                  13-21 MB (RFC-9001/9002)

Total: ~1.25-1.3 GB
```

---

## 🚀 **GCP INSTANCE REQUIREMENTS**

### **Minimum Disk:**
- **Boot Disk:** 100 GB (sufficient for downloads + processing)
- **Recommended:** 150 GB (headroom for processing)

### **Memory:**
- **Minimum:** 4 GB RAM (for SPIRES processing)
- **Recommended:** 8 GB RAM (for faster processing)

### **Network:**
- **Download:** ~1.1 GB (one-time)
- **Upload (packaged):** ~400-500 MB (compressed)

---

## 📊 **COMPARISON: Current vs Full**

| Component | Current | Full Pipeline | Increase |
|-----------|---------|---------------|----------|
| **Raw Downloads** | 42 MB | 1.1 GB | **26x** |
| **SPIRES Ontology** | 6.3 MB | 50-70 MB | **8-11x** |
| **DSL Conversion** | 0 MB | 40-60 MB | **New** |
| **Task Graph** | 0 MB | 45-65 MB | **New** |
| **Hashes** | 0 MB | 13-21 MB | **New** |
| **Total** | 48 MB | 1.25-1.3 GB | **26x** |

---

## ⚡ **OPTIMIZATION TIPS**

1. **Compress after processing:** Use `tar.gz` for storage (400-500 MB vs 1.3 GB)
2. **Delete raw repos:** Keep only processed JSON/DSL (saves ~800 MB)
3. **Stream processing:** Process and delete as you go (saves disk space)
4. **Selective download:** Use `--no-training` flag to skip ML model files

---

## 📝 **NOTES**

- **Current downloads are partial** - many repos not cloned yet (0 B directories)
- **SPIRES processing** adds ~10-15% to raw data size (enriched with relationships)
- **DSL conversion** is roughly same size as raw YAML (structured differently)
- **Task graph** adds structured relationships (predecessors/successors)
- **Hashes** are small but critical for RFC-9001/9002 compliance

**All estimates based on:**
- Current partial downloads (42 MB)
- Known repo sizes (GitHub)
- SPIRES output patterns (6.3 MB current → 50-70 MB full)
- Average JSON compression ratios



