#!/bin/bash
# Container execution for: Money Mules

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Money Mules"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
