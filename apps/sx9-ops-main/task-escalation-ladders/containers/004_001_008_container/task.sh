#!/bin/bash
# Container execution for: Cybercrime Revenue

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Cybercrime Revenue"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
