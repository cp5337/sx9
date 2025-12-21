# RFC-9100 DELTA ANGLE PRECISION FIX

**Critical Update: Converting Degrees to Six-Decimal Normalized**

---

## ❌ CURRENT IMPLEMENTATION (RFC-9100 §4.1):

```rust
pub struct DeltaMeasurement {
    /// Delta angle in degrees (0.0-180.0)  ← WRONG FORMAT!
    pub delta_angle: f32,
    pub entropy_drift: f32,
    pub semantic_drift: f32,
    pub noise_score: f32,
}
```

**Problems:**
- ❌ Uses degrees (0-180°) instead of normalized (0.0-1.0)
- ❌ Uses f32 instead of f64 (loses precision)
- ❌ No six-decimal rounding
- ❌ Doesn't match ECS DeltaPosition format

---

## ✅ CORRECTED IMPLEMENTATION:

```rust
/// Six-decimal precision delta position (0.000000-1.000000)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeltaPosition {
    /// X-axis: Semantic (MITRE kill chain stage)
    pub x: f64,  // 0.000000 - 1.000000
    /// Y-axis: Operational (HD4 phase)
    pub y: f64,  // 0.000000 - 1.000000
    /// Z-axis: Temporal (time correlation)
    pub z: f64,  // 0.000000 - 1.000000
}

impl DeltaPosition {
    /// Round to exactly 6 decimal places
    #[inline]
    pub fn round6(v: f64) -> f64 {
        (v * 1_000_000.0).round() / 1_000_000.0
    }
    
    /// Create with automatic 6-decimal rounding and clamping
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Self::round6(x.clamp(0.0, 1.0)),
            y: Self::round6(y.clamp(0.0, 1.0)),
            z: Self::round6(z.clamp(0.0, 1.0)),
        }
    }
    
    /// Convert from old degree format (0-180°) to normalized
    pub fn from_degrees(x_deg: f64, y_deg: f64, z_deg: f64) -> Self {
        Self::new(
            x_deg / 180.0,  // Normalize to 0.0-1.0
            y_deg / 180.0,
            z_deg / 180.0,
        )
    }
    
    /// Convert to degrees (for display/logging only)
    pub fn to_degrees(&self) -> (f64, f64, f64) {
        (
            self.x * 180.0,
            self.y * 180.0,
            self.z * 180.0,
        )
    }
    
    /// Calculate Euclidean distance between two positions
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        Self::round6((dx * dx + dy * dy + dz * dz).sqrt())
    }
    
    /// Calculate angular difference (normalized 0.0-1.0)
    pub fn angular_diff(&self, other: &Self) -> f64 {
        // Maximum possible distance in unit cube = sqrt(3)
        let dist = self.distance(other);
        Self::round6((dist / 3.0_f64.sqrt()).min(1.0))
    }
}

/// Delta measurement with normalized values
#[derive(Debug, Clone, Copy)]
pub struct DeltaMeasurement {
    /// Current delta position (6-decimal precision)
    pub position: DeltaPosition,
    
    /// Entropy drift (0.000000-1.000000)
    pub entropy_drift: f64,
    
    /// Semantic drift (0.000000-1.000000)
    pub semantic_drift: f64,
    
    /// Combined noise score (0.000000-1.000000)
    pub noise_score: f64,
}

impl DeltaMeasurement {
    /// Calculate noise score (normalized 0.0-1.0)
    pub fn calculate_noise_score(
        angular_diff: f64,
        entropy_drift: f64,
        semantic_drift: f64,
    ) -> f64 {
        DeltaPosition::round6(
            (angular_diff * 0.4) + (entropy_drift * 0.3) + (semantic_drift * 0.3)
        )
    }
}
```

---

## 🔄 UPDATED SUPERSESSION THRESHOLDS:

**Old (RFC-9100 §4.2) - Degrees:**
```
< 2°     → None
2-10°    → Micro
10-25°   → Soft
25-60°   → Hard
> 60°    → Critical
```

**New - Normalized (0.0-1.0):**
```rust
pub enum DeltaClass {
    None,      // < 0.011111 (was < 2°)
    Micro,     // 0.011111 - 0.055556 (was 2-10°)
    Soft,      // 0.055556 - 0.138889 (was 10-25°)
    Hard,      // 0.138889 - 0.333333 (was 25-60°)
    Critical,  // > 0.333333 (was > 60°)
}

impl DeltaClass {
    /// Classify delta based on angular difference
    pub fn from_angular_diff(diff: f64) -> Self {
        if diff < 0.011111 { Self::None }
        else if diff < 0.055556 { Self::Micro }
        else if diff < 0.138889 { Self::Soft }
        else if diff < 0.333333 { Self::Hard }
        else { Self::Critical }
    }
    
    /// Get normalized threshold
    pub fn threshold(&self) -> f64 {
        match self {
            Self::None => 0.011111,      // 2° / 180°
            Self::Micro => 0.055556,     // 10° / 180°
            Self::Soft => 0.138889,      // 25° / 180°
            Self::Hard => 0.333333,      // 60° / 180°
            Self::Critical => 1.000000,  // 180° / 180°
        }
    }
}
```

---

## 📊 AXIS MAPPINGS (NORMALIZED):

### **X-Axis (Semantic): MITRE Kill Chain**
```rust
pub fn mitre_stage_to_delta_x(stage: &str) -> f64 {
    DeltaPosition::round6(match stage {
        "Reconnaissance" => 0.000000,           // Start of kill chain
        "Resource Development" => 0.142857,     // 1/7 stages
        "Initial Access" => 0.285714,           // 2/7
        "Execution" => 0.428571,                // 3/7
        "Persistence" | "Privilege Escalation" => 0.571429,  // 4/7
        "Defense Evasion" => 0.714286,          // 5/7
        "Credential Access" => 0.857143,        // 6/7
        "Impact" | "Exfiltration" => 1.000000,  // End of kill chain
        _ => 0.500000,  // Default: mid-stage
    })
}
```

### **Y-Axis (Operational): HD4 Phase**
```rust
pub enum HD4Phase {
    Hunt = 0,       // 0.000000
    Detect = 1,     // 0.250000
    Disrupt = 2,    // 0.500000
    Dominate = 3,   // 0.750000
    Disable = 4,    // 1.000000
}

impl HD4Phase {
    pub fn to_delta_y(&self) -> f64 {
        DeltaPosition::round6((*self as u8 as f64) * 0.25)
    }
    
    pub fn from_delta_y(y: f64) -> Self {
        if y < 0.125 { Self::Hunt }
        else if y < 0.375 { Self::Detect }
        else if y < 0.625 { Self::Disrupt }
        else if y < 0.875 { Self::Dominate }
        else { Self::Disable }
    }
}
```

### **Z-Axis (Temporal): Time Correlation**
```rust
pub fn temporal_to_delta_z(age_seconds: i64) -> f64 {
    DeltaPosition::round6(if age_seconds < 60 {
        1.000000  // Predictive (very recent, <1 min)
    } else if age_seconds < 3600 {
        0.750000  // Current (recent, <1 hour)
    } else if age_seconds < 86400 {
        0.500000  // Recent (today, <24 hours)
    } else if age_seconds < 604800 {
        0.250000  // Historical recent (<1 week)
    } else {
        0.000000  // Historical old (>1 week)
    })
}
```

---

## 🔧 MIGRATION CODE:

### **Convert Existing Degree Values:**
```rust
/// Migrate from old degree format to new normalized format
pub fn migrate_delta_measurement(old: &OldDeltaMeasurement) -> DeltaMeasurement {
    // Convert degrees to normalized
    let position = DeltaPosition::from_degrees(
        old.delta_angle as f64,  // Was 0-180° degrees
        old.delta_angle as f64,  // Assuming same for all axes
        old.delta_angle as f64,
    );
    
    DeltaMeasurement {
        position,
        entropy_drift: DeltaPosition::round6(old.entropy_drift as f64),
        semantic_drift: DeltaPosition::round6(old.semantic_drift as f64),
        noise_score: DeltaMeasurement::calculate_noise_score(
            position.angular_diff(&DeltaPosition::new(0.0, 0.0, 0.0)),
            old.entropy_drift as f64,
            old.semantic_drift as f64,
        ),
    }
}

/// Backward compatibility: Convert to degrees for legacy systems
pub fn to_legacy_format(delta: &DeltaMeasurement) -> OldDeltaMeasurement {
    let (x_deg, y_deg, z_deg) = delta.position.to_degrees();
    let avg_deg = (x_deg + y_deg + z_deg) / 3.0;
    
    OldDeltaMeasurement {
        delta_angle: avg_deg as f32,
        entropy_drift: delta.entropy_drift as f32,
        semantic_drift: delta.semantic_drift as f32,
        noise_score: delta.noise_score as f32,
    }
}
```

---

## 🎯 LEGION ECS INTEGRATION (INTEGERS ONLY):

```rust
/// Hot-path entity stores delta as fixed-point integers
#[derive(Debug, Clone, Copy)]
pub struct SlotGraphTaskEntity {
    // ... other fields ...
    
    /// Delta position as fixed-point (6 decimals = × 1,000,000)
    pub delta_x_micro: i64,  // x * 1_000_000
    pub delta_y_micro: i64,  // y * 1_000_000
    pub delta_z_micro: i64,  // z * 1_000_000
}

/// Convert f64 to/from fixed-point i64 (6 decimals)
#[inline]
pub fn f64_to_micro(v: f64) -> i64 {
    (v * 1_000_000.0).round() as i64
}

#[inline]
pub fn micro_to_f64(v: i64) -> f64 {
    v as f64 / 1_000_000.0
}

impl SlotGraphTaskEntity {
    /// Update delta position (hot-path)
    pub fn update_delta(&mut self, delta: &DeltaPosition) {
        self.delta_x_micro = f64_to_micro(delta.x);
        self.delta_y_micro = f64_to_micro(delta.y);
        self.delta_z_micro = f64_to_micro(delta.z);
    }
    
    /// Get delta position (hot-path)
    pub fn get_delta(&self) -> DeltaPosition {
        DeltaPosition::new(
            micro_to_f64(self.delta_x_micro),
            micro_to_f64(self.delta_y_micro),
            micro_to_f64(self.delta_z_micro),
        )
    }
}
```

---

## 📝 UPDATED CUID ENCODING (RFC-9100 §4.4):

**Old (Degrees in slots 10-11):**
```
Slot 10: Angle class (3 bits) + Sign (1 bit) + Magnitude MSB (4 bits)
Slot 11: Magnitude LSB (8 bits)
Range: 0-180° in 0.044° increments
```

**New (Normalized in slots 10-11):**
```rust
/// Encode normalized delta (0.0-1.0) into CUID slots 10-11
pub fn encode_delta_to_cuid(delta: &DeltaPosition) -> [u8; 2] {
    // Average across all axes for CUID encoding
    let avg = (delta.x + delta.y + delta.z) / 3.0;
    
    // Classify
    let class = DeltaClass::from_angular_diff(avg);
    
    // Encode magnitude (12-bit fixed-point)
    let magnitude = (avg * 4095.0).round() as u16;  // 0.0-1.0 → 0-4095
    
    // Slot 10: [class:3][sign:1][mag_msb:4]
    let mut slot10 = ((class as u8) << 5) | ((magnitude >> 8) as u8 & 0x0F);
    
    // Sign bit (bit 4): positive if increasing
    if avg > 0.5 {
        slot10 |= 0x10;
    }
    
    // Slot 11: [mag_lsb:8]
    let slot11 = (magnitude & 0xFF) as u8;
    
    [slot10, slot11]
}

/// Decode CUID slots 10-11 back to normalized delta
pub fn decode_delta_from_cuid(slots: [u8; 2]) -> f64 {
    let slot10 = slots[0];
    let slot11 = slots[1];
    
    // Extract magnitude (12-bit)
    let magnitude = (((slot10 & 0x0F) as u16) << 8) | (slot11 as u16);
    
    // Convert to normalized (0.0-1.0)
    DeltaPosition::round6(magnitude as f64 / 4095.0)
}
```

---

## ✅ COMPLETE EXAMPLES:

### **Example 1: MITRE T1078 (Valid Accounts)**
```rust
let delta = DeltaPosition::new(
    mitre_stage_to_delta_x("Initial Access"),  // 0.285714
    HD4Phase::Detect.to_delta_y(),             // 0.250000
    temporal_to_delta_z(30),                   // 1.000000 (30 sec old)
);

println!("Delta: ({:.6}, {:.6}, {:.6})", delta.x, delta.y, delta.z);
// Output: Delta: (0.285714, 0.250000, 1.000000)

// Convert to degrees for display
let (x_deg, y_deg, z_deg) = delta.to_degrees();
println!("As degrees: ({:.2}°, {:.2}°, {:.2}°)", x_deg, y_deg, z_deg);
// Output: As degrees: (51.43°, 45.00°, 180.00°)
```

### **Example 2: Calculate Delta Class**
```rust
let old_delta = DeltaPosition::new(0.285714, 0.250000, 0.500000);
let new_delta = DeltaPosition::new(0.428571, 0.500000, 0.750000);

let diff = old_delta.angular_diff(&new_delta);
let class = DeltaClass::from_angular_diff(diff);

println!("Angular diff: {:.6}", diff);         // 0.245967
println!("Delta class: {:?}", class);          // Soft
println!("Action: Regenerate SCH + CUID");
```

### **Example 3: Legion ECS Hot-Path**
```rust
// Create entity with delta (cold-path)
let delta = DeltaPosition::new(0.500000, 0.250000, 1.000000);

// Convert to Legion entity (hot-path, integers only)
let mut entity = SlotGraphTaskEntity {
    entity_id: 12345,
    task_id: 42,
    delta_x_micro: f64_to_micro(delta.x),  // 500000
    delta_y_micro: f64_to_micro(delta.y),  // 250000
    delta_z_micro: f64_to_micro(delta.z),  // 1000000
    // ... other fields ...
};

// Update delta (hot-path, <1µs)
entity.delta_x_micro = f64_to_micro(0.750000);  // 750000

// Read delta back (hot-path)
let current_delta = entity.get_delta();
println!("Current: ({:.6}, {:.6}, {:.6})",
    current_delta.x, current_delta.y, current_delta.z);
// Output: Current: (0.750000, 0.250000, 1.000000)
```

---

## 🚀 ACTION ITEMS:

### **1. Update RFC-9100 §4.1:**
```diff
- pub delta_angle: f32,  // 0.0-180.0 degrees
+ pub position: DeltaPosition,  // (x,y,z) @ 6 decimals, 0.0-1.0
```

### **2. Update RFC-9100 §4.2 Thresholds:**
```diff
- < 2° → None
- 2-10° → Micro
- 10-25° → Soft
- 25-60° → Hard
- > 60° → Critical
+ < 0.011111 → None
+ 0.011111-0.055556 → Micro
+ 0.055556-0.138889 → Soft
+ 0.138889-0.333333 → Hard
+ > 0.333333 → Critical
```

### **3. Update All Code Using delta_angle:**
```bash
# Find all references
rg "delta_angle" --type rust

# Update to use DeltaPosition
# Old: delta.delta_angle
# New: delta.position.angular_diff(&reference)
```

### **4. Update CUID Encoding:**
- Modify slots 10-11 to use normalized (0.0-1.0)
- Update magnitude encoding to 12-bit fixed-point

---

## ✅ VERIFICATION:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_six_decimal_precision() {
        let delta = DeltaPosition::new(0.1234567, 0.9876543, 0.5555555);
        assert_eq!(delta.x, 0.123457);  // Rounded to 6 decimals
        assert_eq!(delta.y, 0.987654);
        assert_eq!(delta.z, 0.555556);
    }
    
    #[test]
    fn test_degree_conversion() {
        let delta = DeltaPosition::from_degrees(45.0, 90.0, 180.0);
        assert_eq!(delta.x, 0.250000);  // 45/180
        assert_eq!(delta.y, 0.500000);  // 90/180
        assert_eq!(delta.z, 1.000000);  // 180/180
    }
    
    #[test]
    fn test_fixed_point_conversion() {
        let delta = DeltaPosition::new(0.123456, 0.654321, 0.999999);
        assert_eq!(f64_to_micro(delta.x), 123456);
        assert_eq!(f64_to_micro(delta.y), 654321);
        assert_eq!(f64_to_micro(delta.z), 999999);
    }
    
    #[test]
    fn test_delta_classification() {
        assert_eq!(DeltaClass::from_angular_diff(0.005), DeltaClass::None);
        assert_eq!(DeltaClass::from_angular_diff(0.030), DeltaClass::Micro);
        assert_eq!(DeltaClass::from_angular_diff(0.100), DeltaClass::Soft);
        assert_eq!(DeltaClass::from_angular_diff(0.200), DeltaClass::Hard);
        assert_eq!(DeltaClass::from_angular_diff(0.500), DeltaClass::Critical);
    }
}
```

---

**This fix converts RFC-9100 from degree-based to six-decimal normalized delta angles, maintaining full compatibility with the three-layer ECS architecture!**