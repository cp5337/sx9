#!/bin/bash
# Container execution for: Retail Front Laundering

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Retail Front Laundering"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
