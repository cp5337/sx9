#!/bin/bash
# Container execution for: Supply Chain Infiltration

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Supply Chain Infiltration"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
