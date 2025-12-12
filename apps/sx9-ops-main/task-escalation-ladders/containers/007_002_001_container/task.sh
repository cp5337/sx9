#!/bin/bash
# Container execution for: Diversionary Attacks

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Diversionary Attacks"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
