#!/bin/bash
# Container execution for: Bulk Cash Smuggling

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Bulk Cash Smuggling"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
