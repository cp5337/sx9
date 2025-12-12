#!/bin/bash
# Container execution for: Airspace Exploitation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Airspace Exploitation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
