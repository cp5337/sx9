#!/bin/bash
# Container execution for: Network Trust Exploitation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Network Trust Exploitation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
