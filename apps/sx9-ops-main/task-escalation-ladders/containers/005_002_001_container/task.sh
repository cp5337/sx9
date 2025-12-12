#!/bin/bash
# Container execution for: Physical Surveillance

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Physical Surveillance"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
