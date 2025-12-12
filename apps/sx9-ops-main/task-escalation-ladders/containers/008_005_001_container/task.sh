#!/bin/bash
# Container execution for: Redirecting Attribution

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Redirecting Attribution"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
