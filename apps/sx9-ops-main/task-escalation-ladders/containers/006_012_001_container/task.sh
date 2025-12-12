#!/bin/bash
# Container execution for: Tunneling Border Tactics

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Tunneling Border Tactics"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
