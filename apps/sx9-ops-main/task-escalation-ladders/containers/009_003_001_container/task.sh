#!/bin/bash
# Container execution for: Aggravated Assault

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Aggravated Assault"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
