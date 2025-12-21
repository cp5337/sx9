# Deprecated Crates Archive - 2025-12-20

## Reason for Archival

These crates were archived during code quality cleanup due to compilation errors, legacy CTAS7 dependencies, and architectural issues that would require significant refactoring.

## Archived Crates

### **sx9-threat-simulator**

- **Errors:** 67 compilation errors
- **Issues:**
  - Missing 16 HD4 phase types
  - Legacy CTAS7 imports (`ctas7_lisp_reasoning_engine`, `ctas7_streaming_inference_engine`)
  - Duplicate type definitions
  - Conflicting trait implementations
- **Status:** Can be rebuilt if needed using modern SX9 architecture

### **sx9-orbital-simulator**

- **Errors:** 28 compilation errors
- **Issues:**
  - Multiple duplicate type definitions
  - Missing internal modules
  - Type mismatches
- **Status:** Orbital functionality moved to `sx9-orbital-operations` workspace

### **sx9-foundation-manifold**

- **Errors:** Feature dependency issues
- **Issues:**
  - Struct fields dependent on optional `elastic` feature
  - Complex HFT/Neural Mux integration that needs redesign
  - Circular dependencies with foundation crates
- **Status:** Needs architectural redesign for feature-gated dependencies

### **sx9-foundation-orbital**

- **Errors:** Dependency on `sx9-foundation-data`
- **Issues:**
  - Blocked by manifold compilation issues
  - Orbital-specific foundation code
- **Status:** Functionality can be integrated into main orbital workspace

## Restoration

If these crates are needed in the future:

1. Copy from this archive
2. Update all `ctas7_*` imports to `sx9_*`
3. Resolve duplicate type definitions
4. Add missing trait implementations
5. Update to use current SX9 foundation crates
6. For manifold: Redesign to properly handle feature-gated dependencies

## Archive Date

2025-12-20

## Archived By

Antigravity (Code Quality Cleanup - SurrealDB Removal & Workspace Cleanup)

## Impact

After archiving these crates, the workspace compiles cleanly with only warnings.
