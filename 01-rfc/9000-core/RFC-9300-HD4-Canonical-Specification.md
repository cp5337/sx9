# RFC-9300: HD4 Canonical Specification

**Version:** 1.0.0  
**Status:** NORMATIVE  
**Date:** 2025-12-06  
**Author:** CTAS Architecture Team  
**Supersedes:** All prior HD4 definitions in RFCs 9020, 9024, 9025, 9109, 9110  

---

## Abstract

This RFC establishes the canonical definition of the HD4 (Hunt-Detect-Disrupt-Disable-Dominate) framework as a unified operational model for threat analysis across cyber, kinetic, and cognitive domains. HD4 subsumes existing frameworks (MITRE ATT&CK, D3FEND, ENGAGE, Kill Chain, STRIDE, etc.) into a domain-agnostic operational paradigm applicable to physical security, cybersecurity, counterterrorism, and hybrid threats.

---

## 1. Canonical Definition

### 1.1 Phase Order (NORMATIVE)

```
HUNT → DETECT → DISRUPT → DISABLE → DOMINATE
```

This order is **immutable** and **canonical**. All implementations MUST use this sequence.

### 1.2 Phase Semantics

| Phase | Code | Description | Operational Posture |
|-------|------|-------------|---------------------|
| **Hunt** | H | Active threat-seeking; persistent reconnaissance; environmental awareness | Proactive |
| **Detect** | D¹ | Positive identification; confirmation of threat indicators; classification | Reactive-Aware |
| **Disrupt** | D² | Active interference; degradation of adversary capability; friction injection | Active-Offensive |
| **Disable** | D³ | Neutralization of threat capability; functional elimination | Decisive |
| **Dominate** | D⁴ | Full control assertion; exploitation; terrain ownership | Exploitative |

### 1.3 Rust Implementation (NORMATIVE)

```rust
/// HD4 Operational Phase - Canonical Definition
/// RFC-9300 §1.1 - Order is immutable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Hd4Phase {
    Hunt     = 0,
    Detect   = 1,
    Disrupt  = 2,
    Disable  = 3,
    Dominate = 4,
}

impl Hd4Phase {
    pub const COUNT: usize = 5;
    
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Hunt     => Some(Self::Detect),
            Self::Detect   => Some(Self::Disrupt),
            Self::Disrupt  => Some(Self::Disable),
            Self::Disable  => Some(Self::Dominate),
            Self::Dominate => None,
        }
    }
    
    pub fn previous(self) -> Option<Self> {
        match self {
            Self::Hunt     => None,
            Self::Detect   => Some(Self::Hunt),
            Self::Disrupt  => Some(Self::Detect),
            Self::Disable  => Some(Self::Disrupt),
            Self::Dominate => Some(Self::Disable),
        }
    }
}
```

### 1.4 Unicode Allocation (NORMATIVE)

```
U+E700  HD4_HUNT
U+E701  HD4_DETECT
U+E702  HD4_DISRUPT
U+E703  HD4_DISABLE
U+E704  HD4_DOMINATE
```

Reserved: U+E705-E70F for HD4 extensions.

---

## 2. Framework Subsumption

HD4 provides a universal operational paradigm that subsumes specialized frameworks into its five-phase model.

### 2.1 MITRE Ecosystem Mapping

#### 2.1.1 ATT&CK Enterprise Tactics → HD4

| ATT&CK Tactic | HD4 Phase | Rationale |
|---------------|-----------|-----------|
| Reconnaissance | Hunt | Active information gathering |
| Resource Development | Hunt | Capability preparation |
| Initial Access | Detect | Entry point identification triggers detection |
| Execution | Disrupt | Active interference begins |
| Persistence | Disrupt | Maintaining foothold = sustained interference |
| Privilege Escalation | Disrupt | Capability expansion |
| Defense Evasion | Disrupt | Counter-detection activities |
| Credential Access | Disrupt | Credential theft disrupts auth systems |
| Discovery | Detect | Internal reconnaissance |
| Lateral Movement | Disrupt | Network traversal |
| Collection | Disable | Data aggregation precedes exfil |
| Command and Control | Disable | Sustained C2 = control assertion |
| Exfiltration | Dominate | Mission objective achievement |
| Impact | Dominate | Final effect delivery |

#### 2.1.2 D3FEND Tactics → HD4

| D3FEND Tactic | HD4 Phase | Defensive Posture |
|---------------|-----------|-------------------|
| Model | Hunt | Understanding the environment |
| Harden | Hunt | Proactive defense posture |
| Detect | Detect | Threat identification |
| Isolate | Disrupt | Containment = active interference |
| Deceive | Disrupt | Misdirection = capability degradation |
| Evict | Disable | Threat removal |
| Restore | Dominate | Control reassertion |

#### 2.1.3 ENGAGE (Active Defense) → HD4

| ENGAGE Activity | HD4 Phase | Purpose |
|-----------------|-----------|---------|
| Prepare | Hunt | Capability development |
| Expose | Detect | Adversary revelation |
| Affect | Disrupt | Behavior modification |
| Elicit | Disrupt | Intelligence extraction |
| Understand | Dominate | Strategic advantage |

#### 2.1.4 ATLAS (AI/ML Threats) → HD4

| ATLAS Tactic | HD4 Phase | AI-Specific Context |
|--------------|-----------|---------------------|
| Reconnaissance | Hunt | Model enumeration |
| Resource Development | Hunt | Adversarial capability building |
| ML Model Access | Detect | Initial model compromise detected |
| Execution | Disrupt | Model manipulation active |
| Persistence | Disrupt | Backdoor maintenance |
| Defense Evasion | Disrupt | Evasion of ML security |
| Collection | Disable | Training data theft |
| ML Attack Staging | Disable | Attack preparation complete |
| Exfiltration | Dominate | Model/data extraction |
| Impact | Dominate | Model corruption/misuse |

### 2.2 Kill Chain Mappings

#### 2.2.1 Lockheed Martin Cyber Kill Chain → HD4

| Kill Chain Phase | HD4 Phase | Notes |
|------------------|-----------|-------|
| Reconnaissance | Hunt | |
| Weaponization | Hunt | Capability preparation |
| Delivery | Detect | Payload arrival triggers detection |
| Exploitation | Disrupt | Active compromise |
| Installation | Disrupt | Persistence establishment |
| Command & Control | Disable | Control channel established |
| Actions on Objectives | Dominate | Mission execution |

#### 2.2.2 SANS ICS Kill Chain → HD4

| ICS Kill Chain Phase | HD4 Phase | OT Context |
|---------------------|-----------|------------|
| Reconnaissance | Hunt | |
| Weaponization | Hunt | |
| Targeting | Hunt | Target selection |
| Delivery | Detect | |
| Exploitation | Disrupt | |
| Installation | Disrupt | |
| Command & Control | Disable | |
| Act | Dominate | Physical effect |
| Maintain | Dominate | Persistent control |

### 2.3 Threat Modeling Frameworks → HD4

#### 2.3.1 STRIDE → HD4

| STRIDE Category | HD4 Phase(s) | Context |
|-----------------|--------------|---------|
| Spoofing | Disrupt | Identity subversion |
| Tampering | Disrupt / Disable | Data modification |
| Repudiation | Dominate | Evidence manipulation |
| Information Disclosure | Disable / Dominate | Data exfiltration |
| Denial of Service | Disable | Availability destruction |
| Elevation of Privilege | Disrupt / Disable | Access escalation |

#### 2.3.2 DREAD (Risk Scoring) → HD4 Phase Transition Thresholds

| DREAD Factor | HD4 Application |
|--------------|-----------------|
| Damage | Determines phase escalation speed |
| Reproducibility | Affects Hunt→Detect transition confidence |
| Exploitability | Influences Disrupt timing |
| Affected Users | Scales response intensity |
| Discoverability | Impacts Hunt effectiveness |

**Phase Transition Formula:**
```
HD4_Phase = f(DREAD_Score)

if DREAD_avg < 3.0:  Hunt (monitoring)
if DREAD_avg < 5.0:  Detect (confirmation)
if DREAD_avg < 7.0:  Disrupt (active response)
if DREAD_avg < 9.0:  Disable (neutralization)
if DREAD_avg >= 9.0: Dominate (full control)
```

#### 2.3.3 PASTA (7-Stage) → HD4

| PASTA Stage | HD4 Phase |
|-------------|-----------|
| 1. Define Objectives | Hunt |
| 2. Define Technical Scope | Hunt |
| 3. Application Decomposition | Hunt |
| 4. Threat Analysis | Detect |
| 5. Vulnerability Analysis | Detect |
| 6. Attack Modeling | Disrupt |
| 7. Risk & Impact Analysis | Disable/Dominate |

---

## 3. Physical/Kinetic Domain Mappings

### 3.1 CBRN (Chemical, Biological, Radiological, Nuclear) → HD4

| CBRN Activity | HD4 Phase | Defensive Application |
|---------------|-----------|----------------------|
| Intelligence gathering | Hunt | Threat stream monitoring |
| Material detection | Detect | Sensors, sampling |
| Interdiction | Disrupt | Seizure, blocking |
| Neutralization | Disable | Render safe, decontamination |
| Exploitation | Dominate | Forensics, attribution |

### 3.2 IED/Explosive Threats → HD4

| Counter-IED Phase | HD4 Phase | EOD Context |
|-------------------|-----------|-------------|
| Route clearance | Hunt | Area reconnaissance |
| Detection | Detect | Device identification |
| Disruption | Disrupt | Remote neutralization |
| Render safe | Disable | Manual disarmament |
| Exploitation | Dominate | Technical intelligence |

### 3.3 Physical Access Control → HD4

| Physical Security | HD4 Phase | Facility Protection |
|-------------------|-----------|---------------------|
| Surveillance | Hunt | Perimeter monitoring |
| Intrusion detection | Detect | Alarm activation |
| Delay mechanisms | Disrupt | Barriers, locks |
| Neutralization | Disable | Guard response |
| Apprehension | Dominate | Detention, prosecution |

### 3.4 Counter-Terrorism Operations → HD4

| CT Phase | HD4 Phase | Operational Context |
|----------|-----------|---------------------|
| Intelligence collection | Hunt | Human/signals intel |
| Threat identification | Detect | Cell discovery |
| Disruption operations | Disrupt | Financial, comms interdiction |
| Direct action | Disable | Kinetic operations |
| Exploitation | Dominate | Capture, site exploitation |

---

## 4. OODA Loop Integration

### 4.1 OODA → HD4 Mapping

| OODA Phase | Primary HD4 | Secondary HD4 | Notes |
|------------|-------------|---------------|-------|
| Observe | Hunt | Detect | Continuous sensing |
| Orient | Detect | Hunt | Context building |
| Decide | Disrupt | Disable | Action selection |
| Act | Disable | Dominate | Effect delivery |

### 4.2 Temporal Relationship

```
OODA cycles WITHIN HD4 phases:

Hunt:     [O-O-D-A] → [O-O-D-A] → ... (continuous hunting loops)
Detect:   [O-O-D-A] → confirmation achieved → phase transition
Disrupt:  [O-O-D-A] → interference sustained → phase transition
Disable:  [O-O-D-A] → neutralization confirmed → phase transition
Dominate: [O-O-D-A] → control established → mission complete
```

---

## 5. H1/H2 Convergence Integration

Per RFC-9024 and RFC-9025, HD4 phase transitions are governed by convergence scores.

### 5.1 Phase Transition Thresholds

| H2 Score Range | Recommended HD4 Phase | Confidence Level |
|----------------|----------------------|------------------|
| 0.00 - 0.74 | Hunt | Low - Continue reconnaissance |
| 0.75 - 0.79 | Detect | Moderate - Confirm indicators |
| 0.80 - 0.84 | Disrupt | High - Begin interference |
| 0.85 - 0.89 | Disable | Very High - Neutralize |
| 0.90 - 1.00 | Dominate | Confirmed - Assert control |

### 5.2 75% Convergence Line

```
IF (H1 ≥ 0.75) AND (H2 ≥ 0.75) THEN
    ABOVE_LINE = true
    → Transition to next HD4 phase authorized
ELSE
    BELOW_LINE = true
    → Remain in current phase, continue OODA
```

---

## 6. Delta Angle Semantic Mapping

Per RFC-9109, trivariate delta angles map to HD4 phases.

### 6.1 Precision Requirements (NORMATIVE)

**All delta angles MUST:**
1. Include all three axes (X, Y, Z) in every representation
2. Use minimum 6-decimal precision (0.000000)
3. Never omit zero-valued axes

```rust
/// CORRECT - All axes present, 6-decimal precision
pub struct DeltaAngle {
    pub x: f64,  // Semantic axis    - min 6 decimal places
    pub y: f64,  // Operational axis - min 6 decimal places  
    pub z: f64,  // Temporal axis    - min 6 decimal places
}

impl DeltaAngle {
    pub const PRECISION: usize = 6;
    
    pub fn format(&self) -> String {
        format!(
            "Δ({:.6}, {:.6}, {:.6})",
            self.x, self.y, self.z
        )
    }
}

// CORRECT representations:
// Δ(0.000000, 0.250000, 0.000000)  - Detect phase, no semantic/temporal shift
// Δ(0.500000, 0.750000, 0.333333)  - Mid-semantic, Disable phase, partial temporal
// Δ(0.000000, 0.000000, 0.000000)  - Hunt baseline (all zeros still explicit)

// INCORRECT - Never do these:
// Δ(0.25)           - Missing axes
// Δ(0.5, 0.75)      - Missing Z axis
// Δ(.25, .5, .75)   - Insufficient precision
```

### 6.2 Axis Definitions

| Axis | Name | Range | Semantics |
|------|------|-------|-----------|
| **X** | Semantic | 0.000000 → 1.000000 | Threat classification (Recon → Exfil) |
| **Y** | Operational | 0.000000 → 1.000000 | HD4 Phase (Hunt → Dominate) |
| **Z** | Temporal | 0.000000 → 1.000000 | Time perspective (Historical → Predictive) |

### 6.3 Y-Axis (Operational) HD4 Allocation

```
Y = 0.000000: Hunt      (baseline seeking)
Y = 0.250000: Detect    (identification)
Y = 0.500000: Disrupt   (interference)
Y = 0.750000: Disable   (neutralization)
Y = 1.000000: Dominate  (control)
```

**Intermediate values** represent phase transitions:
```
Y = 0.125000: Hunt→Detect transition (50% confidence)
Y = 0.375000: Detect→Disrupt transition
Y = 0.625000: Disrupt→Disable transition
Y = 0.875000: Disable→Dominate transition
```

### 6.4 Complete Delta Angle Examples

| Scenario | X (Semantic) | Y (Operational) | Z (Temporal) | Interpretation |
|----------|--------------|-----------------|--------------|----------------|
| Baseline Hunt | 0.000000 | 0.000000 | 0.500000 | Recon phase, hunting, present-focused |
| Confirmed Detection | 0.250000 | 0.250000 | 0.500000 | Staging identified, detect phase |
| Active Disruption | 0.500000 | 0.500000 | 0.500000 | Mid-attack, disrupting, present |
| Imminent Disable | 0.750000 | 0.750000 | 0.750000 | Near-exfil, disabling, predictive |
| Full Domination | 1.000000 | 1.000000 | 1.000000 | Exfil complete, dominated, future-locked |

### 6.5 Threshold Mapping

| Δ-Angle Magnitude (degrees) | HD4 Phase | Response Urgency |
|-----------------------------|-----------|------------------|
| < 2.000000° | Hunt | Routine monitoring |
| 2.000000° - 10.000000° | Detect | Elevated awareness |
| 10.000000° - 25.000000° | Disrupt | Active response |
| 25.000000° - 60.000000° | Disable | Decisive action |
| > 60.000000° | Dominate | Full engagement |

### 6.6 JSON Serialization (NORMATIVE)

```json
{
  "delta_angle": {
    "x": 0.500000,
    "y": 0.750000,
    "z": 0.333333
  },
  "hd4_phase": "disable",
  "magnitude_degrees": 45.123456,
  "timestamp_utc": "2025-12-06T12:00:00.000000Z"
}
```

**Serialization Rules:**
1. All three axes MUST be present
2. Numeric values MUST have at least 6 decimal places
3. Trailing zeros MUST NOT be truncated
4. Scientific notation MAY be used for very small values (e.g., 1.000000e-10)

---

## 7. Implementation Requirements

### 7.1 MUST Requirements

1. All HD4 enums MUST use the order: Hunt, Detect, Disrupt, Disable, Dominate
2. Phase values MUST be 0-4 (not 1-5)
3. Unicode codepoints MUST be U+E700-U+E704
4. JSON representations MUST use lowercase phase names
5. All phase transitions MUST be logged with timestamps
6. **Delta angles MUST include all three axes (X, Y, Z) in every representation**
7. **Delta angle values MUST use minimum 6-decimal precision (0.000000)**
8. **Zero-valued axes MUST NOT be omitted from delta angle representations**

### 7.2 SHOULD Requirements

1. Implementations SHOULD support bidirectional phase transitions
2. Systems SHOULD integrate H1/H2 convergence for automated transitions
3. UI representations SHOULD use consistent color coding:
   - Hunt: Blue (#2196F3)
   - Detect: Yellow (#FFC107)
   - Disrupt: Orange (#FF9800)
   - Disable: Red (#F44336)
   - Dominate: Purple (#9C27B0)

### 7.3 MAY Requirements

1. Implementations MAY support sub-phases within each HD4 phase
2. Systems MAY implement parallel HD4 tracks for multi-threat scenarios
3. Extensions MAY define domain-specific phase semantics

---

## 8. Drift Corrections Required

This RFC supersedes conflicting definitions in the following RFCs:

| RFC | Current Definition | Required Correction |
|-----|-------------------|---------------------|
| RFC-9020 | `hunt, detect, disrupt, disable, dominate` | ✓ Correct (lowercase) |
| RFC-9024 | `Hunt, Detect, Disable, Disrupt, Dominate` | Swap Disable↔Disrupt |
| RFC-9025 | Disable before Disrupt | Swap order |
| RFC-9109 | Y-axis ends with Disable | Correct - Disable is 0.75, Dominate is 1.0 |
| RFC-9110 | 4 phases: Discover, Detect, Disrupt, Dominate | Add Hunt (rename Discover), add Disable |

---

## 9. Appendix A: Complete Framework Taxonomy

### A.1 Subsumed Frameworks

| Framework | Origin | Domain | HD4 Coverage |
|-----------|--------|--------|--------------|
| MITRE ATT&CK | MITRE | Cyber | Full (14 tactics) |
| MITRE D3FEND | MITRE | Cyber Defense | Full (7 tactics) |
| MITRE ENGAGE | MITRE | Active Defense | Full (5 goals) |
| MITRE ATLAS | MITRE | AI/ML Security | Full (14 tactics) |
| MITRE EMB3D | MITRE | Embedded Devices | Full (threat model) |
| MITRE CAR | MITRE | Analytics | Hunt/Detect phases |
| CAPEC | MITRE | Attack Patterns | Disrupt/Disable phases |
| CWE | MITRE | Weaknesses | Hunt/Detect phases |
| Cyber Kill Chain | Lockheed Martin | Cyber | Full (7 phases) |
| Unified Kill Chain | - | Cyber | Full |
| Diamond Model | - | Threat Intel | Hunt/Detect phases |
| STRIDE | Microsoft | App Security | Disrupt/Disable phases |
| DREAD | Microsoft | Risk Scoring | Phase transition scoring |
| PASTA | - | Threat Modeling | Full (7 stages) |
| VAST | - | Threat Modeling | Full |
| OCTAVE | CMU SEI | Risk Assessment | Hunt/Detect phases |
| LINDDUN | - | Privacy | Detect/Disable phases |
| CBRN Frameworks | NATO/DHS | Physical | Full |
| Counter-IED | NATO | Physical | Full |
| ISA/IEC 62443 | ISA | ICS Security | Full |

### A.2 Domain Applicability Matrix

| HD4 Phase | Cyber | Physical | Cognitive | Hybrid |
|-----------|-------|----------|-----------|--------|
| Hunt | ✓ | ✓ | ✓ | ✓ |
| Detect | ✓ | ✓ | ✓ | ✓ |
| Disrupt | ✓ | ✓ | ✓ | ✓ |
| Disable | ✓ | ✓ | ✓ | ✓ |
| Dominate | ✓ | ✓ | ✓ | ✓ |

---

## 10. References

- RFC-9000: SX9 Agnostic Core Framework
- RFC-9001: Trivariate Hashing System
- RFC-9020: Interview Schema
- RFC-9024: H2 Convergence Service Contract
- RFC-9025: Cognitive Convergence Mathematics
- RFC-9109: Plasma Defender
- RFC-9110: SX9-Lisp Interpreter
- MITRE ATT&CK: https://attack.mitre.org/
- MITRE D3FEND: https://d3fend.mitre.org/
- MITRE ENGAGE: https://engage.mitre.org/
- MITRE ATLAS: https://atlas.mitre.org/
- MITRE EMB3D: https://emb3d.mitre.org/

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-06 | Initial canonical specification |

---

*End of RFC-9300*
