# CX9 Template UI - Tauri Desktop Framework

Cross-platform desktop application built with Tauri, Next.js, and React Native Web.

## Features

- **10x Smaller than Electron** (~10-15 MB vs 150+ MB)
- **Native Performance** - Rust backend
- **Cross-Platform** - macOS, Windows, Linux
- **Secure** - Sandboxed execution
- **Hunt-Inspired Dark Theme**
- **50+ shadcn/ui Components**
- **Type-Safe** - TypeScript + Rust

## Quick Start

### Prerequisites

- Node.js 20+
- Rust (install from https://rustup.rs)

### Development

```bash
# Install dependencies
npm install

# Run web dev (browser preview)
npm run dev

# Run Tauri dev (desktop app)
npm run tauri:dev
```

### Build for Production

```bash
# Build desktop app for current platform
npm run tauri:build
```

### Build Outputs

- **macOS**: `src-tauri/target/release/bundle/macos/CX9 Template UI.app`
- **Windows**: `src-tauri/target/release/bundle/msi/CX9 Template UI_1.0.0_x64.msi`
- **Linux**: `src-tauri/target/release/bundle/appimage/cx9-template-ui_1.0.0_amd64.AppImage`

## Architecture

```
cx9-template-ui/
├── src-tauri/              # Rust backend
│   ├── src/main.rs        # Tauri commands
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri config
├── src/
│   ├── components/        # React Native components
│   ├── lib/tauri.ts      # CX9 API wrapper
│   └── tokens/           # Design tokens
├── app/                   # Next.js pages
└── design-tokens.json     # Hunt-inspired colors
```

## Tauri Commands

### Save to Disk

```typescript
import { cx9API } from '@/src/lib/tauri';

const path = await cx9API.saveToDisk('content', 'filename.txt');
console.log('Saved to:', path);
```

### Get System Info

```typescript
const info = await cx9API.getSystemInfo();
console.log('Platform:', info.platform);
console.log('Architecture:', info.arch);
```

### Check if Running in Tauri

```typescript
if (cx9API.isTauri()) {
  // Running in desktop app
} else {
  // Running in browser
}
```

## Automated Releases

Push a tag to trigger cross-platform builds:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions will build for macOS, Windows, and Linux automatically.

## Design System

All components use the Hunt-inspired design tokens with professional dark theme. See `design-tokens.json` for the complete token system.

## License

MIT
