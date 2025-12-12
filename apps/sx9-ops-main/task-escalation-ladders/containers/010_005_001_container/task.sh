#!/bin/bash
# Container execution for: Police Counter-Surveillance

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Police Counter-Surveillance"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
