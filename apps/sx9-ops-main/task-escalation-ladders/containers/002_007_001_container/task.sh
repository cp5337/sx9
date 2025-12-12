#!/bin/bash
# Container execution for: UAV Target Acquisition

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: UAV Target Acquisition"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
