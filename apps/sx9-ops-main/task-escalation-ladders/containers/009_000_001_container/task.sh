#!/bin/bash
# Container execution for: Major Crimes

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Major Crimes"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
