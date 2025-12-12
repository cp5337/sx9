#!/bin/bash
# Container execution for: Corrupting Officials

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Corrupting Officials"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
