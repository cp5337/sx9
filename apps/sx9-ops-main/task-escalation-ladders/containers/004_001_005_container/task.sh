#!/bin/bash
# Container execution for: Drug and Illicit Goods Trade

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Drug and Illicit Goods Trade"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
