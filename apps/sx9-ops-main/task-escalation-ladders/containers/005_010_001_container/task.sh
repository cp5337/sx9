#!/bin/bash
# Container execution for: Targeting Remote Work Systems

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Targeting Remote Work Systems"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
