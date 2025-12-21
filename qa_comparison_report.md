# QA Comparison Report: SX9 Repository

This report compares findings from **SX9 Lightning QA** (Static/Linter), **Qodo AI** (Simulated Semantic Analysis), and **Human Expert Review**.

| Metric | SonarQube | SX9 Lightning | Qodo AI (Simulated) | Human Expert |
|---|---|---|---|---|
| Methodology | Static Analysis (AST) | Linter (AST) | Semantic Analysis (LLM) | Manual Code Review |
| Type | Legacy Baseline | **Agentic Signal** | **Agentic Intelligence** | Gold Standard |
| Total Issues | 0 | 2055 | 7 | 3 |
| Lines of Code Scanned | ~98877 | 98877 | ~2000 (Simulated) | Focused (High Risk) |

## 🧠 Deep Dive: Agentic vs Static
The gap between **SX9 Lightning** (High Volume, Low Context) and **Qodo AI** (Low Volume, High Context) demonstrates the need for both layers.

### Key Semantic Findings (Missed by Static Tools)
*   **CRITICAL:** GOD_COMPONENT: File is 1675 lines and mixes UI, Data Fetching, and State Management. Extract logic to custom hooks. (`apps/sx9-ops-main/src/components/CTASCLI.tsx`)
*   **CRITICAL:** GOD_COMPONENT: File is 824 lines and mixes UI, Data Fetching, and State Management. Extract logic to custom hooks. (`sx9-dev-forge-rn-migration/src/screens/PromptForgeScreen.tsx`)
*   **HIGH:** SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. (`apps/sx9-ops-main/src/components/MultiCLI.tsx`)
*   **HIGH:** SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. (`apps/sx9-ops-main/src/components/CTASCLI.tsx`)
*   **HIGH:** SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. (`apps/sx9-ops-main/src/pages/Dashboard.tsx`)
*   **HIGH:** SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. (`apps/sx9-ops-main/src/pages/vKali.tsx`)
*   **HIGH:** SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. (`apps/sx9-ops-main/src/components/glaf/GLAFIntelWorksurface.tsx`)
*   **Critical:** Memory Leak: Event listeners attached to `document` in `onMouseDown` are never removed if the component unmounts while dragging. Requires `useEffect` cleanup. (`src/App.tsx`)
*   **UX:** Blocking UX: Use of `alert()` halts the browser thread and provides poor user experience. Should use a Toast/Notification system. (`src/App.tsx`)
*   **Functional:** Broken Feature: `AutoPersistIndicator` status is hardcoded to 'idle'. It effectively lies to the user about sync status. (`src/App.tsx`)

## ⚖️ Methodology Comparison
*   **Sonar/SX9:** Great for finding syntax errors, style violations, and basic bugs at scale (2000+ issues).
*   **Agentic (Qodo):** Crucial for finding **Design Flaws** (God Components) and **Security Context** (Unsanitized Inputs to internal APIs).

## Issue Details

| Tool | Severity | File | Line | Col | Message |
|---|---|---|---|---|---|
| sx9-lightning | ERROR | `/Users/cp5337/Developer/sx9/crates/sx9-foundation-data/.clippy.toml` | 22 | 8 | error reading Clippy's configuration file: invalid table header
expected `.`, `]` |
| sx9-lightning | ERROR | `/Users/cp5337/Developer/sx9/crates/sx9-foundation-interface/.clippy.toml` | 23 | 8 | error reading Clippy's configuration file: invalid table header
expected `.`, `]` |
| qodo-ai | CRITICAL | `apps/sx9-ops-main/src/components/CTASCLI.tsx` | 1 | 0 | GOD_COMPONENT: File is 1675 lines and mixes UI, Data Fetching, and State Management. Extract logic to custom hooks. |
| qodo-ai | HIGH | `apps/sx9-ops-main/src/components/CTASCLI.tsx` | 318 | 0 | SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. |
| qodo-ai | HIGH | `apps/sx9-ops-main/src/components/MultiCLI.tsx` | 95 | 0 | SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. |
| qodo-ai | HIGH | `apps/sx9-ops-main/src/components/glaf/GLAFIntelWorksurface.tsx` | 52 | 0 | SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. |
| qodo-ai | HIGH | `apps/sx9-ops-main/src/pages/Dashboard.tsx` | 216 | 0 | SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. |
| qodo-ai | HIGH | `apps/sx9-ops-main/src/pages/vKali.tsx` | 85 | 0 | SECURITY: Direct call to local backend. Ensure input is sanitized before sending to atomic clipboard. |
| sx9-lightning | WARNING | `crates/sx9-ann-engine/src/lib.rs` | 1 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 3 | 58 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 36 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 47 | 33 | you should put bare URLs between `<`/`>` or make a proper Markdown link |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 51 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 278 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 380 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 386 | 29 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 430 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 440 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 450 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 464 | 1 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bridge.rs` | 471 | 23 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bus.rs` | 114 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bus.rs` | 350 | 9 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/bus.rs` | 350 | 28 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 171 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 184 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 197 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 203 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 209 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 209 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 216 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 216 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 223 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/command.rs` | 223 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 35 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 124 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 151 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 201 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 222 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 243 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 256 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 290 | 17 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 290 | 36 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 291 | 24 | casts from `u16` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 304 | 24 | casting to the same type is unnecessary (`u32` -> `u32`) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 363 | 13 | casting `u64` to `f32` causes a loss of precision (`u64` is 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 363 | 28 | casting `u64` to `f32` causes a loss of precision (`u64` is 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 386 | 26 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 390 | 22 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 392 | 33 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 394 | 48 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 397 | 28 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 402 | 35 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 431 | 19 | casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 447 | 23 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 452 | 16 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 480 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 532 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 542 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 674 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 683 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 692 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 702 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/crystal.rs` | 711 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 10 | 26 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 10 | 55 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 11 | 36 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 11 | 51 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 15 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 40 | 31 | unused import: `CrystalFamily` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 64 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 72 | 34 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 147 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 259 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 343 | 10 | casts from `u16` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 355 | 19 | casting `f32` to `u16` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 355 | 19 | casting `f32` to `u16` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 564 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 565 | 10 | casts from `u16` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 569 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/plasma.rs` | 574 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 173 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 187 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 189 | 45 | casts from `u8` to `u16` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 201 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 201 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 208 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 208 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 216 | 15 | constant `OK` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 217 | 15 | constant `UNKNOWN_COMMAND` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 218 | 15 | constant `INVALID_NODE` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 219 | 15 | constant `PATH_NOT_FOUND` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 220 | 15 | constant `BUFFER_FULL` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 221 | 15 | constant `TIMEOUT` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 222 | 15 | constant `SDT_LOCKED` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 223 | 15 | constant `PLASMA_FAULT` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/result.rs` | 224 | 15 | constant `INTERNAL` is never used |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/ring.rs` | 10 | 34 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/ring.rs` | 62 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/ring.rs` | 160 | 9 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-bus/src/ring.rs` | 160 | 29 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 21 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 25 | 23 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 30 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 45 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 50 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 66 | 57 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 71 | 72 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 85 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 90 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/convergence.rs` | 95 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 32 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 32 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 37 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 43 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 51 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 62 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 74 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 85 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 110 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 110 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 114 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 120 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 120 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 124 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 130 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 140 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/hd4_phases.rs` | 152 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/lib.rs` | 62 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/lib.rs` | 64 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/lib.rs` | 72 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/lib.rs` | 160 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/lib.rs` | 178 | 44 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/main.rs` | 47 | 9 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 59 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 82 | 40 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 106 | 54 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 109 | 29 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 110 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 134 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 137 | 31 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 149 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 159 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 160 | 27 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 207 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 216 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 256 | 6 | variant `CycleBack` is never constructed |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 271 | 46 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 274 | 37 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 275 | 58 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 276 | 9 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 276 | 26 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 280 | 37 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 281 | 10 | casts from `u64` to `u128` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-atlas-daemon/src/ooda_loop.rs` | 281 | 35 | casts from `u64` to `u128` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 3 | 34 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 5 | 5 | unused import: `crate::registry::DatabaseInfo` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 7 | 5 | unused import: `std::path::Path` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 9 | 12 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 10 | 14 | function `load_file` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 16 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 17 | 14 | function `list_layers` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 23 | 12 | called `map(<f>).unwrap_or(false)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 33 | 11 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 34 | 14 | function `query_features` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 43 | 21 | called `map(<f>).unwrap_or(false)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 58 | 25 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 59 | 8 | function `get_bbox` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 88 | 4 | function `extract_coordinates` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 96 | 45 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 104 | 58 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 119 | 74 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 132 | 27 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 133 | 42 | unused variable: `edges` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 133 | 8 | function `graph_to_geojson` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 143 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/geojson.rs` | 148 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/mod.rs` | 17 | 11 | trait `DatabaseAdapter` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/mod.rs` | 33 | 10 | enum `AdapterCapability` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/neo4j.rs` | 58 | 14 | function `get_schema` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/neo4j.rs` | 75 | 14 | function `health_check` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/neo4j.rs` | 84 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/neo4j.rs` | 85 | 8 | function `to_neo4j_import` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 16 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 349 | 1 | this function has too many lines (118/100) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 373 | 22 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 374 | 22 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 410 | 30 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 411 | 22 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 412 | 32 | casts from `u8` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 494 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 524 | 35 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 526 | 32 | casting `usize` to `u8` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 528 | 45 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 529 | 42 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 530 | 47 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/network_flow.rs` | 596 | 29 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 3 | 32 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 23 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 31 | 47 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 36 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 93 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 94 | 14 | function `get_schema` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 95 | 17 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 114 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 115 | 14 | function `health_check` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/postgres.rs` | 129 | 13 | redundant pattern matching, consider using `is_ok()` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 3 | 36 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 9 | 25 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 10 | 14 | function `execute` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 35 | 14 | function `get_all_entities` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 53 | 29 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 54 | 14 | function `get_graph` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 67 | 39 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 68 | 14 | function `get_schema` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 97 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 98 | 14 | function `health_check` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 108 | 8 | function `entities_to_graph` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 113 | 22 | called `map(<f>).unwrap_or_else(<g>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 117 | 36 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/slotgraph.rs` | 119 | 26 | called `map(<f>).unwrap_or_else(<g>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 3 | 32 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 11 | 15 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 35 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 36 | 14 | function `get_schema` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 60 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 61 | 14 | function `health_check` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/adapters/surreal.rs` | 66 | 13 | redundant pattern matching, consider using `is_ok()` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 14 | 5 | unused import: `dashmap::DashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 18 | 21 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 121 | 8 | field `heartbeat_interval_ms` is never read |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 256 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 257 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 274 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 522 | 11 | unused variable: `state` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/main.rs` | 524 | 20 | unused variable: `socket` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/registry.rs` | 8 | 5 | unused import: `std::sync::Arc` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/registry.rs` | 43 | 5 | this function has too many lines (178/100) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 5 | 28 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 9 | 5 | unused import: `serde_json::Value` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 12 | 8 | function `detect_query_language` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 35 | 10 | enum `QueryLanguage` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 36 | 5 | name `SQL` contains a capitalized acronym |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 44 | 8 | function `find_best_database` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 49 | 5 | called `filter(..).next()` on an `Iterator`. This is more succinctly expressed by calling `.find(..)` instead |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 73 | 12 | struct `QueryRoute` is never constructed |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/router.rs` | 80 | 8 | function `plan_route` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 9 | 14 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 11 | 13 | unused imports: `Deserialize` and `Serialize` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 32 | 18 | called `map(<f>).unwrap_or_else(<g>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 36 | 32 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 51 | 18 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 52 | 18 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 125 | 18 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 134 | 27 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 139 | 27 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 152 | 21 | manual implementation of `Option::map` |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 172 | 25 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 181 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 181 | 27 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 194 | 29 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 195 | 8 | function `cypher_to_surql` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 197 | 5 | returning the result of a `let` binding from a block |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 197 | 17 | replacing text with itself |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 204 | 18 | single-character string constant used as pattern |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 209 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 210 | 8 | function `surql_to_cypher` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-data-fabric/src/transform.rs` | 217 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 15 | 12 | fields `config`, `metrics_collector`, `statistical_engine`, `cdn_distributor`, `isolation_boundary`, and `dashboard_interface` are never read |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 289 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 297 | 12 | fields `start_time` and `config` are never read |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 314 | 1 | associated function `get_current_cpu` is never used |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 316 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 327 | 5 | you have declared `#[inline(always)]` on `record_metric`. This is usually a bad idea |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 362 | 5 | you have declared `#[inline(always)]` on `read_cpu_cycles`. This is usually a bad idea |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 370 | 13 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 378 | 5 | you have declared `#[inline(always)]` on `get_current_cpu`. This is usually a bad idea |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 383 | 28 | transmute used without annotations |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 421 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/core.rs` | 432 | 22 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/foundation_integration.rs` | 37 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-isolated-monitoring/src/foundation_integration.rs` | 50 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 7 | 13 | unused imports: `Deserialize` and `Serialize` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 8 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 9 | 22 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 62 | 5 | you should consider adding a `Default` implementation for `AnalysisEngine` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 62 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 116 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 133 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 152 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 173 | 24 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analysis_engine.rs` | 184 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analytics.rs` | 51 | 5 | you should consider adding a `Default` implementation for `CDNAnalytics` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analytics.rs` | 51 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/analytics.rs` | 85 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/gateway-cdn.rs` | 10 | 28 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/gateway-cdn.rs` | 13 | 49 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/gateway-cdn.rs` | 13 | 70 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/gateway-cdn.rs` | 55 | 21 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 7 | 21 | unused imports: `Query` and `http::StatusCode` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 16 | 28 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 23 | 25 | unused import: `EdgeStatus` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 69 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 105 | 19 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 136 | 19 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-edge.rs` | 177 | 19 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-origin.rs` | 7 | 21 | unused imports: `Query` and `http::StatusCode` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-origin.rs` | 15 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-origin.rs` | 18 | 28 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-origin.rs` | 19 | 5 | unused import: `uuid::Uuid` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/bin/shipyard-cdn-origin.rs` | 24 | 13 | unused imports: `GeographicLocation` and `OriginStatus` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cache.rs` | 3 | 14 | unused imports: `DateTime` and `Utc` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cache.rs` | 63 | 5 | you should consider adding a `Default` implementation for `CacheManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cache.rs` | 63 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cache.rs` | 77 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cache.rs` | 85 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 7 | 15 | unused import: `error` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 15 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 28 | 5 | you should consider adding a `Default` implementation for `CTASCDN` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 28 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 39 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 39 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 47 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 47 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 59 | 13 | unused variable: `edge_location` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 101 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 103 | 13 | unused variable: `origin` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 115 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 120 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 120 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 143 | 1 | unused doc comment |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cdn.rs` | 144 | 1 | this macro has been superseded by `std::sync::LazyLock` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 7 | 21 | unused imports: `Query` and `post` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 15 | 15 | unused imports: `error` and `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 96 | 5 | you should consider adding a `Default` implementation for `ComponentCDN` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 96 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 104 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 197 | 53 | this argument is passed by value, but not consumed in the function body |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 204 | 31 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 207 | 18 | use of `or_insert_with` to construct default value |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 213 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 218 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 228 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 232 | 27 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 244 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 249 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/component_cdn.rs` | 263 | 38 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 10 | 15 | unused imports: `error` and `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 186 | 5 | you should consider adding a `Default` implementation for `CyberOperations` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 186 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 196 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 196 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 208 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 208 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 210 | 21 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 221 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 221 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 228 | 5 | you should consider adding a `Default` implementation for `ThreatDatabase` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 228 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/cyber_operations.rs` | 242 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 65 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 76 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 82 | 5 | you should consider adding a `Default` implementation for `LocalCache` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 82 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 94 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 106 | 5 | you should consider adding a `Default` implementation for `PerformanceMetrics` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/edge.rs` | 106 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 10 | 27 | unused import: `Instant` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 11 | 15 | unused imports: `debug`, `error`, and `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 14 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 426 | 5 | you should consider adding a `Default` implementation for `GatewayCDN` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 426 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 438 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 439 | 13 | this `MutexGuard` is held across an await point |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 458 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 458 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 470 | 5 | this function has too many lines (120/100) |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 470 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 612 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 612 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 643 | 5 | you should consider adding a `Default` implementation for `NGINXConfigManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 643 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 651 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 658 | 5 | you should consider adding a `Default` implementation for `CyberOperations` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 658 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 669 | 5 | you should consider adding a `Default` implementation for `ThreatDatabase` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 669 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 679 | 5 | you should consider adding a `Default` implementation for `TrafficIntelligence` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 679 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 689 | 5 | you should consider adding a `Default` implementation for `TrafficAnalysis` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 689 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 702 | 5 | you should consider adding a `Default` implementation for `PortManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 702 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 710 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 712 | 9 | unused variable: `port` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 713 | 9 | unused variable: `service_name` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 714 | 9 | unused variable: `service_type` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 722 | 5 | you should consider adding a `Default` implementation for `ServiceRegistry` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 722 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 731 | 1 | unused doc comment |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn.rs` | 732 | 1 | this macro has been superseded by `std::sync::LazyLock` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 10 | 15 | unused import: `error` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 16 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 152 | 5 | you should consider adding a `Default` implementation for `GatewayCDN` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 152 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 164 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 165 | 13 | this `MutexGuard` is held across an await point |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 188 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 190 | 13 | this `MutexGuard` is held across an await point |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 202 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 202 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 222 | 5 | you should consider adding a `Default` implementation for `PortManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 222 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 230 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 234 | 9 | unused variable: `service_type` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 246 | 5 | you should consider adding a `Default` implementation for `ServiceRegistry` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 246 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 255 | 1 | unused doc comment |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_cdn_refactored.rs` | 256 | 1 | this macro has been superseded by `std::sync::LazyLock` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 7 | 21 | unused imports: `Query` and `http::StatusCode` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 13 | 22 | unused imports: `info` and `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 17 | 58 | unused imports: `ECSService`, `ServiceStatus`, `ServiceType`, and `register_gateway_service` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 23 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 45 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 84 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/gateway_handlers.rs` | 98 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_handlers.rs` | 9 | 15 | unused imports: `error`, `info`, and `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_handlers.rs` | 12 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_handlers.rs` | 46 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_handlers.rs` | 49 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_handlers.rs` | 74 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 12 | 5 | unused import: `crate::cyber_operations::ThreatLevel` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 84 | 5 | you should consider adding a `Default` implementation for `IntelligenceManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 84 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 129 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 137 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 142 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/intelligence_reports.rs` | 147 | 40 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 9 | 15 | unused imports: `error` and `info` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 50 | 5 | you should consider adding a `Default` implementation for `NGINXConfigManager` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 50 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 59 | 5 | this function has too many lines (120/100) |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 59 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 205 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/nginx_manager.rs` | 233 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/origin.rs` | 22 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/origin.rs` | 33 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/routing.rs` | 17 | 5 | you should consider adding a `Default` implementation for `RouteOptimizer` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/routing.rs` | 17 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/routing.rs` | 25 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 8 | 28 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 19 | 5 | you should consider adding a `Default` implementation for `ServiceRegistry` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 19 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 100 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 105 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 110 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 110 | 54 | this argument is passed by value, but not consumed in the function body |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 118 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/service_registry.rs` | 126 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 7 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 8 | 5 | unused import: `uuid::Uuid` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 117 | 5 | you should consider adding a `Default` implementation for `TrafficAnalysis` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 117 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 181 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 187 | 30 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 193 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 204 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 215 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 222 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 228 | 5 | you should consider adding a `Default` implementation for `AnalysisEngine` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis.rs` | 228 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 5 | 14 | unused import: `DateTime` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 7 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 9 | 5 | unused import: `crate::cyber_operations::ThreatLevel` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 26 | 5 | you should consider adding a `Default` implementation for `TrafficAnalysis` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 26 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 90 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 96 | 30 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 102 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 113 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 124 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 131 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_analysis_core.rs` | 136 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 9 | 28 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 207 | 5 | you should consider adding a `Default` implementation for `TrafficIntelligence` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 207 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 217 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 296 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 312 | 30 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 318 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 340 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 356 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 363 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 368 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 376 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 386 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 394 | 40 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 404 | 5 | you should consider adding a `Default` implementation for `TrafficAnalysis` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence.rs` | 404 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 6 | 14 | unused import: `DateTime` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 9 | 15 | unused import: `debug` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 16 | 53 | unused import: `TrafficStatistics` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 18 | 48 | unused import: `ResponseAction` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 30 | 5 | you should consider adding a `Default` implementation for `TrafficIntelligence` |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 30 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 49 | 13 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 81 | 25 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 102 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 119 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 140 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-cdn-monitoring/src/traffic_intelligence_refactored.rs` | 151 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 152 | 32 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 153 | 35 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 154 | 31 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 156 | 33 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 158 | 18 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 159 | 34 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 170 | 60 | casting `u64` to `i64` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 243 | 9 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 315 | 17 | called `Iterator::last` on a `DoubleEndedIterator`; this will needlessly iterate the entire iterator |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 369 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 420 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 424 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 428 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 432 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 436 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 443 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 447 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 451 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 455 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 459 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 468 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 472 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 476 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 484 | 19 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 488 | 19 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 492 | 19 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 496 | 19 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 504 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 508 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 514 | 9 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-cdn-statistical/src/main.rs` | 568 | 17 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/bin/threat_intel_server.rs` | 10 | 21 | unused import: `Query` |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/bin/threat_intel_server.rs` | 17 | 26 | unused import: `Serialize` |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/bin/threat_intel_server.rs` | 18 | 5 | unused import: `tokio::sync::RwLock` |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/bin/threat_intel_server.rs` | 107 | 8 | fields `content_type`, `mitre_id`, `source`, and `limit` are never read |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 15 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 157 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 189 | 14 | use of `or_insert_with` to construct default value |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 196 | 18 | use of `or_insert_with` to construct default value |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 205 | 23 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 231 | 23 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 281 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 286 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 303 | 17 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 303 | 45 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 335 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 335 | 16 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 335 | 24 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-cdn-threat-intel/src/lib.rs` | 385 | 34 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 50 | 42 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 51 | 46 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 52 | 41 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 62 | 12 | field `wasm_enabled` is never read |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 67 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 71 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 90 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/dsl.rs` | 95 | 59 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/executor.rs` | 13 | 12 | field `wasm_runtime` is never read |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/executor.rs` | 20 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/executor.rs` | 20 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/executor.rs` | 57 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/file_watcher.rs` | 8 | 5 | unused import: `std::sync::Arc` |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/file_watcher.rs` | 48 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/file_watcher.rs` | 51 | 17 | called `map(<f>).unwrap_or(false)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/lib.rs` | 16 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/lib.rs` | 27 | 5 | method `default` can be confused for the standard trait method `std::default::Default::default` |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/lib.rs` | 27 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/wasm_runtime.rs` | 6 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/wasm_runtime.rs` | 21 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/wasm_runtime.rs` | 24 | 9 | unused variable: `function` |
| sx9-lightning | WARNING | `crates/sx9-dsl-engine/src/wasm_runtime.rs` | 25 | 9 | unused variable: `args` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 23 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 28 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 34 | 24 | matching over `()` is more explicit |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 34 | 30 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 36 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 57 | 15 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 72 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 75 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 79 | 22 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 92 | 18 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/build.rs` | 118 | 12 | redundant pattern matching, consider using `is_err()` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 7 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 90 | 12 | type `iOSIntegration` should have an upper camel case name |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 98 | 12 | type `macOSIntegration` should have an upper camel case name |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 132 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 145 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 145 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 322 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 327 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 338 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 349 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 360 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 372 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cli_manifest.rs` | 382 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 51 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 91 | 43 | casts from `u64` to `u128` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 133 | 45 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 134 | 19 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 135 | 19 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 149 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 152 | 9 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 156 | 9 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 160 | 9 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 173 | 13 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 175 | 17 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 192 | 13 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/code_watchdog.rs` | 194 | 17 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 8 | 5 | unused import: `legion::world::SubWorld` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 9 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 11 | 5 | unused import: `std::collections::VecDeque` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 34 | 5 | you should consider adding a `Default` implementation for `CognitiveState` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 34 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 89 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 117 | 15 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 120 | 37 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 190 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 284 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 294 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 330 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 334 | 26 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 347 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 351 | 17 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 353 | 41 | casting `i64` to `f32` causes a loss of precision (`i64` is 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 370 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 374 | 17 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 401 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 405 | 17 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cognitive.rs` | 409 | 51 | casting `u128` to `u32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 8 | 5 | unused import: `uuid::Uuid` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 119 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 132 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 143 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 316 | 5 | you should consider adding a `Default` implementation for `RoomManager` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 316 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/conference_engine.rs` | 341 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/context.rs` | 9 | 12 | field `environments` is never read |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/context.rs` | 14 | 5 | you should consider adding a `Default` implementation for `ContextualIntelligence` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/context.rs` | 14 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/context.rs` | 25 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/context.rs` | 30 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 113 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 124 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 146 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 241 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 244 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 249 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 282 | 29 | casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 301 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 314 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 341 | 5 | you should consider adding a `Default` implementation for `AgentRegistry` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 341 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 350 | 5 | you should consider adding a `Default` implementation for `PortManager` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 350 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 378 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 387 | 5 | you should consider adding a `Default` implementation for `RollcallBridge` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/cte_integration.rs` | 387 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 12 | 35 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 18 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 26 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 92 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 101 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 169 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 169 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 236 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 236 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 255 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 260 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 264 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 271 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 288 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 345 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 370 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 437 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 442 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 453 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/database.rs` | 464 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 6 | 26 | unused import: `Error` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 7 | 25 | unused imports: `ExecutionContext` and `Priority` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 8 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 21 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 31 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 58 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 66 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 72 | 17 | casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 76 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 84 | 25 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 84 | 25 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 84 | 26 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 92 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/dsl_unicode_router.rs` | 97 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 13 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 16 | 62 | single-character string constant used as pattern |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 17 | 30 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 22 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 33 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 42 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/foundation_integration.rs` | 53 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/frontend_bridge.rs` | 58 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/frontend_bridge.rs` | 69 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/frontend_bridge.rs` | 144 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/frontend_bridge.rs` | 163 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/frontend_bridge.rs` | 180 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 7 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 44 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 53 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 68 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 72 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 84 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 87 | 27 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 88 | 28 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 94 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 95 | 9 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 99 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 100 | 9 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 121 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 127 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 139 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 140 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 145 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 150 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 152 | 5 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 156 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 157 | 20 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 201 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 202 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash.rs` | 211 | 53 | `to_string` applied to a type that implements `Display` in `format!` args |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 3 | 31 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 4 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 30 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 32 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 41 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 44 | 5 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 47 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 48 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 60 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 81 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 90 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 106 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 110 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 114 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 115 | 20 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 117 | 21 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 128 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash64.rs` | 135 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 88 | 5 | you should consider adding a `Default` implementation for `HashEngine` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 88 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 88 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 118 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 169 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 174 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 184 | 46 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 186 | 17 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 200 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 205 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 210 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 229 | 30 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 330 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 336 | 19 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_engine.rs` | 341 | 25 | casts from `u32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 37 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 47 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 47 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 117 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 131 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 172 | 23 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 192 | 27 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 193 | 22 | useless use of `vec!` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 199 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 206 | 23 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 207 | 23 | useless use of `vec!` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 213 | 24 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/hash_is_ui.rs` | 214 | 25 | useless use of `vec!` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 1 | 12 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 43 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 44 | 12 | field `vault_dir` is never read |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 57 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 64 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 65 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 69 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 70 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 73 | 44 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 101 | 5 | this function's return value is unnecessary |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 106 | 13 | unnecessary `if let` since only the `Ok` variant of the iterator element is used |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 154 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 171 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 214 | 35 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 232 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 289 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 305 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 325 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 356 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 372 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 373 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 403 | 9 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 408 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 413 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 418 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 423 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/keyvault.rs` | 460 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 72 | 26 | use of deprecated struct `trivariate_hash::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 104 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 117 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 154 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 160 | 13 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 167 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 176 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 185 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 190 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 227 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 296 | 1 | item has both inner and outer attributes |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 301 | 39 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 335 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 340 | 5 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 341 | 19 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 414 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 538 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 652 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 662 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 722 | 9 | you should consider adding a `Default` implementation for `TelemetryCollector` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 722 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 748 | 9 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 840 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 863 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 912 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/lib.rs` | 912 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 30 | 15 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 89 | 27 | useless use of `vec!` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 128 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 134 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 164 | 18 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 164 | 56 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 205 | 38 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 205 | 80 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 208 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 209 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 213 | 36 | implicitly cloning a `Vec` by calling `to_vec` on its dereferenced type |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 255 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 256 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 259 | 25 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/main.rs` | 276 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 43 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 120 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 120 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 142 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 149 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 149 | 49 | this argument is passed by value, but not consumed in the function body |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 156 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/mathematical_consciousness.rs` | 163 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/matroid.rs` | 9 | 5 | unused import: `std::collections::HashSet` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/matroid.rs` | 20 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/matroid.rs` | 30 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/matroid.rs` | 42 | 27 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/matroid.rs` | 58 | 26 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/media_processor.rs` | 199 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/media_processor.rs` | 208 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/media_processor.rs` | 236 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/media_processor.rs` | 246 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/native_integration.rs` | 112 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/native_integration.rs` | 154 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/native_integration.rs` | 164 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/native_integration.rs` | 174 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/native_integration.rs` | 183 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 132 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 140 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 156 | 48 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 191 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 260 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/neural_mux.rs` | 264 | 5 | field assignment outside of initializer for an instance created with Default::default() |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 24 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 40 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 73 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 75 | 28 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 76 | 18 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 86 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 93 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 113 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 120 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 127 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 132 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/persona.rs` | 132 | 47 | this argument is passed by value, but not consumed in the function body |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 42 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 58 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 135 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 212 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 218 | 27 | unused variable: `info` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 218 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 220 | 12 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 223 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_detection.rs` | 243 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 7 | 5 | unused import: `tokio::sync::RwLock` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 42 | 5 | variant `iMac` should have an upper camel case name |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 43 | 5 | variant `iPad` should have an upper camel case name |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 44 | 5 | variant `iPhone` should have an upper camel case name |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 50 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 159 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 197 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 206 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 216 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 284 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 374 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 384 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 404 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 518 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 568 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 568 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 660 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 660 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 725 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 725 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 776 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 776 | 5 | this function has too many lines (101/100) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 888 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 914 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 918 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 961 | 9 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 968 | 40 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 1016 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/platform_native_multimedia.rs` | 1029 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 105 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 110 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 115 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 154 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 156 | 12 | manual `RangeInclusive::contains` implementation |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 157 | 31 | casting `u32` to `u8` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 164 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 202 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 212 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 217 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 254 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 327 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 339 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 345 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 351 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 357 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 357 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 363 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/primitives.rs` | 367 | 45 | casts from `u8` to `u32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 1 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 4 | 36 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 75 | 37 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 75 | 59 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 85 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 90 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 99 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 99 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 241 | 26 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 252 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 274 | 14 | unused variable: `i` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 357 | 31 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 357 | 52 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 361 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 402 | 27 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 410 | 22 | single-character string constant used as pattern |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 411 | 22 | single-character string constant used as pattern |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 434 | 31 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 435 | 1 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 444 | 25 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/secrets_watchdog.rs` | 445 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 11 | 5 | unused import: `crate::platform_detection::Platform` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 86 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 188 | 5 | you should consider adding a `Default` implementation for `SessionController` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 188 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 195 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/session_controller.rs` | 215 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/taxonomy.rs` | 25 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/taxonomy.rs` | 29 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/taxonomy.rs` | 62 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 10 | 13 | unused imports: `Deserialize` and `Serialize` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 38 | 5 | you should consider adding a `Default` implementation for `ThalamicFilter` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 38 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 72 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 91 | 26 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 92 | 9 | returning the result of a `let` binding from a block |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 103 | 34 | unused variable: `signal` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/thalamic_filter.rs` | 103 | 27 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 54 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 64 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 112 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 114 | 30 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 115 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 116 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 122 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 122 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 128 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 128 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 146 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 154 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 155 | 31 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 164 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 164 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 177 | 26 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 184 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 185 | 31 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 193 | 35 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 214 | 56 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 214 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 215 | 24 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 220 | 22 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 221 | 62 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 226 | 58 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 228 | 58 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 235 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 238 | 9 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 245 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 249 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 250 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 256 | 28 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 280 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 285 | 31 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 302 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 336 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 341 | 46 | unused variable: `hash` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 341 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash.rs` | 384 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 28 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 53 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 60 | 36 | casting `i32` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 64 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 77 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 78 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 78 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 91 | 20 | casting `u64` to `u16` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 95 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 96 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 163 | 31 | casting `f32` to `u16` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 163 | 31 | casting `f32` to `u16` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 201 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 201 | 5 | implementation of inherent method `to_string(&self) -> String` for type `trivariate_hash_v731::CuidSlots` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 215 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 219 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 228 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 237 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 246 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 262 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 271 | 20 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 294 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 299 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 304 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 322 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 338 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 346 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 366 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 409 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 411 | 31 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 412 | 32 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 418 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 444 | 27 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 450 | 28 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 451 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 456 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 457 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 462 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 479 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 519 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 524 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 524 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 544 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 603 | 17 | useless use of `vec!` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 616 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 618 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 621 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 630 | 18 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 636 | 1 | this function has too many lines (104/100) |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 640 | 21 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 640 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 641 | 21 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 641 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 642 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 643 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 644 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 645 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 646 | 21 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 646 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 647 | 21 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 647 | 5 | adding items after statements is confusing, since items exist from the start of the scope |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 650 | 23 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 651 | 23 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 691 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 694 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 697 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 700 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 703 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 706 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 709 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 713 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 716 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 719 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 722 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 725 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 728 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 731 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 733 | 12 | length comparison to one |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 734 | 19 | casts from `u8` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 772 | 24 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 774 | 24 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/trivariate_hash_v731.rs` | 781 | 18 | it is more concise to loop over references to containers instead of using explicit iteration methods |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 201 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 209 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 217 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 252 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 266 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 266 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 524 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 529 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 554 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 566 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 590 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 602 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 645 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/ui_manifest.rs` | 667 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 3 | 17 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 40 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 56 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 56 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 65 | 13 | unused variable: `effective_priority` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 95 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 95 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/unified_neural_mux.rs` | 125 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 16 | 36 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 51 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 62 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 72 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 82 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 92 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 101 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 105 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 132 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 136 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 136 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 141 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 141 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 146 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 146 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 151 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 151 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 156 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 156 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 161 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 161 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 166 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 166 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 171 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 171 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-core/src/usim.rs` | 176 | 5 | docs for function returning `Result` missing `# Errors` section |
| sx9-lightning | WARNING | `crates/sx9-foundation-data/src/ctas_sled_kvs.rs` | 5 | 5 | unused import: `async_trait::async_trait` |
| sx9-lightning | WARNING | `crates/sx9-foundation-data/src/ctas_sled_kvs.rs` | 11 | 5 | unused import: `uuid::Uuid` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 98 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 99 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 104 | 32 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 112 | 40 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 112 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 147 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 213 | 25 | casting `f64` to `isize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 213 | 48 | casting to the same type is unnecessary (`isize` -> `isize`) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 214 | 25 | casting `f64` to `isize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 214 | 48 | casting to the same type is unnecessary (`isize` -> `isize`) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 216 | 44 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 216 | 72 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 217 | 55 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 217 | 67 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 229 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 231 | 9 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 231 | 9 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 235 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 235 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 243 | 51 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 251 | 49 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 256 | 76 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 272 | 10 | very complex type used. Consider factoring parts into `type` definitions |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 286 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 287 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 350 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 351 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 541 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 574 | 37 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 574 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 590 | 35 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 590 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 669 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 699 | 19 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 704 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 713 | 18 | the loop variable `i` is used to index `combined` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 735 | 18 | the loop variable `i` is used to index `orientation_field` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 767 | 31 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 774 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 809 | 64 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 815 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 815 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 826 | 28 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 852 | 34 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 852 | 45 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 853 | 34 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 853 | 45 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 946 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1040 | 31 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1040 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1046 | 23 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1046 | 23 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1046 | 33 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1062 | 23 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1062 | 23 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1062 | 33 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1063 | 35 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1063 | 53 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1103 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1104 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1118 | 26 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1118 | 37 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1119 | 28 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1120 | 28 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1122 | 32 | casting `f64` to `isize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1123 | 17 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1123 | 32 | casting `f64` to `isize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1127 | 20 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1128 | 21 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1130 | 27 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1130 | 59 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1137 | 19 | casts from `i32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1144 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1145 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1179 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1180 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1185 | 13 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1407 | 27 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1407 | 27 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1433 | 33 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1433 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1440 | 26 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1440 | 37 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1441 | 26 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1441 | 37 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1456 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1457 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1486 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1487 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1494 | 29 | casting `f64` to `usize` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1494 | 29 | casting `f64` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1503 | 29 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1503 | 57 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1504 | 33 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1504 | 61 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1505 | 42 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1505 | 43 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1506 | 42 | casting `isize` to `usize` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1506 | 43 | casting `usize` to `isize` may wrap around the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1510 | 47 | casting `isize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`isize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1549 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/biometric_analysis.rs` | 1563 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 4 | 5 | unused import: `std::sync::Arc` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 5 | 5 | unused import: `tokio::sync::RwLock` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 16 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 19 | 62 | single-character string constant used as pattern |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 20 | 30 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 36 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 56 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 72 | 11 | unexpected `cfg` condition value: `metrics` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 77 | 15 | unexpected `cfg` condition value: `metrics` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 91 | 11 | unexpected `cfg` condition value: `metrics` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/foundation_integration.rs` | 95 | 15 | unexpected `cfg` condition value: `metrics` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 11 | 26 | use of deprecated struct `sx9_foundation_core::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 33 | 42 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 34 | 28 | use of deprecated struct `sx9_foundation_core::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 58 | 32 | use of deprecated struct `sx9_foundation_core::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 58 | 53 | use of deprecated associated function `sx9_foundation_core::TrivariteHashEngine::new`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 69 | 55 | use of deprecated method `sx9_foundation_core::TrivariteHashEngine::generate_trivariate_hash`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 87 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 110 | 30 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 111 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 210 | 28 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 227 | 24 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 227 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 232 | 23 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 232 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 237 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 237 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 242 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 242 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 247 | 24 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 247 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 252 | 22 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 252 | 5 | this function's return value is unnecessarily wrapped by `Result` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 265 | 5 | you should consider adding a `Default` implementation for `UniversalPrimitivesEngine` |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 265 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 335 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 350 | 38 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 390 | 65 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 391 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 416 | 12 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 418 | 18 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 419 | 19 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 420 | 18 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 421 | 19 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 422 | 18 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 423 | 17 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-foundation-math/src/lib.rs` | 469 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 57 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 72 | 51 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 78 | 51 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 84 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 95 | 21 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cdn_bridge.rs` | 102 | 21 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 6 | 28 | unused import: `TacticalError` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 68 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 73 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 94 | 43 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 97 | 38 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 102 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 110 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/cognigraph.rs` | 118 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/foundation_integration.rs` | 5 | 5 | unused import: `sx9_foundation_core::{}` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/foundation_integration.rs` | 32 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/foundation_integration.rs` | 45 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 10 | 12 | fields `gravity_constant` and `friction_coefficient` are never read |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 69 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 74 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 156 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 166 | 28 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 177 | 28 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 197 | 42 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 201 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 215 | 15 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 234 | 42 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 238 | 27 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 242 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 250 | 20 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 258 | 21 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 266 | 25 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 320 | 34 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/haptic_physics.rs` | 326 | 34 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 10 | 26 | use of deprecated struct `sx9_foundation_core::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 63 | 44 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 78 | 53 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 80 | 22 | use of deprecated struct `sx9_foundation_core::TrivariteHashEngine`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 80 | 43 | use of deprecated associated function `sx9_foundation_core::TrivariteHashEngine::new`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 81 | 27 | use of deprecated method `sx9_foundation_core::TrivariteHashEngine::generate_trivariate_hash`: Use trivariate_hash_v731 instead. v7.2 is legacy. |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 87 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 96 | 57 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 100 | 57 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 104 | 17 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 115 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 124 | 57 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 127 | 48 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 132 | 21 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 135 | 48 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 148 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/hash_missions.rs` | 158 | 13 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/lib.rs` | 4 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/lib.rs` | 18 | 5 | unused import: `uuid::Uuid` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/lib.rs` | 85 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/lib.rs` | 95 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 3 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 6 | 13 | unused import: `TacticalError` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 109 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 114 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 126 | 39 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 134 | 48 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 147 | 48 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 172 | 52 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 176 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 195 | 13 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 227 | 24 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-foundation-tactical/src/swift_bridge.rs` | 233 | 24 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 21 | 12 | fields `relationships` and `stats` are never read |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 94 | 18 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 100 | 26 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 108 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 127 | 13 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 127 | 29 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 131 | 13 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 131 | 29 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 135 | 13 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 135 | 29 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 146 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 163 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/glaf_client.rs` | 183 | 19 | the borrowed expression implements the required traits |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 3 | 25 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 8 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 9 | 20 | unused import: `ports` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 12 | 1 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 59 | 26 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 111 | 22 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 117 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 131 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 152 | 9 | `format!(..)` appended to existing `String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 152 | 25 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 227 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 244 | 43 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 251 | 70 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 254 | 43 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 310 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 326 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 339 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 348 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 361 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/handlers.rs` | 389 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/lib.rs` | 6 | 51 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/main.rs` | 4 | 35 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/main.rs` | 7 | 7 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/main.rs` | 40 | 9 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 21 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 31 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 145 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 145 | 34 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 148 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/protocol.rs` | 203 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/server.rs` | 39 | 9 | variable does not need to be mutable |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/server.rs` | 113 | 71 | useless conversion to the same type: `std::string::String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/server.rs` | 123 | 38 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/server.rs` | 128 | 63 | useless conversion to the same type: `std::string::String` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 180 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 189 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 212 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 292 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 376 | 13 | you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let` |
| sx9-lightning | WARNING | `crates/sx9-gateway-primary/src/state.rs` | 429 | 9 | using `clone` on type `Option<PlasmaSnapshot>` which implements the `Copy` trait |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 5 | 5 | unused import: `crate::glaf_core::GLAFCore` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 6 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 27 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 77 | 37 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 81 | 5 | clamp-like pattern without using clamp function |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/convergence.rs` | 105 | 20 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 5 | 13 | unused imports: `Deserialize` and `Serialize` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 6 | 5 | unused import: `serde_json::Value` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 7 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 24 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 57 | 26 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 62 | 58 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/glaf_core.rs` | 65 | 5 | empty line after doc comment |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hawkes.rs` | 6 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hawkes.rs` | 17 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hawkes.rs` | 28 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hawkes.rs` | 33 | 17 | casting `i64` to `f64` causes a loss of precision (`i64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hawkes.rs` | 37 | 23 | casting `i64` to `f64` causes a loss of precision (`i64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 6 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 33 | 5 | you should consider adding a `Default` implementation for `HmmPhaseDetector` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 33 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 46 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 46 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/hmm.rs` | 72 | 30 | called `map(<f>).unwrap_or(<a>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/lib.rs` | 36 | 5 | method `default` can be confused for the standard trait method `std::default::Default::default` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/lib.rs` | 36 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/matroid.rs` | 28 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/matroid.rs` | 35 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/matroid.rs` | 43 | 53 | used `cloned` where `copied` could be used instead |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/matroid.rs` | 72 | 5 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 7 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 13 | 5 | you should consider adding a `Default` implementation for `TethAnalyzer` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 13 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 22 | 9 | unused import: `serde_json::Value` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 29 | 19 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 45 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 46 | 9 | unused import: `serde_json::Value` |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 55 | 48 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/teth.rs` | 68 | 23 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-glaf-core/src/types.rs` | 25 | 1 | more than 3 bools in a struct |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 237 | 30 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 329 | 25 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 330 | 29 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 387 | 46 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 470 | 30 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 529 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 563 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 578 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 589 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 596 | 30 | casting `u128` to `f64` causes a loss of precision (`u128` is 128 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-hashing-engine/src/main.rs` | 668 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 3 | 30 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 19 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 28 | 23 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 37 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 40 | 17 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 60 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 63 | 16 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 64 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 70 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 85 | 27 | called `map(<f>).unwrap_or_else(<g>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 140 | 43 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 145 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 153 | 39 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 235 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 239 | 52 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 259 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/chromadb_client.rs` | 270 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 6 | 7 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 12 | 54 | unused import: `RelationType` |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 130 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 138 | 13 | unused variable: `chromadb` |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 218 | 46 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 288 | 38 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 302 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 325 | 35 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/engine.rs` | 363 | 13 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 82 | 12 | field `config` is never read |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 90 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 110 | 49 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 127 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 131 | 13 | unused variable: `from_idx` |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 133 | 53 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 134 | 13 | unused variable: `to_idx` |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 136 | 53 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 187 | 17 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 203 | 26 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/graph.rs` | 252 | 32 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/lib.rs` | 4 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/lib.rs` | 49 | 61 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/nats_bridge.rs` | 33 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/nats_bridge.rs` | 46 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/nats_bridge.rs` | 56 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/nats_bridge.rs` | 67 | 50 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-leptose/src/nats_bridge.rs` | 79 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/lib.rs` | 9 | 69 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/metrics.rs` | 33 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/metrics.rs` | 80 | 9 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/metrics.rs` | 80 | 58 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/metrics.rs` | 89 | 9 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/metrics.rs` | 89 | 59 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 3 | 10 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 32 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 42 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 44 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 52 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 61 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 80 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 85 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 90 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/route_table.rs` | 95 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 7 | 5 | unused import: `dashmap::DashMap` |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 14 | 7 | unexpected `cfg` condition value: `foundation-core` |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 82 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 93 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 103 | 17 | unused variable: `elapsed` |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 103 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 127 | 11 | unexpected `cfg` condition value: `foundation-core` |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 158 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 163 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-neural-mux/src/router.rs` | 168 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/foundation_integration.rs` | 4 | 5 | unused import: `std::sync::Arc` |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/foundation_integration.rs` | 5 | 5 | unused import: `tokio::sync::RwLock` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/fso_analysis.rs` | 3 | 12 | unresolved import `crate::constants` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/fso_analysis.rs` | 4 | 12 | unresolved import `crate::ground_station` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/fso_analysis.rs` | 5 | 12 | unresolved import `crate::orbit` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/fso_analysis.rs` | 60 | 43 | can't call method `to_radians` on ambiguous numeric type `{float}` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/fso_analysis.rs` | 61 | 57 | can't call method `exp` on ambiguous numeric type `{float}` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 16 | 9 | unresolved import `sx9_foundation_orbital` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 37 | 9 | unresolved import `constellation` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 38 | 9 | unresolved import `coordinates` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 40 | 40 | the name `Result` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 40 | 17 | the name `OrbitalMechanicsError` is defined multiple times |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/lib.rs` | 40 | 17 | unused imports: `OrbitalMechanicsError` and `Result` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 43 | 22 | the name `OrbitalPropagator` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 43 | 41 | the name `PropagatorType` is defined multiple times |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/lib.rs` | 43 | 22 | unused imports: `OrbitalPropagator` and `PropagatorType` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 49 | 68 | the name `SatelliteSimulator` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 49 | 5 | the name `LiveSatellite` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 49 | 20 | the name `MeoEnvironmentalConditions` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 49 | 48 | the name `ObstructionWarning` is defined multiple times |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/lib.rs` | 49 | 5 | unused imports: `LiveSatellite`, `MeoEnvironmentalConditions`, `ObstructionWarning`, `SatelliteSimulator`, `SatelliteUnicodePacket`, and `SimulationStatistics` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 50 | 5 | the name `SatelliteUnicodePacket` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/lib.rs` | 50 | 29 | the name `SimulationStatistics` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/propagator.rs` | 3 | 12 | unresolved import `crate::constants` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/propagator.rs` | 5 | 12 | unresolved import `crate::orbit` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/propagator.rs` | 45 | 57 | no method named `num_seconds` found for struct `chrono::DateTime` in the current scope |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/propagator.rs` | 207 | 65 | no method named `num_seconds` found for struct `chrono::DateTime` in the current scope |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/propagator.rs` | 269 | 51 | no method named `num_seconds` found for struct `chrono::DateTime` in the current scope |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/propagator.rs` | 272 | 24 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | 11 | 29 | unused import: `sleep` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | 14 | 12 | unresolved import `crate::coordinates` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | 16 | 12 | unresolved import `crate::orbit` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/visibility.rs` | 3 | 12 | unresolved import `crate::constants` |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/visibility.rs` | 4 | 20 | unused import: `OrbitalMechanicsError` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/visibility.rs` | 5 | 12 | unresolved import `crate::ground_station` |
| sx9-lightning | ERROR | `crates/sx9-orbital-simulator/src/visibility.rs` | 6 | 12 | unresolved import `crate::orbit` |
| sx9-lightning | WARNING | `crates/sx9-orbital-simulator/src/visibility.rs` | 139 | 21 | redundant field names in struct initialization |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 16 | 27 | structure field `N1` should have a snake case name |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 16 | 38 | structure field `N2` should have a snake case name |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 37 | 25 | unused variable: `else_branch` |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 82 | 16 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 102 | 58 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 115 | 41 | called `map(<f>).unwrap_or(false)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 133 | 9 | binding's name is too similar to existing binding |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 153 | 36 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-phd-analyzer/src/main.rs` | 153 | 49 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 48 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 57 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 110 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 110 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 116 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 116 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 145 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 163 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 178 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 191 | 51 | casts from `u16` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 191 | 76 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 195 | 32 | casts from `u16` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 199 | 19 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 218 | 32 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 224 | 33 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 237 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 247 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 264 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 281 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 299 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 314 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 331 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 349 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 364 | 69 | casts from `u32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 365 | 54 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 367 | 71 | casting `usize` to `f64` causes a loss of precision on targets with 64-bit wide pointers (`usize` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 371 | 23 | casts from `u32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 378 | 22 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 391 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 397 | 29 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 397 | 41 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 408 | 28 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 426 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 438 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 442 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 446 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/agents.rs` | 450 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ann_daemon.rs` | 11 | 15 | unused imports: `info` and `warn` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ann_daemon.rs` | 49 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ann_daemon.rs` | 63 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ann_daemon.rs` | 70 | 26 | manual implementation of `midpoint` which can overflow |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 179 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 207 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 207 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 213 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 218 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 273 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 289 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 314 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 370 | 28 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 396 | 23 | casting `f32` to `u8` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 396 | 23 | casting `f32` to `u8` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 401 | 30 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 404 | 22 | casting `u64` to `u32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 420 | 25 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 424 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 449 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 454 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 459 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 464 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 469 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 473 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 474 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/atlas_integration.rs` | 479 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 9 | 34 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 12 | 33 | unused import: `EeiResponse` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 32 | 40 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 34 | 42 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 67 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 82 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 82 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 88 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 88 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 94 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 94 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 103 | 19 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 138 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 138 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 149 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 157 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 168 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 176 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 195 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 216 | 33 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 217 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 217 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 223 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 223 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/eei_bridge.rs` | 261 | 17 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/mod.rs` | 5 | 7 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 1 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 3 | 37 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 6 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 21 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 24 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 26 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 49 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 85 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 97 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 109 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 145 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 151 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 177 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 184 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 191 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 196 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 201 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 214 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 214 | 36 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 223 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 224 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 224 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 238 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 242 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 280 | 13 | wildcard matches only a single variant and will also match any future added variants |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 423 | 30 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/bridges/slot_bridge.rs` | 468 | 18 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 15 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 25 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 30 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 36 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 42 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/crystal.rs` | 47 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/apecs_layer.rs` | 9 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/apecs_layer.rs` | 19 | 12 | field `next_id` is never read |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/apecs_layer.rs` | 137 | 13 | this `if` statement can be collapsed |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/apecs_layer.rs` | 162 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 26 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 80 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 114 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 125 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 136 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 233 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 320 | 9 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/components.rs` | 324 | 1 | this `impl` can be derived |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 6 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 8 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 87 | 26 | casts from `u32` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 88 | 25 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 91 | 26 | boolean to int conversion using if |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 213 | 31 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 233 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 242 | 38 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 260 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/legion_layer.rs` | 274 | 44 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 5 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 38 | 13 | unnecessary `!=` operation |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 58 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 71 | 36 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 78 | 44 | casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 100 | 25 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 151 | 23 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 152 | 42 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 156 | 23 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 164 | 55 | casts from `u32` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 170 | 39 | casting `f64` to `f32` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 223 | 36 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 257 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/systems.rs` | 285 | 51 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 6 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 8 | 5 | unused import: `crate::ecs::systems::*` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 26 | 23 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 51 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 52 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 56 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 57 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ecs/world.rs` | 57 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/health.rs` | 22 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 23 | 7 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 66 | 12 | field `agents` is never read |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 95 | 26 | called `map(<f>).unwrap_or_else(<g>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 99 | 29 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 177 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 181 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 198 | 55 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 211 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/lib.rs` | 216 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/metrics.rs` | 25 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/monitor.rs` | 64 | 32 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/monitor.rs` | 84 | 17 | casts from `u8` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/monitor.rs` | 96 | 17 | casts from `u8` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 7 | 31 | unused import: `level_to_hd4` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 42 | 34 | possible intra-doc link using quotes instead of backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 44 | 31 | possible intra-doc link using quotes instead of backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 87 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 94 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 100 | 35 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 106 | 32 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 112 | 37 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 113 | 34 | this argument is passed by value, but not consumed in the function body |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 151 | 26 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 200 | 18 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 221 | 36 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 221 | 5 | it is more idiomatic to use `Option<&T>` instead of `&Option<T>` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 252 | 5 | it is more idiomatic to use `Option<&T>` instead of `&Option<T>` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 253 | 9 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 262 | 22 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 269 | 22 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 277 | 24 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 277 | 5 | it is more idiomatic to use `Option<&T>` instead of `&Option<T>` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 281 | 24 | casting `i64` to `u64` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 285 | 24 | casting `i64` to `u64` may lose the sign of the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 289 | 9 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 296 | 35 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 296 | 5 | it is more idiomatic to use `Option<&T>` instead of `&Option<T>` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/alert_parser.rs` | 318 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 56 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 58 | 37 | transmute used without annotations |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 65 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 70 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 75 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 126 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 142 | 9 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 147 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 159 | 9 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 176 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 187 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 223 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 320 | 59 | redundant closure |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 320 | 63 | calling `to_string` on `&&str` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 346 | 18 | use of `or_insert_with` to construct default value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 352 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 357 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 369 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 370 | 9 | called `map(<f>).unwrap_or(<a>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 377 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 378 | 9 | called `map(<f>).unwrap_or(<a>)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 385 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/mitre_map.rs` | 398 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 7 | 5 | unused import: `crate::ecs::components::OssecAlertComponent` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 61 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 61 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 67 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 67 | 5 | missing `#[must_use]` attribute on a method returning `Self` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 163 | 29 | unused `self` argument |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 192 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 197 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ossec/ossec_agent.rs` | 248 | 12 | called `map(<f>).unwrap_or(false)` on an `Option` value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/plasma_bus.rs` | 10 | 21 | unused import: `warn` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/plasma_bus.rs` | 12 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/plasma_bus.rs` | 26 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/plasma_bus.rs` | 63 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 6 | 7 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 11 | 59 | unused import: `stream::Stream` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 14 | 5 | unused import: `std::sync::Arc` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 25 | 5 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 93 | 20 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 193 | 32 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 333 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 338 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 342 | 13 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 343 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 352 | 22 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 373 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 396 | 21 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 489 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 489 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 496 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 504 | 5 | docs for function which may panic missing `# Panics` section |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 504 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/ring_bus.rs` | 511 | 27 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/sdt.rs` | 21 | 25 | casts from `f32` to `f64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/sdt.rs` | 31 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/sdt.rs` | 65 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/sdt.rs` | 88 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/server.rs` | 11 | 5 | unused import: `tower::ServiceBuilder` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/server.rs` | 17 | 12 | fields `health_endpoint`, `metrics_endpoint`, `plasma`, and `plasma_bus` are never read |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/server.rs` | 26 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 6 | 25 | unused import: `AnnConfig` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 18 | 8 | fields `success` and `timestamp` are never read |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 26 | 53 | item in documentation is missing backticks |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 54 | 13 | unused variable: `result_bytes` |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 109 | 20 | matching over `()` is more explicit |
| sx9-lightning | WARNING | `crates/sx9-plasma-defender/src/tool_handler.rs` | 137 | 22 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 6 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 7 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 42 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 53 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 80 | 24 | casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 97 | 28 | casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 116 | 56 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 117 | 32 | casting `u64` to `f32` causes a loss of precision (`u64` is 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 117 | 59 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 119 | 52 | long literal lacking separators |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 120 | 32 | casting `u64` to `f32` causes a loss of precision (`u64` is 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 120 | 59 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 168 | 26 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 172 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 177 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 182 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 187 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/ann_layer.rs` | 192 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/apecs_layer.rs` | 50 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/apecs_layer.rs` | 59 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/components.rs` | 29 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/components.rs` | 55 | 13 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/legion_layer.rs` | 5 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/legion_layer.rs` | 76 | 13 | unused variable: `world` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/lib.rs` | 21 | 5 | unused import: `anyhow::Result` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/lib.rs` | 31 | 5 | method `default` can be confused for the standard trait method `std::default::Default::default` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/lib.rs` | 31 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 5 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 12 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 14 | 13 | unused variable: `component` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 26 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 36 | 40 | casts from `u16` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 46 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 53 | 35 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 67 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 81 | 23 | casts from `u16` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 84 | 23 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 84 | 47 | casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 87 | 23 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 90 | 23 | casts from `u8` to `f32` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 98 | 55 | casts from `u32` to `u64` can be expressed infallibly using `From` |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 119 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 141 | 17 | these match arms have identical bodies |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 150 | 41 | used `cloned` where `copied` could be used instead |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 166 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 177 | 13 | assigning the result of `Clone::clone()` may be inefficient |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 182 | 26 | casting `u128` to `u64` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 194 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/systems.rs` | 215 | 26 | casting `usize` to `f32` causes a loss of precision (`usize` is 32 or 64 bits wide, but `f32`'s mantissa is only 23 bits wide) |
| sx9-lightning | WARNING | `crates/sx9-plasma-ecs/src/world.rs` | 6 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 14 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 14 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 17 | 1 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 80 | 16 | casting `u64` to `u16` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 80 | 16 | casting `u64` to `u16` may truncate the value |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 119 | 12 | matching over `()` is more explicit |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/handlers.rs` | 119 | 12 | matching over `()` is more explicit |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/main.rs` | 10 | 18 | unused imports: `Value` and `json` |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/main.rs` | 19 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 5 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 5 | 5 | usage of wildcard import |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 11 | 1 | methods `allocate_orbital_port`, `allocate_cdn_port`, `allocate_neural_port`, and `allocate_orbital_services` are never used |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 12 | 5 | you should consider adding a `Default` implementation for `PortManager` |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 12 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 66 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 66 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 73 | 12 | manual `!RangeInclusive::contains` implementation |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 73 | 12 | manual `!RangeInclusive::contains` implementation |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 113 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 113 | 5 | unused `async` for function with no await statements |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 121 | 51 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 121 | 51 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 125 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 134 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 138 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 142 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/port_manager.rs` | 146 | 5 | this method could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/types.rs` | 36 | 5 | name `CDN` contains a capitalized acronym |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/types.rs` | 37 | 5 | name `XSD` contains a capitalized acronym |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/types.rs` | 77 | 10 | variants `NoPortsAvailable`, `MirrorBlockError`, and `DeceptionError` are never constructed |
| sx9-lightning | WARNING | `crates/sx9-port-manager/src/types.rs` | 96 | 12 | struct `PortManagerState` is never constructed |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 16 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 30 | 13 | unexpected `cfg` condition value: `slsa-provenance` |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 34 | 13 | unexpected `cfg` condition value: `zero-trust` |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 38 | 13 | unexpected `cfg` condition value: `hermetic-builds` |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 60 | 13 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 73 | 9 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 134 | 14 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 148 | 1 | this function has too many lines (303/100) |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 288 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 359 | 13 | unnecessary hashes around raw string literal |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 479 | 14 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 505 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `crates/sx9-smart-crate-orchestrator/build.rs` | 512 | 15 | the borrowed expression implements the required traits |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/caldera_integration.rs` | 163 | 55 | type annotations needed |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/caldera_integration.rs` | 163 | 61 | type annotations needed |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/caldera_integration.rs` | 218 | 40 | no variant or associated item named `ValidationError` found for enum `EmulationError` in the current scope |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/cognitive_pipeline.rs` | 7 | 14 | unused imports: `DateTime` and `Utc` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/cognitive_pipeline.rs` | 9 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/cognitive_pipeline.rs` | 153 | 31 | cannot find type `EnduringEnhancement` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/cognitive_pipeline.rs` | 824 | 17 | cannot find type `EnduringEnhancement` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/cognitive_pipeline.rs` | 825 | 12 | failed to resolve: use of undeclared type `EnduringEnhancement` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 15 | 19 | unused import: `Mutex` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 18 | 13 | unused import: `ElitePersona` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 38 | 10 | conflicting implementations of trait `std::fmt::Debug` for type `data_consolidation::PtccDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 38 | 17 | conflicting implementations of trait `std::clone::Clone` for type `data_consolidation::PtccDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 38 | 24 | conflicting implementations of trait `_::_serde::Serialize` for type `data_consolidation::PtccDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 38 | 35 | conflicting implementations of trait `_::_serde::Deserialize<'_>` for type `data_consolidation::PtccDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 57 | 10 | conflicting implementations of trait `std::fmt::Debug` for type `data_consolidation::ScenarioDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 57 | 17 | conflicting implementations of trait `std::clone::Clone` for type `data_consolidation::ScenarioDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 57 | 24 | conflicting implementations of trait `_::_serde::Serialize` for type `data_consolidation::ScenarioDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 57 | 35 | conflicting implementations of trait `_::_serde::Deserialize<'_>` for type `data_consolidation::ScenarioDatabase` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 74 | 10 | conflicting implementations of trait `std::fmt::Debug` for type `data_consolidation::GlobalThreatChessboard` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 74 | 17 | conflicting implementations of trait `std::clone::Clone` for type `data_consolidation::GlobalThreatChessboard` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 74 | 24 | conflicting implementations of trait `_::_serde::Serialize` for type `data_consolidation::GlobalThreatChessboard` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 74 | 35 | conflicting implementations of trait `_::_serde::Deserialize<'_>` for type `data_consolidation::GlobalThreatChessboard` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 258 | 13 | redundant field names in struct initialization |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 432 | 55 | failed to resolve: use of unresolved module or unlinked crate `md5` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 469 | 9 | unused variable: `path` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 698 | 1 | the name `PtccDatabase` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 709 | 1 | the name `ScenarioDatabase` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/data_consolidation.rs` | 719 | 1 | the name `GlobalThreatChessboard` is defined multiple times |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/entropy_caldera_bridge.rs` | 7 | 5 | unresolved import `anyhow` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/entropy_caldera_bridge.rs` | 16 | 25 | unused import: `CalderaOperation` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/entropy_caldera_bridge.rs` | 74 | 29 | no variant or associated item named `ValidationError` found for enum `EmulationError` in the current scope |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/entropy_caldera_bridge.rs` | 140 | 13 | unused import: `sx9_foundation_core::hashing::murmur3_64` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/entropy_caldera_bridge.rs` | 249 | 21 | this `continue` expression is redundant |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 12 | 19 | unused import: `Mutex` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 15 | 13 | unused import: `ElitePersona` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 82 | 28 | cannot find type `ThreatMonitoring` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 84 | 29 | cannot find type `IndicatorAnalysis` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 86 | 30 | cannot find type `BehavioralAnalysis` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 88 | 35 | cannot find type `IntelligenceCorrelation` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 90 | 31 | cannot find type `DetectionValidation` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 112 | 32 | cannot find type `ThreatNeutralization` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 114 | 33 | cannot find type `CapabilityElimination` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 116 | 27 | cannot find type `SystemHardening` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 118 | 36 | cannot find type `ReconstitutionPrevention` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 120 | 35 | cannot find type `EffectivenessValidation` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 127 | 30 | cannot find type `OperationalControl` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 129 | 32 | cannot find type `DominanceMaintenance` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 131 | 32 | cannot find type `FollowupCoordination` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 133 | 26 | cannot find type `LessonsLearned` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 135 | 27 | cannot find type `StrategicImpact` in this scope |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 139 | 35 | the trait bound `ptcc_personas::ValidatedScenario: std::cmp::Eq` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 139 | 35 | the trait bound `ptcc_personas::ValidatedScenario: std::hash::Hash` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 154 | 27 | the trait bound `ptcc_personas::ValidatedScenario: std::cmp::Eq` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 154 | 27 | the trait bound `ptcc_personas::ValidatedScenario: std::hash::Hash` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 154 | 27 | the trait bound `ptcc_personas::ValidatedScenario: std::cmp::Eq` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 154 | 27 | the trait bound `ptcc_personas::ValidatedScenario: std::hash::Hash` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 348 | 29 | `?` couldn't convert the error to `EmulationError` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 457 | 29 | `?` couldn't convert the error to `EmulationError` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 480 | 9 | unused variable: `specialists` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 778 | 5 | the trait bound `HD4Phase: std::default::Default` is not satisfied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 817 | 10 | conflicting implementations of trait `std::fmt::Debug` for type `hd4_orchestrator::ThreatLevel` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 817 | 17 | conflicting implementations of trait `std::clone::Clone` for type `hd4_orchestrator::ThreatLevel` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 817 | 24 | conflicting implementations of trait `_::_serde::Serialize` for type `hd4_orchestrator::ThreatLevel` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/hd4_orchestrator.rs` | 817 | 35 | conflicting implementations of trait `_::_serde::Deserialize<'_>` for type `hd4_orchestrator::ThreatLevel` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/lib.rs` | 17 | 5 | unused import: `async_trait::async_trait` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/lib.rs` | 23 | 19 | unused import: `Mutex` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 26 | 5 | unresolved import `ctas7_lisp_reasoning_engine` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 29 | 5 | unresolved import `ctas7_streaming_inference_engine` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/lib.rs` | 50 | 9 | ambiguous glob re-exports |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/lib.rs` | 50 | 9 | ambiguous glob re-exports |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/lib.rs` | 51 | 9 | ambiguous glob re-exports |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 82 | 30 | `TacticalDecisionEngine` is ambiguous |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 274 | 66 | this function takes 2 arguments but 0 arguments were supplied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 281 | 58 | this function takes 1 argument but 0 arguments were supplied |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 283 | 39 | `TacticalDecisionEngine` is ambiguous |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 345 | 44 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 353 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 354 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 355 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 356 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 357 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 358 | 17 | mismatched types |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/lib.rs` | 359 | 17 | mismatched types |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/nyx_integration.rs` | 9 | 5 | unused import: `std::collections::HashMap` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/nyx_integration.rs` | 11 | 5 | unused import: `std::process::Command` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/nyx_integration.rs` | 13 | 19 | unused import: `Mutex` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/nyx_integration.rs` | 16 | 62 | unused import: `ValidatedScenario` |
| sx9-lightning | ERROR | `crates/sx9-threat-simulator/src/nyx_integration.rs` | 353 | 26 | failed to resolve: use of undeclared type `Utuc` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/scenario_engine.rs` | 16 | 19 | unused import: `Mutex` |
| sx9-lightning | WARNING | `crates/sx9-threat-simulator/src/scenario_engine.rs` | 20 | 35 | unused imports: `ElitePersona`, `HD4Phase`, `NyxPtcc`, `ThreatEmulationScenario`, and `ValidatedScenario` |
| sx9-lightning | WARNING | `rust/firefly-axum/src/main.rs` | 10 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `rust/firefly-axum/src/main.rs` | 19 | 5 | variables can be used directly in the `format!` string |
| sx9-lightning | WARNING | `rust/firefly-core/src/lib.rs` | 4 | 1 | this function could have a `#[must_use]` attribute |
| sx9-lightning | WARNING | `rust/firefly-core/src/lib.rs` | 9 | 1 | this function could have a `#[must_use]` attribute |
| manual-expert | Critical | `src/App.tsx` | 90 | 5 | Memory Leak: Event listeners attached to `document` in `onMouseDown` are never removed if the component unmounts while dragging. Requires `useEffect` cleanup. |
| manual-expert | UX | `src/App.tsx` | 128 | 7 | Blocking UX: Use of `alert()` halts the browser thread and provides poor user experience. Should use a Toast/Notification system. |
| manual-expert | Functional | `src/App.tsx` | 282 | 49 | Broken Feature: `AutoPersistIndicator` status is hardcoded to 'idle'. It effectively lies to the user about sync status. |
| qodo-ai | CRITICAL | `sx9-dev-forge-rn-migration/src/screens/PromptForgeScreen.tsx` | 1 | 0 | GOD_COMPONENT: File is 824 lines and mixes UI, Data Fetching, and State Management. Extract logic to custom hooks. |
