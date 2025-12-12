#!/bin/bash
# Container execution for: Insider Threats for Access

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Insider Threats for Access"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
