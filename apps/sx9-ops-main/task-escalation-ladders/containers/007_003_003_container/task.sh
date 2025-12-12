#!/bin/bash
# Container execution for: Emergency Services Disruption

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Emergency Services Disruption"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
