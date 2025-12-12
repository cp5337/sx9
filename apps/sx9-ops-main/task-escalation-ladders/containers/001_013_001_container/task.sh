#!/bin/bash
# Container execution for: Supply Chain Vulnerability ID

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Supply Chain Vulnerability ID"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
