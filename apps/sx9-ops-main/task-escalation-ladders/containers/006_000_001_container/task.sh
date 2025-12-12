#!/bin/bash
# Container execution for: Infiltration and Access

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Infiltration and Access"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
