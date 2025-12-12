#!/bin/bash
# Container execution for: Dark Web Transactions

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Dark Web Transactions"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
