#!/bin/bash
# Container execution for: Deploying Volatile Malware

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Deploying Volatile Malware"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
