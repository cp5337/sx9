#!/bin/bash
# Container execution for: Hostage Crisis Initiation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Hostage Crisis Initiation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
