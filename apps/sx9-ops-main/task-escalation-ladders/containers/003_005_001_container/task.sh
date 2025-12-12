#!/bin/bash
# Container execution for: Covert Storage and Transport

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Covert Storage and Transport"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
