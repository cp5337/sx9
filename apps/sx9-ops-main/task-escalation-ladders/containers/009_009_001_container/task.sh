#!/bin/bash
# Container execution for: Arson

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Arson"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
