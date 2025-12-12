#!/bin/bash
# Container execution for: Obtain Operational Resources

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Obtain Operational Resources"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
