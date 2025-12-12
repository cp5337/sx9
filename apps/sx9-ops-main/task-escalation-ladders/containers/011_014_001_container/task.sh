#!/bin/bash
# Container execution for: Border Crossing Counter-Surveillance

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Border Crossing Counter-Surveillance"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
