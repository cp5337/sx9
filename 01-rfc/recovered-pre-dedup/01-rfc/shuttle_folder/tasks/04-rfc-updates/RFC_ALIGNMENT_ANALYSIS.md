# RFC Bundle Analysis & Antigravity Correction Prompt

**Date:** December 14, 2025  
**Purpose:** Align RFCs with new TOML-native design, HashRef heredity, and 32 primitives architecture

---

## 1. Summary of Design Evolution

### What Changed
| Aspect | OLD (RFCs) | NEW (TOML-native) |
|--------|-----------|-------------------|
| Configuration | XSD schemas | `smart-crate.toml` |
| Hash Reference | TrivariateRef (full 48 chars) | HashRef (16 bytes packed) |
| Hash Heredity | Not specified | Lisp S-expression operators |
| Command Size | ~304 bytes | 64 bytes (SmallCommand) |
| Primitives | Unstructured | 32 canonical primitives |

### New Unicode Block Allocation (TOML-defined)
```toml
# From smart-crate.toml - AUTHORITATIVE
[unicode.cuid_base]     # U+E200-E2FF - CUID slot runes
[unicode.triggers]      # U+E500-E5FF - Command type triggers  
[unicode.priority]      # U+E600-E60F - Priority encoding
[unicode.domains]       # U+E700-E70F - Domain encoding
[unicode.exec_env]      # U+E710-E71F - Execution environment
[unicode.state]         # U+E720-E72F - Execution state
[unicode.heredity]      # U+E800-E8FF - Lisp hash operators
```

---

## 2. RFC-by-RFC Discrepancies

### RFC-9001 (Trivariate Hashing) - NEEDS UPDATE

**Issues:**
1. ❌ Uses 128-bit Murmur3 → Now using 64-bit packed
2. ❌ SCH = 24 chars → Now 8 bytes (u64) in HashRef
3. ❌ CUID = 16 chars → Now 8 bytes (u64) packed in heredity
4. ❌ Total = 48 Base96 chars → Now 16 bytes binary
5. ❌ No mention of hash heredity / Lisp operators
6. ⚠️ Delta-angle thresholds are correct but need linkage to supersession levels

**Required Changes:**
```diff
- SCH length = 24 chars
- CUID length = 16 chars  
- UUID = 36 chars
+ HashRef = 16 bytes:
+   sch: u64      (primary semantic hash)
+   heredity: u64 ([op:8][operand:56])
+
+ Hash Heredity Operators (U+E800-E8FF):
+   nil (0xE800)       - Terminal, single trivariate
+   cons (0xE801)      - Pair (semantic . operational)
+   derive (0xE806)    - Derived from parent
+   supersede (0xE809) - Delta-angle supersession
```

---

### RFC-9002 (Unicode Routing) - NEEDS MAJOR UPDATE

**Issues:**
1. ❌ E800-E9FF = "Experimental" → Now E800-E8FF = Lisp heredity operators
2. ❌ No command trigger block → Need U+E500-E5FF
3. ❌ No priority encoding → Need U+E600-E60F
4. ❌ Domain in E300-E3FF (Class C) → Now also at E700-E70F
5. ❌ Class E (E700-E7FF) = UI only → Need to split: E700-E72F system, E730-E7FF UI

**Current RFC Classes vs New Design:**
```
RFC-9002 Classes:          NEW TOML Design:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
E000-E1FF: Class A (exec)  E000-E1FF: Class A (unchanged)
E200-E2FF: Class B (CUID)  E200-E2FF: CUID runes (unchanged)
E300-E3FF: Class C (route) E300-E3FF: Semantic route (unchanged)
E400-E6FF: Class D (mux)   E400-E4FF: Complex route
                           E500-E5FF: Command triggers (NEW)
                           E600-E6FF: Priority/status (NEW)
E700-E7FF: Class E (UI)    E700-E72F: System encoding (NEW)
                           E730-E7FF: UI elements (REDUCED)
E800-E9FF: Experimental    E800-E8FF: Lisp heredity (NEW)
                           E900-E9FF: Reserved
```

**Required Changes:**
```diff
## 2. Unicode Allocation

| Range     | Class        | Purpose                      |
| :-------- | :----------- | :--------------------------- |
| E000–E1FF | **Class A**  | Direct execution (Kali)      |
| E200–E2FF | Class B      | CUID slot runes              |
| E300–E3FF | Class C      | Semantic routing             |
| E400–E4FF | Class D      | Complex Neural Mux           |
+ | E500–E5FF | **Class T**  | **Command type triggers**    |
+ | E600–E6FF | **Class P**  | **Priority/status encoding** |
+ | E700–E72F | **Class S**  | **System encoding (domain/env/state)** |
- | E700–E7FF | **Class E**  | Frontend UI elements         |
+ | E730–E7FF | **Class E**  | Frontend UI elements         |
- | E800–E9FF | Experimental | Research modes               |
+ | E800–E8FF | **Class H**  | **Hash heredity (Lisp ops)** |
+ | E900–E9FF | Reserved     | Future expansion             |
```

---

### RFC-9002 Class E Promotion - NEEDS UPDATE

**Issues:**
1. ❌ Response runes at E7F0-E7FF → Conflicts with system encoding
2. ⚠️ Correlation ID format is good but needs HashRef integration

**Required Changes:**
```diff
- ### 4.1 Response Runes (U+E7F0-E7FF)
+ ### 4.1 Response Runes (U+E7E0-E7FF)
+ Note: Shifted to accommodate system encoding at E700-E72F

+ ### 4.2 HashRef Integration
+ Promotion context SHALL include HashRef for lineage:
+ 
+ struct PromotionContext {
+     source_rune: char,
+     target_rune: char,
+     hash_ref: HashRef,  // Lisp-encoded heredity
+     // (chain source_hash promotion_depth)
+ }
```

---

### RFC-9002 Tool Chains - NEEDS UPDATE

**Issues:**
1. ⚠️ Good concept but needs alignment with 32 primitives
2. ❌ No mention of HashRef for chain hashing

**Required Changes:**
```diff
+ ### Tool Chain Hashing
+ 
+ Each tool chain generates a HashRef:
+ - sch: Hash of chain signature (tool sequence)
+ - heredity: (fold tool1 tool2 tool3...) using U+E80A
+ 
+ Example:
+ Chain: 🔗E000→E020→E060
+ HashRef: (fold nmap_sch msf_sch sqlmap_sch)
```

---

### RFC-9003 (Operation Classifier) - NEEDS UPDATE

**Issues:**
1. ❌ No mapping to Unicode command triggers
2. ❌ Escalation tiers not linked to execution environment runes

**Required Changes:**
```diff
+ ## 3. Unicode Mapping
+ 
+ ### 3.1 Operation Category → Command Trigger
+ | Category       | Trigger Range   | Examples          |
+ | :------------- | :-------------- | :---------------- |
+ | Intelligence   | U+E500-E507     | observe, collect  |
+ | Defensive      | U+E508-E50F     | harden, monitor   |
+ | Offensive      | U+E510-E517     | disrupt, exploit  |
+ | Administrative | U+E518-E51F     | config, status    |
+ 
+ ### 3.2 Escalation Tier → Execution Environment Rune
+ | Tier          | Rune    | Meaning           |
+ | :------------ | :------ | :---------------- |
+ | WASM          | U+E710  | Browser sandbox   |
+ | Container     | U+E711  | Docker container  |
+ | Native        | U+E712  | Host process      |
+ | Kernel        | U+E713  | Kernel module     |
+ | Microkernel   | U+E714  | µKernel space     |
+ | Firefly       | U+E715  | Firecracker VM    |
+ | Orb           | U+E716  | OrbStack VM       |
```

---

### RFC-9004 (Deterministic Routing) - MINOR UPDATES

**Issues:**
1. ⚠️ IAC triggers at EA00-EA2F - acceptable but should document in master allocation
2. ⚠️ Neural Mux needs HashRef integration

**Required Changes:**
```diff
+ ### 3.5 HashRef-Based Routing
+ 
+ Neural Mux SHALL route based on HashRef:
+ 
+ pub fn route(&self, hash_ref: &HashRef) -> Option<RouteEntry> {
+     // Primary: use sch for direct lookup
+     let sch_prefix = (hash_ref.sch >> 48) as u16;
+     
+     // Secondary: check Lisp operator for routing hints
+     match hash_ref.op() {
+         HashOp::Ref => self.route_to_arena(hash_ref),
+         HashOp::Derive => self.route_with_lineage(hash_ref),
+         _ => self.standard_route(sch_prefix),
+     }
+ }
```

---

## 3. The 32 Primitives Architecture

All commands SHALL map to one of 32 canonical primitives:

### Primitive Categories (8 categories × 4 primitives = 32)

```
CATEGORY 0: GRAPH (U+E500-E503)
  00: TRAVERSE    - Path finding (Dijkstra, BFS, DFS)
  01: SEARCH      - Pattern matching, lookup
  02: AGGREGATE   - Collect, fold, reduce
  03: TRANSFORM   - Map, filter, project

CATEGORY 1: MATROID (U+E504-E507)  
  04: RANK        - Independence rank
  05: CLOSURE     - Matroid closure
  06: CIRCUIT     - Circuit detection
  07: SPAN        - Spanning set operations

CATEGORY 2: CONVERGENCE (U+E508-E50B)
  08: DETECT      - Convergence detection
  09: MEASURE     - Rate measurement
  10: THRESHOLD   - Threshold checking
  11: STABILIZE   - Stability operations

CATEGORY 3: HASH (U+E50C-E50F)
  12: COMPUTE     - Hash generation
  13: VERIFY      - Hash verification
  14: CHAIN       - Hash chaining
  15: DERIVE      - Hash derivation

CATEGORY 4: TICK (U+E510-E513)
  16: SYNC        - Tick synchronization
  17: QUERY       - Tick status query
  18: ADVANCE     - Tick advancement
  19: RESET       - Tick reset

CATEGORY 5: SDT/GATE (U+E514-E517)
  20: TRIGGER     - SDT thyristor trigger
  21: RELEASE     - SDT release
  22: QUERY       - Gate status query
  23: CONFIGURE   - Gate configuration

CATEGORY 6: PLASMA (U+E518-E51B)
  24: EXCITE      - Plasma excitation
  25: DAMPEN      - Plasma dampening
  26: QUERY       - Field query
  27: COUPLE      - Field coupling

CATEGORY 7: CONTROL (U+E51C-E51F)
  28: PING        - Health check
  29: STATS       - Statistics request
  30: SHUTDOWN    - Graceful shutdown
  31: EMERGENCY   - Emergency stop
```

### SmallCommand Mapping

```rust
#[repr(u8)]
pub enum Primitive {
    // Graph (0-3)
    Traverse = 0,
    Search = 1,
    Aggregate = 2,
    Transform = 3,
    
    // Matroid (4-7)
    Rank = 4,
    Closure = 5,
    Circuit = 6,
    Span = 7,
    
    // Convergence (8-11)
    Detect = 8,
    Measure = 9,
    Threshold = 10,
    Stabilize = 11,
    
    // Hash (12-15)
    Compute = 12,
    Verify = 13,
    Chain = 14,
    Derive = 15,
    
    // Tick (16-19)
    Sync = 16,
    TickQuery = 17,
    Advance = 18,
    TickReset = 19,
    
    // SDT (20-23)
    Trigger = 20,
    Release = 21,
    GateQuery = 22,
    Configure = 23,
    
    // Plasma (24-27)
    Excite = 24,
    Dampen = 25,
    PlasmaQuery = 26,
    Couple = 27,
    
    // Control (28-31)
    Ping = 28,
    Stats = 29,
    Shutdown = 30,
    Emergency = 31,
}

impl Primitive {
    /// Get command trigger rune
    pub const fn trigger(&self) -> char {
        unsafe { char::from_u32_unchecked(0xE500 + (*self as u32)) }
    }
    
    /// Get category (0-7)
    pub const fn category(&self) -> u8 {
        (*self as u8) >> 2
    }
}
```

---

## 4. Antigravity Correction Prompt

```markdown
# RFC Alignment Task

You are updating the CTAS RFC bundle to align with the new TOML-native architecture.

## Context Files (Read First)
1. `smart-crate.toml` - Authoritative Unicode allocation
2. `heredity.rs` - HashRef and Lisp operator implementation
3. `small_command.rs` - SmallCommand (64 bytes) implementation
4. `unicode.rs` - Trigger/priority/domain encoding

## Required Updates

### RFC-9001 (Trivariate Hashing)
1. Add Section 10: "HashRef Compact Encoding"
   - Explain 16-byte HashRef (sch:u64 + heredity:u64)
   - Document Lisp operator encoding in heredity byte
   - Show pack/unpack operations
2. Update Section 4: "Algorithm Specifications"
   - Keep Murmur3-64 as canonical
   - Add subsection on 64-bit packing for HFT paths
3. Add Section 11: "Hash Heredity"
   - Document Lisp operators (nil, cons, derive, supersede, etc.)
   - Explain dual-trivariate compression into HashRef

### RFC-9002 (Unicode Routing)
1. Update Section 2: "Unicode Allocation"
   - Add Class T (E500-E5FF): Command triggers
   - Add Class P (E600-E6FF): Priority/status
   - Add Class S (E700-E72F): System encoding
   - Reduce Class E to E730-E7FF (UI elements)
   - Add Class H (E800-E8FF): Hash heredity operators
2. Add Section 11: "TOML-Native Configuration"
   - Reference smart-crate.toml as authoritative
   - Deprecate XSD schema mentions
3. Update all code examples to use HashRef instead of full trivariate strings

### RFC-9002 Addendums
1. Class E Promotion: Shift response runes to E7E0-E7FF
2. Tool Chains: Add HashRef chain hashing using (fold ...) operator

### RFC-9003 (Operation Classifier)
1. Add Section 3: "Unicode Mapping"
   - Map 4 operation categories to trigger ranges
   - Map 7 escalation tiers to exec_env runes
2. Add Section 4: "32 Primitives"
   - Document 8 categories × 4 primitives = 32 total
   - Each primitive maps to exactly one command trigger

### RFC-9004 (Deterministic Routing)
1. Update Neural Mux to accept HashRef
2. Add routing logic based on Lisp operator inspection
3. Document arena routing for HashOp::Ref commands

## Validation Checklist
After updates, verify:
- [ ] All Unicode ranges are non-overlapping
- [ ] All 32 primitives have assigned triggers
- [ ] HashRef is documented as HFT-path standard
- [ ] Lisp operators documented at U+E800-E80C
- [ ] smart-crate.toml is referenced as authoritative source
- [ ] XSD mentions are deprecated/removed
- [ ] All code examples use HashRef (not 48-char strings)

## Output Format
For each RFC, provide:
1. Diff-style changes (- old, + new)
2. New sections in full markdown
3. Updated code examples in Rust
```

---

## 5. Master Unicode Allocation (Authoritative)

```
U+E000-E1FF: Class A - Direct Execution (Kali tools, syscalls)
U+E200-E2FF: Class B - CUID Slot Runes (identity resolution)
U+E300-E3FF: Class C - Semantic Routing (Neural Mux)
U+E400-E4FF: Class D - Complex Routing (multi-stage)
U+E500-E51F: Class T - Command Triggers (32 primitives)
U+E520-E5FF: Class T - Reserved triggers
U+E600-E60F: Class P - Priority encoding (normal/urgent/critical)
U+E610-E6FF: Class P - Reserved status codes
U+E700-E70F: Class S - Domain encoding (cyber/space/maritime/etc.)
U+E710-E71F: Class S - Execution environment (wasm/container/native/etc.)
U+E720-E72F: Class S - Execution state (cold/warm/hot/L2)
U+E730-E7DF: Class E - UI Elements (pages/modals/forms/buttons)
U+E7E0-E7FF: Class E - Response runes (success/failure/etc.)
U+E800-E80C: Class H - Lisp Heredity Operators (nil/cons/derive/etc.)
U+E80D-E8FF: Class H - Reserved heredity
U+E900-E9FF: Reserved - Future expansion
U+EA00-EA2F: Class I - IAC Triggers (spawn manifolds)
U+EA30-EFFF: Reserved - Future expansion
```
