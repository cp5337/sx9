#!/bin/bash
# Container execution for: Energy Utilities Compromise

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Energy Utilities Compromise"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
