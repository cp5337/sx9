#!/bin/bash
# Container execution for: Islamic Terror Cell Activation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Islamic Terror Cell Activation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
