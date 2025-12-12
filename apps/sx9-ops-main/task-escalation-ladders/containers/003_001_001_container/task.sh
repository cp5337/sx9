#!/bin/bash
# Container execution for: Encrypted Communication

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Encrypted Communication"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
