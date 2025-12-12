#!/bin/bash
# Container execution for: Comms Tech Sabotage

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Comms Tech Sabotage"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
