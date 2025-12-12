#!/bin/bash
# Container execution for: RF Jamming for Disruption

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: RF Jamming for Disruption"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
