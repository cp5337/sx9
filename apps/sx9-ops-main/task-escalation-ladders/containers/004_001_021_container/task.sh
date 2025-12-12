#!/bin/bash
# Container execution for: Middle East Terror Funding

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Middle East Terror Funding"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
