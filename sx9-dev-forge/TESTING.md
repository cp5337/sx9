# Dev Forge Testing & Verification

This document outlines the testing strategy and routine for the SX9 Dev Forge (`sx9-dev-forge`).

## 1. Backend Integration Tests

We use Rust integration tests to verify the core functionality of the backend systems (Vault, Atomic Clipboard, File Index) without needing to run the full Tauri application.

### Location

Tests are located in `src-tauri/tests/integration_test.rs`.

### Running Tests

To run the integration test suite:

```bash
cd src-tauri
cargo test --test integration_test
```

### Coverage

The suite covers:

- **KeyVault**: Verifies initialization, key retrieval, and stats (via `KeyVaultExt`).
- **Atomic Clipboard**: Tests initialization with a custom path, read/write/append operations, and format handling.
- **File Index**: Verifies initialization in the current working directory, file scanning, and extension tracking.

## 2. Frontend Smoke Test (Manual)

Since the frontend relies on the Tauri IPC bridge, full automation requires a complex driver. We use a manual "Smoke Test" mission to verify the UI and bridge.

### Frontend Smoke Test Mission

1.  **Launch App**: Run `npm run tauri dev`.
2.  **Verify UI Loads**: Ensure the main "Prompt HUD" interface appears.
3.  **Check Status Indicators**:
    - **Vault**: Should show "Unlock" or "Active" (green lock icon).
    - **Clipboard**: Should reflect recent clipboard activity.
4.  **Execute a Command**:
    - Type `/help` in the command bar.
    - Verify the system responds with available commands.
5.  **Test "Atomic Loop"**:
    - Copy text from an external app.
    - Verify it appears in the "Context" pane of Dev Forge.
    - Edit the text in Dev Forge.
    - Verify the changes are reflected in the external clipboard (if bidirectional sync is active).

## 3. Continuous Integration

- Run `cargo test` in `src-tauri` on every PR.
- Run `npm run build` in `sx9-dev-forge` to verify frontend compilation.
