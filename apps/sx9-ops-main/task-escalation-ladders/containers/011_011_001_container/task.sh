#!/bin/bash
# Container execution for: Border Tunnel Operations

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Border Tunnel Operations"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
