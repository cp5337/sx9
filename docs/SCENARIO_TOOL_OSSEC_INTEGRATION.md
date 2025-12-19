# Scenario → Tool Executor → OSSEC Integration Architecture

**Version:** 1.0  
**Date:** December 16, 2025  
**Status:** Design Complete, Implementation Pending

---

## 1. Overview

This document describes the integration architecture between:
- **Scenario Engine** (169 validated scenarios)
- **PTCC Personas** (12 elite personas + 312 PTCC operators)
- **Tool Executor** (95 Kali tools × 4 tiers)
- **OSSEC/Wazuh** (700 TOML rules)
- **Correlation Engine** (TETH entropy + PTCC matching)

The goal is a **closed-loop feedback system** where:
1. Scenarios drive tool execution through PTCC personas
2. Tool outputs are hashed and stored
3. OSSEC monitors outputs and generates alerts
4. Correlation engine validates detection effectiveness
5. Feedback refines scenario parameters and persona proficiencies

---

## 2. Component Inventory

### 2.1 Scenarios (169 validated)
- **Source:** Nyx-Trace repository
- **Types:** APT campaigns, ransomware, supply chain, convergent attacks
- **Examples:** APT29, WannaCry, Volt Typhoon, Mumbai 2008

### 2.2 PTCC Personas
- **Elite Team:** 12 personas (Natasha Volkov, Michael Hayes, etc.)
- **PTCC Operators:** 312 (FrostFlux, SteelViper, ByteStrike, etc.)
- **Skill Levels:** 1.0-5.0+ (Script Kiddie → APT Elite)
- **Entropy Range:** 0.27-0.66 (TETH calculated)

### 2.3 Tools (95 Kali + OSINT + Commercial)
- **Reconnaissance:** nmap, masscan, Shodan, theHarvester
- **Exploitation:** Metasploit, Hydra, sqlmap, Burp Suite
- **Evasion:** Tor, proxychains, shred
- **Execution:** msfvenom, Empire, Cobalt Strike

### 2.4 OSSEC TOML Rules (700 rules)
- **Location:** `tools/abe/iac/node-interview-generator/output/ossec_toml_rules/`
- **Format:** TOML with RFC-9001 trivariate hashes
- **Features:**
  - PTCC primitive mapping (`unicode_trigger`)
  - Nine-sided analytics (`[nine_sided]`)
  - Active response commands
  - 1NF/2NF evasion tactics

### 2.5 Unified Crosswalk
- **Location:** `tools/abe/iac/node-interview-generator/output/unified_task_tool_ptcc_crosswalk.json`
- **Coverage:** 164 tasks, 95 tools, 32 PTCC primitives, 6 skill categories

---

## 3. Data Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SCENARIO SELECTION                               │
│  Input: scenario_id (e.g., "APT29SpearPhishing")                        │
│  Output: ScenarioConfig with HD4 phases, personas, tool chains          │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         PERSONA ASSIGNMENT                               │
│  Select PTCC persona based on:                                          │
│  - Scenario type                                                        │
│  - Required skill level                                                 │
│  - HD4 phase preferences                                                │
│  - Tool proficiencies                                                   │
│  Output: PersonaAssignment with tool_chain, entropy, skill_level        │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         TOOL EXECUTOR                                    │
│  For each tool in tool_chain:                                           │
│    1. Select tier (0=help, 1=localhost, 2=safe, 3=synthetic)           │
│    2. Execute in Docker container                                       │
│    3. Capture stdout/stderr                                             │
│    4. Generate dual trivariate hash (h1=TOML, h2=JSON)                 │
│    5. Create short code (e.g., NMP7X2D)                                │
│    6. Store in tool-corpus/                                            │
│    7. Emit to NATS: sx9.tool.{tool_name}.output                        │
│  Output: ToolOutput with hash_ref, short_code, raw_output              │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         OSSEC RULE MATCHING                              │
│  For each tool output:                                                  │
│    1. Parse output format (XML, JSON, text)                            │
│    2. Match against 700 TOML rules                                     │
│    3. Extract PTCC primitive from rule (unicode_trigger)               │
│    4. Calculate nine_sided analytics                                   │
│    5. Generate alert if level >= threshold                             │
│    6. Execute active_response if configured                            │
│  Output: OssecAlert with rule_id, level, sch_id, primitives            │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         CORRELATION ENGINE                               │
│  Correlate tool outputs with OSSEC alerts:                              │
│    1. Match tool_output.hash_ref with alert.sch_id                     │
│    2. Calculate detection_rate = alerts_generated / tools_executed     │
│    3. Calculate false_positive_rate                                    │
│    4. Update TETH entropy based on actual vs expected                  │
│    5. Score persona effectiveness                                      │
│  Output: CorrelationResult with detection_metrics, entropy_delta       │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         FEEDBACK LOOP                                    │
│  Update scenario parameters:                                            │
│    1. Adjust persona skill_level based on detection_rate               │
│    2. Update tool proficiency scores                                   │
│    3. Refine entropy_h calculation                                     │
│    4. Update Monte Carlo success_probability                           │
│    5. Store lessons_learned                                            │
│  Output: Updated ScenarioConfig, PersonaConfig                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 4. NATS Subject Hierarchy

```
sx9.scenario.{scenario_id}.start      # Scenario initiated
sx9.scenario.{scenario_id}.phase      # HD4 phase transition
sx9.scenario.{scenario_id}.complete   # Scenario finished

sx9.persona.{persona_id}.assigned     # Persona assigned to scenario
sx9.persona.{persona_id}.tool         # Tool execution by persona
sx9.persona.{persona_id}.feedback     # Performance feedback

sx9.tool.{tool_name}.execute          # Tool execution request
sx9.tool.{tool_name}.output           # Tool output captured
sx9.tool.{tool_name}.hash             # Hash generated

sx9.ossec.rule.match                  # OSSEC rule matched
sx9.ossec.alert.generated             # Alert generated
sx9.ossec.response.active             # Active response triggered

sx9.correlate.detection               # Detection correlation
sx9.correlate.entropy                 # Entropy calculation
sx9.correlate.feedback                # Feedback to scenario
```

---

## 5. Hash Integration

### 5.1 Tool Output Hash (RFC-9001 Compliant)

```toml
# h1 - Semantic Hash (TOML)
[semantic]
short_code = "NMP7X2D"
hash = "a1b2c3d4e5f67890"
seed = "C7A50000"

[tool]
name = "nmap"
trigger_rune = "U+E500"
parser = "nmap_xml"

[context]
scenario_id = "APT29SpearPhishing"
persona_id = "natasha-volkov"
hd4_phase = "Hunt"
task_ids = ["uuid-001-006-001", "uuid-010-004-001"]
ptcc_primitives = ["READ", "CONNECT", "RECEIVE", "VALIDATE", "FILTER"]
```

```json
// h2 - Operational Hash (JSON)
{
  "operational": {
    "short_code": "NMP7X2D",
    "hash": "f0e1d2c3b4a59687",
    "seed": "C7A5FFFF"
  },
  "execution": {
    "timestamp": "2025-12-16T12:00:00Z",
    "duration_ms": 3500,
    "exit_code": 0,
    "tier": 1
  },
  "heredity": "(cons h1 h2)"
}
```

### 5.2 OSSEC Rule Hash Linking

```toml
# OSSEC rule with hash linkage
[rule]
id = 60503
level = 10
description = "Browser fingerprint spoofing detected"
primitive = "ROUTE"
unicode_trigger = "U+E40A"
sch_id = "SCH81d5c001e0ea6720"

# Link to tool output
[rule.tool_linkage]
expected_tools = ["nmap", "masscan"]
hash_match_pattern = "SCH.*"
```

---

## 6. Implementation Plan

### Phase 1: Tool Executor Enhancement
1. Add scenario_id, persona_id, hd4_phase to tool profiles
2. Update exerciser.sh to accept context parameters
3. Emit NATS events on tool execution

### Phase 2: OSSEC Integration
1. Create OSSEC rule loader from TOML
2. Implement rule matching engine
3. Generate alerts with hash linkage

### Phase 3: Correlation Engine
1. Build detection rate calculator
2. Implement TETH entropy updater
3. Create persona feedback mechanism

### Phase 4: Feedback Loop
1. Update scenario parameters based on correlation
2. Adjust persona proficiencies
3. Refine Monte Carlo probabilities

---

## 7. Files and Locations

| Component | Location |
|-----------|----------|
| Scenario Engine | `crates/sx9-threat-simulator/src/scenario_engine.rs` |
| PTCC Personas | `crates/sx9-threat-simulator/src/ptcc_personas.rs` |
| Tool Executor | `tools/kali-plasma/tool-exerciser/` |
| OSSEC Rules | `tools/abe/iac/node-interview-generator/output/ossec_toml_rules/` |
| Unified Crosswalk | `tools/abe/iac/node-interview-generator/output/unified_task_tool_ptcc_crosswalk.json` |
| Tool Corpus | `data/tool-corpus/` |
| PTCC Rules | `tools/abe/iac/output/ontology/ptcc_rules.json` |

---

## 8. Next Steps

1. [ ] Wire unified crosswalk into tool executor
2. [ ] Add PTCC primitive tagging to tool outputs
3. [ ] Create OSSEC rule matching service
4. [ ] Build correlation engine
5. [ ] Implement feedback loop to scenario engine
6. [ ] Test with APT29 scenario end-to-end



