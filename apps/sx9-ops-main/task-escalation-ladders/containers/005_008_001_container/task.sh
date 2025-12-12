#!/bin/bash
# Container execution for: Dark Web Intel Gathering

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Dark Web Intel Gathering"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
