#!/bin/bash
# Container execution for: Using Safe Houses

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Using Safe Houses"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
