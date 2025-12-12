#!/bin/bash
# Container execution for: Border Corruption

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Border Corruption"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
