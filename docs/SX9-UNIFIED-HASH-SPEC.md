# SX9 Unified Hash Specification

## Overview

The SX9 hash system provides a complete pipeline from semantic content to hardware-level gate control:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SX9 UNIFIED HASH PIPELINE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   Content ──► Trivariate ──► Unicode ──► SDT Frame ──► Crystal ──► Gate    │
│               Hash           Runes       EtherType     Resonance   Control  │
│                                          0xSD77                              │
│                                                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 7: Semantic Content                                            │   │
│   │   • Domain text (cyber/geo/space/maritime)                          │   │
│   │   • HD4 phase (hunt/detect/disrupt/disable/dominate)                │   │
│   │   • N-V-N-N structure (noun-verb-noun-noun)                         │   │
│   │   • Thalmic annotation (priority/confidence/suppression)            │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 6: Trivariate Hash (3 × 64-bit Base96)                         │   │
│   │   • SCH: 64 bits extracted from 128-bit source                      │   │
│   │   • CUID: 64 bits extracted from 128-bit source                     │   │
│   │   • SX9-UUID: 64 bits extracted from 128-bit source                 │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 5: Encoding                                                    │   │
│   │   • T-Line: Shorthand ligatures (6-9 bytes for SX9-UUID)           │   │
│   │   • Base96: Full canonical (55 chars)                               │   │
│   │   • Base96 Compact: 64-bit extract (24 chars)                       │   │
│   │   • Base64: Fallback (43 chars for 256 bits)                        │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 4: Unicode Runes (Private Use Area)                            │   │
│   │   • U+E000-E3FF: SCH components                                     │   │
│   │   • U+E400-EBFF: CUID slots                                         │   │
│   │   • U+E800-E9FF: Thalmic annotation                                 │   │
│   │   • U+EE00-EFFF: Tool triggers/responses                            │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 3: SDT Frame (EtherType 0xSD77)                                │   │
│   │   • Header: VER(2) + STATE(2) + Δθ(4) + H(4) + HASH(4) + TYPE(2)   │   │
│   │   • Payload: Unicode runes + tool-specific data                     │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 2: Crystal Resonance (Polycrystal)                             │   │
│   │   • Multiple crystal families vote in parallel                      │   │
│   │   • Voting policies: Any/All/Majority/Weighted/Quorum               │   │
│   │   • Ring strength: 0.0-1.0                                          │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 1: SDT Gate (Software-Defined Thyristor)                       │   │
│   │   • States: Off → Primed → Conducting → Latched                     │   │
│   │   • Gate threshold, holding current, anode drop                     │   │
│   │   • Command lives or dies based on gate state                       │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                               │
│                              ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ LAYER 0: eBPF/XDP (5-12 ns)                                          │   │
│   │   • Zero-copy, zero-allocation                                      │   │
│   │   • BPF_MAP_TYPE_HASH with 8-byte keys                              │   │
│   │   • Tool execution in kernel                                        │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 1. Trivariate Hash (3 × 64-bit Base96)

Each trivariate component is **64 bits extracted from a 128-bit source**, encoded in **Base96** for compact representation. This gives us 192 bits of extracted identity (from 384 bits of source data).

### 1.1 SCH - Semantic Content Hash (64 bits)

```
┌────────────────────────────────────────────────────────────────┐
│                    SCH (64 bits)                                │
├────────────┬────────────┬────────────┬────────────────────────┤
│  Domain    │ Execution  │   N-V-N-N  │    Delta Angle         │
│  (16 bits) │ (16 bits)  │  (16 bits) │    (16 bits)           │
├────────────┼────────────┼────────────┼────────────────────────┤
│ Cyber=0x10 │ Hunt=0x10  │ Murmur3 of │ 0-65535 → 0-360°       │
│ Geo=0x20   │ Detect=0x20│ semantic   │ Cognitive state delta  │
│ Space=0x30 │ Disrupt=0x30 structure  │ TICK-ALIGNED           │
│ Maritime=0x40 Disable=0x40           │                        │
│ Fusion=0x50│ Dominate=0x50           │                        │
└────────────┴────────────┴────────────┴────────────────────────┘
```

**Generation:**
```rust
let sch = SchHash::from_semantic(
    b"cyber",           // domain_text
    b"hunt",            // phase_text  
    b"target",          // noun1
    b"scan",            // verb
    b"port",            // noun2
    b"service",         // noun3
    delta_angle,        // current cognitive state
);
```

### 1.2 CUID - Cognitive Unique Identifier (64-bit from 128-bit)

The CUID is stored as 128 bits (16 slots) but **extracted to 64 bits** for the trivariate hash. The extraction pulls the critical slots: Agent ID, Sequence, Delta Angle, and Entropy.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CUID SOURCE (128 bits = 16 slots)                     │
├────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┤
│ Slot 0 │ Slot 1 │ Slot 2 │ Slot 3 │ Slot 4 │ Slot 5 │ Slot 6 │ Slot 7 │
│ Agent  │ Agent  │ Task   │ Task   │ Seq    │ Seq    │ TS Hi  │ TS Hi  │
│ ID Hi  │ ID Lo  │ ID Hi  │ ID Lo  │ Hi     │ Lo     │        │        │
├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
│ Slot 8 │ Slot 9 │Slot 10 │Slot 11 │Slot 12 │Slot 13 │Slot 14 │Slot 15 │
│ TS Lo  │ TS Lo  │ Δθ Hi  │ Δθ Lo  │ H Hi   │ H Lo   │ Chk Hi │ Chk Lo │
│        │        │ CRITICAL│CRITICAL│Entropy │Entropy │        │        │
└────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┘
```

**64-bit Extraction (Minimum Viable):**
```rust
// Extract essence from 128-bit CUID
let cuid64 = cuid.extract_64();
// = [Agent ID:16][Sequence:16][Delta Angle:16][Entropy:16]
```

### 1.3 SX9-UUID - Lineage Anchor (128 bits)

**NOT a standard UUID.** This is the immutable anchor that persists when data retires from motion to rest.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SX9-UUID (128 bits = 16 slots)                          │
├────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┤
│ Slot 0 │ Slot 1 │ Slot 2 │ Slot 3 │ Slot 4 │ Slot 5 │ Slot 6 │ Slot 7 │
│ Origin │ Origin │ Birth  │ Birth  │ Birth  │ Birth  │ Parent │ Parent │
│ Domain │ Agent  │ TS Hi  │ TS Mid │ TS Lo  │ TS Micro│ Ptr Hi │ Ptr Lo │
├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
│ Slot 8 │ Slot 9 │Slot 10 │Slot 11 │Slot 12 │Slot 13 │Slot 14 │Slot 15 │
│ Parent │ Parent │ Gen    │ Gen    │ Rand   │ Rand   │ Rand   │ Rand   │
│ Ptr Mid│ Ptr Lo │ Count  │ Count  │        │        │        │        │
└────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┘
```

| Field | Bits | Description |
|-------|------|-------------|
| Origin Domain | 8 | Domain that birthed this entity (cyber/geo/space/maritime) |
| Origin Agent | 8 | Agent ID that created it |
| Birth Timestamp | 32 | Unix timestamp at creation (immutable) |
| Parent Pointer | 32 | SX9-UUID of parent lineage (0 = root) |
| Generation Count | 16 | How many generations from root |
| Random | 32 | Entropy at birth for uniqueness |

**Lifecycle:**
```
1. Entity born → SX9-UUID minted (immutable forever)
2. Entity active → SCH/CUID mutate per tick, SX9-UUID unchanged
3. Entity retired → SCH/CUID dropped, SX9-UUID persists in cold storage
4. Entity resurrected → New SCH/CUID, same SX9-UUID re-illuminated
```

**Database Correlation:**
- Databases assign their own UUIDs (Postgres, Supabase, etc.)
- Their UUID → FK to our SX9-UUID
- We don't track their UUID; they correlate to ours
- Query: `SELECT * FROM entities WHERE sx9_uuid = ?`

### 1.4 Canonical Formats

| Format | Structure | Length | Use Case |
|--------|-----------|--------|----------|
| **Full** | `triv:[SCH]_[CUID]_[SX9-UUID]` | ~55 chars | Storage, audit |
| **Compact** | `trc:[SCH]_[CUID64]` | ~24 chars | Network, cache |
| **Micro** | `[CUID64]` | ~10 chars | eBPF keys |

---

## 2. Thalmic Annotation (Semantic Filtering)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      THALMIC ANNOTATION                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   Named after the thalamus - the brain's relay station that filters     │
│   sensory input before it reaches the cortex.                           │
│                                                                          │
│   ┌────────────┬────────────┬────────────────┬──────────────────┐       │
│   │  Priority  │ Confidence │  Suppression   │   Agent Route    │       │
│   │  (0-127)   │  (0-127)   │     Code       │     (0-255)      │       │
│   ├────────────┼────────────┼────────────────┼──────────────────┤       │
│   │ U+E800+val │ U+E880+val │   U+E900+val   │   U+E980+val     │       │
│   └────────────┴────────────┴────────────────┴──────────────────┘       │
│                                                                          │
│   Suppression Codes:                                                    │
│   ├── 0x00: None (pass through)                                        │
│   ├── 0x01: Noise (filter out)                                         │
│   ├── 0x02: Legacy (deprecated)                                        │
│   ├── 0x03: Overlap (duplicate)                                        │
│   ├── 0x04: Redundant (already processed)                              │
│   └── 0x05: LowConfidence (below threshold)                            │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Unicode Rune Allocation (Private Use Area)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    UNICODE PRIVATE USE AREA                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   U+E000 ─────────────────────────────────────────────────── U+F8FF     │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ SCH COMPONENTS (U+E000 - U+E3FF)                                 │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E000-E0FF: Domain mask                                        │   │
│   │   • U+E010: Cyber                                               │   │
│   │   • U+E020: Geo                                                 │   │
│   │   • U+E030: Space                                               │   │
│   │   • U+E040: Maritime                                            │   │
│   │   • U+E050: Fusion                                              │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E100-E1FF: Execution mask (HD4)                               │   │
│   │   • U+E110: Hunt                                                │   │
│   │   • U+E120: Detect                                              │   │
│   │   • U+E130: Disrupt                                             │   │
│   │   • U+E140: Disable                                             │   │
│   │   • U+E150: Dominate                                            │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E200-E2FF: N-V-N-N structure                                  │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E300-E3FF: Delta angle                                        │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ CUID SLOTS (U+E400 - U+EBFF)                                     │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E400-E4FF: Slots 0-1 (Agent ID)                               │   │
│   │ U+E500-E5FF: Slots 2-3 (Task ID)                                │   │
│   │ U+E600-E6FF: Slots 4-5 (Sequence)                               │   │
│   │ U+E700-E7FF: Slots 6-7 (Timestamp Hi)                           │   │
│   │ U+E800-E8FF: Slots 8-9 (Timestamp Lo) ← OVERLAPS THALMIC        │   │
│   │ U+E900-E9FF: Slots 10-11 (Delta Angle) ← CRITICAL               │   │
│   │ U+EA00-EAFF: Slots 12-13 (Entropy)                              │   │
│   │ U+EB00-EBFF: Slots 14-15 (Checksum)                             │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ THALMIC ANNOTATION (U+E800 - U+E9FF)                             │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+E800-E87F: Priority (0-127)                                   │   │
│   │ U+E880-E8FF: Confidence (0-127)                                 │   │
│   │ U+E900-E97F: Suppression codes                                  │   │
│   │ U+E980-E9FF: Agent routing                                      │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ SDT / CRYSTAL / TOOL (U+EC00 - U+EFFF)                           │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+EC00-ECFF: SDT state (Off/Primed/Conducting/Latched)          │   │
│   │ U+ED00-EDFF: Crystal family                                     │   │
│   │ U+EE00-EEFF: Tool triggers                                      │   │
│   │ U+EF00-EFFF: Tool responses                                     │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   U+F8FF: Completion byte                                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.1 T-Line Shorthand Symbols (U+F000 - U+F7FF)

Reserved range for high-compression encoding of immutable SX9-UUID lineage anchors.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    T-LINE SHORTHAND ALPHABET                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   The t-line (transmission line) alphabet uses ligatures and compound   │
│   symbols to encode common SX9-UUID patterns in fewer bytes.            │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ DOMAIN+AGENT LIGATURES (U+F000 - U+F0FF)                        │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+F010: Cyber+Agent0   (2 bytes → 1 rune)                       │   │
│   │ U+F011: Cyber+Agent1                                            │   │
│   │ ...                                                              │   │
│   │ U+F020: Geo+Agent0                                              │   │
│   │ U+F030: Space+Agent0                                            │   │
│   │ U+F040: Maritime+Agent0                                         │   │
│   │ U+F050: Fusion+Agent0                                           │   │
│   │ Each domain × 16 agents = 80 ligatures                          │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ TIMESTAMP COMPRESSION (U+F100 - U+F3FF)                         │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+F100-F1FF: Year-Month (256 values = ~21 years)                │   │
│   │ U+F200-F2FF: Day-Hour (256 values = 10+ days @ hour res)        │   │
│   │ U+F300-F3FF: Minute-Second (256 values = 4+ hours @ sec res)    │   │
│   │                                                                  │   │
│   │ Example: 2024-12-05T14:30:00                                    │   │
│   │   Standard: 4 bytes timestamp                                   │   │
│   │   T-Line:   [U+F12C][U+F205][U+F31E] = 3 runes (6 bytes UTF-16) │   │
│   │   But:      3 runes pack to 24 bits in our encoding = 3 bytes   │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ LINEAGE PATTERNS (U+F400 - U+F5FF)                              │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+F400: ROOT (parent=0, gen=0) - single rune for root entity   │   │
│   │ U+F401: GEN1 (gen=1)                                            │   │
│   │ U+F402: GEN2 (gen=2)                                            │   │
│   │ ...                                                              │   │
│   │ U+F4FF: GEN255                                                  │   │
│   │                                                                  │   │
│   │ U+F500-F5FF: Common parent pointers (frequently seen lineages)  │   │
│   │   • Pre-registered "well-known" ancestor UUIDs                  │   │
│   │   • Single rune instead of 32-bit pointer                       │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ NUMERIC COMPRESSION (U+F600 - U+F7FF)                           │   │
│   ├─────────────────────────────────────────────────────────────────┤   │
│   │ U+F600-F6FF: Common 16-bit values (0, 1, 255, 256, 1000, etc.) │   │
│   │ U+F700-F7FF: Entropy patterns (low, medium, high, max)          │   │
│   │                                                                  │   │
│   │ Reserved patterns:                                               │   │
│   │   U+F600: 0x0000 (zero)                                         │   │
│   │   U+F601: 0x0001 (one)                                          │   │
│   │   U+F6FF: 0xFFFF (max)                                          │   │
│   │   U+F700: LOW_ENTROPY (H < 1000)                                │   │
│   │   U+F740: MED_ENTROPY (1000 ≤ H < 10000)                        │   │
│   │   U+F780: HIGH_ENTROPY (H ≥ 10000)                              │   │
│   │   U+F7FF: MAX_ENTROPY (H = 0xFFFFFFFF)                          │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 SX9-UUID Compression Ratios

| Encoding | SX9-UUID Size | Notes |
|----------|---------------|-------|
| Raw binary | 128 bits (16 bytes) | Uncompressed |
| Base64 | 22 chars | Standard fallback |
| Base96 | 20 chars | Full alphabet |
| T-Line (typical) | 8-12 runes | Domain+agent ligature + compressed TS |
| T-Line (root) | 4-6 runes | ROOT ligature + minimal entropy |
| T-Line (packed) | 6-9 bytes | Runes → 8-bit packed |

**Example: Root Entity SX9-UUID**
```
Full:    [Cyber][Agent0][2024-12-05T14:30:00][parent:0][gen:0][rand:0xABCD]
         = 16 bytes raw

T-Line:  [U+F010][U+F12C][U+F205][U+F400][U+F6AB][U+F6CD]
         = 6 runes → 6 bytes packed (62.5% compression)
```

**Example: 3rd Generation Entity**
```
Full:    [Space][Agent5][2024-12-05T15:45:30][parent:0x12345678][gen:3][rand:0x9999]
         = 16 bytes raw

T-Line:  [U+F035][U+F12C][U+F20F][U+F403][U+F599][U+F699]
         = 6 runes → 6 bytes packed (62.5% compression)
         (assuming parent 0x12345678 is pre-registered as U+F599)
```

---

## 4. SDT Frame Format

### 4.1 Ethernet Frame

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        ETHERNET + SDT FRAME                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌───────┬───────┬──────────┬──────────────────────────────────────┐   │
│   │  DST  │  SRC  │ EtherType│           SDT PAYLOAD                │   │
│   │  MAC  │  MAC  │  0xSD77  │                                      │   │
│   │ 6 B   │ 6 B   │   2 B    │          variable                    │   │
│   └───────┴───────┴──────────┴──────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 SDT Header (18 bytes)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SDT HEADER (18 bytes)                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌─────┬─────┬───────┬───────┬───────┬─────┬───────────────────────┐   │
│   │ VER │STATE│  Δθ   │   H   │ HASH  │ TYP │       PAYLOAD         │   │
│   │ 2B  │ 2B  │  4B   │  4B   │  4B   │ 2B  │      variable         │   │
│   └─────┴─────┴───────┴───────┴───────┴─────┴───────────────────────┘   │
│                                                                          │
│   VER:   Protocol version (0x0001)                                      │
│   STATE: SDT gate state (0=Off, 1=Primed, 2=Conducting, 3=Latched)     │
│   Δθ:    Delta angle (fixed point, 0.001° resolution)                  │
│   H:     Entropy value                                                  │
│   HASH:  Identity hash (truncated Murmur3 of trivariate)               │
│   TYP:   Payload type (see table below)                                │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.3 Payload Types

| Type | Code | Payload Format | Description |
|------|------|----------------|-------------|
| PING | 0x00 | `[seq:2][timestamp:8]` | Keepalive |
| TRIG | 0x01 | `[gate_id:4][reason:2][data:N]` | Trigger gate |
| STATE | 0x02 | `[sdt_id:4][new_state:2][plasma:12]` | State change |
| LATCH | 0x03 | `[sdt_id:4][lock_hash:4]` | Latch gate |
| RESET | 0x04 | `[sdt_id:4][auth_sig:32]` | Reset gate |
| PLASMA | 0x05 | `[field_id:4][Δθ:4][H:4][excited:1]` | Plasma update |
| ROUTE | 0x06 | `[src:4][dst:4][path:N]` | Routing |
| CANARY | 0x07 | `[trip_type:2][evidence_hash:4]` | Canary trip |
| SWARM | 0x08 | `[swarm_id:4][cmd:2][params:N]` | Swarm command |
| **KEY** | **0x09** | `[bird_id:2][bits:128][harvest_ts:8]` | **ENTROPY_DRIP** |
| TRIV | 0x0A | `[sch:8][cuid:16][uuid:16]` | Full trivariate |
| TRIV64 | 0x0B | `[triv64:8]` | Compact trivariate |

### 4.4 Tool Triggers (0x10-0xAF)

| Range | Tool | Subtypes |
|-------|------|----------|
| 0x10-0x1F | nmap | SynScan, UdpScan, VersionDetect, OsFingerprint |
| 0x20-0x2F | masscan | TcpScan, UdpScan, BannerGrab |
| 0x30-0x3F | nuclei | TemplateScan, CveScan, CustomScan |
| 0x40-0x4F | sqlmap | Detect, Exploit, Dump |
| 0x50-0x5F | hydra | SSH, FTP, HTTP, SMB |
| 0x60-0x6F | metasploit | Exploit, Payload, Post, Auxiliary |
| 0x70-0x7F | responder | LLMNR, NBTNS, MDNS |
| 0x80-0x8F | impacket | SMB, WMI, DCE, Kerberos |
| 0x90-0x9F | bloodhound | Collect, Analyze |
| 0xA0-0xAF | crackmapexec | SMB, WinRM, SSH, MSSQL |

---

## 5. Crystal Resonance

### 5.1 Crystal Families

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        CRYSTAL FAMILIES                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌────────────────┬─────────────────────────────────────────────────┐  │
│   │    Family      │                  Behavior                        │  │
│   ├────────────────┼─────────────────────────────────────────────────┤  │
│   │ Orbital        │ High entropy tolerance, Van Allen belt ops      │  │
│   │ GroundStation  │ Stable, strict thresholds, corporate            │  │
│   │ TarPit         │ INVERTED - rings on anomalies (honeypot)        │  │
│   │ Silent         │ Only perfect matches ring (stealth)             │  │
│   │ Adaptive       │ Learns from traffic patterns                    │  │
│   └────────────────┴─────────────────────────────────────────────────┘  │
│                                                                          │
│   Resonance Profile:                                                    │
│   ├── perfect_thresh: ≥0.98 → None class                               │
│   ├── micro_thresh:   ≥0.90 → Micro class                              │
│   ├── soft_thresh:    ≥0.75 → Soft class                               │
│   ├── hard_thresh:    ≥0.50 → Hard class                               │
│   └── critical_thresh: <0.50 → Critical class                          │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Polycrystal Voting

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      POLYCRYSTAL VOTING                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   Multiple crystals vote in parallel (like doping a semiconductor)      │
│                                                                          │
│   ┌─────────────────┬───────────────────────────────────────────────┐   │
│   │  Voting Policy  │                  Behavior                      │   │
│   ├─────────────────┼───────────────────────────────────────────────┤   │
│   │ Any             │ ANY crystal fires → pass (tripwire)           │   │
│   │ All             │ ALL crystals must fire → pass (strict)        │   │
│   │ Majority        │ >50% must fire → pass                         │   │
│   │ WeightedAverage │ Σ(weight × ring_strength) ≥ threshold         │   │
│   │ Quorum(N)       │ At least N crystals must fire                 │   │
│   └─────────────────┴───────────────────────────────────────────────┘   │
│                                                                          │
│   Preset Configurations:                                                │
│   ┌──────────────────┬─────────────────────┬────────────────────────┐  │
│   │     Preset       │      Crystals       │        Policy          │  │
│   ├──────────────────┼─────────────────────┼────────────────────────┤  │
│   │ tripwire()       │ Silent + Ground     │ Any                    │  │
│   │ corporate_strict │ Ground + TarPit     │ All                    │  │
│   │ van_allen()      │ Orbital+Adaptive+Silent│ Weighted ≥0.93      │  │
│   │ normal_ops()     │ Ground(0.8)+Adaptive(0.2)│ Weighted ≥0.90    │  │
│   │ honeypot()       │ TarPit(0.7)+Adaptive(0.3)│ Weighted ≥0.50    │  │
│   └──────────────────┴─────────────────────┴────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Resonance Calculation

```rust
// Crystal resonance formula
ring_strength = (entropy_weight × normalized_entropy)
              + (delta_weight × (1.0 - normalized_delta))
              + (hash_weight × hash_coherence)

// Polycrystal voting
for crystal in crystals {
    let strength = crystal.resonate(entropy, delta_angle, hash);
    weighted_sum += strength * crystal.weight;
    if crystal.is_ringing(strength) {
        fired_count += 1;
    }
}
final_strength = weighted_sum / total_weight;
```

---

## 6. SDT Gate (Software-Defined Thyristor)

### 6.1 State Machine

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      SDT STATE MACHINE                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐         │
│   │   OFF    │───▶│  PRIMED  │───▶│CONDUCTING│───▶│  LATCHED │         │
│   │ (Block)  │    │ (Watch)  │    │  (Flow)  │    │  (Lock)  │         │
│   └──────────┘    └──────────┘    └──────────┘    └──────────┘         │
│        ▲                                               │                │
│        └───────────────── RESET ───────────────────────┘                │
│                                                                          │
│   Thyristor Terminals:                                                  │
│   ├── Anode:   Incoming command payload + entropy                      │
│   ├── Cathode: PlasmaState + delta_angle resonance                     │
│   └── Gate:    Monte-Carlo entropy burst + delta threshold             │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Firing Rules

```rust
// SDT transition based on crystal ring strength
match (current_state, ring_strength) {
    // Perfect ring → latch forever
    (_, r) if r >= 0.98 => SdtGate::Latched,
    
    // Already latched → check anode drop
    (Latched, r) if r < holding_thresh => SdtGate::Off,
    (Latched, _) => SdtGate::Latched,
    
    // Off or Primed → check gate threshold
    (Off | Primed, r) if r >= gate_thresh => SdtGate::Conducting,
    
    // Conducting → check holding current
    (Conducting, r) if r < holding_thresh => SdtGate::Off,
    (Conducting, _) => SdtGate::Conducting,
    
    _ => current_state,
}
```

### 6.3 Thyristor Configuration

```rust
pub struct ThyristorConfig {
    pub gate_thresh: f32,      // Min ring_strength to fire (0.50)
    pub holding_thresh: f32,   // Below this → off (0.35)
    pub perfect_thresh: f32,   // Above this → auto-latch (0.98)
    pub entropy_drought: u32,  // Entropy below this → anode drop
}
```

---

## 7. Delta Class & Supersession

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    DELTA CLASS & SUPERSESSION                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌────────────┬────────────┬───────────────┬────────────────────────┐  │
│   │ Ring Score │  Δθ Class  │ Crystal Rings │       Action           │  │
│   ├────────────┼────────────┼───────────────┼────────────────────────┤  │
│   │   ≥0.98    │ None (<2°) │ Yes (perfect) │ Fast-path, no regen    │  │
│   │ 0.90-0.98  │ Micro      │ Yes           │ Tweak CUID slots 10-11 │  │
│   │ 0.75-0.90  │ Soft       │ Weak          │ Regen SCH + CUID       │  │
│   │ 0.50-0.75  │ Hard       │ Barely        │ Full trivariate regen  │  │
│   │   <0.50    │ Critical   │ No            │ Supersede lineage      │  │
│   └────────────┴────────────┴───────────────┴────────────────────────┘  │
│                                                                          │
│   Supersession: When delta class is Critical, the command's entire     │
│   lineage is killed. The crystal rejected it → command dies.           │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 8. eBPF Integration

### 8.1 Map Key Generation

```rust
// 8-byte eBPF map key from trivariate
pub fn to_ebpf_key(triv: &TrivariateHash) -> [u8; 8] {
    let sch64 = triv.sch.to_u64();
    let cuid64 = triv.cuid.extract_64();
    
    // XOR for maximum entropy
    (sch64 ^ cuid64).to_be_bytes()
}
```

### 8.2 XDP Processing

```rust
#[xdp]
pub fn sdt_processor(ctx: XdpContext) -> u32 {
    let eth = parse_eth(&ctx)?;
    
    if eth.ethertype != 0xSD77 {
        return XDP_PASS;  // Not SDT
    }
    
    let sdt = parse_sdt(&ctx)?;
    
    // 1. Extract trivariate from payload
    let triv = parse_trivariate(&sdt.payload)?;
    
    // 2. Generate eBPF key
    let key = triv.to_ebpf_key();
    
    // 3. Crystal resonance
    let ring_strength = POLYCRYSTAL.resonate(&sdt.payload, sdt.delta_angle);
    
    // 4. SDT gate decision
    if ring_strength >= GATE_THRESH {
        // Command lives
        execute_tool(sdt.payload_type, &sdt.payload)
    } else {
        // Command dies
        XDP_DROP
    }
}
```

---

## 9. Complete Flow Example

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      COMPLETE FLOW EXAMPLE                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   1. SEMANTIC INPUT                                                     │
│      Domain: "cyber"                                                    │
│      Phase: "hunt"                                                      │
│      N-V-N-N: "target scan port service"                               │
│      Priority: 64, Confidence: 100                                      │
│                                                                          │
│   2. TRIVARIATE HASH                                                    │
│      SCH: 0x1234_5678_9ABC_DEF0 (64 bits)                              │
│      CUID: [agent:0xAAAA][seq:0xBBBB][delta:0xCCCC][entropy:0xDDDD]    │
│      SX9-UUID: [origin:0x1001][birth:0x12345678][parent:0x0][gen:1]    │
│                                                                          │
│   3. CANONICAL ENCODING                                                 │
│      Full: triv:0K3Mq7Xp2_1A2B3C4D5E6F7G8H9I0J_0192345678...           │
│      Compact: trc:0K3Mq7Xp2_AAAABBBBCCCCDDDDe                          │
│                                                                          │
│   4. UNICODE RUNES                                                      │
│      [U+E012][U+E156][U+E29A][U+E3BC]  ← SCH                           │
│      [U+E4AA][U+E5AA][U+E6BB][U+E7BB]  ← CUID                          │
│      [U+E840][U+E864][U+E900][U+E980]  ← Thalmic                       │
│                                                                          │
│   5. SDT FRAME                                                          │
│      EtherType: 0xSD77                                                  │
│      Header: [VER:0001][STATE:02][Δθ:0000CCCC][H:0000DDDD]...         │
│      Payload: [Unicode runes] + [tool data]                            │
│                                                                          │
│   6. CRYSTAL RESONANCE                                                  │
│      Polycrystal: normal_ops() = Ground(0.8) + Adaptive(0.2)           │
│      Ground ring: 0.92                                                  │
│      Adaptive ring: 0.88                                                │
│      Weighted: (0.92 × 0.8) + (0.88 × 0.2) = 0.912                     │
│      Delta class: Micro (tweak CUID slots 10-11)                       │
│                                                                          │
│   7. SDT GATE                                                           │
│      Current: Primed                                                    │
│      Ring strength: 0.912 ≥ gate_thresh (0.50)                         │
│      Transition: Primed → Conducting                                   │
│      Result: COMMAND LIVES                                              │
│                                                                          │
│   8. eBPF EXECUTION                                                     │
│      Map key: 0xDEADBEEFCAFEBABE                                       │
│      Tool: nmap (0x10)                                                  │
│      Action: SYN scan target                                           │
│      Latency: 8 ns                                                      │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 10. Summary

| Layer | Component | Bits | Format | Latency |
|-------|-----------|------|--------|---------|
| 7 | Semantic Content | variable | Text | - |
| 6 | Trivariate Hash | 320 | SCH+CUID+SX9-UUID | ~100ns |
| 5 | Encoding | - | Base96/64 | ~50ns |
| 4 | Unicode Runes | - | U+E000-F8FF | ~10ns |
| 3 | SDT Frame | 18+ | EtherType 0xSD77 | ~5ns |
| 2 | Crystal Resonance | - | Polycrystal | ~10ns |
| 1 | SDT Gate | - | Thyristor | ~2ns |
| 0 | eBPF/XDP | 64 | Map key | 5-12ns |

**Total pipeline: ~200ns semantic → ~12ns wire**

---

*"The crystal is the quartz. The thyristor is the switch it triggers. Never merge them."*


