#!/bin/bash
# Container execution for: Execution of Cyber Physical Attacks

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Execution of Cyber Physical Attacks"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
