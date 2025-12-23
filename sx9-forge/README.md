# CX9 Template UI - Vite + React Native Web + Tauri

**Cross-Platform Design System Framework**

A production-ready design system that works across web, iOS, Android, and desktop using Vite, React Native Web, and Tauri.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Design Tokens (Single Source of Truth)                    │
│  design-tokens.json                                         │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ├──────────────┬──────────────┬─────────────┐
                   ▼              ▼              ▼             ▼
         ┌──────────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
         │  Web (Vite)  │  │   iOS    │  │ Android  │  │  Desktop │
         │  RN Web      │  │  Swift   │  │  Kotlin  │  │  Tauri   │
         └──────────────┘  └──────────┘  └──────────┘  └──────────┘
```

## Key Features

- **Vite for blazing fast development** - HMR, native ESM, optimized builds
- **WebAssembly (WASM) support** - Ready for Cesium and other WASM modules
- **React Native Web** - Write once, run on web and native
- **Tauri desktop apps** - 10x smaller than Electron with Rust backend
- **Design tokens** - Single JSON source for all platforms
- **TypeScript** - Full type safety across the codebase
- **Dark mode only** - WCAG 2.1 AA compliant
- **Touch-optimized** - 44px minimum touch targets

## Quick Start

```bash
# Install dependencies
npm install

# Development (web with Vite)
npm run dev

# Development (desktop with Tauri)
npm run tauri:dev

# Build for production (web)
npm run build

# Build desktop apps (macOS, Windows, Linux)
npm run tauri:build
```

## WASM Support (Cesium, etc.)

The project includes `vite-plugin-wasm` and `vite-plugin-top-level-await` for full WebAssembly support:

```typescript
// Example: Using WASM modules
import init, { my_function } from './my-wasm-module.wasm';

async function loadWasm() {
  await init();
  const result = my_function();
}
```

For Cesium specifically:
1. Install: `npm install cesium`
2. Import: `import * as Cesium from 'cesium'`
3. WASM files are automatically handled by Vite plugins

## Project Structure

```
cx9-template-ui/
├── src/
│   ├── main.tsx                      # Entry point
│   ├── App.tsx                       # Main app component
│   ├── components/                   # React Native components
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   ├── Input.tsx
│   │   └── ...
│   ├── hooks/
│   │   └── useResponsive.ts
│   ├── tokens/
│   │   └── index.ts                  # TypeScript token exports
│   └── index.css                     # Global styles
├── src-tauri/                        # Tauri desktop app
│   ├── src/
│   │   └── main.rs                   # Rust backend
│   ├── Cargo.toml
│   └── tauri.conf.json
├── design-tokens.json                # Single source of truth
├── vite.config.ts                    # Vite configuration
├── tsconfig.json                     # TypeScript config
├── tailwind.config.json              # Tailwind CSS v4
└── index.html                        # HTML entry
```

## Design Tokens

All design decisions live in `design-tokens.json`:

```json
{
  "color": {
    "primary": { "value": "#3b82f6" },
    "background": {
      "primary": { "value": "#0f1419" }
    }
  },
  "spacing": { "md": { "value": 12 } },
  "typography": { 
    "fontSize": { "base": { "value": 14 } } 
  }
}
```

These tokens are automatically exported to:
- **CSS Variables** (web via Tailwind)
- **TypeScript** (`src/tokens/index.ts`)
- **Swift Extensions** (iOS, generated in CI/CD)
- **Kotlin Objects** (Android)

## Using Components

```tsx
import { Button } from './components/Button';
import { Card } from './components/Card';
import { Stack } from './components/Stack';

function MyScreen() {
  return (
    <Stack spacing="lg">
      <Card title="Welcome">
        <Text>This works on web, iOS, Android, and desktop!</Text>
      </Card>
      <Button 
        title="Click Me" 
        onPress={() => console.log('Pressed')} 
        variant="primary" 
      />
    </Stack>
  );
}
```

## Desktop App (Tauri)

Build native desktop applications with a Rust backend:

```bash
# Development mode (hot reload)
npm run tauri:dev

# Build for production
npm run tauri:build

# Outputs to: src-tauri/target/release/bundle/
# - macOS: .dmg
# - Windows: .msi
# - Linux: .deb, .AppImage
```

### Tauri Features Available

- File system access
- System information
- Native dialogs
- System tray
- Auto-updates
- Deep linking

See `src-tauri/src/main.rs` for custom Rust commands.

## CI/CD Pipeline

Every push to `main` triggers:

1. **Install** - Dependencies installation
2. **Type Check** - TypeScript validation
3. **Build** - Vite production build
4. **Export Tokens** - Generate Swift/Kotlin files
5. **Deploy** - Push to Vercel

See `.github/workflows/vite-deploy.yml` for configuration.

## Development

### Adding a New Component

1. Create component in `src/components/MyComponent.tsx`
2. Use React Native primitives (View, Text, etc.)
3. Style with tokens from `src/tokens`
4. Export from component file

```tsx
import { View, Text, StyleSheet } from 'react-native';
import { colors, spacing, typography } from '../tokens';

export function MyComponent() {
  return (
    <View style={styles.container}>
      <Text style={styles.text}>Hello World</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    padding: spacing.md,
    backgroundColor: colors.surface.primary,
  },
  text: {
    fontSize: typography.fontSize.base,
    color: colors.text.primary,
  },
});
```

### Environment Variables

Create `.env` file:

```bash
VITE_APP_NAME=CX9 Template UI
VITE_API_URL=https://api.example.com
```

Access in code:
```typescript
const apiUrl = import.meta.env.VITE_API_URL;
```

## Transpiling to Native

### iOS (Swift/SwiftUI)

Design tokens automatically generate Swift extensions during CI/CD. Component patterns map to SwiftUI:

| React Native | SwiftUI |
|--------------|---------|
| `<View>` | `VStack/HStack` |
| `<Text>` | `Text` |
| `<Pressable>` | `Button` |
| `<ScrollView>` | `ScrollView` |

### Android (Kotlin/Jetpack Compose)

Similar mapping for Android:

| React Native | Compose |
|--------------|---------|
| `<View>` | `Column/Row` |
| `<Text>` | `Text` |
| `<Pressable>` | `Button` |

## Performance

- Vite HMR: < 50ms updates
- Production build: Optimized chunks, tree-shaking
- Bundle size: < 200kb gzipped (without WASM modules)
- Tauri app: 10MB (vs 150MB+ with Electron)

## Browser Support

- Chrome/Edge: Last 2 versions
- Firefox: Last 2 versions
- Safari: 14+
- Mobile browsers: iOS 13+, Android 8+

## License

MIT

---

**Built with Vite + React Native Web + Tauri**

## Stack

- **Vite 6** - Build tool and dev server
- **React 18** - UI library
- **React Native Web** - Cross-platform components
- **Tauri 2** - Desktop app framework with Rust backend
- **TypeScript 5** - Type safety
- **Tailwind CSS 4** - Utility-first styling
- **WASM Support** - WebAssembly via Vite plugins

**Note:** This project uses **Vite, not Next.js**. All files are in the `src/` directory, with entry point at `src/main.tsx`.
