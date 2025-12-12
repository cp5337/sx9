#!/bin/bash
# Container execution for: Border Security Targeting

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Border Security Targeting"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
