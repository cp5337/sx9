#!/bin/bash
# Container execution for: Homicide

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Homicide"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
