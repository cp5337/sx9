# ANTIGRAVITY RFC CORRECTION PROMPT

You are updating the CTAS-7 RFC bundle to align with the new architecture. Read all context files before making changes.

---

## CRITICAL CHANGES

### 1. XSD → TOML
All XML Schema Definition (XSD) references are **DEPRECATED**. The authoritative source is now `smart-crate.toml`.

### 2. Full Trivariate → HashRef (16 bytes)
The 48-character Base96 trivariate string is replaced by:
```rust
#[repr(C)]
pub struct HashRef {
    pub sch: u64,       // Primary SCH (Murmur3-64)
    pub heredity: u64,  // [lisp_op:8][operand:56]
}
```

### 3. Lisp Heredity Operators (U+E800-E80C)
Hash relationships are encoded using Lisp S-expressions:
| Op | Char | Meaning |
|----|------|---------|
| nil | U+E800 | Terminal (single trivariate) |
| cons | U+E801 | Pair (semantic . operational) |
| car | U+E802 | Extract primary |
| cdr | U+E803 | Extract secondary |
| quote | U+E804 | Literal |
| ref | U+E805 | Arena reference |
| derive | U+E806 | Parent derivation |
| xor | U+E807 | XOR combination |
| chain | U+E808 | Hash lineage |
| supersede | U+E809 | Delta-angle replacement |
| fold | U+E80A | Merkle fold |
| lambda | U+E80B | Deferred |
| apply | U+E80C | Arena apply |

### 4. 32 Primitives
All commands map to exactly one of 32 primitives (8 categories × 4 each):

```
Cat 0 - GRAPH:       traverse, search, aggregate, transform
Cat 1 - MATROID:     rank, closure, circuit, span  
Cat 2 - CONVERGENCE: detect, measure, threshold, stabilize
Cat 3 - HASH:        compute, verify, chain, derive
Cat 4 - TICK:        sync, query, advance, reset
Cat 5 - SDT/GATE:    trigger, release, query, configure
Cat 6 - PLASMA:      excite, dampen, query, couple
Cat 7 - CONTROL:     ping, stats, shutdown, emergency
```

Command triggers: U+E500 + primitive_id (0-31)

---

## UNICODE MASTER ALLOCATION (NEW)

```
E000-E1FF: Class A - Direct Execution (unchanged)
E200-E2FF: Class B - CUID Slot Runes (unchanged)
E300-E3FF: Class C - Semantic Routing (unchanged)
E400-E4FF: Class D - Complex Routing (reduced)
E500-E51F: Class T - Command Triggers (32 primitives) ← NEW
E520-E5FF: Class T - Reserved triggers
E600-E60F: Class P - Priority (normal/urgent/critical) ← NEW
E610-E6FF: Class P - Reserved status
E700-E70F: Class S - Domain encoding ← NEW
E710-E71F: Class S - Execution environment ← NEW
E720-E72F: Class S - Execution state ← NEW
E730-E7DF: Class E - UI Elements (reduced from E700)
E7E0-E7FF: Class E - Response runes (shifted from E7F0)
E800-E80C: Class H - Lisp Heredity Operators ← NEW
E80D-E8FF: Class H - Reserved
E900-E9FF: Reserved
EA00-EA2F: Class I - IAC Triggers (unchanged)
```

---

## RFC-SPECIFIC CORRECTIONS

### RFC-9001 (Trivariate Hashing)

**ADD Section 10: HashRef Compact Encoding**
```markdown
## 10. HashRef Compact Encoding

For HFT paths, the full trivariate is compressed to 16 bytes:

### 10.1 Structure
\`\`\`rust
pub struct HashRef {
    pub sch: u64,       // Primary SCH lower 64 bits
    pub heredity: u64,  // [op:8][operand:56]
}
\`\`\`

### 10.2 Operand Interpretations
| Op | Operand Contents |
|----|------------------|
| nil | CUID packed (56 bits) |
| cons | XOR(secondary.sch, secondary.cuid) |
| ref | slot:24 \| size:16 \| checksum:16 |
| derive | parent_sch:48 \| delta:8 |
| supersede | old_sch:48 \| level:8 |

### 10.3 Construction
\`\`\`rust
// Terminal (single trivariate)
let href = HashRef::terminal(sch, cuid);

// Dual (semantic + operational)
let href = HashRef::cons(primary_sch, secondary_sch, secondary_cuid);

// Derived from parent
let href = HashRef::derive(new_sch, parent_sch, delta_quantized);
\`\`\`
```

**ADD Section 11: Hash Heredity**
```markdown
## 11. Hash Heredity

Hash relationships are encoded using Lisp-style operators (U+E800-E8FF).

### 11.1 Operators
See smart-crate.toml [unicode.heredity] for authoritative definitions.

### 11.2 Dual-Trivariate Compression
When a thing has both semantic and operational hashes:
1. Primary SCH → HashRef.sch
2. Lisp CONS operator → HashRef.heredity[63:56]
3. XOR(secondary.sch, secondary.cuid) → HashRef.heredity[55:0]
```

---

### RFC-9002 (Unicode Routing)

**REPLACE Section 2: Unicode Allocation**
```markdown
## 2. Unicode Allocation

**U+E000–EFFF SHALL be reserved for Synaptix9**

| Range     | Class   | Purpose                      | Notes |
|-----------|---------|------------------------------|-------|
| E000–E1FF | Class A | Direct execution             | Kali tools, syscalls |
| E200–E2FF | Class B | CUID slot runes              | Identity resolution |
| E300–E3FF | Class C | Semantic routing             | Neural Mux |
| E400–E4FF | Class D | Complex routing              | Multi-stage |
| E500–E51F | Class T | **Command triggers**         | 32 primitives |
| E520–E5FF | Class T | Reserved triggers            | Future commands |
| E600–E60F | Class P | **Priority encoding**        | normal/urgent/critical |
| E610–E6FF | Class P | Reserved status              | Future status |
| E700–E70F | Class S | **Domain encoding**          | cyber/space/etc. |
| E710–E71F | Class S | **Execution environment**    | wasm/container/native |
| E720–E72F | Class S | **Execution state**          | cold/warm/hot/L2 |
| E730–E7DF | Class E | UI elements                  | pages/modals/buttons |
| E7E0–E7FF | Class E | Response runes               | success/failure |
| E800–E8FF | Class H | **Hash heredity operators**  | Lisp S-expressions |
| E900–E9FF | Reserved| Future expansion             | |
| EA00–EA2F | Class I | IAC triggers                 | Manifold spawning |
```

**ADD Section 11: TOML-Native Configuration**
```markdown
## 11. TOML-Native Configuration

### 11.1 Authoritative Source
The file `smart-crate.toml` in each crate is the authoritative source for Unicode allocations.

### 11.2 Deprecation
XSD schema definitions are **DEPRECATED** as of v2.1.

### 11.3 Example
\`\`\`toml
[unicode.triggers]
atlas_daemon = 0xE500
plasma_state = 0xE501
glaf_matroid = 0xE502

[unicode.heredity]
nil       = 0xE800
cons      = 0xE801
derive    = 0xE806
supersede = 0xE809
\`\`\`
```

---

### RFC-9002 Class E Promotion

**UPDATE Section 4.1**
```diff
- ### 4.1 Response Runes (U+E7F0-E7FF)
+ ### 4.1 Response Runes (U+E7E0-E7FF)
+ 
+ Note: Shifted from E7F0 to accommodate system encoding at E700-E72F.

| Rune    | Status    | Description                      |
|---------|-----------|----------------------------------|
- | U+E7F0  | Success   | Execution completed successfully |
+ | U+E7E0  | Success   | Execution completed successfully |
- | U+E7F1  | Failure   | Execution failed                 |
+ | U+E7E1  | Failure   | Execution failed                 |
...
```

**ADD Section 4.3: HashRef Integration**
```markdown
### 4.3 HashRef Integration

Promotion context SHALL include HashRef for lineage tracking:

\`\`\`rust
struct PromotionContext {
    source_rune: char,
    target_rune: char,
    hash_ref: HashRef,  // (chain source_sch promotion_depth)
    correlation_id: Uuid,
}

// Example: UI button promotes to Kali tool execution
let promotion = HashRef::chain(
    execution_sch,  // New execution hash
    ui_event_sch,   // Source UI event hash
    1,              // Promotion depth
);
\`\`\`
```

---

### RFC-9003 (Operation Classifier)

**ADD Section 3: Unicode Mapping**
```markdown
## 3. Unicode Mapping

### 3.1 Category → Trigger Range
| Category       | Primitives | Trigger Range |
|----------------|------------|---------------|
| Intelligence   | 0-7        | U+E500-E507   |
| Defensive      | 8-15       | U+E508-E50F   |
| Offensive      | 16-23      | U+E510-E517   |
| Administrative | 24-31      | U+E518-E51F   |

### 3.2 Escalation Tier → Execution Environment
| Tier        | Rune    | Meaning         |
|-------------|---------|-----------------|
| WASM        | U+E710  | Browser sandbox |
| Container   | U+E711  | Docker/OCI      |
| Native      | U+E712  | Host process    |
| Kernel      | U+E713  | Kernel module   |
| Microkernel | U+E714  | µKernel space   |
| Firefly     | U+E715  | Firecracker VM  |
| Orb         | U+E716  | OrbStack VM     |
```

**ADD Section 4: 32 Primitives**
```markdown
## 4. The 32 Primitives

All operations SHALL map to exactly one of 32 canonical primitives.

### 4.1 Primitive Table
| ID | Name | Category | Trigger | Description |
|----|------|----------|---------|-------------|
| 0 | TRAVERSE | Graph | U+E500 | Path finding |
| 1 | SEARCH | Graph | U+E501 | Pattern matching |
| 2 | AGGREGATE | Graph | U+E502 | Collect/fold |
| 3 | TRANSFORM | Graph | U+E503 | Map/filter |
| 4 | RANK | Matroid | U+E504 | Independence rank |
| 5 | CLOSURE | Matroid | U+E505 | Matroid closure |
| 6 | CIRCUIT | Matroid | U+E506 | Circuit detection |
| 7 | SPAN | Matroid | U+E507 | Spanning sets |
| 8 | DETECT | Convergence | U+E508 | Detect convergence |
| 9 | MEASURE | Convergence | U+E509 | Rate measurement |
| 10 | THRESHOLD | Convergence | U+E50A | Threshold check |
| 11 | STABILIZE | Convergence | U+E50B | Stability ops |
| 12 | COMPUTE | Hash | U+E50C | Hash generation |
| 13 | VERIFY | Hash | U+E50D | Hash verification |
| 14 | CHAIN | Hash | U+E50E | Hash chaining |
| 15 | DERIVE | Hash | U+E50F | Hash derivation |
| 16 | SYNC | Tick | U+E510 | Tick sync |
| 17 | TICK_QUERY | Tick | U+E511 | Status query |
| 18 | ADVANCE | Tick | U+E512 | Tick advance |
| 19 | TICK_RESET | Tick | U+E513 | Tick reset |
| 20 | TRIGGER | SDT | U+E514 | Thyristor trigger |
| 21 | RELEASE | SDT | U+E515 | Gate release |
| 22 | GATE_QUERY | SDT | U+E516 | Gate query |
| 23 | CONFIGURE | SDT | U+E517 | Gate config |
| 24 | EXCITE | Plasma | U+E518 | Field excitation |
| 25 | DAMPEN | Plasma | U+E519 | Field dampening |
| 26 | PLASMA_QUERY | Plasma | U+E51A | Field query |
| 27 | COUPLE | Plasma | U+E51B | Field coupling |
| 28 | PING | Control | U+E51C | Health check |
| 29 | STATS | Control | U+E51D | Statistics |
| 30 | SHUTDOWN | Control | U+E51E | Graceful stop |
| 31 | EMERGENCY | Control | U+E51F | Emergency stop |

### 4.2 Primitive Resolution
\`\`\`rust
pub fn resolve_primitive(operation: &str) -> Primitive {
    // All operations map to exactly one primitive
    match classify_operation(operation) {
        ("dijkstra", _) => Primitive::Traverse,
        ("bfs", _) => Primitive::Traverse,
        ("matroid_rank", _) => Primitive::Rank,
        ("convergence_check", _) => Primitive::Detect,
        // ... etc
    }
}
\`\`\`
```

---

### RFC-9004 (Deterministic Routing)

**UPDATE Section 3.2: Route Table Structure**
```diff
pub struct NeuralMuxRouter {
-   /// Primary route table: SCH prefix → endpoint
-   routes: Arc<DashMap<u16, RouteEntry>>,
+   /// Primary route table: HashRef.sch prefix → endpoint
+   routes: Arc<DashMap<u64, RouteEntry>>,
    
+   /// Lisp operator routes: op → handler
+   heredity_routes: Arc<DashMap<HashOp, RouteHandler>>,
}
```

**ADD Section 3.5: HashRef Routing**
```markdown
### 3.5 HashRef-Based Routing

Neural Mux SHALL inspect HashRef Lisp operator for routing hints:

\`\`\`rust
pub fn route(&self, hash_ref: &HashRef) -> Option<RouteEntry> {
    // Check if Lisp operator affects routing
    match hash_ref.op() {
        HashOp::Ref => {
            // Arena reference: route to arena service
            let slot = hash_ref.arena_slot()?;
            self.route_to_arena(slot)
        }
        HashOp::Derive | HashOp::Chain => {
            // Lineage: may need parent resolution
            self.route_with_lineage(hash_ref)
        }
        _ => {
            // Standard: use SCH prefix
            let prefix = (hash_ref.sch >> 48) as u16;
            self.routes.get(&prefix).map(|e| e.clone())
        }
    }
}
\`\`\`
```

---

## VALIDATION CHECKLIST

After applying corrections, verify:

- [ ] All Unicode ranges E000-EFFF are non-overlapping
- [ ] 32 primitives have triggers at E500-E51F
- [ ] Lisp operators documented at E800-E80C
- [ ] HashRef (16 bytes) replaces 48-char strings in HFT paths
- [ ] Response runes shifted to E7E0-E7FF
- [ ] Execution environment runes at E710-E71F
- [ ] Domain runes at E700-E70F
- [ ] smart-crate.toml referenced as authoritative
- [ ] All XSD references marked DEPRECATED
- [ ] Code examples use HashRef, not TrivariateHash strings
