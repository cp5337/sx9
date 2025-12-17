# RFC-9001 — Synaptix9 Trivariate Hashing Standard

**Version:** 1.1
**Status:** Final
**Date:** November 27, 2025
**Applies To:** Synaptix9, CTAS-7.3.1, PLASMA, ATLAS, GLAF, OrbitalOS
**Author:** CTAS Core Engineering Group

## 1. Purpose

This RFC defines the canonical hashing standard used throughout Synaptix9 and CTAS-7.3.1, including:

- **SCH** (Synaptic Convergent Hash)
- **CUID** (Contextual Unique Identifier)
- **UUIDv7** (Storage and lineage)
- Dual-Trivariate operation
- Supersession logic
- N-V-N-N grammar integration
- Domain masks
- Execution masks
- Delta-angle impact on hashing

This RFC SHALL be treated as authoritative for all future development.

## 2. Definitions

### 2.1 Trivariate Hash

A structured, 3-element hash set:

`[ SCH | CUID | UUID ]`

### 2.2 Dual-Trivariate

A primary and secondary trivariate set:

- **Primary:** `[SCH]_[CUID]_[UUID]`
- **Secondary:** `[SCH*]_[CUID*]_[UUID*]`

The secondary SHALL be generated automatically for:

- Synaptix9
- ATLAS
- PLASMA
- GLAF
- OrbitalOS

The secondary MAY be omitted for:

- Low-tier toolchain operations
- Local execution
- Administrative operations

## 3. MUST / SHALL Requirements

### 3.1 Trivariate MUST

Every operational artifact MUST have a trivariate hash.

**SCH MUST encode:**

- Domain mask
- Execution mask
- N-V-N-N grammar tokens

**CUID MUST encode:**

- ContextFrame
- Slot mapping
- Δ-angle derivative
- Execution environment

**UUID MUST be UUIDv7.**

A canonical format MUST be available:
`triv:[SCH]_[CUID]_[UUID]`

## 4. Algorithm Specifications

### 4.1 Murmur3-64 (Canonical)

**Effective v1.1:** All hashing operations use 64-bit MurmurHash3.

The 64-bit hash is extracted from `murmur3_x64_128`:

```rust
use murmur3::murmur3_x64_128;

pub fn murmur3_64(data: &[u8], seed: u32) -> u64 {
    let hash_128 = murmur3_x64_128(&mut Cursor::new(data), seed).unwrap_or(0);
    hash_128 as u64  // Lower 64 bits
}
```

**Rationale:**
- 64-bit provides ~5 billion hashes before 50% collision probability
- Sufficient for entity-scale systems
- Produces 16 hex chars or 11 Base96 chars per component
- Trivariate total: 48 Base96 chars (16 + 16 + 16)

### 4.2 Standard Seeds

| Component | Seed | Hex |
|-----------|------|-----|
| SCH | 0xC7A5_0000 | Schema Context Hash |
| CUID | 0xC7A5_0001 | Context User ID |
| UUID | 0xC7A5_0002 | Universal Unique ID |
| ENV | 0xC7A5_00FF | Environmental |
| SLOT | 0xC7A5_0100 | Unicode Slot Assignment |

### 4.3 Base96 Encoding

All hash outputs MUST be Base96 encoded:

```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`"'\
```

### 4.4 Canonical Implementation

The canonical implementation is in `ctas7-foundation-core::hash64`:

```rust
use ctas7_foundation_core::hashing::{
    murmur3_64,           // Raw 64-bit hash
    murmur3_64_base96,    // Base96 encoded
    trivariate_from_key,  // Full trivariate (48 chars)
    unicode_slot,         // PUA slot (U+E000-E9FF)
    seeds,                // Standard seed constants
};
```

## 5. SCH (Synaptic Convergent Hash)

### 5.1 Input Material

SCH SHALL ingest:

- Raw operation text
- N-V-N-N parsing
- Domain bitmask (4 bits)
- Execution bitmask (4 bits)
- Delta angle class
- Tail state

### 5.2 N-V-N-N Grammar

Grammar SHALL tokenize as:
`NOUN VERB NOUN NOUN`

Invalid or incomplete lines SHALL be normalized via:
`NOUN VERB OBJECT CONTEXT`

## 6. CUID — Slot Encoding Specification

The CUID SHALL be 16 characters, Base96.

### 6.1 Exact Slot Map

| Slots | Meaning                 | Source                 |
| :---- | :---------------------- | :--------------------- |
| 1–4   | Timestamp shard (T1–T4) | ContextFrame.timestamp |
| 5–7   | Execution Env (E1–E3)   | ContextFrame.exec_env  |
| 8–9   | Agent ID                | agent_id               |
| 10–11 | Δ-Angle Derivative      | delta_angle            |
| 12    | State Flag              | Cold/Warm/Hot/L2       |
| 13–14 | Lineage                 | ContextFrame.lineage   |
| 15–16 | Nonce/Salt              | ContextFrame.nonce     |

## 7. Supersession Specification

Supersession SHALL occur when:

### 7.1 Δ-Angle Thresholds

| Δ-Angle | Class    | Meaning                      |
| :------ | :------- | :--------------------------- |
| < 2°    | None     | No supersession              |
| 2–10°   | Micro    | Adjust CUID only             |
| 10–25°  | Soft     | Regenerate SCH + CUID        |
| 25–60°  | Hard     | Full trivariate regeneration |
| > 60°   | Critical | Supersede, new lineage       |

## 8. Canonical Format

`triv:[SCH]_[CUID]_[UUID]`

Validation SHALL require:

- SCH length = 24 chars
- CUID length = 16 chars
- UUID = 36 chars

## 9. Examples

**Generation Example (Pseudo)**

```rust
let sch = engine.generate_sch("scan port 22 target host");
let cuid = engine.generate_cuid(ctxframe);
let uuid = Uuid::now_v7();

let canonical = format!("triv:{sch}_{cuid}_{uuid}");
```
