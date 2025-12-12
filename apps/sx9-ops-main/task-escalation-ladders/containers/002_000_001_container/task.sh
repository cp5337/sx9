#!/bin/bash
# Container execution for: Reconnaissance and Targeting

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Reconnaissance and Targeting"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
