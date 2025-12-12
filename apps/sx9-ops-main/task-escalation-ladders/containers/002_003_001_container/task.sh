#!/bin/bash
# Container execution for: Insider Threat Development

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Insider Threat Development"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
