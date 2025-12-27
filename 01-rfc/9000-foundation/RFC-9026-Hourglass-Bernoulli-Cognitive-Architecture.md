# RFC-9026: Hourglass-Bernoulli Cognitive Architecture

**Version:** 7.3.1  
**Status:** NORMATIVE  
**Date:** December 2025  
**Author:** Charles E. Payne / CTAS Architecture Team  
**Depends-On:** RFC-9024 (Neurological Foundation), RFC-9021 (Convergence), RFC-9001 (Trivariate Hashing)  
**Related:** RFC-9301 (TCR Triad), RFC-9108 (Thalamic Filter)

---

## Abstract

This RFC defines the **Horizontal Hourglass** cognitive architecture for the SYNAPTIX9/CTAS system. The architecture is biomimetic, modeling biological neural processing through four latency-classified **Bernoulli Zones** that govern when and where different computational resources (deterministic code vs. LLMs) may be deployed.

The hourglass is oriented **horizontally** with bidirectional flow:
- **Left Big End (Creation/Now):** Where analytics create tools and systems
- **Bernoulli Zone (Reaction/Fast):** The narrow waist where fast probabilistic decisions occur
- **Right Big End (Analysis/Reflect):** Where analysis occurs and routes back through the system

---

## 1. The Horizontal Hourglass

```
══════════════════════════════════════════════════════════════════════════════════════════════════════

◄────────────────────────────────────────── BIDIRECTIONAL FLOW ──────────────────────────────────────►

    LEFT BIG END                        BERNOULLI ZONE                         RIGHT BIG END
   (CREATION / NOW)                    (REACTION / FAST)                    (ANALYSIS / REFLECT)
   
         ████████████                        ████                          ████████████
        ████████████████                   ████████                       ████████████████
       ██████████████████                 ████████████                   ██████████████████
      ████████████████████               ██████████████                 ████████████████████
     ██████████████████████             ████████████████               ██████████████████████
    ████████████████████████           ██████████████████             ████████████████████████
   ██████████████████████████         ████████████████████           ██████████████████████████
  ████████████████████████████       ██████████████████████         ████████████████████████████
 ██████████████████████████████     ████████████████████████       ██████████████████████████████
████████████████████████████████   ██████████████████████████     ████████████████████████████████
██████████████████████████████████████████████████████████████████████████████████████████████████████
████████████████████████████████   ██████████████████████████     ████████████████████████████████
 ██████████████████████████████     ████████████████████████       ██████████████████████████████
  ████████████████████████████       ██████████████████████         ████████████████████████████
   ██████████████████████████         ████████████████████           ██████████████████████████
    ████████████████████████           ██████████████████             ████████████████████████
     ██████████████████████             ████████████████               ██████████████████████
      ████████████████████               ██████████████                 ████████████████████
       ██████████████████                 ████████████                   ██████████████████
        ████████████████                   ████████                       ████████████████
         ████████████                        ████                          ████████████

══════════════════════════════════════════════════════════════════════════════════════════════════════

    BIG MODELS HERE                   SMALL MODELS                      BIG MODELS HERE
    (Claude, GPT-4)                   (Phi-3, DistilBERT)               (Claude, GPT-4)
    
    DETERMINISTIC:                    DETERMINISTIC:                    DETERMINISTIC:
    - Daemons                         - Thalamic Filter                 - Graph Analysis
    - CDNs                            - Ring Bus                        - Convergence Calc
    - Agents                          - Neural Mux                      - Report Generation
    - Microkernels                    - OODA Tick                       - Pattern Mining
```

---

## 2. Bernoulli Zones

The system is divided into four latency-classified zones. Each zone has strict rules about what computational resources are permitted.

### 2.1 Zone Definitions

| Zone | Name | Latency | LLM Allowed | Purpose |
|------|------|---------|-------------|---------|
| **A** | Tactical | <50μs | ❌ NEVER | Hash routing, Ring Bus L2, Neural Mux, Legion ECS |
| **B** | Operational | 50μs-1ms | ❌ NEVER | ATLAS tick, OODA loop, H1 Score, Lisp interpreter |
| **C** | Analytical | 1ms-100ms | ❌ NEVER | GLAF traversal, CDN retrieval, H2 Score, Matroid |
| **D** | Infrastructure | 100ms-60s | ✅ YES | LLMs, OSINT Machine, IAC, Terraform, Batch ML |

### 2.2 Zone Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    BERNOULLI ZONE ARCHITECTURE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE A (Tactical)                           │   │
│  │                         Latency: <50μs                              │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │   │
│  │  │ Neural Mux  │  │ Ring Bus L2 │  │ Legion ECS  │                 │   │
│  │  │ (routing)   │  │ (hot path)  │  │ (entities)  │                 │   │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                 │   │
│  └─────────┼────────────────┼────────────────┼─────────────────────────┘   │
│            │                │                │                              │
│            ▼                ▼                ▼                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE B (Operational)                        │   │
│  │                         Latency: 50μs-1ms                           │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │   │
│  │  │ H1 Score    │  │ Cognitive   │  │ HD4 Phase   │                 │   │
│  │  │ (fast)      │──│ Tick Loop   │──│ Router      │                 │   │
│  │  └─────────────┘  └──────┬──────┘  └─────────────┘                 │   │
│  │                          │ ASYNC H2 Query                           │   │
│  └──────────────────────────┼──────────────────────────────────────────┘   │
│                             ▼                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE C (Analytical)                         │   │
│  │                         Latency: 1ms-100ms                          │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │   │
│  │  │ GLAF Core   │  │ Matroid     │  │ H2 Semantic │                 │   │
│  │  │ (graphs)    │  │ Rank Calc   │  │ Score       │                 │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                             │                                               │
│                             ▼                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE D (Infrastructure)                     │   │
│  │                         Latency: 100ms-60s                          │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │   │
│  │  │ LLM Calls   │  │ OSINT       │  │ IAC/        │                 │   │
│  │  │ (Phi-3,etc) │  │ Machine     │  │ Terraform   │                 │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.3 Zone Implementation (Rust)

```rust
/// Latency classification zones
/// 
/// Operations are classified by their latency budget:
/// - Zone A: <50μs (hot path - hash lookup, cache hit, neural mux)
/// - Zone B: 50μs-1ms (warm - crypto, simple queries)
/// - Zone C: 1ms-100ms (cold - graph traversal, computation)
/// - Zone D: >100ms (async - ML inference, batch, export)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum BernoulliZone {
    /// <50μs - Hot path operations
    A = 0,
    /// 50μs-1ms - Warm operations
    B = 1,
    /// 1ms-100ms - Cold operations
    C = 2,
    /// >100ms - Async operations
    D = 3,
}

impl BernoulliZone {
    /// Maximum latency for this zone in microseconds
    pub const fn max_latency_us(&self) -> u64 {
        match self {
            Self::A => 50,
            Self::B => 1_000,
            Self::C => 100_000,
            Self::D => u64::MAX,
        }
    }
    
    /// Minimum latency for this zone in microseconds
    pub const fn min_latency_us(&self) -> u64 {
        match self {
            Self::A => 0,
            Self::B => 50,
            Self::C => 1_000,
            Self::D => 100_000,
        }
    }
    
    /// Get zone from latency
    pub const fn from_latency_us(latency_us: u64) -> Self {
        if latency_us < 50 {
            Self::A
        } else if latency_us < 1_000 {
            Self::B
        } else if latency_us < 100_000 {
            Self::C
        } else {
            Self::D
        }
    }
    
    /// Check if LLMs are allowed in this zone
    pub const fn llm_allowed(&self) -> bool {
        matches!(self, Self::D)
    }
}
```

---

## 3. Dual Neurotransmitter Systems

The architecture follows the **Dual Neurotransmitter Principle** from biological neural systems:

### 3.1 H1/H2 Hash System

| System | Analogy | Zone | Latency | Purpose |
|--------|---------|------|---------|---------|
| **H1** | Dopamine (fast excitatory) | Zone B | <1ms | Operational signal, immediate action |
| **H2** | Serotonin (slow modulatory) | Zone C | 1-100ms | Semantic context, stability |

### 3.2 Signal Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                 DUAL NEUROTRANSMITTER FLOW                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  INPUT ──▶ ┌──────────────┐                                    │
│            │ Thalamic     │  Zone A: Fast gate (<50μs)         │
│            │ Filter       │                                     │
│            └──────┬───────┘                                     │
│                   │                                             │
│          ┌───────┴───────┐                                      │
│          │               │                                      │
│          ▼               ▼                                      │
│   ┌──────────────┐ ┌──────────────┐                            │
│   │ H1 SCORE     │ │ H2 SCORE     │                            │
│   │ (Dopamine)   │ │ (Serotonin)  │                            │
│   │              │ │              │                            │
│   │ Zone B       │ │ Zone C       │                            │
│   │ <1ms         │ │ 1-100ms      │                            │
│   │ Operational  │ │ Semantic     │                            │
│   └──────┬───────┘ └──────┬───────┘                            │
│          │               │                                      │
│          └───────┬───────┘                                      │
│                  ▼                                              │
│         ┌──────────────────┐                                   │
│         │ DUAL-HASH FUSION │                                   │
│         │                  │                                   │
│         │ Semantic ⊗ Oper  │                                   │
│         │ Joint Δ-Angle    │                                   │
│         │ Probabilistic    │                                   │
│         │ Gate Decision    │                                   │
│         └────────┬─────────┘                                   │
│                  │                                              │
│                  ▼                                              │
│         ┌──────────────────┐                                   │
│         │ HD4 PHASE ROUTER │                                   │
│         │ Hunt │ Detect │  │                                   │
│         │ Disable │ Disrupt│                                   │
│         │ Dominate         │                                   │
│         └──────────────────┘                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 Joint Delta Angle Decision Matrix

| H1 (Operational) | H2 (Semantic) | Classification | Action |
|------------------|---------------|----------------|--------|
| High | High | **CRITICAL** | Immediate response |
| High | Low | **KNOWN** | Execute playbook |
| Low | High | **EMERGING** | Prepare/Monitor |
| Low | Low | **ROUTINE** | Background monitor |

---

## 4. Thalamic Filter (Pre-AI Processing)

The **Thalamic Filter** is the biomimetic gatekeeper that processes inputs BEFORE any AI/LLM involvement. This is critical for:
- Cost reduction (no wasted LLM tokens on noise)
- Latency control (deterministic pre-processing)
- Security (no prompt injection through raw data)

### 4.1 OSINT Machine + Needle Extractor

```
OSINT MACHINE (200 tools):        Zone D (30 seconds typical)
  │
  │  • Shodan, Censys, VirusTotal
  │  • WHOIS, DNS enumeration
  │  • Social media scrapers
  │  • Dark web monitors
  │
  ▼
NEEDLE EXTRACTOR (regex):         Zone D (2 seconds typical)
  │
  │  • Entity extraction (IPs, domains, hashes)
  │  • Pattern matching (IOC signatures)
  │  • Structured data extraction
  │  • NO LLM - Pure regex/heuristics
  │
  ▼
GRAPH CONSTRUCTION (rules):       Zone C (500ms typical)
  │
  │  • Build relationship graph
  │  • Apply threat ontology
  │  • Calculate initial scores
  │
  ▼
AI SEMANTIC ANALYSIS (Phi-3):     Zone D (2.5 seconds typical)
  │
  │  • Contextual enrichment
  │  • Threat narrative generation
  │  • Small model, GPU batched
  │
  ▼
THREAT CLASSIFICATION (DistilBERT): Zone C (50ms typical)
  │
  │  • Fast classification
  │  • Threat type, severity
  │  • Trained on threat corpus
  │
  ▼
HASH GENERATION (trivariate):     Zone B (10μs typical)
  │
  │  • SCH (Semantic Content Hash)
  │  • CUID (Contextual Unique ID)
  │  • UUID (Universal Unique ID)
  │
  ▼
ROUTING DECISION (Neural Mux):    Zone A (200ns typical)
  │
  │  • Unicode PUA routing
  │  • Hot path selection
  │  • Zero-copy dispatch
  │
  ▼
EXECUTION (Ring Bus):             Zone A (<1μs typical)
  │
  │  • L2 memory fabric
  │  • Agent dispatch
  │  • Tool execution
```

### 4.2 Thalamic Pathologies (Failure Modes)

| Condition | Neural Analogy | CTAS Manifestation | Fix |
|-----------|----------------|-------------------|-----|
| **Sensory overload** | Thalamic damage | No pre-filtering, AI drowns in haystack | Add NeedleExtractor |
| **Sensory neglect** | Over-filtering | Regex too strict, needles rejected | Relax patterns |
| **Hallucination** | Thalamus passes noise | Bad regex → garbage in → garbage out | Fix regex patterns |
| **Cortical bypass** | Missing thalamus | Raw data → AI → expensive + slow | Add thalamic layer |

### 4.3 LLM-Free Execution Principle

**CRITICAL RULE:** Tool execution in Zones A, B, and C is **LLM-FREE**.

The tried-and-true scripts (OSINT Machine, Needle Extractor) run deterministically. LLMs are only used in Zone D for:
- Semantic enrichment (after extraction)
- Report generation (after analysis)
- Complex reasoning (when time permits)

This ensures:
- Predictable latency
- Reproducible results
- Audit trail integrity
- Cost control

---

## 5. Performance Targets

### 5.1 Latency SLAs

| Operation | Target | P99 | Zone |
|-----------|--------|-----|------|
| Route lookup | <200ns | <250ns | A |
| Persona lookup | <50ns | <100ns | A |
| Hash generation | <10μs | <15μs | B |
| Full cognitive tick | <500μs | <800μs | B |
| Graph traversal | <50ms | <80ms | C |
| H2 score calculation | <100ms | <150ms | C |

### 5.2 Throughput Targets

| Metric | Target |
|--------|--------|
| Operations per second | >100,000 |
| Concurrent agents | >1,000 |
| Graph nodes | >10,000,000 |
| Active subscriptions | >10,000 |

---

## 6. Unicode Allocation (Zone Routing)

**Canonical allocation per kali-plasma/ebpf-tools/common/src/lib.rs (SOURCE OF TRUTH):**

```
═══════════════════════════════════════════════════════════════════════════════
 TRIVARIATE HASH ENCODING (SCH + CUID + Thalmic)
═══════════════════════════════════════════════════════════════════════════════

ZONE A (<50μs) - SCH HASH COMPONENTS
───────────────────────────────────────────────────────────────────────────────
U+E000-E0FF   │ Domain mask          │ Cyber=0x10, Geo=0x20, Space=0x30,
              │                      │ Maritime=0x40, Fusion=0x50
U+E100-E1FF   │ Execution mask (HD4) │ Hunt=0x10, Detect=0x20, Disrupt=0x30,
              │                      │ Disable=0x40, Dominate=0x50
U+E200-E2FF   │ N-V-N-N structure    │ Noun-Verb-Noun-Noun semantic encoding
U+E300-E3FF   │ Delta angle          │ Cognitive state delta (0-360°)

ZONE B (50μs-1ms) - CUID SLOTS (16 slots × 8 bits)
───────────────────────────────────────────────────────────────────────────────
U+E400-E4FF   │ CUID slots 0-1       │ Agent ID
U+E500-E5FF   │ CUID slots 2-3       │ Task ID
U+E600-E6FF   │ CUID slots 4-5       │ Sequence number
U+E700-E7FF   │ CUID slots 6-7       │ Timestamp (high)
  U+E710-E712 │   Thyristor          │ OFF=0, ON=1, RECOVERY=2 (RFC-9301)
  U+E720-E723 │   Crystal            │ INJ/PROP/INT/DEF (RFC-9301)
  U+E730-E733 │   Ring Bus           │ SND/RCV/TOK/BC (RFC-9301)
  U+E740-E74B │   Nonagon            │ 9-vertex indicators (RFC-9302)
  U+E750-E758 │   Realm              │ 9 realm indicators (RFC-9302)
U+E800-E8FF   │ CUID slots 8-9 +     │ Timestamp (low) + Priority/Confidence
  U+E800-E87F │   Priority           │ 0x00=lowest, 0x7F=highest
  U+E880-E8FF │   Confidence         │ 0x00=0%, 0x7F=100%
U+E900-E9FF   │ CUID slots 10-11 +   │ Delta angle (tick-aligned) + Suppression
  U+E900-E97F │   Suppression        │ None=0, Noise=1, Legacy=2, Overlap=3,
              │                      │ Redundant=4, LowConf=5
  U+E980-E9FF │   Agent Route        │ Target agent ID for routing
U+EA00-EAFF   │ CUID slots 12-13     │ Entropy sample
U+EB00-EBFF   │ CUID slots 14-15     │ Checksum

ZONE C (1-100ms) - SDT/CRYSTAL/TOOL SYSTEM
───────────────────────────────────────────────────────────────────────────────
U+EC00-ECFF   │ SDT state            │ Off=0, Primed=1, Conducting=2, Latched=3
U+ED00-EDFF   │ Crystal family       │ Orbital=0, GroundStation=1, TarPit=2,
              │                      │ Silent=3, Adaptive=4

ZONE D (>100ms) - TOOL TRIGGERS & RESPONSES
───────────────────────────────────────────────────────────────────────────────
U+EE00-EEFF   │ Tool triggers        │
  U+EE10-EE14 │   nmap               │ SynScan, UdpScan, Version, OS, Script
  U+EE20-EE22 │   masscan            │ TcpScan, UdpScan, BannerGrab
  U+EE30-EE32 │   nuclei             │ Template, CVE, Custom
  U+EE40-EE42 │   sqlmap             │ Detect, Exploit, Dump
  U+EE50-EE53 │   hydra              │ SSH, FTP, HTTP, SMB
  U+EE60-EE63 │   metasploit         │ Exploit, Payload, Post, Auxiliary
  U+EE70-EE72 │   responder          │ LLMNR, NBTNS, MDNS
  U+EE80-EE83 │   impacket           │ SMB, WMI, DCE, Kerberos
  U+EE90-EE91 │   bloodhound         │ Collect, Analyze
  U+EEA0-EEA3 │   crackmapexec       │ SMB, WinRM, SSH, MSSQL
U+EF00-EFFF   │ Tool responses       │ Return codes, output encoding
U+F8FF        │ Completion marker    │ Apple PUA reserved

PLASMA DEFENDER ECS
───────────────────────────────────────────────────────────────────────────────
- Legion (Layer 2): Hot-path, deterministic batch processing (<50μs)
- apecs (Layer 1): Async I/O, cold-path, database persistence (>100ms)
- Threat entity unicode_trigger: U+E000-E9FF range
- OSSEC triggers: U+E400-E41F (subset of CUID slots)
```

**Zone Compliance:**
- Zone A (<50μs): U+E000-E3FF (SCH hash), U+E710-E758 (TCR/Nonagon/Realm)
- Zone B (50μs-1ms): U+E400-EBFF (CUID slots, Priority, Confidence, Suppression)
- Zone C (1-100ms): U+EC00-EDFF (SDT state, Crystal family)
- Zone D (>100ms): U+EE00-EFFF (Tool triggers, Tool responses)

---

## 7. Integration with Other RFCs

| RFC | Integration Point |
|-----|-------------------|
| **RFC-9001** | Trivariate hashing (SCH/CUID/UUID) used in Zone B |
| **RFC-9021** | Convergence calculations in Zone C |
| **RFC-9024** | Neurological foundation, H1/H2 dual hash |
| **RFC-9108** | Thalamic filter implementation |
| **RFC-9301** | TCR Triad (Thyristor/Crystal/Ring Bus) in Zone A |
| **RFC-9003-A** | HD4 phase routing in Zone B |

---

## 8. Summary

```
HOURGLASS-BERNOULLI COGNITIVE ARCHITECTURE
══════════════════════════════════════════

HORIZONTAL HOURGLASS
────────────────────
LEFT BIG END ◄──────── BERNOULLI ZONE ────────► RIGHT BIG END
(Create)                 (React)                 (Analyze)

BERNOULLI ZONES
───────────────
Zone A: <50μs    │ Tactical      │ NO LLM  │ Ring Bus, Neural Mux
Zone B: 50μs-1ms │ Operational   │ NO LLM  │ ATLAS, H1, OODA
Zone C: 1-100ms  │ Analytical    │ NO LLM  │ GLAF, H2, Matroid
Zone D: 100ms+   │ Infrastructure│ LLM OK  │ OSINT, IAC, Reports

DUAL NEUROTRANSMITTER
─────────────────────
H1 (Dopamine)  : Fast excitatory  : Zone B : Operational
H2 (Serotonin) : Slow modulatory  : Zone C : Semantic

THALAMIC FILTER
───────────────
OSINT Machine → Needle Extractor → Graph → AI → Hash → Route → Execute
   Zone D           Zone D         Zone C  Zone D  Zone B  Zone A  Zone A
   
CRITICAL RULE: LLMs NEVER in Zone A, B, or C. Deterministic code only.
```

---

## Appendix A: Historical Context

The Hourglass-Bernoulli architecture emerged from the need to:

1. **Control costs** - LLM calls are expensive; pre-filter aggressively
2. **Ensure latency** - Operational systems need <1ms response
3. **Maintain auditability** - Deterministic code leaves clear trails
4. **Enable scale** - 100K+ ops/sec requires hot path optimization

The biomimetic approach (modeling biological neural systems) provides:
- Proven architectural patterns (billions of years of evolution)
- Intuitive mental models for operators
- Clear separation of concerns (thalamus vs. cortex)

---

## Appendix B: Revision History

| Version | Date | Changes |
|---------|------|---------|
| 7.3.1 | Dec 2025 | Initial normative specification |
| 7.3.0 | Nov 2025 | Draft with zone definitions |
| 7.2.0 | Oct 2025 | Thalamic filter concept added |

---

*End of RFC-9026*
