#!/bin/bash
# Container execution for: Mobile Device Compromise

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Mobile Device Compromise"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
