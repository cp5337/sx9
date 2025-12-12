#!/bin/bash
# Container execution for: Crowdfunding Fraud

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Crowdfunding Fraud"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
