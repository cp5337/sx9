#!/bin/bash
# Container execution for: Target Vulnerability Assessment

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Target Vulnerability Assessment"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
