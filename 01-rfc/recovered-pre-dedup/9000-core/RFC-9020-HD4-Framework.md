# RFC-9020: HD4 Framework - Hunt, Detect, Disable, Disrupt, Dominate

**Status:** Canonical
**Author:** Charlie Payne (derived from FAA Cybersecurity Threat Hunting RFI, SIXGEN 2024)
**Version:** 7.3.1

---

## Abstract

HD4 is a cognitive framework for threat analysis that operates across dual perspectives (1n/2n) and vertical context levels (tactical to national). It formalizes how operators actually think under pressure, enabling both human analysts and automated systems to weave between defensive and offensive postures while scaling context appropriately.

---

## 1. The Five Phases

> "We actively hunt, detect, disable, disrupt, and dominate threats."

| Phase | Purpose | 1n (Defensive) | 2n (Offensive) |
|-------|---------|----------------|----------------|
| **Hunt** | Proactive search | Hunt for threats to our assets | Hunt for targets/vulnerabilities |
| **Detect** | Identify indicators | Detect adversary presence | Detect defensive gaps |
| **Disable** | Remove capability | Disable adversary tools/access | Disable defensive controls |
| **Disrupt** | Break tempo | Disrupt adversary operations | Disrupt incident response |
| **Dominate** | Control battlespace | Dominate through deception/denial | Dominate through persistence |

---

## 2. Dual Perspective (1n/2n)

### 2.1 The Weave

At every operational level, analysts constantly shift perspective:

```
1n → 2n → 1n → 2n → ...

"What am I seeing?" → "What would THEY do?" → "What do I do?" → "What will they do next?"
```

This is not sequential - it's a continuous cognitive oscillation that provides:
- **Anticipation** - Predict adversary next moves
- **Validation** - Test defensive assumptions
- **Adaptation** - Adjust posture in real-time

### 2.2 Node Interviews

Every task in the CTAS graph has TWO interviews:
- **1n Interview** - First-person defender perspective
- **2n Interview** - First-person adversary perspective

This ensures the graph encodes both sides of every engagement.

---

## 3. Vertical Context Shift (DDA)

Dynamic Domain Adaptation scales the same HD4 phases across operational levels:

| Level | JP 3-0 Equivalent | HD4 Scope | Time Horizon |
|-------|-------------------|-----------|--------------|
| **Tactical** | Unit | Individual device/node | Seconds-minutes |
| **Operational** | Campaign | Network/organization | Hours-days |
| **Strategic** | Theater | Sector/region | Days-weeks |
| **National** | Policy | Coalition/global | Weeks-months |

**Key Principle:** The 164 tasks remain constant - only the context changes.

---

## 4. Framework Integration

HD4 integrates with established frameworks:

| Framework | Integration Point |
|-----------|-------------------|
| **MITRE ATT&CK** | Adversary TTPs mapped to tasks |
| **MITRE ENGAGE** | Deception techniques for Dominate phase |
| **MITRE D3FEND** | Defensive techniques for Disable/Disrupt |
| **MITRE ATLAS** | AI/ML threat techniques |
| **Diamond Model** | Relational analysis across nodes |
| **NIST CSF** | Risk management alignment |
| **ODNI CTF** | Intelligence community interoperability |

---

## 5. Relative Superiority

Per McRaven's "Theory of Special Operations," relative superiority is achieved by operating inside the adversary's decision cycle.

### 5.1 Nested OODA

HD4 enables nested OODA loops at every level:

```
Operator:   OODA on device     ←→  Adversary OODA on target
Team:       OODA on objective  ←→  Adversary OODA on mission
Commander:  OODA on operation  ←→  Adversary OODA on campaign
National:   OODA on conflict   ←→  Adversary OODA on strategy
```

### 5.2 The Window

Relative superiority occurs when YOUR convergence exceeds THEIR convergence:

```
When: Your_H1 + Your_H2 > Their_H1 + Their_H2
Then: ACT (you're inside their cycle)
```

---

## 6. Convergent Threats

HD4 addresses convergent threats - multi-domain attacks across:
- Cyber
- Physical
- RF/Communications
- Financial
- Human/Social

The same HD4 phases apply regardless of domain. DDA enables seamless context switching between domains.

---

## 7. Playbooks and Maturity Model

### 7.1 Playbook Structure

```toml
[playbook.apt_detection]
phases = ["hunt", "detect", "disable", "disrupt", "dominate"]
trigger = "anomalous_credential_access"
perspective = "1n"  # or "2n" for red team

[playbook.apt_detection.hunt]
watch_nodes = ["T1003", "T1078", "T1021"]
convergence_threshold = 0.65

[playbook.apt_detection.detect]
indicators = ["unusual_logon_times", "service_account_interactive"]
escalation = "tier_2"

[playbook.apt_detection.disable]
actions = ["isolate_endpoint", "revoke_credentials"]

[playbook.apt_detection.disrupt]
actions = ["deploy_decoy", "throttle_lateral"]

[playbook.apt_detection.dominate]
actions = ["controlled_engagement", "attribution_collection"]
```

### 7.2 Maturity Model

Playbook results drive maturity progression:

| Level | Capability | HD4 Integration |
|-------|------------|-----------------|
| 0 | Reactive | Post-incident only |
| 1 | Baseline | Hunt phase initiated |
| 2 | Informed | Detect phase reliable |
| 3 | Proactive | Disable/Disrupt automated |
| 4 | Adaptive | Full HD4 with 1n/2n weave |
| 5 | Predictive | Convergence-driven anticipation |

---

## 8. Implementation in CTAS

### 8.1 Task Mapping

All 164 CTAS tasks map to HD4 phases:

```sql
-- Example: Task phase assignment
SELECT task_id, task_name, hd4_phase
FROM ctas_tasks
WHERE hd4_phase = 'hunt';
```

### 8.2 Hash Integration

- **H1 (Operational)** - Senses operational HD4 state
- **H2 (Semantic)** - Senses pattern match to known HD4 playbooks

### 8.3 Graph Convergence

The dual trivariate enables convergence detection per HD4 phase:

```
Hunt phase convergence:     ████░░░░░░ 42%
Detect phase convergence:   ██████░░░░ 61%
Disable phase ready:        BLOCKED (detect < 75%)
```

---

## 9. References

- FAA Cybersecurity Threat Hunting RFI Response, SIXGEN Inc., 2024
- McRaven, W. "The Theory of Special Operations" (Naval Postgraduate School, 1993)
- Joint Publication 3-0, Joint Operations
- MITRE ATT&CK Framework
- MITRE ENGAGE Framework
- Boyd, J. "OODA Loop" (Patterns of Conflict, 1986)

---

## 10. Related RFCs

- RFC-9000: SX9 Core Architecture
- RFC-9100: Dual Trivariate PTCC Integration
- RFC-9021: Graph Convergence Theory
- RFC-9022: OODA-HD4 Integration
