#!/bin/bash
# Container execution for: Social Eng. for Access

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Social Eng. for Access"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
