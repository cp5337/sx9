#!/bin/bash
# Container execution for: Radicalizing Students

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Radicalizing Students"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
