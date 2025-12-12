#!/bin/bash
# Container execution for: Exploiting Legal Systems

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Exploiting Legal Systems"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
