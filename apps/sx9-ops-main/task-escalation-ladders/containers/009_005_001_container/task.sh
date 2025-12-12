#!/bin/bash
# Container execution for: Robbery

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Robbery"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
