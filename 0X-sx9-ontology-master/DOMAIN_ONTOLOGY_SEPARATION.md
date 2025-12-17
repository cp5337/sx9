# 🎯 CTAS-7 Domain Ontology Separation Strategy

**Date:** November 9, 2025  
**Purpose:** Separate intelligence collection by domain for targeted, cost-effective OSINT stacks

---

## 🧠 THE PROBLEM WITH ONE MASSIVE GRAPH

**Old Approach (Expensive & Messy):**
- Single 165-task graph with everything mixed
- LLMs processing low-value noise
- No domain expertise
- High token costs for irrelevant data

**New Approach (Domain-Specific Needle-Rich Hay):**
- Separate ontologies per domain
- Domain-specific OSINT stacks
- Pre-filtered, high-value targets for LLMs
- Cost comparison: Old-school vs. AI

---

## 📊 DOMAIN ONTOLOGY BREAKDOWN

### **1. CYBER DOMAIN (Plasma Core)** 🔴

**Purpose:** Wazuh + AXON + Kali tools orchestration

**Knowledge Sources:**
- **Exploit-DB** (CVEs, exploits, PoCs)
- **MITRE ATT&CK** (tactics, techniques, procedures)
- **MITRE CALDERA** (adversary emulation)
- **PTCC Framework** (threat classification, all 7 levels)
- **APT Groups** (geopolitical heuristics, TTPs)
- **Kali Tool Catalog** (164 tools mapped to tasks)
- **Wazuh Rules** (detection signatures)
- **NIST CVE Database** (vulnerability feeds)
- **CWE (Common Weakness Enumeration)**
- **CAPEC (Common Attack Pattern Enumeration)**

**OSINT Stack (Pre-AI):**
1. **Scrapy** → Exploit-DB, CVE feeds, APT reports
2. **RSS/Atom** → Security blogs, vendor advisories
3. **GitHub API** → PoC exploits, security tools
4. **Twitter/X API** → Threat intel accounts
5. **Shodan API** → Exposed infrastructure
6. **VirusTotal API** → Malware signatures

**AI Layer (Post-Filter):**
- Marcus (Gemini 2M) → Entity extraction, PTCC classification
- Elena (Grok) → APT geopolitical mapping
- Natasha (GPT-4) → Wazuh rule generation

**Output:**
- Wazuh detection rules (YAML)
- CALDERA adversary profiles (YAML)
- Kali tool → CTAS task mappings (JSON)
- PTCC threat matrix (SurrealDB graph)

**Cost Comparison:**
- **Old-school OSINT:** ~$50/month (APIs, compute)
- **LLM processing (filtered):** ~$100/month (10K high-value targets)
- **LLM processing (unfiltered):** ~$2,000/month (500K low-value targets)
- **Savings:** 95% reduction

---

### **2. GEOSPATIAL DOMAIN (Infrastructure)** 🌍

**Purpose:** Critical infrastructure mapping, target selection

**Knowledge Sources:**
- **Submarine Cable Landing Stations** (TeleGeography, Infrapedia)
- **Power Grid** (EIA, HIFLD, OpenInfra)
- **Internet Backbone** (BGPView, PeeringDB, RIPE)
- **GeoIP Databases** (MaxMind, IP2Location)
- **Google Earth Engine** (satellite imagery, nightlights)
- **OpenStreetMap** (infrastructure layers)
- **Natural Earth Data** (borders, cities)

**OSINT Stack (Pre-AI):**
1. **GEE Python API** → Bulk satellite data download
2. **Overpass API** → OSM infrastructure queries
3. **REST APIs** → EIA, TeleGeography, BGPView
4. **KMZ/GeoJSON parsers** → Spatial data extraction
5. **PostGIS** → Geospatial indexing

**AI Layer (Post-Filter):**
- Zoe (Orbital) → Optimal ground station placement
- Marcus (Gemini 2M) → Infrastructure vulnerability analysis
- Cove (Claude) → Geospatial relationship mapping

**Output:**
- Legion ECS entities (power, telecom, internet)
- Mapbox/Cesium layers (KMZ, GeoJSON)
- SurrealDB geospatial nodes
- Target selection for scenarios

**Cost Comparison:**
- **Old-school OSINT:** ~$200/month (GEE, APIs)
- **LLM processing (filtered):** ~$150/month (5K infrastructure targets)
- **LLM processing (unfiltered):** ~$5,000/month (1M OSM nodes)
- **Savings:** 97% reduction

---

### **3. WMD DOMAIN (CBRNE)** ☢️

**Purpose:** Chemical, Biological, Radiological, Nuclear, Explosive intelligence

**Knowledge Sources:**
- **Australia Group Precursor Lists** (chemical, biological)
- **IAEA Safeguards** (nuclear fuel cycle)
- **OPCW** (chemical weapons convention)
- **CDC Select Agents** (biological threat list)
- **ATF Explosives Database** (precursors, IEDs)
- **Nuclear Threat Initiative (NTI)** (proliferation data)
- **Your IED TTL** (task structure, indicators)

**OSINT Stack (Pre-AI):**
1. **PDF extraction** → Australia Group lists, IAEA reports
2. **Scrapy** → NTI database, CDC select agents
3. **Docling** → Scientific papers (proliferation)
4. **NLTK** → Entity extraction (precursors, facilities)

**AI Layer (Post-Filter):**
- Marcus (Gemini 2M) → Proliferation pathway analysis
- Elena (Grok) → Geopolitical risk assessment
- Natasha (GPT-4) → Scenario narrative generation

**Output:**
- Precursor → CTAS task mappings
- Proliferation cycle graphs (fuel, chemical)
- Scenario templates (Bio, Nuclear, Radiological)
- Third-click depth (hidden unless you know)

**Cost Comparison:**
- **Old-school OSINT:** ~$20/month (PDF processing)
- **LLM processing (filtered):** ~$50/month (500 high-value entities)
- **LLM processing (unfiltered):** ~$1,000/month (50K scientific papers)
- **Savings:** 95% reduction

---

### **4. PHYSICAL DOMAIN (Kinetic)** 💥

**Purpose:** Physical attacks, infrastructure disruption, terrorism

**Knowledge Sources:**
- **FBI Most Wanted** (actors, indictments)
- **DOJ Complaints** (court documents, evidence)
- **NCTC** (terrorism incidents)
- **START GTD** (Global Terrorism Database)
- **Your Scenarios** (Beslan, OKC, Brussels, London)
- **DHS IED TTL** (task structure, your IP evolution)

**OSINT Stack (Pre-AI):**
1. **Scrapy** → FBI, DOJ, NCTC websites
2. **Docling** → Court documents, indictments
3. **NLTK** → N-V-N-N extraction (actors, events)
4. **Crawl4AI** → Deep narrative extraction (post-filter)

**AI Layer (Post-Filter):**
- Elena (Grok) → Actor profiling, geopolitical context
- Marcus (Gemini 2M) → Event timeline construction
- Natasha (GPT-4) → Frontline narrative generation

**Output:**
- Actor → Task → Event graphs
- Scenario templates (modernized 2006 scenarios)
- Frontline documentary script
- Third-click IED/WMD depth

**Cost Comparison:**
- **Old-school OSINT:** ~$30/month (web scraping)
- **LLM processing (filtered):** ~$80/month (1K high-value incidents)
- **LLM processing (unfiltered):** ~$3,000/month (100K news articles)
- **Savings:** 97% reduction

---

### **5. CYBER-PHYSICAL CONVERGENCE** ⚡

**Purpose:** Attacks that span cyber and physical (e.g., power grid cyber attack)

**Knowledge Sources:**
- **ICS-CERT Advisories** (industrial control systems)
- **NERC CIP** (critical infrastructure protection)
- **Stuxnet Analysis** (cyber-physical case study)
- **Colonial Pipeline Incident** (ransomware → physical)
- **Ukraine Power Grid Attacks** (cyber → kinetic)

**OSINT Stack (Pre-AI):**
1. **Scrapy** → ICS-CERT, NERC advisories
2. **GitHub** → ICS exploit repositories
3. **Shodan** → Exposed SCADA systems
4. **Docling** → Incident reports, forensics

**AI Layer (Post-Filter):**
- Marcus (Gemini 2M) → Cyber-physical kill chain mapping
- Cove (Claude) → Cross-domain relationship analysis
- Natasha (GPT-4) → Scenario generation

**Output:**
- Cyber → Physical attack pathways
- ICS vulnerability → Infrastructure target mappings
- Wazuh rules for ICS/SCADA
- Scenario templates (power, water, transport)

**Cost Comparison:**
- **Old-school OSINT:** ~$40/month (Shodan, APIs)
- **LLM processing (filtered):** ~$60/month (500 ICS targets)
- **LLM processing (unfiltered):** ~$2,000/month (50K ICS devices)
- **Savings:** 97% reduction

---

## 🔗 CROSS-DOMAIN INTEGRATION

**After domain-specific collection, merge via:**
1. **SCH Hashing** → Universal content addressing
2. **SurrealDB Graph** → Cross-domain relationships
3. **USIM Headers** → Unified symbolic messages
4. **Legion ECS** → Multi-world entity management

**Example Cross-Domain Query:**
```cypher
// Find cyber-enabled WMD proliferation pathways
MATCH (actor:Actor)-[:PERFORMS]->(task:Task)-[:USES]->(tool:CyberTool)
WHERE task.domain = 'WMD' AND tool.type = 'C2'
RETURN actor, task, tool
```

---

## 📈 TOTAL COST COMPARISON

### **Old-School OSINT (All Domains):**
- APIs + Compute: **$340/month**
- Human analysis: **$0** (automated)
- **Total: $340/month**

### **LLM Processing (Filtered, Needle-Rich Hay):**
- OSINT stack: **$340/month**
- LLM processing: **$440/month** (high-value only)
- **Total: $780/month**

### **LLM Processing (Unfiltered, Naive Approach):**
- OSINT stack: **$340/month**
- LLM processing: **$13,000/month** (everything)
- **Total: $13,340/month**

### **Savings with Needle-Rich Hay:**
- **94% cost reduction** ($13,340 → $780)
- **Same intelligence quality**
- **Faster processing** (less noise)

---

## 🚀 IMPLEMENTATION PRIORITY

### **Phase 1: Cyber Domain (Plasma Core)** - IMMEDIATE
- Exploit-DB, ATT&CK, CALDERA integration
- Wazuh rule generation
- Kali tool orchestration
- **Timeline:** 1 week

### **Phase 2: Geospatial Domain (Zoe's Mission)** - PARALLEL
- Infrastructure API collection
- GEE satellite data
- Mapbox/Cesium layers
- **Timeline:** 1 week

### **Phase 3: Physical Domain (Frontline Narrative)** - WEEK 2
- FBI/DOJ scraping
- Scenario extraction
- Actor profiling
- **Timeline:** 1 week

### **Phase 4: WMD Domain (Third-Click Depth)** - WEEK 3
- Australia Group precursors
- Proliferation pathways
- Hidden until needed
- **Timeline:** 1 week

### **Phase 5: Cyber-Physical Convergence** - WEEK 4
- ICS/SCADA integration
- Cross-domain scenarios
- Advanced threat modeling
- **Timeline:** 1 week

---

## 🎯 SUCCESS METRICS

**For each domain:**
1. ✅ OSINT stack operational (pre-AI)
2. ✅ High-value targets identified (needle-rich)
3. ✅ LLM processing cost < $100/month
4. ✅ SurrealDB graph populated
5. ✅ Cross-domain queries working
6. ✅ Scenario generation functional

**Overall:**
- **94%+ cost reduction** vs. naive LLM approach
- **Same or better intelligence quality**
- **Faster processing** (less noise)
- **Domain expertise preserved** (ontology-driven)

---

**Ready to start with Cyber Domain (Plasma Core)?** 🔴⚡

