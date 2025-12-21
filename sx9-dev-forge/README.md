# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Shipping & Building

### Manual Build

To build the application locally:

```bash
npm run tauri build
```

The output will be in `src-tauri/target/release/bundle/`.

### Commissioned Release

This application is commissioned with a CI/CD pipeline. To ship a new version:

```bash
# From repository root
./scripts/ship-dev-forge.sh patch  # or minor/major
```

This will:

1. Bump the version in `package.json`
2. Create a git tag
3. Push to GitHub
4. Trigger the `.github/workflows/release-dev-forge.yml` action to build and release the binary.
