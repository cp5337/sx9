#!/bin/bash
# Container execution for: Theft

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Theft"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
