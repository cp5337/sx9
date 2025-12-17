# Training Workflow - Download First, Then Train

**Date:** 2025-12-07

---

## ✅ **WORKFLOW ORDER**

The script follows this exact order:

1. **DOWNLOAD** - Fetch all 27+ threat intelligence sources
2. **SPIRES** - Generate ontology from downloaded data
3. **DSL CONVERSION** - Convert YAMLs to DSL (with TOML output now)
4. **TRAINING** - Train DistilBERT, Phi-3, GNN on downloaded data

**Training CANNOT run without downloaded data first!**

---

## 📥 **STEP 1: DOWNLOAD (Currently Running)**

**Command:**
```bash
python3 threat_content_fetcher.py --all --no-training
```

**What it downloads:**
- MITRE ATT&CK (Enterprise, ICS, Mobile)
- MITRE Defense Suite (D3FEND, CAR, ATLAS, ENGAGE)
- Atomic Red Team
- Nuclei Templates
- Sigma Rules
- YARA Rules
- Wazuh Rules
- Nmap Scripts
- LOLBAS, GTFOBins, etc.
- ExploitDB
- OSINT Resources
- Kali Tools
- PTCC Configurations
- TETH Algorithms

**Output:** `output/threat_content/`

**Status:** Check with `ps aux | grep threat_content_fetcher`

---

## 🧠 **STEP 2: SPIRES ONTOLOGY (After Download)**

**Automatic** - Runs after downloads complete (unless `--no-ontology`)

**Output:** `output/ontology/ontology_raw.json`

---

## 🔧 **STEP 3: DSL CONVERSION (After Download)**

**Automatic** - Runs after downloads complete (unless `--no-dsl`)

**Output:** 
- `output/sx9_dsl/sx9_entities.yaml`
- `output/sx9_dsl/sx9_entities.json`
- `output/sx9_dsl/sx9_entities.toml` ✅ (NOW ADDED)

---

## 🤖 **STEP 4: DISTILBERT TRAINING (After Download)**

**Command (after downloads complete):**
```bash
python3 threat_content_fetcher.py --train-only
```

**OR run full pipeline:**
```bash
python3 threat_content_fetcher.py --all
# (removes --no-training flag)
```

**What it needs:**
- ✅ Downloaded threat content in `output/threat_content/`
- ✅ `sx9-ml-models` package installed
- ✅ GPU access (H100 recommended)

**Output:** `output/models/distilbert-mitre/`

**Cost:** ~$13-20 (4-6 hours on H100 @ $3.29/hr)

---

## ⚠️ **IMPORTANT**

**Training will FAIL if:**
- Downloads haven't completed
- `output/threat_content/` is empty
- `sx9-ml-models` package not installed

**Check download status:**
```bash
# Check if process is running
ps aux | grep threat_content_fetcher

# Check downloaded files
ls -lh output/threat_content/

# Check file count
find output/threat_content -type f | wc -l
```

---

## 🚀 **RECOMMENDED APPROACH**

1. **Wait for downloads to complete** (30-60 minutes)
2. **Verify downloads** - Check `output/threat_content/` has files
3. **Then run training:**
   ```bash
   python3 threat_content_fetcher.py --train-only
   ```

**OR run everything in one go:**
```bash
python3 threat_content_fetcher.py --all
# (downloads → SPIRES → DSL → Training)
```

---

**Current Status:** Downloads in progress... ⏳



