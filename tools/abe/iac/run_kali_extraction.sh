#!/bin/bash
# Run Kali CLI Extraction with Dual-Gemini
# Cost: $0.0005 for 335 tools

set -e

echo "🚀 Starting Kali CLI Extraction..."
echo "   Tools: 335"
echo "   Strategy: 3-tier (regex → GitLab → dual-Gemini)"
echo "   Cost: ~\$0.0005"
echo ""

# Set environment
export GEMINI_API_KEY="AIzaSyA9wvdjofLJjzLpnEgfsoSyKgU0OhSnCeM"

# Navigate to IAC directory
cd /Users/cp5337/Developer/sx9/tools/abe/iac

# Install dependencies if needed
echo "📦 Checking dependencies..."
pip install -q requests beautifulsoup4 google-generativeai google-cloud-aiplatform 2>&1 | grep -v "already satisfied" || true

echo ""
echo "🤖 Running extraction..."
echo ""

# Run extractor
python3 kali_cli_dual_llm_extractor.py 2>&1 | tee kali_extraction_full.log

echo ""
echo "✅ Extraction complete!"
echo "📁 Output: kali_tools_with_commands.json"
echo "📊 Log: kali_extraction_full.log"
