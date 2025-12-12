#!/bin/bash
# Container execution for: Compromising Officials

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Compromising Officials"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
