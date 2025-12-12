#!/bin/bash

# Exit on error
set -e

# Load environment variables
source .env.production

# Install dependencies
npm ci

# Build the application
npm run build

# Deploy to Netlify (if not using GitHub Actions)
if [ -z "$CI" ]; then
  netlify deploy --prod
fi

# Set up monitoring
curl -sL https://sentry.io/get-cli/ | bash
sentry-cli releases new "$(git rev-parse HEAD)"
sentry-cli releases set-commits "$(git rev-parse HEAD)" --auto

# Notify team
curl -X POST -H 'Content-type: application/json' \
  --data "{\"text\":\"New CTAS deployment completed! Commit: $(git rev-parse --short HEAD)\"}" \
  "$SLACK_WEBHOOK_URL"