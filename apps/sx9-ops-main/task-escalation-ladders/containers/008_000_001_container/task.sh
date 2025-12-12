#!/bin/bash
# Container execution for: National Asset Assault

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: National Asset Assault"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
