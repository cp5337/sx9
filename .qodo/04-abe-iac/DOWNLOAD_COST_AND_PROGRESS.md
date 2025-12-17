# Threat Content Download - Cost & Progress Monitoring

**Date:** 2025-12-07  
**Purpose:** Track download progress and costs

---

## 💰 **COST BREAKDOWN**

### **Data Download Costs**
- **Data Ingress (Downloading):** **FREE** (all cloud providers)
- **Data Egress (if needed):** ~$0.09/GB
- **Download Cost: $0** ✅

### **Compute Costs (If Running on GCP)**
- **Preemptible CPU Instance:** ~$0.10/hour
- **Estimated Download Time:** 30-60 minutes
- **Estimated Total Cost:** **~$0.40-0.60** ✅

### **Total Estimated Cost**
- **Best Case (Local):** **$0** (just bandwidth, which is free for ingress)
- **GCP Preemptible:** **~$0.40-0.60**
- **GCP Standard:** **~$1.00-2.00** (not recommended)

---

## 📊 **DATA SIZE ESTIMATE**

| Category | Size (Compressed) | Size (Uncompressed) |
|----------|-------------------|---------------------|
| MITRE Suite | 165 MB | ~300 MB |
| Adversary Emulation | 600 MB | ~1.2 GB |
| Detection Rules | 2.75 GB | ~5 GB |
| LOLTL | 240 MB | ~500 MB |
| OSINT | 115 MB | ~200 MB |
| Threat Intel Feeds | 9 GB | ~15 GB |
| Historical/Archive | 12 GB | ~20 GB |
| **TOTAL** | **~27 GB** | **~50-75 GB** |

---

## 📥 **PROGRESS MONITORING**

### **Run with Progress Monitor**

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac
./monitor_download.sh
```

**What it shows:**
- ✅ Real-time file counts per source
- ✅ Total size downloaded
- ✅ SPIRES ontology status
- ✅ Elapsed time and cost tracking
- ✅ Recent log activity
- ✅ Final summary

### **Manual Monitoring**

```bash
# Watch log file
tail -f /tmp/threat_download.log

# Check output directory
ls -lh output/threat_content/

# Check file counts
find output/threat_content -type f | wc -l

# Check SPIRES ontology
ls -lh ../output/ontology/
```

---

## 📋 **EXPECTED PROGRESS**

### **Phase 1: MITRE Suite** (~5-10 minutes)
- MITRE ATT&CK Enterprise
- MITRE ATT&CK ICS
- MITRE ATT&CK Mobile
- MITRE D3FEND
- MITRE CAR
- MITRE ENGAGE
- MITRE ATLAS

### **Phase 2: Adversary Emulation** (~10-15 minutes)
- Atomic Red Team (~200+ tests)
- Caldera Adversaries

### **Phase 3: Detection Rules** (~15-20 minutes)
- Nuclei Templates (thousands)
- Sigma Rules (~1,000+ rules)
- YARA Rules (~500+ rules)
- Wazuh Rules (~1,000+ rules)

### **Phase 4: LOLTL** (~5-10 minutes)
- LOLBAS (~400+ binaries)
- GTFOBins (~1,000+ entries)
- LOLDrivers (~200+ drivers)
- HijackLibs (~100+ libraries)
- WADComs (~300+ commands)

### **Phase 5: OSINT** (~5 minutes)
- OSINT Framework
- Awesome OSINT
- OSINT Map CSV

### **Phase 6: SPIRES Ontology** (~5-10 minutes)
- Extract terms and relations
- Generate JSON, Cypher, SurrealQL
- Deduplicate with existing ontology

### **Phase 7: YAML to DSL** (~10-15 minutes)
- Convert YAML rules to SX9 DSL
- Generate dual-trivariate hashes
- Map to Unicode operations

**Total Estimated Time: 30-60 minutes**

---

## ✅ **SUCCESS CRITERIA**

### **Download Complete When:**
- ✅ All source directories created
- ✅ File counts match expected ranges
- ✅ SPIRES ontology generated (ontology_raw.json exists)
- ✅ DSL conversion complete (sx9_dsl directory exists)
- ✅ No errors in log file

### **Expected File Counts:**
- MITRE ATT&CK: 3+ files
- Atomic Red Team: 200+ files
- Nuclei Templates: 1,000+ files
- Sigma Rules: 1,000+ files
- YARA Rules: 500+ files
- Wazuh Rules: 1,000+ files
- LOLBAS: 400+ files
- GTFOBins: 1,000+ files

**Total Expected: 5,000+ files**

---

## 🔍 **VERIFICATION**

### **Check Download Status**

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac
python3 check_existing_content.py
```

**This will show:**
- Which sources are downloaded
- File counts per source
- SPIRES ontology status
- What's missing

---

## 📝 **LOG FILES**

- **Download Log:** `/tmp/threat_download.log` (or `logs/download_*.log`)
- **SPIRES Log:** Check console output for ontology generation
- **DSL Log:** Check console output for DSL conversion

---

## 🚨 **TROUBLESHOOTING**

### **If Download Stalls:**
1. Check log file for errors
2. Verify network connection
3. Check disk space (`df -h`)
4. Verify Python dependencies

### **If Cost Concerns:**
- Run locally (cost: $0)
- Use preemptible instances (80% savings)
- Monitor with `monitor_download.sh`

---

## 💡 **QUICK START**

```bash
# Start download with monitoring
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac
./monitor_download.sh

# Or run directly
cd node-interview-generator
python3 threat_content_fetcher.py --all --no-training
```

**The monitor will show real-time progress, file counts, and cost tracking.**



