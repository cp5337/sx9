#!/bin/bash
# Container execution for: Maritime Smuggling

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Maritime Smuggling"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
