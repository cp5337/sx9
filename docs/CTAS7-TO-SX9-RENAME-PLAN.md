# CTAS7 to SX9 Rename Plan
## Cloning and Renaming Crates for Gateway Build

**Date:** December 2025  
**Status:** Implementation Plan  
**Goal:** Clone crates and rename from `ctas7` to `sx9` for gateway system

---

## Executive Summary

**Current State:**
- Existing `sx9-gateway` crate in `synaptix9-workflow-system/crates/`
- Need to create/rename neural retrofit crates: `sx9-ann-engine`, `sx9-glaf-core`, `sx9-dsl-engine`, `sx9-plasma-defender`, `sx9-atlas-daemon`
- Need to rename foundation references from `ctas7-foundation-core` to maintain compatibility while using `sx9` naming

**Strategy:**
1. Keep `ctas7-foundation-core` as the foundation (it's the gold disk)
2. Create new `sx9-*` crates for gateway-specific components
3. Use `sx9-gateway-primary` as the main gateway crate name
4. Maintain backward compatibility with existing `ctas7-*` infrastructure

---

## 1. Existing SX9 Gateway Crate

**Location:** `/Users/cp5337/Developer/synaptix9-workflow-system/crates/sx9-gateway/`

**Status:** ✅ Already exists and uses `sx9-` naming

**Current Structure:**
```
sx9-gateway/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── protocol.rs
│   ├── handlers.rs
│   ├── server.rs
│   └── state.rs
└── smart-crate.toml  (NEW - just created)
```

**Action:** ✅ Smart crate manifest created with all neural retrofit sections

---

## 2. Crates to Create/Rename

### 2.1 Neural Retrofit Crates (New - Need to Create)

| Crate | Status | Source | Action |
|-------|--------|--------|--------|
| `sx9-ann-engine` | ⚠️ Not Found | Create new | Create from scratch or integrate with existing neural systems |
| `sx9-glaf-core` | ⚠️ Not Found | `CTAS7-GLAF-SYSTEM/` | Create wrapper or integrate existing GLAF |
| `sx9-dsl-engine` | ⚠️ Not Found | Create new | Create DSL engine for symbolic control |
| `sx9-plasma-defender` | ⚠️ Not Found | Create new | Create PLASMA defender for health monitoring |
| `sx9-atlas-daemon` | ⚠️ Not Found | `sx9-atlas-bus/` | May integrate with existing `sx9-atlas-bus` |

### 2.2 Foundation Crates (Keep CTAS7 Naming)

| Crate | Status | Action |
|-------|--------|--------|
| `ctas7-foundation-core` | ✅ Exists | **KEEP** - This is the gold disk foundation |
| `ctas7-real-port-manager` | ✅ Exists | **KEEP** - Port allocation service |
| `ctas7-hashing-engine` | ✅ Exists | **KEEP** - Trivariate hash engine |
| `ctas7-neural-mux` | ✅ Exists | **KEEP** - Neural mux routing |

**Rationale:** Foundation crates remain `ctas7-*` because:
- They are the gold disk foundation
- They are shared infrastructure
- Changing them would break all existing smart crates
- Gateway uses them via `foundation = "ctas7-foundation-core"` in smart-crate.toml

---

## 3. Rename Strategy

### 3.1 Gateway-Specific Crates → SX9

**Pattern:** `ctas7-*-gateway` → `sx9-gateway-*` or `sx9-*-gateway`

**Examples:**
- `ctas7-api-gateway` → `sx9-gateway-primary` ✅ (already done)
- `ctas7-gateway-*` → `sx9-gateway-*`

### 3.2 Neural Retrofit Crates → SX9

**Pattern:** Create new `sx9-*` crates for gateway-specific neural components

**New Crates:**
- `sx9-ann-engine` - ANN observer mode
- `sx9-glaf-core` - GLAF topology mirror
- `sx9-dsl-engine` - DSL symbolic control
- `sx9-plasma-defender` - PLASMA health monitoring
- `sx9-atlas-daemon` - ATLAS cognitive tick (may use `sx9-atlas-bus`)

### 3.3 Foundation Crates → Keep CTAS7

**Pattern:** Keep `ctas7-*` for foundation/infrastructure

**Keep As-Is:**
- `ctas7-foundation-core` - Gold disk foundation
- `ctas7-real-port-manager` - Port allocation
- `ctas7-hashing-engine` - Hash engine
- `ctas7-neural-mux` - Neural mux

---

## 4. Clone and Rename Process

### 4.1 For New Neural Retrofit Crates

**Step 1: Create Crate Structure**
```bash
cd /Users/cp5337/Developer/synaptix9-workflow-system/crates

# Create new crate
cargo new --lib sx9-ann-engine
cargo new --lib sx9-glaf-core
cargo new --lib sx9-dsl-engine
cargo new --lib sx9-plasma-defender
cargo new --bin sx9-atlas-daemon
```

**Step 2: Add Smart Crate Manifests**
```bash
# Copy template and customize
cp sx9-gateway/smart-crate.toml sx9-ann-engine/smart-crate.toml
# Edit for each crate's specific configuration
```

**Step 3: Update Cargo.toml**
```toml
[package]
name = "sx9-ann-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
# Foundation dependencies
ctas7-foundation-core = { path = "../../ctas-7-shipyard-staging/ctas7-foundation-core" }
sx9-atlas-bus = { path = "../sx9-atlas-bus" }
```

### 4.2 For Existing Crates (If Cloning)

**If cloning from existing `ctas7-*` crates:**

```bash
# Example: Clone and rename
cd /Users/cp5337/Developer/synaptix9-workflow-system/crates

# Clone existing crate
cp -r ../../ctas-7-shipyard-staging/ctas7-some-crate sx9-some-crate

# Rename in Cargo.toml
sed -i '' 's/ctas7-some-crate/sx9-some-crate/g' sx9-some-crate/Cargo.toml

# Rename in source files
find sx9-some-crate/src -type f -name "*.rs" -exec sed -i '' 's/ctas7_some_crate/sx9_some_crate/g' {} \;
find sx9-some-crate/src -type f -name "*.rs" -exec sed -i '' 's/ctas7::/sx9::/g' {} \;
```

---

## 5. Smart Crate Manifest Updates

### 5.1 Gateway Smart Crate (Already Created)

**File:** `synaptix9-workflow-system/crates/sx9-gateway/smart-crate.toml`

**Status:** ✅ Created with all sections including:
- ✅ `[ann]` - ANN engine configuration
- ✅ `[glaf]` - GLAF core configuration
- ✅ `[atlas]` - ATLAS daemon configuration
- ✅ `[dsl]` - DSL engine configuration
- ✅ `[plasma_defender]` - PLASMA defender configuration

### 5.2 Neural Retrofit Crates Smart Crate Manifests

**Template for each neural retrofit crate:**

```toml
[smart-crate]
name = "sx9-{component}"
version = "0.1.0"
edition = "2021"
smart_crate_version = "1.2.0"
foundation = "ctas7-foundation-core"
classification = "neural-retrofit|gateway-component"
tesla_grade = true

[integration]
gold_disk_compatible = true
neural_mux_enabled = true
hash_engine_integrated = true
unicode_assembly_support = true
multi_tenant = true
real_time_capable = true

[deployment.docker]
base_image = "ctas7-foundation-core:gold-disk"
```

---

## 6. Migration Script

**File:** `synaptix9-workflow-system/tools/clone-rename-ctas7-to-sx9.sh`

```bash
#!/bin/bash
# Clone and rename crates from ctas7 to sx9

set -e

SOURCE_DIR="$1"
DEST_DIR="$2"
CRATE_NAME="$3"

if [ -z "$SOURCE_DIR" ] || [ -z "$DEST_DIR" ] || [ -z "$CRATE_NAME" ]; then
    echo "Usage: $0 <source-dir> <dest-dir> <crate-name>"
    echo "Example: $0 ctas7-some-crate sx9-some-crate sx9-some-crate"
    exit 1
fi

# Clone
cp -r "$SOURCE_DIR" "$DEST_DIR"

# Rename in Cargo.toml
sed -i '' "s/name = \"ctas7-.*\"/name = \"$CRATE_NAME\"/g" "$DEST_DIR/Cargo.toml"

# Rename in source files
find "$DEST_DIR/src" -type f -name "*.rs" -exec sed -i '' \
    -e "s/ctas7_/sx9_/g" \
    -e "s/ctas7::/sx9::/g" \
    -e "s/CTAS7/SX9/g" \
    {} \;

# Rename in smart-crate.toml if exists
if [ -f "$DEST_DIR/smart-crate.toml" ]; then
    sed -i '' "s/name = \"ctas7-.*\"/name = \"$CRATE_NAME\"/g" "$DEST_DIR/smart-crate.toml"
fi

echo "✅ Cloned and renamed: $SOURCE_DIR → $DEST_DIR"
```

---

## 7. Implementation Checklist

### Phase 1: Gateway Smart Crate ✅
- [x] Create `sx9-gateway/smart-crate.toml` with all sections
- [x] Include `[ann]`, `[glaf]`, `[atlas]`, `[dsl]`, `[plasma_defender]` sections
- [x] Set `gold_disk_compatible = true`
- [x] Set `base_image = "ctas7-foundation-core:gold-disk"`

### Phase 2: Neural Retrofit Crates
- [ ] Create `sx9-ann-engine` crate
- [ ] Create `sx9-glaf-core` crate (or integrate with existing GLAF)
- [ ] Create `sx9-dsl-engine` crate
- [ ] Create `sx9-plasma-defender` crate
- [ ] Create `sx9-atlas-daemon` crate (or integrate with `sx9-atlas-bus`)
- [ ] Add smart-crate.toml to each

### Phase 3: Integration
- [ ] Update `sx9-gateway/Cargo.toml` to depend on neural retrofit crates
- [ ] Update `sx9-gateway/src/lib.rs` to integrate neural retrofit components
- [ ] Test gateway with all neural retrofit components
- [ ] Verify gold disk compatibility

### Phase 4: Documentation
- [ ] Update RFC-9114 Rev 1.1 with crate locations
- [ ] Document which crates are `sx9-*` vs `ctas7-*`
- [ ] Create migration guide

---

## 8. Naming Convention Summary

**SX9 Naming (Gateway-Specific):**
- `sx9-gateway-primary` ✅
- `sx9-gateway-{domain}` (orbital, maritime, etc.)
- `sx9-ann-engine`
- `sx9-glaf-core`
- `sx9-dsl-engine`
- `sx9-plasma-defender`
- `sx9-atlas-daemon`
- `sx9-atlas-bus` ✅ (already exists)

**CTAS7 Naming (Foundation/Infrastructure - Keep):**
- `ctas7-foundation-core` (gold disk)
- `ctas7-real-port-manager`
- `ctas7-hashing-engine`
- `ctas7-neural-mux`
- `ctas7-*` (all other foundation crates)

**Rationale:**
- Gateway-specific components use `sx9-*` naming
- Foundation/infrastructure uses `ctas7-*` naming (gold disk)
- Gateway depends on foundation via `foundation = "ctas7-foundation-core"`

---

## 9. Next Steps

1. **Immediate:**
   - ✅ Gateway smart-crate.toml created with all sections
   - Create neural retrofit crates (ann, glaf, dsl, plasma, atlas)

2. **Short-term:**
   - Integrate neural retrofit crates into gateway
   - Test gold disk compatibility
   - Update documentation

3. **Long-term:**
   - Create domain-specific gateways (`sx9-gateway-orbital`, etc.)
   - Migrate other gateway-related crates to `sx9-*` naming
   - Maintain `ctas7-*` for foundation/infrastructure

---

**Status:** Gateway smart-crate.toml created with all neural retrofit sections. Ready to create neural retrofit crates.



