#!/bin/bash
# Container execution for: OSINT Collection

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: OSINT Collection"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
