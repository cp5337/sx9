# Node & Crate Interviews - CTAS v7.3.1

**Purpose:** First-person adversary narratives and system requirements for the 165 CTAS tasks

**Generated:** $(date)
**Status:** Core Intelligence Architecture

---

## Overview

Node and Crate Interviews are the foundation of CTAS's intelligence collection and system orchestration. They define **what** adversaries do (nodes) and **what systems** are required to execute or detect those actions (crates).

### Key Concepts

- **Node Interview:** First-person adversary narrative describing capabilities, TTPs, indicators, and EEI requirements for a specific task
- **Crate Interview:** System/infrastructure requirements to execute or detect a task, linking to node interviews
- **EEI (Essential Elements of Information):** Critical intelligence requirements defined in node interviews
- **TOML Format:** Human-readable, Rust-native format for editing interviews
- **Executable Specifications:** Interviews are code - they define what to collect and how to execute

---

## Node Interviews

### Purpose

Node interviews answer the question: **"What does an adversary need to do this task?"**

They are written from the **adversary's first-person perspective**, describing:
- Capabilities required
- Limitations and constraints
- Tactics, Techniques, and Procedures (TTPs)
- Relationships to other tasks
- Observable indicators
- Historical references
- Toolchain requirements
- **EEI Priority Questions** - What intelligence is critical to detect/execute this task

### Structure

```toml
[node]
id = "uuid-100-000-000-A"
name = "Reconnaissance - Open Source Intelligence"
task_number = 1
category = "Hunt"
hd4_phase = "Hunt"
description = "Collect publicly available information about targets"

[capabilities]
required = ["Internet access", "OSINT tools", "Language skills"]
optional = ["VPN", "Burner accounts", "Proxy chains"]
limitations = ["Rate limiting", "Geo-blocking", "CAPTCHA"]

[ttps]
mitre_tactics = ["TA0043"]
mitre_techniques = ["T1593", "T1594"]
methods = [
    "Social media scraping",
    "WHOIS lookups",
    "DNS enumeration",
    "Public records search"
]

[indicators]
network = ["Unusual WHOIS queries", "Rapid DNS lookups"]
behavioral = ["Multiple account creation", "Systematic data collection"]
temporal = ["Off-hours activity", "Automated patterns"]

[relationships]
prerequisites = []
enables = ["uuid-100-001-000-B", "uuid-100-002-000-C"]
conflicts_with = []

[historical]
references = [
    "APT28 OSINT campaigns 2016-2020",
    "Bellingcat investigations methodology"
]
case_studies = ["Mumbai 2008 - Pre-attack reconnaissance"]

[toolchain]
primary = ["Maltego", "Shodan", "theHarvester"]
secondary = ["Recon-ng", "SpiderFoot", "FOCA"]
kali_tools = ["dmitry", "fierce", "dnsenum"]

[eei]
priority = "high"
questions = [
    "What public data sources is the actor accessing?",
    "What patterns indicate systematic collection?",
    "What targets are being researched?"
]

[ndex_mapping]
fields = ["actor_id", "target_entity", "collection_method"]
niem_core = ["nc:Person", "nc:Organization", "j:IntelligenceActivity"]

[time_of_value]
collection_window = "days to weeks"
actionable_window = "weeks to months"
decay_rate = "slow"
persistence_condition = "target_active"
```

### EEI (Essential Elements of Information)

EEI questions in node interviews drive the **intelligence collection requirements**:

1. **Detection EEIs** (1n - Defensive)
   - What indicators reveal this task is being executed?
   - What data sources provide visibility?
   - What patterns distinguish normal from malicious?

2. **Execution EEIs** (2n - Offensive)
   - What information is needed to execute this task?
   - What gaps exist in current intelligence?
   - What collection methods fill those gaps?

### N-DEx & NIEM Integration

Node interviews map to **N-DEx (National Data Exchange)** and **NIEM (National Information Exchange Model)** for law enforcement interoperability:

- **N-DEx Fields:** Actor ID, Target Entity, Collection Method, Incident Type
- **NIEM Core Elements:** `nc:Person`, `nc:Organization`, `j:IntelligenceActivity`, `j:CriminalActivity`
- **IEPDs (Information Exchange Package Documentation):** Standardized XML schemas for Justice, Intelligence, Immigration

This enables CTAS to **share intelligence with law enforcement** using federal standards.

---

## Crate Interviews

### Purpose

Crate interviews answer: **"What systems/infrastructure are needed for this task?"**

They describe:
- Required crates (Rust modules)
- System dependencies
- Resource requirements
- Integration points
- Escalation tiers (script → microkernel → kernel → crate → system)

### Structure

```toml
[crate]
id = "crate-osint-collector"
name = "OSINT Collection Engine"
version = "7.3.1"
category = "Intelligence"

[supports_nodes]
primary = ["uuid-100-000-000-A"]
secondary = ["uuid-100-001-000-B"]

[dependencies]
crates = ["ctas7-foundation-math", "ctas7-hashing-engine"]
external = ["Scrapy", "Playwright", "Crawl4AI"]
databases = ["SurrealDB", "Supabase"]

[resources]
memory = "512MB"
cpu = "2 cores"
storage = "10GB"
network = "broadband"

[escalation]
tier_1 = "Python script (Scrapy basic scrape)"
tier_2 = "WASM microkernel (Scorpion)"
tier_3 = "Rust binary (full OSINT engine)"
tier_4 = "Container (with Playwright/browser)"
tier_5 = "VM (full Kali OSINT suite)"

[integration]
inputs = ["Target list", "Collection parameters"]
outputs = ["USIM records", "SurrealDB entities", "Supabase logs"]
apis = ["OSINT APIs", "Social media APIs", "Public records APIs"]

[time_of_value]
execution_time = "minutes to hours"
result_freshness = "real-time to 24 hours"
```

### Escalation Ladder

Crate interviews define the **escalation ladder** - starting with the lightest tool and scaling up as needed:

1. **Script** - Python/Bash for simple tasks
2. **Microkernel (WASM)** - Scorpion-size, portable
3. **Kernel** - Rust binary, full features
4. **Crate** - Smart Crate with dependencies
5. **System** - Full containerized environment

This follows the **"hourglass principle"** - deterministic, resource-conscious execution.

---

## Generation Process

### 1. Source Material

Node and crate interviews are generated from:
- **IED TTL (Terrorist IED Task List)** - DHS OBP document (user-created)
- **MITRE ATT&CK** - Adversary tactics and techniques
- **Historical case studies** - Mumbai 2008, Beslan, etc.
- **Kali tool documentation** - Tool capabilities and use cases
- **Scenario analysis** - 27 DHS planning scenarios

### 2. Gemini 2M Generation

Using **Marcus (Gemini 2M)** with 2M token context:

```bash
# Generate all 165 node interviews
./scripts/gemini-ea-generator.sh --mode node-interviews

# Generate crate interviews
./scripts/gemini-ea-generator.sh --mode crate-interviews
```

**Process:**
1. Load TTL tasks, MITRE ATT&CK, scenarios
2. Generate first-person adversary narrative
3. Extract EEI questions from indicators
4. Map to N-DEx/NIEM fields
5. Define time-of-value windows
6. Output TOML format

### 3. Storage

**Primary:** SurrealDB (graph + document capabilities)
- Graph relationships between nodes
- Document storage for full interviews
- Query by EEI, MITRE tactic, HD4 phase

**Secondary:** Supabase (ACID + permanent records)
- Blockchain anchoring for immutability
- Audit trail for changes
- Backup and recovery

**Indexing:** Sled KVS (key-value store)
- Hash-based lookup (SCH+CUID+UUID)
- Fast retrieval by node ID
- Ephemeral caching

---

## Intelligence Collection

### Node Interviews → EEI → Collection

1. **Node interview defines EEI questions**
   - "What public data sources is the actor accessing?"

2. **EEI triggers collection requirements**
   - Monitor WHOIS queries, DNS lookups, social media scraping

3. **247 WASM microkernels act as distributed sniffers**
   - Deployed at strategic points (ground stations, network nodes)
   - Feed data to 165-node graph detector

4. **Graph detector analyzes patterns**
   - Node states: normal, investigating, increasing, high activity
   - Convergence Meter (ctas7-intelligence-generator) calculates probability
   - Foundation math (KNN, A*, Matroid) drives convergence

5. **Intelligence "vibrates" to prediction**
   - Electric Football analogy - disparate pieces converge
   - OODA loop triggers on convergence threshold
   - Matroids reveal optimal action paths

### Convergence Meter

**Crate:** `ctas7-intelligence-generator`

**Purpose:** Measure how scattered intelligence converges on a single prediction

**Algorithms:** (from `ctas7-foundation-math`)
- **KNN** - Nearest neighbor clustering
- **A*** - Path finding through graph
- **Matroid** - Optimal subset selection
- **Chinese Postman** - Efficient graph traversal
- **PageRank** - Node importance
- **ARIMA** - Time series forecasting
- **Hawkes Process** - Event clustering
- **CUSUM** - Change point detection

**Output:** Convergence score (0-100%) indicating confidence in prediction

---

## OSINT Integration

### Multi-Stage Pipeline

Node interviews drive a **cost-optimized OSINT pipeline**:

1. **Deterministic Scraping** (Pre-AI)
   - Scrapy, Docling, Scapy for basic extraction
   - N-V-N-N pattern matching (Noun-Verb-Noun-Noun)
   - Creates "needle-rich hay" for AI models

2. **LoRA-trained Phi-3** (Specialized Extraction)
   - Domain-specific fine-tuning
   - Entity extraction (people, vehicles, locations)
   - Relationship mapping

3. **Crawl4AI** (Validation & Enrichment)
   - Validate extracted data
   - Enrich with additional context
   - Cost-effective compared to full GPT-4 processing

### Domain-Specific Modules

Based on node interviews:
- **People Module** - Actor identification, relationships
- **Vehicle Module** - Transportation, logistics
- **Location Module** - Geospatial intelligence
- **Event Module** - Incident timeline reconstruction
- **Object Module** - Weapons, equipment, materials

---

## Time-of-Value

Node and crate interviews include **time-of-value** analysis:

### Intelligence Decay Curves

Different intelligence types have different actionable windows:

| Type | Collection Window | Actionable Window | Decay Rate |
|------|-------------------|-------------------|------------|
| OSINT | Days to weeks | Weeks to months | Slow |
| SIGINT | Minutes to hours | Hours to days | Fast |
| HUMINT | Weeks to months | Months to years | Very slow |
| Tactical | Real-time | Minutes to hours | Very fast |
| Strategic | Months | Years | Minimal |

### Sliding Window Theory

Intelligence value **peaks** in specific operational windows:
- **Pre-attack:** Reconnaissance intelligence most valuable
- **During attack:** Real-time tactical intelligence critical
- **Post-attack:** Forensic intelligence for attribution

### Ephemeral Intelligence Rules

Based on time-of-value, intelligence is:
- **Persisted** if: High strategic value, legal requirement, attribution need
- **Discarded** if: Expired actionable window, low confidence, superseded by newer intel

---

## Scenario-Based Testing

### Las Vegas Algorithm

Test node interviews using **known outcomes** (Las Vegas algorithm):

1. **Select historical scenario** (e.g., Beslan school attack)
2. **Run OSINT collection** using node interview EEIs
3. **Compare extracted intelligence** to known facts
4. **Tune node interview** based on gaps/false positives
5. **Iterate** until convergence matches known outcome

### Scenarios Used

- **Beslan School Attack** (2004) - Hostage, IED, multi-actor
- **Mumbai Attacks** (2008) - Maritime, urban assault, C2
- **Blue Dusk Black Sky** (Scenario 27) - Converged multi-domain
- **Cartel Operations** - Drug trafficking, border crossing
- **APT Campaigns** - Cyber reconnaissance, lateral movement

---

## Integration with CTAS Systems

### 1. PLASMA Dashboard

Node interviews feed threat intelligence:
- EEI requirements → OSINT collection
- Indicators → Wazuh rules
- TTPs → MITRE ATT&CK mapping

### 2. Neural Mux

Crate interviews define routing:
- Escalation tier selection
- Resource allocation
- Multi-LLM coordination

### 3. Foundation v7.3.1

Node interviews hashed for addressing:
- SCH (Semantic Content Hash) from interview content
- CUID masks include temporal/geographic context
- UUID for global uniqueness

### 4. XSD Playbooks

Crate interviews → XSD orchestration:
- Define crate execution order
- Specify dependencies
- Handle failure modes

---

## Example: Node Interview in Action

### Scenario: Detect OSINT Reconnaissance

1. **Node Interview:** "uuid-100-000-000-A" (OSINT Recon)
   - EEI: "What public data sources is the actor accessing?"

2. **247 WASM Microkernels:** Monitor DNS, WHOIS, social media APIs

3. **Pattern Detection:** Unusual spike in WHOIS queries from single IP

4. **Graph Detector:** Node "uuid-100-000-000-A" state → "investigating"

5. **Convergence Meter:** Analyzes related nodes (network mapping, target profiling)
   - Convergence score: 73% (high confidence)

6. **OODA Loop Triggered:** Alert operator, initiate counter-recon

7. **Matroid Analysis:** Reveals optimal interdiction points

8. **Action:** Deploy deception (honeypot), monitor for next phase

---

## Files & Locations

```
ctas7-command-center/
├── ctas7-intelligence-generator/
│   ├── src/main.rs                    # Convergence Meter
│   ├── generated_interviews/
│   │   ├── nodes/
│   │   │   ├── uuid-100-000-000-A.toml
│   │   │   ├── uuid-100-001-000-B.toml
│   │   │   └── ... (165 total)
│   │   └── crates/
│   │       ├── crate-osint-collector.toml
│   │       └── ... (per crate)
│   └── schemas/
│       ├── node-interview-schema.toml
│       └── crate-interview-schema.toml

ctas6-reference/
├── docs/architecture/
│   └── NODE-CRATE-INTERVIEWS.md       # This document
└── scripts/
    └── gemini-ea-generator.sh         # Generation script
```

---

## Next Steps

1. **Generate All 165 Node Interviews**
   - Use Gemini 2M with TTL + MITRE + scenarios
   - Store in SurrealDB + Supabase

2. **Generate Crate Interviews**
   - Map to existing Smart Crates
   - Define escalation ladders

3. **Deploy 247 WASM Microkernels**
   - At ground stations, network nodes
   - Feed graph detector

4. **Tune Convergence Meter**
   - Test with historical scenarios
   - Optimize math algorithms

5. **Integrate with PLASMA**
   - Display EEI collection status
   - Show convergence scores
   - Alert on high-confidence predictions

---

**Status:** Architecture defined, generation in progress
**Owner:** Marcus (Gemini 2M) - EA & Intelligence Architecture
**Related:** IED TTL, MITRE ATT&CK, N-DEx/NIEM, Convergence Meter

