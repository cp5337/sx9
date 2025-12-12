#!/bin/bash
# Container execution for: UAV Cyber Payload Delivery

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: UAV Cyber Payload Delivery"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
