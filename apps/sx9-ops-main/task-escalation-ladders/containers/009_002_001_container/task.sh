#!/bin/bash
# Container execution for: Sexual Violence

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Sexual Violence"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
