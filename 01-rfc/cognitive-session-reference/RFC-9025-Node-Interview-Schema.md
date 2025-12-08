# RFC-9025: Node Interview Schema

**Status:** Canonical
**Author:** Charlie Payne
**Version:** 7.3.1
**Related:** RFC-9020 (HD4), RFC-9023 (Frameworks), RFC-9024 (Neurological)

---

## Abstract

This RFC defines the canonical schema for **Node Interviews** - first-person adversary narratives that illuminate the 164 CTAS tasks. Node interviews are NOT generic capability descriptions; they are **specific, tool-linked, EEI-driven specifications** that enable detection and execution.

---

## 1. Problem Statement

### 1.1 The Current Mess

Generated node interviews suffer from:
- **Generic language:** "I operate through concept mechanisms" (meaningless)
- **Disconnected tools:** "primary_tool: nmap - tactical integration" (no specificity)
- **Vague EEIs:** "Critical ideation capability confirmation" (not actionable)
- **No MITRE linking:** Techniques mentioned but not mapped
- **No time-of-value:** When does this intelligence expire?

### 1.2 What We Need

Node interviews must:
- Map directly to **specific 164 tasks**
- Reference **actual tools** (Kali, COTS, OSINT)
- Define **concrete EEI questions** (what do we need to know?)
- Link to **MITRE techniques** (ATT&CK, D3FEND)
- Specify **time-of-value decay** (when does intel expire?)
- Support **1n/2n perspective weave** (defender AND adversary view)

---

## 2. Canonical Node Interview Schema

### 2.1 TOML Format

```toml
[node]
id = "uuid-001-006-001"                    # Direct link to ctas_tasks
task_seq = 1006001                         # Task sequence number
name = "Cyber Reconnaissance"
category = "Cyber Reconnaissance"
hd4_phase = "Hunt"
primitive_type = "Object"
description = "Identifying live hosts and network topology"

# THE NODE SPEAKS - First-person identity and ownership
[identity]
voice = """
I am Cyber Reconnaissance. I am the eyes before the attack.

I map your network topology using nmap, dnsenum, and fierce. I enumerate
your subdomains through crt.sh and SecurityTrails. I passively observe
your exposed services through Shodan and Censys before I ever touch your
network directly.

You have seen me in APT29's SolarWinds campaign where I spent weeks mapping
Microsoft's infrastructure before the breach. You have seen me in APT28's
DNC operation where subdomain enumeration revealed the targets. I was the
first step in the Equifax breach, the Capital One compromise, every major
intrusion this decade.

My indicators are TCP SYN packets to sequential ports. Rapid DNS queries
for non-existent subdomains. Service banner grabbing attempts. ICMP sweeps
across your subnets. I try to stay below your IDS thresholds, but if you're
watching carefully, you'll see my patterns.

My success means your attack surface is mapped. My failure means detection,
blocked IPs, and a warned defender. I feed Vulnerability Scanning
(uuid-001-007-001) and enable every task that follows. Without me, the
operation is blind.
"""

purpose = "Map attack surface completely while remaining undetected"
ownership = "I own the intelligence that enables targeting decisions"

# What I need to succeed (adversary perspective)
[needs]
required = [
    "Network access to target perimeter",
    "DNS resolution capability",
    "Time for slow, careful enumeration",
]
optional = [
    "VPN for source obfuscation",
    "Cloud infrastructure for distributed scanning",
    "Shodan/Censys API keys for passive recon",
]

# What defeats me (defender perspective)
[counters]
detection = [
    "Network flow analysis detecting scan patterns",
    "DNS query logging with anomaly detection",
    "Honeypots that waste my time and reveal my presence",
]
prevention = [
    "Aggressive rate limiting on external queries",
    "DNS sinkholing of non-existent domains",
    "Service banner hardening to hide versions",
]

# My success and failure
[outcomes]
success_means = [
    "Complete network topology mapped",
    "All exposed services identified with versions",
    "Actionable targets prioritized for exploitation",
    "Zero alerts triggered during reconnaissance",
]
failure_means = [
    "Detected by IDS - IP blocked, defender alerted",
    "Incomplete mapping due to firewall restrictions",
    "Time wasted on honeypots instead of real targets",
    "Forensic evidence left for attribution",
]

# Capabilities required by adversary
[capabilities]
required = [
    "Network access to target",
    "DNS resolution capability",
    "Port scanning tools",
    "Patience for slow reconnaissance"
]
optional = [
    "VPN for source obfuscation",
    "Cloud infrastructure for distributed scanning",
    "Shodan API access for passive recon"
]
limitations = [
    "Rate limiting may slow enumeration",
    "Firewall may block scans",
    "IDS may detect and alert on patterns",
    "NAT may hide internal topology"
]

# TTPs with SPECIFIC MITRE mappings
[ttps]
mitre_tactics = ["TA0043"]                 # Reconnaissance
mitre_techniques = [
    "T1595.001",                           # Scanning IP Blocks
    "T1595.002",                           # Vulnerability Scanning
    "T1590.002",                           # DNS
    "T1590.004",                           # Network Topology
]
d3fend_countermeasures = [
    "D3-SYSM",                             # System Monitoring
    "D3-NTA",                              # Network Traffic Analysis
    "D3-OTF",                              # Outbound Traffic Filtering
]

# Actual tools - NOT generic
[toolchain.kali]
primary = [
    { tool = "nmap", use = "Port scanning and service detection", flags = "-sS -sV -O" },
    { tool = "dnsenum", use = "DNS enumeration", flags = "--enum" },
    { tool = "fierce", use = "DNS reconnaissance", flags = "--domain target.com" },
]
secondary = [
    { tool = "masscan", use = "Fast port scanning at scale" },
    { tool = "nikto", use = "Web server vulnerability scanning" },
    { tool = "whatweb", use = "Web fingerprinting" },
]

[toolchain.osint]
sources = [
    { tool = "Shodan", use = "Passive service enumeration", api = true },
    { tool = "Censys", use = "Internet-wide scan data", api = true },
    { tool = "crt.sh", use = "Certificate transparency logs", web = true },
    { tool = "SecurityTrails", use = "DNS history", api = true },
]

[toolchain.commercial]
detection = [
    { tool = "Wazuh", rule_category = "network_scan" },
    { tool = "Suricata", rule_sid = "2100366" },  # Port scan detection
    { tool = "Zeek", script = "scan.zeek" },
]

# Observable indicators for detection
[indicators]
network = [
    "TCP SYN packets to sequential ports from single source",
    "DNS queries for many subdomains in rapid succession",
    "Service banner grabbing attempts",
    "ICMP sweep across subnet",
]
behavioral = [
    "Reconnaissance activity outside business hours",
    "Geographic source inconsistent with legitimate users",
    "Multiple failed connection attempts",
]
temporal = [
    "Sustained low-rate scanning over days/weeks",
    "Reconnaissance peaks before business hours",
]

# EEI - What intelligence do we need?
[eei.detection]
priority = "high"
questions = [
    "What IP ranges are being scanned?",
    "What services is the attacker enumerating?",
    "What is the scanning rate and pattern?",
    "Where is the scan originating from?",
    "Is this distributed or single-source?",
]
collection_methods = [
    "Network flow logs",
    "IDS/IPS alerts",
    "DNS query logs",
    "Firewall connection logs",
]

[eei.execution]
priority = "medium"
questions = [
    "What is the target's external attack surface?",
    "What services are exposed?",
    "What versions are running?",
    "Where are the likely vulnerabilities?",
]
collection_methods = [
    "Active scanning (nmap, masscan)",
    "Passive OSINT (Shodan, Censys)",
    "DNS enumeration",
]

# Relationships to other tasks
[relationships]
prerequisites = ["uuid-001-000-001"]        # Pre-Operational Planning
enables = [
    "uuid-001-007-001",                     # Vulnerability Scanning
    "uuid-002-000-001",                     # Reconnaissance and Targeting
]
often_combined_with = [
    "uuid-001-001-001",                     # OSINT Collection
    "uuid-001-012-001",                     # Packet Capture Analysis
]

# Historical references and case studies
[historical]
apt_examples = [
    { apt = "APT29", campaign = "SolarWinds", year = 2020, note = "Extensive DNS recon before breach" },
    { apt = "APT28", campaign = "DNC", year = 2016, note = "Subdomain enumeration revealed targets" },
]
case_studies = [
    "Equifax breach - exposed Apache Struts found via scanning",
    "Capital One breach - SSRF discovered through web recon",
]

# Time-of-value decay
[time_of_value]
collection_window = "hours to days"         # How long to collect
actionable_window = "days to weeks"         # How long intel is useful
decay_rate = "medium"                       # slow/medium/fast/immediate
persistence_condition = "target_network_stable"
refresh_trigger = "topology_change"

# N-DEx/NIEM mapping for law enforcement sharing
[ndex]
activity_type = "Cyber Reconnaissance"
niem_elements = [
    "j:IntelligenceActivity",
    "nc:Computer",
    "nc:NetworkIdentifier",
]
iepd = "cyber_reconnaissance_iepd.xml"

# Hash addresses
[addressing]
sch_context = "CYBERRECON"
cuid_mask = "geo=ANY|sector=IT|temporal=PERSISTENT"
unicode_ref = "\\u{E006}"
```

---

## 3. Schema Components

### 3.1 Node Block

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Direct link to `ctas_tasks.task_id` |
| `task_seq` | Integer | Task sequence number |
| `name` | String | Task name from TTL |
| `category` | String | Task category |
| `hd4_phase` | Enum | Hunt/Detect/Disable/Disrupt/Dominate |
| `primitive_type` | Enum | Concept/Event/Actor/Object/Attribute |
| `description` | String | Task description |

### 3.2 Identity Block - THE NODE SPEAKS

The most critical block. **The node itself speaks in first person:**

| Field | Type | Description |
|-------|------|-------------|
| `voice` | String | First-person narrative: "I am X. I do Y using Z. You have seen me in A, B, C. My indicators are..." |
| `purpose` | String | What I exist to accomplish |
| `ownership` | String | What I own in the attack/defense chain |

**Voice Template:**
```
I am [Task Name]. I am [role in attack chain].

I [primary action] using [specific tools]. I [secondary actions] through
[methods/sources]. I [how I operate] [operational details].

You have seen me in [APT campaign] where I [specific use]. You have seen
me in [another example] where [specific details]. I was [role] in [famous
incident], [another incident], [pattern].

My indicators are [network indicators]. [Behavioral indicators]. [Temporal
patterns]. I try to [evasion technique], but if you're [detection method],
you'll see [what reveals me].

My success means [outcome for adversary]. My failure means [outcome when
detected]. I feed [downstream tasks] and enable [what depends on me].
Without me, [consequence of absence].
```

### 3.3 Needs Block

What the adversary needs to execute this task:

| Field | Type | Description |
|-------|------|-------------|
| `required` | Array[String] | Must-have capabilities |
| `optional` | Array[String] | Nice-to-have capabilities |

### 3.4 Counters Block

What defeats this task (defender perspective):

| Field | Type | Description |
|-------|------|-------------|
| `detection` | Array[String] | How to detect this task |
| `prevention` | Array[String] | How to prevent this task |

### 3.5 Outcomes Block

Success and failure from both perspectives:

| Field | Type | Description |
|-------|------|-------------|
| `success_means` | Array[String] | What success looks like |
| `failure_means` | Array[String] | What failure looks like |

### 3.3 Capabilities Block

| Field | Type | Description |
|-------|------|-------------|
| `required` | Array[String] | Must-have capabilities |
| `optional` | Array[String] | Nice-to-have capabilities |
| `limitations` | Array[String] | Constraints on execution |

### 3.4 TTPs Block

| Field | Type | Description |
|-------|------|-------------|
| `mitre_tactics` | Array[String] | ATT&CK tactic IDs (TAxxxx) |
| `mitre_techniques` | Array[String] | ATT&CK technique IDs (Txxxx.xxx) |
| `d3fend_countermeasures` | Array[String] | D3FEND defensive technique IDs |

### 3.5 Toolchain Blocks

**kali:** Actual Kali Linux tools with specific flags/usage
**osint:** OSINT sources with API/web distinction
**commercial:** Detection tools with rule references

### 3.6 Indicators Block

| Type | Description |
|------|-------------|
| `network` | Network-observable indicators |
| `behavioral` | Behavioral patterns |
| `temporal` | Time-based patterns |

### 3.7 EEI Blocks

**detection:** What intelligence do we need to DETECT this task?
**execution:** What intelligence do we need to EXECUTE this task?

Each contains:
- `priority`: critical/high/medium/low
- `questions`: Specific intelligence questions
- `collection_methods`: How to collect answers

### 3.8 Time-of-Value Block

| Field | Type | Description |
|-------|------|-------------|
| `collection_window` | String | How long to collect (real-time, hours, days, weeks) |
| `actionable_window` | String | How long intel is useful |
| `decay_rate` | Enum | slow/medium/fast/immediate |
| `persistence_condition` | String | When to keep intel |
| `refresh_trigger` | String | When to re-collect |

---

## 4. Generation Requirements

### 4.1 Source Material

Node interviews MUST be generated from:
1. **IED TTL (164 tasks)** - Task definitions
2. **MITRE ATT&CK** - Technique mappings
3. **Kali tool documentation** - Actual tool capabilities
4. **APT case studies** - Historical examples
5. **Detection rule databases** - CAR, Sigma, Wazuh

### 4.2 LLM Prompt Requirements

When generating node interviews:
1. MUST link to specific task from ctas_tasks table
2. MUST include actual tool names with flags/usage
3. MUST include specific MITRE technique IDs (not just names)
4. MUST include concrete EEI questions (not generic)
5. MUST include time-of-value estimates
6. MUST include both 1n and 2n perspectives

### 4.3 Validation Rules

```python
def validate_node_interview(interview: dict) -> bool:
    # Must link to valid task
    assert interview['node']['id'] in ctas_task_ids

    # Must have both perspectives
    assert 'perspective' in interview
    assert '1n' in interview['perspective']
    assert '2n' in interview['perspective']

    # Must have specific MITRE links
    assert len(interview['ttps']['mitre_techniques']) > 0
    for tech in interview['ttps']['mitre_techniques']:
        assert tech.startswith('T') and '.' in tech or tech.startswith('T1')

    # Must have actual tool references
    assert len(interview['toolchain']['kali']['primary']) > 0
    for tool in interview['toolchain']['kali']['primary']:
        assert 'tool' in tool and 'use' in tool

    # Must have concrete EEI questions
    assert len(interview['eei']['detection']['questions']) >= 3
    for q in interview['eei']['detection']['questions']:
        assert q.endswith('?')  # Must be actual questions

    # Must have time-of-value
    assert interview['time_of_value']['decay_rate'] in ['slow', 'medium', 'fast', 'immediate']

    return True
```

---

## 5. Storage and Indexing

### 5.1 Primary Storage: Supabase

```sql
CREATE TABLE node_interviews (
    interview_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id TEXT NOT NULL REFERENCES ctas_tasks(task_id),
    version TEXT NOT NULL DEFAULT '7.3.1',

    -- Core content
    perspective_1n TEXT NOT NULL,
    perspective_2n TEXT NOT NULL,
    capabilities JSONB NOT NULL,
    ttps JSONB NOT NULL,
    toolchain JSONB NOT NULL,
    indicators JSONB NOT NULL,
    eei JSONB NOT NULL,
    relationships JSONB NOT NULL,
    historical JSONB,
    time_of_value JSONB NOT NULL,
    ndex JSONB,

    -- Addressing
    sch_context TEXT,
    h1_hash TEXT,
    h2_hash TEXT,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    generated_by TEXT,
    validated BOOLEAN DEFAULT FALSE,

    UNIQUE(task_id, version)
);

-- Index for fast MITRE lookups
CREATE INDEX idx_node_interviews_mitre ON node_interviews
    USING GIN ((ttps->'mitre_techniques'));

-- Index for EEI priority
CREATE INDEX idx_node_interviews_eei_priority ON node_interviews
    ((eei->'detection'->>'priority'));
```

### 5.2 Graph Storage: SurrealDB

```surql
DEFINE TABLE node_interview SCHEMAFULL;

DEFINE FIELD task_id ON node_interview TYPE string;
DEFINE FIELD perspective ON node_interview TYPE object;
DEFINE FIELD ttps ON node_interview TYPE object;
DEFINE FIELD toolchain ON node_interview TYPE object;
DEFINE FIELD eei ON node_interview TYPE object;

-- Relationships as edges
DEFINE TABLE enables SCHEMAFULL;
DEFINE FIELD weight ON enables TYPE float;

DEFINE TABLE requires SCHEMAFULL;
DEFINE FIELD priority ON requires TYPE string;

-- Link interviews
RELATE node_interview:001_006_001->enables->node_interview:001_007_001
    SET weight = 0.85;
```

### 5.3 Vector Storage

For semantic search across interviews:
```python
# Embed narratives for similarity search
interview_embedding = embed(
    interview['perspective']['2n']['narrative'] +
    interview['perspective']['1n']['narrative']
)

# Store in pgvector
INSERT INTO node_interview_vectors (interview_id, embedding)
VALUES (interview_id, interview_embedding);
```

---

## 6. Integration Points

### 6.1 Graph Convergence (RFC-9021)

Node interviews feed convergence:
- EEI questions drive collection requirements
- Indicators feed H1 operational convergence
- Historical patterns feed H2 semantic convergence

### 6.2 HD4 Phases (RFC-9020)

Each interview maps to an HD4 phase:
- Hunt: Reconnaissance, planning tasks
- Detect: Initial access, OPSEC tasks
- Disable: Persistence, credential tasks
- Disrupt: Lateral movement, C2 tasks
- Dominate: Exfiltration, impact tasks

### 6.3 Thalamic Filter (RFC-9024)

Node interview indicators become thalamic filter rules:
```python
# Generate Wazuh rules from interview indicators
for indicator in interview['indicators']['network']:
    generate_wazuh_rule(indicator, interview['ttps']['mitre_techniques'])
```

### 6.4 MITRE Integration (RFC-9023)

Crosswalk between interviews and frameworks:
- mitre_techniques → ATT&CK technique pages
- d3fend_countermeasures → D3FEND defensive actions
- toolchain.commercial.detection → CAR analytics

---

## 7. Summary

Node interviews are the **ground truth** for CTAS threat analysis. They must be:
- **Specific:** Link to actual tasks, tools, techniques
- **Dual-perspective:** Both 1n (defender) and 2n (adversary)
- **EEI-driven:** Concrete intelligence questions
- **Time-aware:** Time-of-value decay built in
- **Actionable:** Generate detection rules and collection requirements

The current generic interviews must be regenerated using this schema.

---

## 8. ATL-Physical Domain Interviews

### 8.1 Physical Domain Overview

The Node Interview schema extends to **ATL-Physical** (Adversary Task List - Physical Domain) for kinetic threat modeling. Physical domain interviews follow the same RFC-9025 structure with domain-specific adaptations.

### 8.2 Physical Domain Voice Template

```
I am [Task Name]. I am [role in physical attack chain].

I [primary action] using [methods and materials]. I [secondary action] through
[operational security measures].

You have seen me in [Incident 1] where [specific details]. You have seen me
in [Incident 2] where [specific details]. I was [historical context] in
[famous attacks].

My indicators are [observable indicators]. [Behavioral patterns]. [Temporal
signatures]. I try to [concealment method], but if you're [detection approach],
you'll see [detection signature].

My success means [attack outcome]. My failure means [interdiction outcome].
I feed [downstream tasks] and enable [dependent phases]. Without me,
[consequence of absence].

[Interdiction guidance for Left-of-Bang intervention]
```

### 8.3 Physical Domain Schema Extensions

```json
{
  "task_id": "1.2.3",
  "domain": "physical",
  "modality": "IED",
  "voice": "<first-person narrative>",
  "purpose": "<attack chain role>",
  "ownership": {
    "actor_types": ["lone_wolf", "cell", "network", "state_sponsored"],
    "skill_level": "low|medium|high|expert",
    "resource_requirements": "minimal|moderate|substantial|extensive"
  },
  "ttl_classification": "MANDATORY|DESIRABLE|OPTIONAL",
  "phase_in_chain": 1-6,
  "hd4_mapping": {
    "primary_phase": "HUNT|DETECT|DISABLE|DISRUPT|DOMINATE",
    "secondary_phases": []
  },
  "indicators": {
    "observable": ["<what can be seen/detected>"],
    "behavioral": ["<suspicious behaviors>"],
    "temporal": ["<timing patterns>"],
    "material": ["<physical evidence>"]
  },
  "detection_methods": {
    "technical": ["<sensors, cameras, screening>"],
    "human": ["<behavioral analysis, informants>"],
    "procedural": ["<background checks, verification>"]
  },
  "interdiction": {
    "is_interdiction_point": true|false,
    "intervention_methods": ["<how to disrupt>"],
    "window_of_opportunity": "<time available>",
    "consequences_of_miss": "<what happens if missed>"
  },
  "mundanity_analysis": {
    "score": 0.0-1.0,
    "cover_activities": ["<legitimate activities this resembles>"],
    "distinguishing_factors": ["<what separates hostile from benign>"]
  },
  "historical_examples": {
    "incidents": ["<real attacks>"],
    "lessons_learned": ["<key takeaways>"]
  },
  "countermeasures": {
    "preventive": ["<stop before>"],
    "detective": ["<identify when>"],
    "responsive": ["<react after>"]
  }
}
```

### 8.4 Physical Domain Storage

ATL-Physical interviews are stored in a separate Neo4j container with `:ATLPhysical` label:

```
Container: neo4j-atl-physical
Browser:   http://localhost:7475
Bolt:      bolt://localhost:7688

Nodes:
- :ATLPhysical:AdversaryTask (157)
- :ATLPhysical:Interview (156)
- :ATLPhysical:Indicator (2,285)
- :ATLPhysical:HD4Phase (5)

Relationships:
- :HAS_INTERVIEW (task → interview)
- :HAS_INDICATOR (interview → indicator)
- :MAPS_TO_HD4 (task → phase)
```

### 8.5 Cross-Domain Pattern Recognition

Physical domain interviews enable cross-domain analysis:

```cypher
-- Find HD4 phase overlaps between cyber and physical
MATCH (cyber:Technique)-[:BELONGS_TO]->(tactic:Tactic)
WHERE tactic.name = 'reconnaissance'
MATCH (physical:ATLPhysical:AdversaryTask)-[:MAPS_TO_HD4]->(phase:HD4Phase)
WHERE phase.phase_name = 'HUNT'
RETURN cyber.name AS cyber_technique,
       physical.title AS physical_task,
       'HUNT/Reconnaissance' AS common_phase
```

---

## 9. Related RFCs

- RFC-9011-A: Canonical Intelligence Ingestion Pipeline (Section 9: ATL-Physical)
- RFC-9020: HD4 Framework
- RFC-9021: Graph Convergence Theory
- RFC-9023: Security Framework Integration Map
- RFC-9024: Neurological Foundation
- RFC-9100: Dual Trivariate PTCC Integration
- RFC-9101: Smart Crate System
