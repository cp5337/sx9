#!/bin/bash
# Container execution for: Escape and Evasion Planning

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Escape and Evasion Planning"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
