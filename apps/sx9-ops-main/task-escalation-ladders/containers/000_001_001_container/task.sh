#!/bin/bash
# Container execution for: Planning Initiation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Planning Initiation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
