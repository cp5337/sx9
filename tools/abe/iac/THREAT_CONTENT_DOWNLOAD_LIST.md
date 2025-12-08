# Complete Threat Content Download List

**Last Updated:** 2025-12-07  
**Source:** `threat_content_fetcher.py` - `fetch_all()` method

---

## 📊 **DOWNLOAD SUMMARY**

**Total Sources:** 24+ threat intelligence sources  
**Total Categories:** 8 major categories  
**Estimated Size:** ~1.1 GB raw, ~200 MB processed

---

## 🎯 **1. MITRE ATT&CK SUITE** (3 sources)

### Enterprise ATT&CK
- **Source:** `mitre/cti` GitHub (JSON)
- **URL:** `https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json`
- **Size:** ~33 MB
- **Content:** Techniques, tactics, groups, software, campaigns
- **Status:** ✅ Already downloaded

### ICS ATT&CK
- **Source:** `mitre/cti` GitHub (JSON)
- **URL:** `https://raw.githubusercontent.com/mitre/cti/master/ics-attack/ics-attack.json`
- **Size:** ~2.2 MB
- **Content:** ICS-specific attack techniques
- **Status:** ✅ Already downloaded

### Mobile ATT&CK
- **Source:** `mitre/cti` GitHub (JSON)
- **URL:** `https://raw.githubusercontent.com/mitre/cti/master/mobile-attack/mobile-attack.json`
- **Size:** ~3.1 MB
- **Content:** Mobile device attack techniques
- **Status:** ✅ Already downloaded

---

## 🛡️ **2. MITRE DEFENSE SUITE** (4 sources)

### D3FEND
- **Source:** MITRE D3FEND (JSON)
- **URL:** `https://d3fend.mitre.org/ontologies/d3fend.json`
- **Size:** ~3.0 MB
- **Content:** Countermeasure ontology
- **Status:** ✅ Already downloaded

### CAR (Cyber Analytics Repository)
- **Source:** `mitre-attack/car` GitHub (YAML)
- **Repo:** `https://github.com/mitre-attack/car`
- **Path:** `analytics/`
- **Content:** Detection analytics, data models, sensors
- **Status:** ✅ Downloaded (425 files)

### ATLAS
- **Source:** `mitre-atlas/atlas-data` GitHub (YAML)
- **Repo:** `https://github.com/mitre-atlas/atlas-data`
- **Path:** `data/`
- **Content:** AI/ML adversarial attack techniques
- **Status:** ✅ Downloaded (97 files)

### ENGAGE
- **Source:** `mitre/engage` GitHub (YAML)
- **Repo:** `https://github.com/mitre/engage`
- **Path:** `data/`
- **Content:** Adversary engagement activities
- **Status:** ⚠️ Needs download

---

## 🎭 **3. ADVERSARY EMULATION** (2 sources)

### Atomic Red Team
- **Source:** `redcanaryco/atomic-red-team` GitHub (YAML)
- **Repo:** `https://github.com/redcanaryco/atomic-red-team`
- **Path:** `atomics/`
- **Content:** Thousands of atomic test definitions
- **Status:** ⚠️ Directory exists but empty (0 files)

### Caldera
- **Source:** `mitre/caldera` GitHub (YAML)
- **Repo:** `https://github.com/mitre/caldera`
- **Path:** `data/`
- **Content:** Adversary profiles, abilities, plugins
- **Status:** ⚠️ Directory exists but empty (0 files)

---

## 🔍 **4. DETECTION RULES** (4 sources)

### Nuclei Templates
- **Source:** `projectdiscovery/nuclei-templates` GitHub (YAML)
- **Repo:** `https://github.com/projectdiscovery/nuclei-templates`
- **Content:** Thousands of vulnerability detection templates
- **Status:** ⚠️ Directory exists but empty (0 files)

### Sigma Rules
- **Source:** `SigmaHQ/sigma` GitHub (YAML)
- **Repo:** `https://github.com/SigmaHQ/sigma`
- **Path:** `rules/`
- **Content:** Thousands of SIEM detection rules
- **Status:** ⚠️ Directory exists but empty (0 files)

### YARA Rules
- **Source:** `Yara-Rules/rules` GitHub (YAR)
- **Repo:** `https://github.com/Yara-Rules/rules`
- **Content:** Malware detection signatures
- **Status:** ⚠️ Directory exists but empty (0 files)

### Wazuh Rules
- **Source:** `wazuh/wazuh` GitHub (XML)
- **Repo:** `https://github.com/wazuh/wazuh`
- **Path:** `ruleset/rules/`
- **Content:** Wazuh SIEM detection rules
- **Status:** ⚠️ Directory exists but empty (0 files)

---

## 🔎 **5. RECONNAISSANCE** (1 source)

### Nmap NSE Scripts
- **Source:** `nmap/nmap` GitHub (Lua)
- **Repo:** `https://github.com/nmap/nmap`
- **Path:** `scripts/`
- **Content:** Network scanning scripts
- **Status:** ⚠️ Directory exists but empty (0 files)

---

## 🦠 **6. LIVING OFF THE LAND (LOLTL)** (5 sources)

### LOLBAS
- **Source:** `LOLBAS-Project/LOLBAS` GitHub (YAML)
- **Repo:** `https://github.com/LOLBAS-Project/LOLBAS`
- **Path:** `yml/OSBinaries/`
- **Content:** Windows binaries used for living off the land
- **Status:** ⚠️ Directory exists but empty (0 files)

### GTFOBins
- **Source:** `GTFOBins/GTFOBins.github.io` GitHub (Markdown)
- **Repo:** `https://github.com/GTFOBins/GTFOBins.github.io`
- **Path:** `_gtfobins/`
- **Content:** Unix binaries for privilege escalation
- **Status:** ⚠️ Directory exists but empty (0 files)

### LOLDrivers
- **Source:** `magicsword-io/LOLDrivers` GitHub (YAML)
- **Repo:** `https://github.com/magicsword-io/LOLDrivers`
- **Path:** `yaml/drivers/`
- **Content:** Vulnerable Windows drivers
- **Status:** ⚠️ Directory exists but empty (0 files)

### HijackLibs
- **Source:** `wietze/HijackLibs` GitHub (YAML)
- **Repo:** `https://github.com/wietze/HijackLibs`
- **Path:** `yml/`
- **Content:** DLL hijacking database
- **Status:** ⚠️ Directory exists but empty (0 files)

### WADComs
- **Source:** `WADComs/WADComs.github.io` GitHub (Markdown)
- **Repo:** `https://github.com/WADComs/WADComs.github.io`
- **Path:** `_wadcoms/`
- **Content:** Windows/AD offensive cheatsheets
- **Status:** ⚠️ Directory exists but empty (0 files)

---

## 🐧 **7. KALI TOOLS** (1 source)

### Kali Tools Inventory
- **Source:** Embedded inventory (from `ctas7-exploit-arsenal`)
- **Method:** `fetch_kali_tools()` - indexes from embedded categories
- **Content:** 74+ tools across 11 categories:
  - NetworkRecon (nmap, masscan, netdiscover, arp-scan, unicornscan)
  - WebApplicationTesting (nikto, sqlmap, gobuster, dirb, wfuzz, burpsuite, zaproxy)
  - ExploitationFrameworks (metasploit, armitage, beef-xss, social-engineer-toolkit)
  - PasswordCracking (hydra, hashcat, john, medusa, ncrack, ophcrack)
  - WirelessNetworks (aircrack-ng, wifite, reaver, fern-wifi-cracker, kismet)
  - OSINT (theharvester, recon-ng, maltego, spiderfoot, shodan)
  - Forensics (autopsy, binwalk, volatility, sleuthkit, foremost)
  - ReverseEngineering (ghidra, radare2, gdb, objdump, ida-free)
  - Sniffing (wireshark, tcpdump, ettercap, bettercap, dsniff)
  - VulnerabilityAnalysis (nessus, openvas, nikto, lynis, wapiti)
  - PostExploitation (empire, covenant, sliver, bloodhound, mimikatz)
- **Status:** ✅ Indexed (but not scraped from kali.org)

**Note:** Full scraping requires `kali_tools_scraper.py` to get all ~600+ tools from kali.org

---

## 🌐 **8. OSINT RESOURCES** (3 sources)

### Awesome OSINT
- **Source:** `jivoi/awesome-osint` GitHub (Markdown)
- **Repo:** `https://github.com/jivoi/awesome-osint`
- **Path:** `README.md`
- **Content:** Comprehensive OSINT tools list
- **Status:** ⚠️ Directory exists but empty (0 files)
- **Note:** ✅ **Used as reference** - Already built OSINT systems (`sx9_solutions_osint`, `ctas7-nyx-osint`) based on this. Download for completeness/reference only.

### OSINT Framework
- **Source:** `lockfale/osint-framework` GitHub (JSON)
- **Repo:** `https://github.com/lockfale/osint-framework`
- **Path:** `arf.json`
- **Content:** Structured OSINT resource tree
- **Status:** ⚠️ Directory exists but empty (0 files)

### Sherlock Sites
- **Source:** `sherlock-project/sherlock` GitHub (JSON)
- **Repo:** `https://github.com/sherlock-project/sherlock`
- **Path:** `sherlock/resources/data.json`
- **Content:** Username search across 500+ sites
- **Status:** ⚠️ Directory exists but empty (0 files)

---

## 📦 **POST-PROCESSING**

After download, the pipeline:

1. **SPIRES Ontology Extraction**
   - Extracts terms, relationships, categories
   - Generates JSON, Cypher (Neo4j), SurrealQL (SurrealDB)
   - **Current:** 1,730 terms, 1,089 relations

2. **YAML to DSL Conversion**
   - Converts YAML rules to SX9 DSL format
   - Generates dual-trivariate hashes (RFC-9001)
   - Maps to Unicode operations (RFC-9002)

3. **ML Model Training** (optional)
   - DistilBERT fine-tuning
   - Phi-3 LoRA training
   - GNN training

---

## ⚠️ **CURRENT STATUS**

**✅ Downloaded:**
- MITRE ATT&CK (Enterprise, ICS, Mobile) - 3 files
- D3FEND - 1 file
- CAR - 425 files
- ATLAS - 97 files
- SPIRES Ontology - 1,730 terms

**❌ Missing/Empty:**
- ENGAGE - not downloaded
- Atomic Red Team - 0 files
- Nuclei Templates - 0 files
- Sigma Rules - 0 files
- YARA Rules - 0 files
- Wazuh Rules - 0 files
- Nmap Scripts - 0 files
- LOLBAS - 0 files
- GTFOBins - 0 files
- LOLDrivers - 0 files
- HijackLibs - 0 files
- WADComs - 0 files
- Awesome OSINT - 0 files
- OSINT Framework - 0 files
- Sherlock - 0 files
- Kali Tools - only indexed, not scraped

---

## 🚀 **TO DOWNLOAD EVERYTHING**

```bash
cd 04-abe-iac/node-interview-generator
python threat_content_fetcher.py --all
```

This will:
1. Download all missing sources
2. Generate SPIRES ontology (with deduplication)
3. Convert YAMLs to DSL
4. Train ML models (if enabled)

---

**The lattice is watching.** 🔥

