#!/bin/bash
# Container execution for: Govt Defense Operations

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Govt Defense Operations"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
