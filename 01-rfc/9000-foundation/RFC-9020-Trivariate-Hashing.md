# RFC-9020: Trivariate Hashing System

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

The Trivariate Hash Structure provides a three-component identifier system for CTAS 7.x entities.

### 1.1 Format

```
triv:[SCH]_[CUID]_[UUID]
```

| Component | Purpose | Size |
|-----------|---------|------|
| SCH | Synaptic Convergent Hash | 128-bit Murmur3 → 24 chars Base96 |
| CUID | Contextual Unique Identifier | 16 slots, Base96 |
| UUID | Universal Unique Identifier | UUIDv7 for persistence |

---

## 2. SCH (Synaptic Convergent Hash)

### 2.1 Generation

```rust
use murmur3::murmur3_x64_128;

pub fn generate_sch(input: &[u8], seed: u32) -> String {
    let hash = murmur3_x64_128(input, seed);
    encode_base96(&hash.to_le_bytes())
}

/// Base96 encoding (NOT Base64)
pub fn encode_base96(bytes: &[u8]) -> String {
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_`{|}~ ";
    // ... encoding logic producing 24 chars
}
```

### 2.2 Properties

- 128-bit Murmur3 (NOT Blake3, NOT SHA)
- Deterministic from content
- Base96 encoding for 50% more semantic resolution vs Base64
- 24 character output

---

## 3. CUID (Contextual Unique Identifier)

### 3.1 Slot Encoding (16 slots)

| Slots | Meaning | Source |
|-------|---------|--------|
| 1-4 | Timestamp shard (T1-T4) | ContextFrame.timestamp |
| 5-7 | Execution Env (E1-E3) | ContextFrame.exec_env |
| 8-9 | Agent ID | agent_id |
| 10-11 | Δ-Angle Derivative | delta_angle |
| 12 | State Flag | Cold/Warm/Hot/L2 |
| 13-14 | Lineage | ContextFrame.lineage |
| 15-16 | Nonce/Salt | ContextFrame.nonce |

### 3.2 State Flags

| Flag | Meaning | Lane |
|------|---------|------|
| C | Cold | Standard path |
| W | Warm | Cached |
| H | Hot | Active execution |
| L | L2 | Kernel fast-path |

### 3.3 Rust Implementation

```rust
pub struct Cuid {
    pub slots: [u8; 16],
}

impl Cuid {
    pub fn from_context(ctx: &ContextFrame) -> Self {
        let mut slots = [0u8; 16];
        
        // Slots 1-4: Timestamp shard
        let ts_bytes = ctx.timestamp.timestamp_micros().to_le_bytes();
        slots[0..4].copy_from_slice(&ts_bytes[0..4]);
        
        // Slots 5-7: Execution environment
        slots[4..7].copy_from_slice(&ctx.exec_env_hash()[0..3]);
        
        // Slots 8-9: Agent ID
        slots[7..9].copy_from_slice(&ctx.agent_id.to_le_bytes()[0..2]);
        
        // Slots 10-11: Delta angle derivative
        let delta = ctx.delta_angle.derivative_hash();
        slots[9..11].copy_from_slice(&delta[0..2]);
        
        // Slot 12: State flag
        slots[11] = ctx.state_flag as u8;
        
        // Slots 13-14: Lineage
        slots[12..14].copy_from_slice(&ctx.lineage[0..2]);
        
        // Slots 15-16: Nonce
        slots[14..16].copy_from_slice(&ctx.nonce[0..2]);
        
        Self { slots }
    }
    
    pub fn to_base96(&self) -> String {
        encode_base96(&self.slots)
    }
}
```

---

## 4. Delta Angle Representation

### 4.1 Normalized Format

- **Range:** 0.000000 - 1.000000 (NOT degrees)
- **Precision:** 6-decimal places enforced
- **Resolution:** 1,000,000 discrete positions around ring
- **Each tick:** 0.00036° (about 1.3 arcseconds)

### 4.2 DeltaPosition Structure

```rust
pub struct DeltaPosition {
    pub x: f64,  // 0.000000 - 1.000000 (Semantic axis)
    pub y: f64,  // 0.000000 - 1.000000 (Operational axis)
    pub z: f64,  // 0.000000 - 1.000000 (Temporal axis)
}
```

### 4.3 Relationship Anchors (Normalized)

| Anchor | Value | Meaning |
|--------|-------|---------|
| PRECEDES | 0.000000 | Before |
| ENABLES | 0.166667 | Makes possible |
| INFORMS | 0.333333 | Provides data |
| FOLLOWS | 0.500000 | After |
| BLOCKS | 0.666667 | Prevents |
| CONFLICTS | 0.833333 | Contradicts |

### 4.4 Axis Mappings

| Axis | Domain | Range |
|------|--------|-------|
| X | Semantic | MITRE Kill Chain stages (Recon → Exfil) |
| Y | Operational | HD4 phases (Hunt → Dominate) |
| Z | Temporal | Historical → Current → Predictive |

---

## 5. Supersession Thresholds

### 5.1 Normalized Thresholds

| Δ-Angle (Normalized) | Class | Action |
|----------------------|-------|--------|
| < 0.011111 | None | No action |
| 0.011111 - 0.055556 | Micro | Adjust CUID only |
| 0.055556 - 0.138889 | Soft | Regenerate SCH + CUID |
| 0.138889 - 0.333333 | Hard | Full trivariate regeneration |
| > 0.333333 | Critical | New lineage |

### 5.2 Degree Equivalent (Reference Only)

| Δ-Angle (Degrees) | Class |
|-------------------|-------|
| < 2° | None |
| 2° - 10° | Micro |
| 10° - 25° | Soft |
| 25° - 60° | Hard |
| > 60° | Critical |

---

## 6. Noise Score Formula

```rust
pub fn calculate_noise_score(
    delta_magnitude: f64,      // 0.0 - 1.0 (maps to 0° - 180°)
    entropy_drift: f64,        // 0.0 - 1.0
    semantic_drift: f64,       // 0.0 - 1.0
) -> f64 {
    0.4 * (delta_magnitude / 0.5) + // Δ/180° normalized
    0.3 * entropy_drift +
    0.3 * semantic_drift
}
```

---

## 7. TTL Values (Canonical 42)

| Context | TTL | Value |
|---------|-----|-------|
| Default | 42 seconds | 42000ms |
| Hot Lane | 4.2 seconds | 4200ms |
| L2 Kernel | 0.42 seconds | 420ms |

---

## 8. Legion ECS Fixed-Point Encoding

For hot-path performance (<1µs updates):

```rust
pub struct DeltaPositionFixed {
    pub delta_x_micro: i32,  // 0 - 1,000,000
    pub delta_y_micro: i32,  // 0 - 1,000,000
    pub delta_z_micro: i32,  // 0 - 1,000,000
}

impl DeltaPositionFixed {
    pub fn from_float(x: f64, y: f64, z: f64) -> Self {
        Self {
            delta_x_micro: (x * 1_000_000.0) as i32,
            delta_y_micro: (y * 1_000_000.0) as i32,
            delta_z_micro: (z * 1_000_000.0) as i32,
        }
    }
    
    pub fn to_float(&self) -> DeltaPosition {
        DeltaPosition {
            x: self.delta_x_micro as f64 / 1_000_000.0,
            y: self.delta_y_micro as f64 / 1_000_000.0,
            z: self.delta_z_micro as f64 / 1_000_000.0,
        }
    }
}
```

---

## 9. Base96 Charset

```rust
pub const BASE96_CHARSET: &[u8; 96] = b"\
    0123456789\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    !#$%&()*+,-./:;<=>?@[]^_`{|}~ ";
```

### 9.1 Benefits Over Base64

- 50% more semantic resolution per position
- Better mathematical alignment with trivariate structure
- Enhanced prefix/suffix operations
- Human-readable alphanumerics prioritized

---

## 10. Full Trivariate Assembly

```rust
pub struct Trivariate {
    pub sch: String,      // 24 chars Base96
    pub cuid: Cuid,       // 16 slots
    pub uuid: Uuid,       // UUIDv7
}

impl Trivariate {
    pub fn new(content: &[u8], ctx: &ContextFrame) -> Self {
        Self {
            sch: generate_sch(content, 0),
            cuid: Cuid::from_context(ctx),
            uuid: Uuid::now_v7(),
        }
    }
    
    pub fn to_shortcode(&self) -> String {
        format!("triv:{}_{}_{}",
            self.sch,
            self.cuid.to_base96(),
            self.uuid.as_hyphenated()
        )
    }
}
```

---

## Critical Constraints

- **NO Blake3** - Use Murmur3 only
- **NO SHA variants** - Use Murmur3 only
- **6-decimal precision** - Always format as `{:.6}`
- **Base96 encoding** - NOT Base64
- **Normalized angles** - 0.0 - 1.0, NOT degrees

---

## References

- RFC-9024: H1 Operational Layer
- RFC-9025: H2 Semantic Layer
- RFC-9301: TCR Triad
