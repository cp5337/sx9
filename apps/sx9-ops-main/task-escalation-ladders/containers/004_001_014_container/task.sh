#!/bin/bash
# Container execution for: Virtual Card Fraud

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Virtual Card Fraud"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
