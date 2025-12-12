#!/bin/bash
# Container execution for: Attack Method Selection

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Attack Method Selection"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
