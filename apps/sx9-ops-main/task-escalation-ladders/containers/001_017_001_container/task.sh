#!/bin/bash
# Container execution for: Multi-Actor Coordination

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Multi-Actor Coordination"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
