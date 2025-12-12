#!/bin/bash
# Container execution for: Self Identification

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Self Identification"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
