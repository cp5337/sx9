#!/bin/bash
# Container execution for: Precious Metals Laundering

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Precious Metals Laundering"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
