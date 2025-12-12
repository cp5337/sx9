#!/bin/bash
# Container execution for: Digital Identity Harvesting

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Digital Identity Harvesting"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
