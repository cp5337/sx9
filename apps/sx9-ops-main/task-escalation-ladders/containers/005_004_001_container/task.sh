#!/bin/bash
# Container execution for: Cloud Reconnaissance

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Cloud Reconnaissance"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
