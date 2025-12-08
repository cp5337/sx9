# RFC-9023: Security Framework Integration Map

**Status:** Canonical
**Author:** Charlie Payne
**Version:** 7.3.1
**Related:** RFC-9020 (HD4), RFC-9021 (Convergence), RFC-9022 (OODA)

---

## Abstract

This RFC defines how CTAS integrates **all major security frameworks** as data sources and reference vocabularies while maintaining HD4/DDA as the primary analytical methodology. Frameworks provide vocabulary and structure; CTAS provides the cognitive engine.

---

## 1. Philosophy

### 1.1 Frameworks as Data, Not Doctrine

CTAS consumes frameworks to:
- Provide common vocabulary with external partners
- Ingest structured threat intelligence
- Map our primitives to industry standards
- Enable interoperability and compliance

CTAS does NOT:
- Replace HD4 with any external kill chain
- Require framework mapping for internal operations
- Treat any external framework as the source of truth

### 1.2 Why We Need Multiple Frameworks

No single framework covers the full threat landscape:

| Domain | Primary Framework |
|--------|------------------|
| Adversary TTPs | MITRE ATT&CK |
| Defensive countermeasures | MITRE D3FEND |
| Deception operations | MITRE ENGAGE |
| AI/ML security | MITRE ATLAS |
| Risk management | NIST CSF |
| Threat relationships | Diamond Model |
| Intelligence sharing | ODNI CTF |
| Maturity progression | Hunting Maturity Model |
| SOC operations | Industry SOC models |

---

## 2. Master Framework Comparison

*Source: FAA Cybersecurity Threat Hunting RFI Response, SIXGEN 2024*

| Framework | Approach Type | Key Strengths | Best For |
|-----------|---------------|---------------|----------|
| **MITRE ATT&CK** | Tactical/Technique-Based | Widely used, detailed taxonomy, regularly updated | Cross-industry, structured threat detection |
| **Cyber Kill Chain** | Linear Attack Lifecycle | Emphasizes early detection and prevention | Environments prioritizing early prevention |
| **CrowdStrike Maturity Model** | Maturity/Capability Building | Gradual improvement, hypothesis-driven approach | Organizations focused on maturity building |
| **Cisco Threat Hunting** | Product-Integrated | High visibility, automation, centralized behavioral analysis | Cisco-dependent organizations |
| **PEAK (Splunk)** | Multi-Tiered Hunting Model | Hypothesis-driven, balances manual and automated techniques | Mature SOCs using Splunk |
| **CTAS with HD4** | Multi-Domain, Offensive Analysis | Real-time adaptability, adversary control, deception techniques | High-stakes environments (critical infrastructure) |
| **Diamond Model** | Relational Analysis | Focuses on adversary-victim relationships, predictive insights | SOCs needing relational analysis |
| **NIST Cybersecurity Framework** | Comprehensive Security Framework | Broad coverage, regulatory alignment, risk management | Regulated industries, broad risk management |
| **ODNI Cyber Threat Framework** | Objective-Based | Emphasizes adversary intent, supports intelligence sharing | Government, intelligence sectors |
| **Hunting Maturity Model (Sqrrl)** | Maturity/Capability Building | Structured growth from reactive to proactive hunting | Organizations building threat-hunting maturity |

### 2.1 Where CTAS Fits

CTAS HD4 is positioned for **high-stakes, multi-domain environments** where:
- Static hypothesis-driven approaches are insufficient
- Adversary deception and engagement is required
- Real-time adaptability across cyber/physical/RF domains is critical
- Vertical escalation (tactical → national) is necessary

---

## 3. MITRE Framework Suite

### 3.1 ATT&CK (Adversarial Tactics, Techniques, and Common Knowledge)

**What it is:** Taxonomy of adversary behaviors post-compromise

**How CTAS uses it:**
- TTP vocabulary for threat intelligence sharing
- Node labeling in 164-task graph
- STIX/TAXII ingest format

**Mapping to HD4:**

| ATT&CK Tactic | HD4 Phase |
|---------------|-----------|
| Reconnaissance | Hunt (2n perspective) |
| Resource Development | Hunt (2n) |
| Initial Access | Detect |
| Execution | Detect/Disable |
| Persistence | Disable |
| Privilege Escalation | Disable |
| Defense Evasion | Disrupt |
| Credential Access | Disable |
| Discovery | Detect |
| Lateral Movement | Disrupt |
| Collection | Disrupt |
| Command and Control | Disrupt/Dominate |
| Exfiltration | Dominate |
| Impact | Dominate |

### 3.2 D3FEND (Detection, Denial, and Disruption Framework)

**What it is:** Countermeasure taxonomy - defensive techniques

**How CTAS uses it:**
- Reference for Disable/Disrupt actions
- Defensive playbook building blocks
- 1n response options

**Integration:**
```cypher
// Query D3FEND countermeasures for detected technique
MATCH (t:Technique {mitre_id: 'T1003.001'})
CALL d3fend.countermeasures(t.mitre_id) YIELD defense, effectiveness
WHERE effectiveness > 0.7
RETURN defense.name, defense.implementation
```

### 3.3 ENGAGE (Adversary Engagement)

**What it is:** Framework for denial, deception, and adversary engagement

**How CTAS uses it:**
- Dominate phase tactics
- Deception operations
- Controlled adversary interaction

**Mapping to HD4:**

| ENGAGE Goal | HD4 Phase | Application |
|-------------|-----------|-------------|
| Expose | Detect | Reveal adversary presence |
| Affect | Disrupt | Degrade adversary capability |
| Elicit | Dominate | Gather intelligence through engagement |
| Understand | Hunt | Learn adversary TTPs |

### 3.4 ATLAS (Adversarial Threat Landscape for AI Systems)

**What it is:** ATT&CK-style framework for AI/ML attacks (14 tactics, 82 techniques)

**How CTAS uses it:**
- Protect GNN, Phi3, DistilBERT components
- Model evasion detection
- Data poisoning prevention

**Critical for CTAS because:** We run AI/ML systems that are themselves attack targets.

**Key techniques to monitor:**
- AML.T0043 - Model Evasion
- AML.T0044 - Model Theft
- AML.T0020 - Data Poisoning
- AML.T0010 - ML Supply Chain Compromise

### 3.5 CAR (Cyber Analytics Repository)

**What it is:** Detection analytics with pseudocode and implementations

**How CTAS uses it:**
- Sigma rule generation
- Wazuh rule templates
- Detection engineering starting points

### 3.6 Caldera

**What it is:** Automated adversary emulation platform

**How CTAS uses it:**
- 2n perspective automation
- Red team playbook execution
- Ability/operation library

### 3.7 Navigator

**What it is:** Visualization tool for ATT&CK coverage

**How CTAS uses it:**
- Export coverage heatmaps for customer deliverables
- Gap analysis reports

**NOT used for:** Internal analysis (we use Cesium/Graph viewer)

---

## 4. NIST Framework Integration

### 4.1 NIST Cybersecurity Framework (CSF)

**What it is:** Risk-based approach to managing cybersecurity risk

**Five Core Functions:**

| NIST Function | HD4 Phase Mapping | CTAS Application |
|---------------|-------------------|------------------|
| **Identify** | Pre-Hunt | Asset inventory, risk assessment |
| **Protect** | Pre-Hunt | Defensive controls, hardening |
| **Detect** | Detect | Anomaly detection, monitoring |
| **Respond** | Disable/Disrupt | Incident response, containment |
| **Recover** | Post-Dominate | Restoration, lessons learned |

**How CTAS uses it:**
- Compliance reporting for regulated industries
- Risk prioritization for task graph nodes
- Framework for stakeholder communication

### 4.2 NIST SP 800-53 Controls

**What it is:** Security and privacy controls for federal systems

**How CTAS uses it:**
- Control mapping for government customers
- Audit trail documentation
- RMF (Risk Management Framework) compliance

---

## 5. Intelligence Community Frameworks

### 5.1 ODNI Cyber Threat Framework (CTF)

**What it is:** Objective-based framework emphasizing adversary intent

**How CTAS uses it:**
- Intelligence sharing with government partners
- Attribution support
- Strategic-level threat analysis

**Key Concept:** ODNI CTF focuses on **adversary objectives** not just techniques - aligns with HD4 2n perspective.

### 5.2 Intelligence Cycle Integration

CTAS maps to the traditional intelligence cycle:

| Intel Cycle Phase | CTAS Component |
|-------------------|----------------|
| Planning & Direction | Task graph configuration, EEI definition |
| Collection | 247 WASM sensors, OSINT pipeline |
| Processing | Thalamic filter (NeedleExtractor) |
| Analysis | Graph convergence (H1 + H2) |
| Dissemination | PLASMA dashboard, alerts |
| Feedback | Node interview refinement |

---

## 6. Diamond Model

### 6.1 Overview

**What it is:** Relational analysis model focusing on adversary-victim relationships

**Four Vertices:**
1. **Adversary** - Who is attacking
2. **Capability** - What tools/techniques
3. **Infrastructure** - What systems used
4. **Victim** - Who is targeted

### 6.2 CTAS Integration

```
Diamond Model:
                 Adversary
                    │
         ┌──────────┼──────────┐
         │          │          │
    Capability ─────┼───── Infrastructure
         │          │          │
         └──────────┼──────────┘
                    │
                  Victim

CTAS Mapping:
  Adversary     → 2n perspective persona
  Capability    → Node interview toolchain
  Infrastructure → Crate interview resources
  Victim        → Target profile in task graph
```

**How CTAS uses it:**
- Threat attribution support
- Campaign clustering
- Predictive analysis (given 3 vertices, predict 4th)

---

## 7. SOC and Maturity Models

### 7.1 Hunting Maturity Model (HMM)

*Originally developed by Sqrrl (now AWS)*

| Level | Description | CTAS Equivalent |
|-------|-------------|-----------------|
| HM0 | Initial - Primarily reactive | No convergence monitoring |
| HM1 | Minimal - Some indicator matching | H1 only (operational) |
| HM2 | Procedural - Follows documented procedures | Static playbooks |
| HM3 | Innovative - Creates new procedures | Dynamic node interviews |
| HM4 | Leading - Automates and shares new procedures | Full HD4 + DDA |

### 7.2 SOC-CMM (SOC Capability Maturity Model)

**How CTAS uses it:**
- Customer maturity assessment
- Phased deployment planning
- Capability gap analysis

### 7.3 PEAK Framework (Splunk)

**Three Hunting Types:**
1. **Hypothesis-driven** - Start with theory, validate
2. **Baseline** - Know normal, find abnormal
3. **Model-Assisted** - ML-driven detection

**CTAS equivalent:**
- Hypothesis-driven → 2n adversary emulation
- Baseline → H1 operational convergence
- Model-Assisted → GNN + HMM phase detection

---

## 8. Industry-Specific Frameworks

### 8.1 ICS/OT Frameworks

| Framework | Scope | CTAS Integration |
|-----------|-------|------------------|
| MITRE ATT&CK for ICS | Industrial control systems | Task graph ICS nodes |
| IEC 62443 | Industrial cybersecurity | Compliance mapping |
| NERC CIP | Electric utilities | Regulatory requirements |

### 8.2 Healthcare

| Framework | Scope | CTAS Integration |
|-----------|-------|------------------|
| HIPAA Security Rule | Healthcare data | Privacy controls |
| HITRUST CSF | Healthcare IT | Risk assessment |

### 8.3 Financial

| Framework | Scope | CTAS Integration |
|-----------|-------|------------------|
| PCI DSS | Payment card data | Control mapping |
| FFIEC CAT | Financial institutions | Maturity assessment |

---

## 9. Crosswalk Tables

### 9.1 Master HD4 ↔ All Frameworks

| HD4 Phase | ATT&CK | NIST CSF | Diamond | ODNI CTF |
|-----------|--------|----------|---------|----------|
| **Hunt** | Reconnaissance | Identify | Adversary analysis | Preparation |
| **Detect** | Initial Access, Discovery | Detect | Capability identification | Engagement |
| **Disable** | Persistence, PrivEsc | Respond | Infrastructure mapping | Presence |
| **Disrupt** | Lateral Movement, C2 | Respond | Campaign disruption | Effect/Consequence |
| **Dominate** | Exfiltration, Impact | Recover | Full attribution | Objective achievement |

### 9.2 1n/2n Perspective ↔ Framework Purpose

| Perspective | Framework | Purpose |
|-------------|-----------|---------|
| **1n (Defender)** | D3FEND | Countermeasure selection |
| **1n** | NIST CSF | Risk management |
| **1n** | CAR | Detection rules |
| **2n (Adversary)** | ATT&CK | Adversary emulation |
| **2n** | Caldera | Automated red team |
| **2n** | Diamond | Adversary profiling |
| **Both** | ENGAGE | Deception operations |
| **Both** | ATLAS | AI attack/defense |
| **Both** | ODNI CTF | Intelligence sharing |

### 9.3 DDA Levels ↔ Framework Applicability

| DDA Level | Primary Frameworks | Scope |
|-----------|-------------------|-------|
| **Tactical** | ATT&CK Enterprise, CAR | Single host/network |
| **Operational** | ATT&CK + Mobile + ICS, NIST | Organization |
| **Strategic** | Diamond, ODNI CTF | Sector/Region |
| **National** | All + Intelligence sharing | Coalition |

---

## 10. What We Don't Take

### 10.1 Kill Chain Theology

ATT&CK assumes linear progression. HD4 is non-linear - you can be in Dominate phase on one node while still Hunting on another.

### 10.2 Single Perspective Lock

Most frameworks are defender-centric. CTAS requires constant 1n/2n weave.

### 10.3 Static Detection Model

Frameworks provide signatures. CTAS watches **convergence** - the combination matters more than individual techniques.

### 10.4 Flat Organizational View

Frameworks don't have built-in vertical escalation. CTAS/DDA does (tactical → national).

### 10.5 Compliance as Security

NIST compliance ≠ security. Framework adherence is necessary but not sufficient.

---

## 11. Implementation Priority

| Framework | Priority | Integration Status | Notes |
|-----------|----------|-------------------|-------|
| ATT&CK | P1 | Partial | TTP vocabulary |
| D3FEND | P1 | Not started | Critical for 1n |
| ATLAS | P1 | Not started | Protect our AI |
| NIST CSF | P1 | Not started | Customer compliance |
| Diamond | P2 | Not started | Attribution |
| ENGAGE | P2 | Not started | Deception ops |
| CAR | P2 | Not started | Detection rules |
| ODNI CTF | P2 | Not started | Gov customers |
| Caldera | P3 | Stub exists | 2n automation |
| Navigator | P3 | Export only | Reporting |

---

## 12. Summary

Security frameworks are **data sources** and **common vocabulary** for CTAS, not the analytical framework itself.

**CTAS HD4/DDA provides:**
- Cognitive model (Hunt/Detect/Disable/Disrupt/Dominate)
- 1n/2n perspective weave
- Vertical escalation (tactical → national)
- Graph convergence detection
- 164 tasks as primitive layer

**External frameworks provide:**
- Common vocabulary for interoperability
- Compliance mapping for regulated industries
- Detection rules and countermeasure catalogs
- Intelligence sharing standards

**The combination:** HD4 thinking + framework vocabulary = effective threat hunting for complex environments.

---

## 13. References

### MITRE
- ATT&CK: https://attack.mitre.org
- D3FEND: https://d3fend.mitre.org
- ENGAGE: https://engage.mitre.org
- ATLAS: https://atlas.mitre.org
- CAR: https://car.mitre.org
- Caldera: https://caldera.mitre.org

### NIST
- CSF: https://www.nist.gov/cyberframework
- SP 800-53: https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final

### Intelligence Community
- ODNI CTF: https://www.dni.gov/index.php/cyber-threat-framework

### Other
- Diamond Model: https://www.threatintel.academy/diamond/
- Hunting Maturity Model: https://www.sans.org/white-papers/36852/

---

## 14. Related RFCs

- RFC-9020: HD4 Framework
- RFC-9021: Graph Convergence Theory
- RFC-9022: OODA Vertical Escalation
- RFC-9024: Neurological Foundation
