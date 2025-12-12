#!/bin/bash
# Container execution for: Cyber Exploitation for Access

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Cyber Exploitation for Access"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
