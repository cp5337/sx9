#!/bin/bash
# Container execution for: Illicit Untaxed Goods Sales

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Illicit Untaxed Goods Sales"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
