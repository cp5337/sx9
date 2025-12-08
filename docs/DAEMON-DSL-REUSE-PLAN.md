# Daemon & DSL Reuse Plan

## Existing Implementations Found

### 1. **ctas7-atlas-daemon** (OODA Loop)
**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-atlas-daemon/`

**What exists:**
- ✅ Complete OODA loop implementation (`ooda_loop.rs`)
- ✅ HD4 phase management (`hd4_phases.rs`)
- ✅ Delta angle calculations (`delta_angles.rs`)
- ✅ Convergence scoring (`convergence.rs`)
- ✅ Main daemon structure (`lib.rs`)

**What needs adaptation:**
- ❌ Uses `tokio::broadcast` → needs `sx9-atlas-bus`
- ❌ No integration with `PlasmaState`
- ❌ No SDT gate integration
- ❌ No crystal resonance

**Reuse Strategy:**
- Copy OODA loop logic
- Replace `tokio::broadcast` with `sx9-atlas-bus::AtlasBus`
- Integrate `PlasmaState` for delta angle tracking
- Add crystal/SDT gating

---

### 2. **ctas7-foundation-daemon** (DSL Engine)
**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-foundation-daemon/`

**What exists:**
- ✅ Complete DSL parser (`dsl/parser.rs`)
- ✅ Playbook executor (`dsl/playbook_executor.rs`)
- ✅ Unicode bridge (`dsl/unicode_bridge.rs`)
- ✅ Hash classifier (`dsl/hash_classifier.rs`)
- ✅ Unicode registry (`dsl/unicode_registry.rs`)
- ✅ TOML compiler (`dsl/toml_unicode_compiler.rs`)

**What needs adaptation:**
- ❌ May have dependencies on `ctas7-foundation-core` internals
- ❌ Needs WASM runtime integration
- ❌ Needs reload-on-change support

**Reuse Strategy:**
- Copy DSL modules
- Adapt dependencies to use `sx9-atlas-bus` and foundation crates
- Add WASM runtime (wasmtime)
- Add file watching for reload

---

## Implementation Plan

### Phase 1: sx9-atlas-daemon (Adapt OODA Loop)

1. **Copy core modules:**
   - `ooda_loop.rs` → adapt to `sx9-atlas-bus`
   - `hd4_phases.rs` → reuse as-is
   - `delta_angles.rs` → reuse as-is
   - `convergence.rs` → reuse as-is

2. **Integrate sx9-atlas-bus:**
   - Replace `tokio::broadcast` with `AtlasBus`
   - Use `PlasmaState` for delta angle
   - Add crystal resonance checks
   - Add SDT gate evaluation

3. **Add main.rs:**
   - CLI argument parsing
   - Configuration loading
   - OODA loop startup
   - Health/metrics endpoints

### Phase 2: sx9-dsl-engine (Adapt DSL)

1. **Copy DSL modules:**
   - `dsl/parser.rs`
   - `dsl/playbook_executor.rs`
   - `dsl/unicode_bridge.rs`
   - `dsl/unicode_registry.rs`
   - `dsl/hash_classifier.rs`

2. **Add WASM runtime:**
   - Integrate `wasmtime`
   - WASM module loading
   - Execution context

3. **Add file watching:**
   - Watch for playbook changes
   - Hot reload support

---

## Files to Copy/Adapt

### From ctas7-atlas-daemon:
- `src/ooda_loop.rs` → adapt
- `src/hd4_phases.rs` → copy
- `src/delta_angles.rs` → copy
- `src/convergence.rs` → copy

### From ctas7-foundation-daemon:
- `src/dsl/parser.rs` → copy
- `src/dsl/playbook_executor.rs` → copy
- `src/dsl/unicode_bridge.rs` → copy
- `src/dsl/unicode_registry.rs` → copy
- `src/dsl/hash_classifier.rs` → copy
- `src/dsl/toml_unicode_compiler.rs` → copy

---

## Next Steps

1. Start with `sx9-atlas-daemon`
2. Copy and adapt OODA loop code
3. Integrate with `sx9-atlas-bus`
4. Test OODA cycle execution
5. Then move to `sx9-dsl-engine`



