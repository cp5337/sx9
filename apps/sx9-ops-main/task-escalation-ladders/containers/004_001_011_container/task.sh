#!/bin/bash
# Container execution for: Structuring Transactions

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Structuring Transactions"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
