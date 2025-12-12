#!/bin/bash
# Container execution for: Controlling Propaganda

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Controlling Propaganda"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
