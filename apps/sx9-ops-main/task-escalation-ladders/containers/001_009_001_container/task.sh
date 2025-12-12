#!/bin/bash
# Container execution for: Malware Deployment for Access

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Malware Deployment for Access"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
