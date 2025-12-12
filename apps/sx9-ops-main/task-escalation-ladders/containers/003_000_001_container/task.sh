#!/bin/bash
# Container execution for: Operational Security

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Operational Security"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
