#!/bin/bash
# Container execution for: State-Sponsored Financing

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: State-Sponsored Financing"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
