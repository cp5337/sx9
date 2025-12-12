#!/bin/bash
# Container execution for: Witness Questioning

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Witness Questioning"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
