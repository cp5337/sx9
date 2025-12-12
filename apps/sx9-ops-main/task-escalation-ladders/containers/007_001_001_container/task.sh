#!/bin/bash
# Container execution for: Disabling Security Systems

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Disabling Security Systems"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
