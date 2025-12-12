#!/bin/bash
# Container execution for: Explosives Target Assessment

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Explosives Target Assessment"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
