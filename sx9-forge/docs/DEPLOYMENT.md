# Deployment Guide

## Development Setup

```bash
# Install dependencies
npm install

# Run web development server (Vite)
npm run dev
# Access at http://localhost:5173

# Run desktop development (Tauri + Vite)
npm run tauri:dev
# Opens native desktop window
```

## Production Build

### Web Application

```bash
# Build optimized web bundle
npm run build

# Preview production build locally
npm run preview
```

Output: `dist/` directory

### Desktop Applications

```bash
# Build native apps for current platform
npm run tauri:build
```

Outputs:
- **macOS**: `src-tauri/target/release/bundle/macos/CX9 Template UI.app`
- **Windows**: `src-tauri/target/release/bundle/msi/cx9-template-ui_1.0.0_x64.msi`
- **Linux**: `src-tauri/target/release/bundle/deb/cx9-template-ui_1.0.0_amd64.deb`

### Cross-Platform Builds

```bash
# Build for all platforms (requires runners)
npm run tauri build -- --target universal-apple-darwin  # macOS universal
npm run tauri build -- --target x86_64-pc-windows-msvc  # Windows
npm run tauri build -- --target x86_64-unknown-linux-gnu # Linux
```

## Environment Variables

Create `.env` file:

```bash
# App Configuration
VITE_APP_NAME=CX9 Template UI
VITE_APP_VERSION=1.0.0

# API Endpoints
VITE_LINEAR_API_KEY=your_linear_api_key
VITE_SLACK_WEBHOOK_URL=your_slack_webhook

# Supabase (if using database features)
VITE_SUPABASE_URL=https://your-project.supabase.co
VITE_SUPABASE_ANON_KEY=your_anon_key

# NATS (for intelligence system)
VITE_NATS_SERVER=nats://localhost:4222
```

**Never commit `.env` to version control!**

## CI/CD with GitHub Actions

The project includes `.github/workflows/vite-deploy.yml` for automated deployment:

```yaml
name: Deploy to Vercel
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npm run build
      - run: vercel --prod
```

### GitHub Secrets Required

Add these in repository settings:

- `VERCEL_TOKEN` - Vercel deployment token
- `VERCEL_ORG_ID` - Organization ID
- `VERCEL_PROJECT_ID` - Project ID

## Vercel Deployment

### Option 1: GitHub Integration

1. Connect repository to Vercel
2. Vercel auto-deploys on push to `main`
3. Preview deployments on pull requests

### Option 2: Manual Deployment

```bash
# Install Vercel CLI
npm install -g vercel

# Login
vercel login

# Deploy
vercel --prod
```

## Docker Deployment (Optional)

Create `Dockerfile`:

```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

Build and run:

```bash
docker build -t cx9-template-ui .
docker run -p 8080:80 cx9-template-ui
```

## Tauri Desktop Distribution

### Code Signing (macOS)

```bash
# Set environment variables
export APPLE_CERTIFICATE="path/to/cert.p12"
export APPLE_CERTIFICATE_PASSWORD="your_password"
export APPLE_ID="your@apple.id"
export APPLE_PASSWORD="app-specific-password"

# Build with signing
npm run tauri:build
```

### Windows Signing

```bash
# Set environment variables
export WINDOWS_CERTIFICATE="path/to/cert.pfx"
export WINDOWS_CERTIFICATE_PASSWORD="your_password"

# Build with signing
npm run tauri:build
```

### Auto-Updates

Configure in `src-tauri/tauri.conf.json`:

```json
{
  "updater": {
    "active": true,
    "endpoints": [
      "https://your-update-server.com/releases/{{target}}/{{current_version}}"
    ],
    "dialog": true
  }
}
```

## Performance Optimization

### Bundle Analysis

```bash
# Analyze bundle size
npm run build -- --report
```

### Lighthouse Audit

```bash
# Run Lighthouse CI
npm install -g @lhci/cli
lhci autorun --upload.target=temporary-public-storage
```

Target scores:
- Performance: > 90
- Accessibility: > 95
- Best Practices: > 90
- SEO: > 90

## Monitoring

### Sentry Integration

```bash
npm install @sentry/react
```

```typescript
import * as Sentry from '@sentry/react';

Sentry.init({
  dsn: 'your_sentry_dsn',
  environment: import.meta.env.MODE,
  tracesSampleRate: 1.0,
});
```

### Analytics

Add to `index.html`:

```html
<script defer data-domain="yourdomain.com" src="https://plausible.io/js/script.js"></script>
```

## Troubleshooting

### Build Failures

```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json dist
npm install
npm run build
```

### Tauri Build Issues

```bash
# Update Rust
rustup update

# Clear Tauri cache
rm -rf src-tauri/target
npm run tauri:build
```

### WASM Module Issues

Ensure `vite-plugin-wasm` is properly configured in `vite.config.ts`:

```typescript
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default {
  plugins: [wasm(), topLevelAwait()],
};
```

---

**For production deployment, always test thoroughly in staging environment first.**
