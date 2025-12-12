#!/bin/bash
# Container execution for: Escaping Operational Areas

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Escaping Operational Areas"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
