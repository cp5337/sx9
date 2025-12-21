# Mission: Dev Forge Frontend Smoke Test

**Objective**: Verify the operational integrity of the Dev Forge Frontend and its bridge to the Rust backend.
**Operator**: Manual (Human)
**Frequency**: On every major UI change or release candidate.

## 1. Environment Check

- [ ] Ensure `sx9-dev-forge` is running via `npm run tauri dev`.
- [ ] Verify window title is "SX9 Dev Forge".
- [ ] Confirm no "Connection Refused" errors in the console.

## 2. Visual Inspection (The "Glass" Check)

- [ ] **Transparency**: Verify the window background is semi-transparent/blurred (if supported by OS).
- [ ] **Typography**: Confirm fonts are crisp and using the system sans-serif stack (Inter/San Francisco).
- [ ] **Layout**: resizing the window should adapt the layout (Responsive Design).

## 3. Bridge Connectivity (The "Pulse" Check)

- [ ] **Vault Indicator**:
  - Locate the "Vault" icon/status in the footer or header.
  - **Pass**: Green Icon / "Active" / "Unlocked".
  - **Fail**: Red Icon / "Locked" / "Error".
- [ ] **Clipboard Monitor**:
  - Copy text from _outside_ the app.
  - **Pass**: Text appears in the "Clipboard" or "Context" stream within < 1s.
  - **Fail**: No update or delayed update (> 3s).

## 4. Operational Capability

- [ ] **Command Bar**:
  - Press `Cmd+K` (or click input).
  - Type `/version`.
  - **Pass**: App displays current version string.
- [ ] **Mission Load**:
  - Navigate to "Missions" tab.
  - Select a sample mission (e.g., "Hello World").
  - **Pass**: Mission details load into the main view.

## 5. Failure Injection (Optional)

- [ ] Kill the backend process.
- [ ] Verify UI handles disconnection gracefully (e.g., "Reconnecting..." overlay).
