#!/bin/bash
# Container execution for: Targeting Critical Infrastructure

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Targeting Critical Infrastructure"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
