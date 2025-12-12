#!/bin/bash
# Container execution for: Unwitting Accomplice Manipulation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Unwitting Accomplice Manipulation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
