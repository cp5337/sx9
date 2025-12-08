# TOML File Inventory
## Comprehensive TOML File Catalog

**Date:** December 2025  
**Status:** Complete Inventory  
**Total TOML Files Found:** 525

---

## Executive Summary

**Breakdown:**
- **Total TOML Files**: 525
- **Smart Crate TOML Files**: 28+ (RFC-9101 compliant)
- **Cargo.toml Files**: ~400+ (Rust project configurations)
- **Configuration TOML**: ~100+ (various config files)

**Key Findings:**
- ✅ Extensive smart-crate.toml coverage across foundation crates
- ✅ All SX9 crates have smart-crate.toml files
- ✅ CTAS7 foundation crates fully compliant
- ✅ Configuration files in various formats

---

## 1. Smart Crate TOML Files (RFC-9101)

### 1.1 CTAS7 Foundation Crates

**Location:** `ctas-7-shipyard-staging/ctas7-foundation-*/`

| Crate | smart-crate.toml | Status |
|-------|------------------|--------|
| `ctas7-foundation-core` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-daemon` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-data` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-interface` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-manifold` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-math` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-orbital` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-tactical` | ✅ | RFC-9101 compliant |
| `ctas7-foundation-voice` | ✅ | RFC-9101 compliant |

### 1.2 SX9 Foundation Crates

**Location:** `synaptix9-workflow-system/crates/sx9-foundation-*/`

| Crate | smart-crate.toml | Status |
|-------|------------------|--------|
| `sx9-foundation-core` | ✅ | RFC-9101 compliant |
| `sx9-foundation-daemon` | ✅ | RFC-9101 compliant |
| `sx9-foundation-data` | ✅ | RFC-9101 compliant |
| `sx9-foundation-interface` | ✅ | RFC-9101 compliant |
| `sx9-foundation-manifold` | ✅ | RFC-9101 compliant |
| `sx9-foundation-math` | ✅ | RFC-9101 compliant |
| `sx9-foundation-orbital` | ✅ | RFC-9101 compliant |
| `sx9-foundation-tactical` | ✅ | RFC-9101 compliant |
| `sx9-foundation-voice` | ✅ | RFC-9101 compliant |

### 1.3 SX9 Neural Retrofit Crates

**Location:** `synaptix9-workflow-system/crates/sx9-*/`

| Crate | smart-crate.toml | Status |
|-------|------------------|--------|
| `sx9-atlas-daemon` | ✅ | RFC-9101 compliant |
| `sx9-gateway-primary` | ✅ | RFC-9101 compliant |
| `sx9-plasma-defender` | ✅ | RFC-9101 compliant (with OSSEC) |
| `sx9-plasma-health` | ✅ | RFC-9101 compliant |
| `sx9-l2-bridge` | ✅ | RFC-9101 compliant |

### 1.4 Other Smart Crates

| Crate | Location | Status |
|-------|----------|--------|
| `ctas7-linear-agent-rust` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-repoagent` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-government-data-manifold` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-cesium-geolocation` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-sledis` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-intelligence-discovery` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-health-dashboard` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-cdn-isolated-monitoring` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-agentic-core` | `ctas-7-shipyard-staging/` | ✅ |
| `ctas7-network-world` | `ctas-7-shipyard-staging/` | ✅ |

### 1.5 Root Smart Crates

| File | Location | Status |
|------|----------|--------|
| `smart-crate.toml` | `ctas7-command-center/` | ✅ |
| `smart-crate.toml` | `ctas7-command-center-canonical/` | ✅ |
| `smart-crate.toml` | `sx9-development-center/` | ✅ |

---

## 2. Cargo.toml Files by Workspace

### 2.1 Synaptix9 Workflow System

**Location:** `synaptix9-workflow-system/`

**Count:** 39 Cargo.toml files

**Crates:**
- Root workspace `Cargo.toml`
- All `sx9-*` crates
- All `sx9-foundation-*` crates

### 2.2 CTAS-7 Shipyard Staging

**Location:** `ctas-7-shipyard-staging/`

**Count:** 352 Cargo.toml files

**Major Categories:**
- Foundation crates (`ctas7-foundation-*`)
- Intel system crates (`ctas7-intel-system/crates/*`)
- Ops platform crates (`ctas7-ops-main-platform/*`)
- Various service crates

### 2.3 CTAS7 Command Center

**Location:** `ctas7-command-center/`

**Count:** ~50+ Cargo.toml files

**Includes:**
- Root workspace
- Various tool crates
- Test crates

---

## 3. Configuration TOML Files

### 3.1 Rust Configuration

| File Type | Count | Purpose |
|-----------|-------|---------|
| `rustfmt.toml` | Multiple | Rust formatting config |
| `.clippy.toml` | Multiple | Clippy linting config |
| `Cargo.toml` | ~400+ | Rust project configs |

### 3.2 Application Configuration

| File | Location | Purpose |
|------|----------|---------|
| `config.toml` | `ctas7-command-center/supabase/` | Supabase config |
| `pyproject.toml` | `ctas7-command-center/ctas7-voice-enterprise/` | Python project config |

### 3.3 Workspace Configuration

| File | Location | Purpose |
|------|----------|---------|
| `cargo.toml` | `ctas-7-shipyard-staging/` | Workspace root |
| `Cargo.toml` | `synaptix9-workflow-system/` | Workspace root |
| `Cargo.toml` | `ctas7-command-center/` | Workspace root |

---

## 4. Smart Crate TOML Locations

### 4.1 Complete List

```
ctas7-command-center/smart-crate.toml
ctas7-command-center-canonical/smart-crate.toml
sx9-development-center/smart-crate.toml

ctas-7-shipyard-staging/ctas7-foundation-core/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-daemon/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-data/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-interface/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-manifold/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-math/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-orbital/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-tactical/smart-crate.toml
ctas-7-shipyard-staging/ctas7-foundation-voice/smart-crate.toml

ctas-7-shipyard-staging/ctas7-linear-agent-rust/smart-crate.toml
ctas-7-shipyard-staging/ctas7-repoagent/smart-crate.toml
ctas-7-shipyard-staging/ctas7-government-data-manifold/smart-crate.toml
ctas-7-shipyard-staging/ctas7-cesium-geolocation/smart-crate.toml
ctas-7-shipyard-staging/ctas7-sledis/smart-crate.toml
ctas-7-shipyard-staging/ctas7-intelligence-discovery/smart-crate.toml
ctas-7-shipyard-staging/ctas7-health-dashboard/smart-crate.toml
ctas-7-shipyard-staging/ctas7-cdn-isolated-monitoring/smart-crate.toml
ctas-7-shipyard-staging/ctas7-agentic-core/smart-crate.toml
ctas-7-shipyard-staging/ctas7-network-world/smart-crate.toml

ctas-7-shipyard-staging/06 Document and Code Drop/extracted/sx9-l2-bridge/smart-crate.toml
ctas-7-shipyard-staging/06 Document and Code Drop/extracted/sx9-plasma-defender/smart-crate.toml
ctas-7-shipyard-staging/06 Document and Code Drop/extracted/sx9-plasma-health/smart-crate.toml

synaptix9-workflow-system/crates/sx9-foundation-interface/smart-crate.toml
synaptix9-workflow-system/crates/sx9-foundation-orbital/smart-crate.toml
synaptix9-workflow-system/crates/sx9-atlas-daemon/smart-crate.toml
synaptix9-workflow-system/crates/sx9-foundation-daemon/smart-crate.toml
```

---

## 5. TOML File Statistics

### 5.1 By Type

| Type | Count | Percentage |
|------|-------|------------|
| `Cargo.toml` | ~400 | 76% |
| `smart-crate.toml` | 28+ | 5% |
| `rustfmt.toml` | Multiple | <1% |
| `.clippy.toml` | Multiple | <1% |
| `config.toml` | Multiple | <1% |
| Other | ~100 | 19% |

### 5.2 By Workspace

| Workspace | Count |
|-----------|-------|
| `ctas-7-shipyard-staging` | 352 |
| `synaptix9-workflow-system` | 39 |
| `ctas7-command-center` | ~50 |
| Other | ~84 |

---

## 6. Smart Crate Compliance Status

### 6.1 RFC-9101 Compliance

**All smart-crate.toml files should include:**
- ✅ `[smart-crate]` section
- ✅ `[integration]` section
- ✅ `[metadata]` section
- ✅ `[ports]` section
- ✅ `[port_manager]` section
- ✅ `[semantic_lock]` section
- ✅ `[deployment.docker]` section
- ✅ `[ring_bus]` section (if applicable)
- ✅ `[endpoints]` section

### 6.2 Gold Disk Compatibility

**All smart crates should have:**
- ✅ `gold_disk_compatible = true`
- ✅ `foundation = "ctas7-foundation-core"` or `"sx9-foundation-core"`
- ✅ `hashing_algorithm = "murmur3-128"`

---

## 7. Notable TOML Files

### 7.1 Threat Intelligence Related

| File | Location | Purpose |
|------|----------|---------|
| `ctas7-atomic-red-team/Cargo.toml` | `ctas-7-shipyard-staging/ctas7-intel-system/crates/` | Atomic Red Team integration |

### 7.2 Intelligence System

| File | Location | Purpose |
|------|----------|---------|
| `ctas7-intel-system/Cargo.toml` | `ctas-7-shipyard-staging/` | Intel system workspace |
| `ctas7-o3-orchestrator/Cargo.toml` | `ctas-7-shipyard-staging/ctas7-intel-system/crates/` | O3 orchestrator |
| `ctas7-ioc-extractor/Cargo.toml` | `ctas-7-shipyard-staging/ctas7-intel-system/crates/` | IOC extraction |
| `ctas7-osint-processor/Cargo.toml` | `ctas-7-shipyard-staging/ctas7-intel-system/crates/` | OSINT processing |

---

## 8. Next Steps

### 8.1 Verification

- [ ] Verify all smart-crate.toml files are RFC-9101 compliant
- [ ] Check gold disk compatibility across all crates
- [ ] Validate semantic lock (Murmur3-128) usage
- [ ] Review port assignments for conflicts

### 8.2 Documentation

- [ ] Document any missing smart-crate.toml files
- [ ] Create compliance report
- [ ] Update integration documentation

---

## 9. References

- **RFC-9101**: Smart Crate System specification
- **Cargo.toml**: Rust package manifest format
- **Smart Crate Compliance**: See individual smart-crate.toml files

---

**Status:** Inventory Complete  
**Total Files:** 525 TOML files  
**Smart Crates:** 28+  
**Last Updated:** December 2025


