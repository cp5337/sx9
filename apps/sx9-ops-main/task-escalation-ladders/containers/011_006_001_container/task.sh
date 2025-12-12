#!/bin/bash
# Container execution for: Arms Trafficking

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Arms Trafficking"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
