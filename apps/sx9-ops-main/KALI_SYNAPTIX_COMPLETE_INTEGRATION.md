# 🔥 **KALI SYNAPTIX: Complete 1n/2n Integration Architecture**

## 📋 **Executive Summary**

Kali Synaptix integrates **5 major intelligence systems** to provide complete offensive (2n) and defensive (1n) cyber operations:

1. **Kali Linux Tools** (~600 tools) - Offensive/defensive tooling
2. **MITRE CALDERA** - Adversary emulation platform
3. **Atomic Red Team** - Pre-built attack scenarios (MITRE ATT&CK)
4. **Exploit-DB** - Vulnerability database (without POCs for safety)
5. **Synaptix Plasma** - Defensive platform (Wazuh + AXON + Legion)

---

## 🎯 **Architecture Overview**

```
┌─────────────────────────────────────────────────────────────────┐
│               🔥 KALI SYNAPTIX ISO 🔥                           │
│        Full Kali Purple + Plasma + CALDERA + ART               │
└─────────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┴─────────────────────┐
        │                                           │
        ▼                                           ▼
┌───────────────────────┐                 ┌───────────────────────┐
│   1n: DEFENSIVE       │                 │   2n: OFFENSIVE       │
│   (Plasma)            │                 │   (CALDERA + Kali)    │
├───────────────────────┤                 ├───────────────────────┤
│ • Wazuh (alerts)      │                 │ • CALDERA (emulation) │
│ • AXON (processing)   │                 │ • Atomic Red Team     │
│ • Legion (tracking)   │                 │ • Kali Tools          │
│ • Phi-3 (validation)  │                 │ • Exploit-DB          │
│ • Suricata, Zeek      │                 │ • Metasploit          │
│ • Detection rules     │                 │ • Custom payloads     │
└───────────────────────┘                 └───────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │   Intelligence Orchestrator             │
        │   (ctas_intelligence_orchestrator.py)   │
        ├─────────────────────────────────────────┤
        │ • Harvests Kali tools metadata          │
        │ • Extracts CALDERA abilities            │
        │ • Processes Atomic Red Team tests       │
        │ • Maps to 165 CTAS tasks                │
        │ • Stores in Sled + SurrealDB            │
        └─────────────────────────────────────────┘
```

---

## 📂 **Existing Resources (Found)**

### **1. Atomic Red Team**
- **Location**: `/Users/cp5337/Developer/HOLD DURING REORG/.../atomic-red-team/`
- **Contents**: 
  - `atomics/` - Individual ATT&CK technique tests (T1543.004, T1543.001, etc.)
  - `atomic_red_team/` - Ruby execution framework
- **Integration**: Use `ctas_intelligence_orchestrator.py` to extract and map to CTAS tasks

### **2. CALDERA Integration Crate**
- **Location**: `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas-7.0-main-ops-platform/ctas-caldera-integration/`
- **Contents**:
  - `src/abilities.rs` - CALDERA abilities API
  - `src/api.rs` - CALDERA REST API client
  - `src/client.rs` - Main client
  - `src/facts.rs` - Fact management
  - `src/operations.rs` - Operation orchestration
  - `src/types.rs` - Data structures
- **Status**: Rust crate ready for integration

### **3. Exploit-DB**
- **Location**: UI screenshots found in `/Users/cp5337/Developer/ctas-v6.6/CTAS_UI_IMAGES/`
  - `ExploitDB Page.png`
  - `ExploitDB_Detect_phase.png`
- **Status**: Database exists but **without POCs** (for safety/legal compliance)
- **Integration**: Map exploits to CTAS tasks and Kali tools

### **4. Intelligence Pipeline Scripts**
- **Location**: `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/intelligence-pipeline/`
- **Contents**:
  - `ctas_intelligence_orchestrator.py` - Master orchestrator
  - `ctas_narrative_processor.py` - Processes 165 CTAS tasks
  - `kali_tools_harvester.py` - Scrapes kali.org for tool metadata
  - `sled_data_collector.rs` - High-performance Rust/Sled storage

---

## 🔗 **Integration Strategy**

### **Phase 1: Data Collection (Intelligence Orchestrator)**

```python
# Run the complete intelligence collection pipeline
python3 ctas_intelligence_orchestrator.py

# This will:
# 1. Scrape ~600 Kali tools from kali.org
# 2. Extract CALDERA abilities from API
# 3. Process Atomic Red Team tests
# 4. Map all to CTAS primitives (SENSE, ACT, ENCODE, ANALYZE, ORCHESTRATE, MONITOR)
# 5. Store in Sled KVS for fast lookup
# 6. Generate unified intelligence dataset
```

### **Phase 2: CALDERA Deployment (Docker)**

```yaml
# Add to docker-compose.ctas-v7.3.1.yml
caldera:
  image: mitre/caldera:latest
  container_name: ctas-caldera
  ports:
    - "8888:8888"  # CALDERA web UI
    - "8443:8443"  # CALDERA API
  environment:
    - CALDERA_URL=http://caldera:8888
    - AXON_ENDPOINT=http://axon:15176
  volumes:
    - caldera_data:/opt/caldera/data
    - ./atomic-red-team:/opt/caldera/plugins/atomic
  networks:
    - ctas_network
  restart: unless-stopped
```

### **Phase 3: Atomic Red Team Integration**

```bash
# Copy Atomic Red Team to CALDERA plugins
cp -r /Users/cp5337/Developer/HOLD\ DURING\ REORG/.../atomic-red-team \
     /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/atomic-red-team

# CALDERA will auto-discover and import Atomic tests
# Map each test to:
# - MITRE ATT&CK technique (e.g., T1543.004)
# - CTAS task (e.g., SCH003.015)
# - HD4 phase (Hunt, Detect, Disrupt, Disable, Dominate)
# - Kali tool (e.g., nmap, metasploit)
```

### **Phase 4: Exploit-DB Integration**

```rust
// Create ctas7-exploit-db crate
// Stores exploit metadata (NO POCs)
pub struct ExploitRecord {
    pub exploit_id: String,      // EDB-12345
    pub title: String,
    pub cve: Option<String>,      // CVE-2024-1234
    pub platform: String,         // Linux, Windows, etc.
    pub exploit_type: String,     // Remote, Local, DoS, etc.
    pub date_published: u64,
    pub author: String,
    pub description: String,
    pub affected_software: Vec<String>,
    pub mitre_attack: Vec<String>, // T1210, T1068, etc.
    pub ctas_tasks: Vec<String>,   // SCH004.012, etc.
    pub kali_tools: Vec<String>,   // Tools that can exploit this
    // NO POC CODE - metadata only
}
```

### **Phase 5: Unified Task Mapping**

```
CTAS Task → MITRE ATT&CK → Atomic Test → CALDERA Ability → Kali Tool → Exploit-DB

Example:
SCH004.012 (Exploit Public-Facing Application)
  ↓
T1190 (Exploit Public-Facing Application)
  ↓
Atomic Test: T1190-001 (SQL Injection)
  ↓
CALDERA Ability: sql_injection_scanner
  ↓
Kali Tool: sqlmap
  ↓
Exploit-DB: EDB-49876 (SQL Injection in WebApp v2.3)
```

---

## 🐳 **Updated Docker Compose**

Add these services to `docker-compose.ctas-v7.3.1.yml`:

```yaml
  # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  # ADVERSARY EMULATION (2n: Offensive)
  # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  # MITRE CALDERA (Adversary Emulation Platform)
  caldera:
    image: mitre/caldera:latest
    container_name: ctas-caldera
    ports:
      - "8888:8888"  # Web UI
      - "8443:8443"  # API
    environment:
      - CALDERA_URL=http://caldera:8888
      - AXON_ENDPOINT=http://axon:15176
      - PLASMA_URL=http://wazuh-manager:55000
    volumes:
      - caldera_data:/opt/caldera/data
      - ./atomic-red-team:/opt/caldera/plugins/atomic
      - ./caldera-config:/opt/caldera/conf
    networks:
      - ctas_network
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8888"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Exploit-DB API (Metadata Only - No POCs)
  exploit-db:
    image: ctas7/exploit-db:7.3.1
    container_name: ctas-exploit-db
    ports:
      - "18500:18500"  # Exploit-DB API
    environment:
      - RUST_LOG=info
      - SURREALDB_URL=http://surrealdb:8000
      - CALDERA_URL=http://caldera:8888
    networks:
      - ctas_network
    depends_on:
      - surrealdb
      - caldera
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:18500/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Intelligence Orchestrator (Python)
  intelligence-orchestrator:
    build:
      context: ./intelligence-pipeline
      dockerfile: Dockerfile
    container_name: ctas-intelligence-orchestrator
    environment:
      - CALDERA_URL=http://caldera:8888
      - KALI_TOOLS_URL=http://kali-tools:15178
      - SURREALDB_URL=http://surrealdb:8000
      - SLEDIS_URL=http://sledis:19014
    volumes:
      - intelligence_data:/data
      - ./atomic-red-team:/atomic-red-team:ro
    networks:
      - ctas_network
    depends_on:
      - caldera
      - kali-tools
      - surrealdb
      - sledis
    restart: unless-stopped
    command: ["python3", "ctas_intelligence_orchestrator.py", "--continuous"]
```

---

## 📊 **Data Flow**

### **1n: Defensive Flow (Plasma)**
```
Wazuh Agent → Wazuh Manager → AXON → Legion → Phi-3 → Response
     ↓            ↓              ↓       ↓       ↓         ↓
  Detects    Collects      Processes  Tracks  Validates  Blocks
  Activity   Alerts        USIMs      Entities Threats   Threat
```

### **2n: Offensive Flow (CALDERA)**
```
Operator → CALDERA → Atomic Test → Kali Tool → Target
   ↓          ↓           ↓            ↓          ↓
 Selects   Executes   Runs ATT&CK  Exploits   Reports
 Mission   Ability    Technique    Vuln       Results
```

### **Intelligence Flow (Orchestrator)**
```
Kali.org → Harvester → CALDERA API → ART Parser → Sled → SurrealDB
   ↓          ↓            ↓             ↓          ↓        ↓
 ~600     Scrapes      Abilities      Tests      Fast    Permanent
 Tools    Metadata     Extracted      Mapped     Cache   Storage
```

---

## 🎯 **CTAS Task Mapping**

### **165 CTAS Tasks → ATT&CK → Tools**

| CTAS Task | HD4 Phase | ATT&CK | Atomic Test | CALDERA | Kali Tool | Exploit-DB |
|-----------|-----------|--------|-------------|---------|-----------|------------|
| SCH001.001 | Hunt | T1595 | T1595-001 | active_scan | nmap | - |
| SCH003.015 | Hunt | T1190 | T1190-001 | sql_inject | sqlmap | EDB-49876 |
| SCH004.012 | Detect | T1055 | T1055-001 | proc_inject | volatility | - |
| SCH007.008 | Disrupt | T1498 | T1498-001 | dos_attack | hping3 | - |
| SCH009.003 | Disable | T1486 | T1486-001 | ransomware | - | EDB-50123 |

---

## 🚀 **Deployment Steps**

### **Step 1: Copy Atomic Red Team**
```bash
cp -r "/Users/cp5337/Developer/HOLD DURING REORG/.../atomic-red-team" \
     /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/
```

### **Step 2: Build Intelligence Orchestrator Image**
```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/intelligence-pipeline
docker build -t ctas7/intelligence-orchestrator:7.3.1 .
```

### **Step 3: Build Exploit-DB Image**
```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/docker-builds/exploit-db
docker build -t ctas7/exploit-db:7.3.1 .
```

### **Step 4: Deploy Complete Stack**
```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference
docker-compose -f docker-compose.ctas-v7.3.1.yml up -d
```

### **Step 5: Run Intelligence Collection**
```bash
docker exec -it ctas-intelligence-orchestrator \
  python3 ctas_intelligence_orchestrator.py
```

### **Step 6: Verify Integration**
```bash
# Check CALDERA
curl http://localhost:8888/api/v2/abilities

# Check Exploit-DB
curl http://localhost:18500/api/exploits

# Check Kali Tools
curl http://localhost:15178/tasks

# Check Intelligence Orchestrator
docker logs ctas-intelligence-orchestrator
```

---

## 📈 **Expected Results**

### **Intelligence Collection**
- **~600 Kali Tools** scraped and mapped to primitives
- **~200 CALDERA Abilities** extracted from API
- **~300 Atomic Red Team Tests** processed
- **~50,000 Exploit-DB Records** (metadata only, no POCs)
- **165 CTAS Tasks** fully mapped to all systems

### **Unified Dataset**
```json
{
  "ctas_task": "SCH003.015",
  "task_name": "Exploit Public-Facing Application",
  "hd4_phase": "hunt",
  "primitive_type": "ACT",
  "mitre_attack": ["T1190"],
  "atomic_tests": ["T1190-001", "T1190-002"],
  "caldera_abilities": ["sql_injection_scanner", "web_exploit"],
  "kali_tools": ["sqlmap", "burpsuite", "nikto"],
  "exploit_db": ["EDB-49876", "EDB-50123"],
  "usim_hash": "🔥a3f2...💻b8c1...🚀d4e5",
  "1n_defensive": {
    "wazuh_rule": "100210",
    "sigma_rule": "web_exploit_attempt",
    "detection_tools": ["suricata", "zeek"]
  },
  "2n_offensive": {
    "caldera_operation": "web_app_exploitation",
    "atomic_test": "T1190-001",
    "kali_tool": "sqlmap"
  }
}
```

---

## 🔐 **Security Considerations**

### **Exploit-DB: Metadata Only**
- **NO POC CODE** stored or distributed
- Only metadata: CVE, title, description, affected software
- Links to official Exploit-DB for authorized users
- Legal compliance: No weaponized exploits

### **CALDERA: Controlled Emulation**
- Runs in isolated Docker network
- Requires operator authentication
- All operations logged to Plasma
- Can be disabled in "defensive-only" mode

### **Atomic Red Team: Safe Execution**
- Tests run in ephemeral containers
- No persistence on host
- Automatic cleanup after execution
- Operator approval required for destructive tests

---

## 🎯 **Success Metrics**

1. **✅ All 165 CTAS tasks mapped** to ATT&CK, Atomic, CALDERA, Kali, Exploit-DB
2. **✅ ~600 Kali tools** harvested and categorized by primitive
3. **✅ ~200 CALDERA abilities** integrated and executable
4. **✅ ~300 Atomic tests** processed and mapped
5. **✅ ~50K Exploit-DB records** (metadata only)
6. **✅ 1n/2n mode switching** operational (defensive ↔ offensive)
7. **✅ Intelligence orchestrator** running continuously
8. **✅ Sled + SurrealDB** storing all intelligence data

---

## 📞 **Next Steps**

1. **Copy Atomic Red Team** from legacy archive
2. **Build Exploit-DB Docker image** (metadata only)
3. **Deploy CALDERA** in Docker Compose
4. **Run Intelligence Orchestrator** to collect all data
5. **Test 1n/2n mode switching** (Plasma ↔ CALDERA)
6. **Generate unified intelligence report**
7. **Create Kali Synaptix ISO** with all components

---

**🔥 Kali Synaptix: One ISO. Two Modes. Complete Cyber Operations. 🔥**

