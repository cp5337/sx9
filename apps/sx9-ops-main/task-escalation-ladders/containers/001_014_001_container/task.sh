#!/bin/bash
# Container execution for: OPSEC Countermeasures

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: OPSEC Countermeasures"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
