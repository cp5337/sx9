#!/bin/bash
set -e

# SX9 Dev Forge Shipping Script

VERSION_TYPE=$1

if [ -z "$VERSION_TYPE" ]; then
    echo "Usage: ./ship-dev-forge.sh [patch|minor|major]"
    exit 1
fi

echo "🚀 preparing to ship sx9-dev-forge..."

# 1. Navigate to app directory
cd sx9-dev-forge

# 2. Bump version (requires npm version to be working, or we just do it manually)
# Using npm version to bump package.json
echo "📦 Bumping version ($VERSION_TYPE)..."
npm version $VERSION_TYPE

# Get new version
NEW_VERSION=$(node -p "require('./package.json').version")
echo "✨ New version: $NEW_VERSION"

# 3. Commit and Tag
cd ..
git add sx9-dev-forge/package.json sx9-dev-forge/package-lock.json 2>/dev/null || true
git commit -m "chore(release): bump sx9-dev-forge to $NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo "📡 Pushing to origin..."
git push origin main
git push origin "v$NEW_VERSION"

echo "✅ Shipped! Check GitHub Actions for build status."
