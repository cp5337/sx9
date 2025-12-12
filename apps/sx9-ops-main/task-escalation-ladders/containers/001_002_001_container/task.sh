#!/bin/bash
# Container execution for: Social Eng. Pretexting

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Social Eng. Pretexting"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
