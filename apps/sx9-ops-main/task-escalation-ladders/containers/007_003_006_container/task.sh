#!/bin/bash
# Container execution for: Transport Supply Chain Attacks

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Transport Supply Chain Attacks"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
