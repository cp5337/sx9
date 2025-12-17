#!/bin/bash
# Scrape All Kali Tools + SPIRES Ontology Generation
# Uses the comprehensive kali_tools_scraper.py and SPIRES extractor

set -e

echo "🔧 Scraping All Kali Tools + SPIRES Ontology"
echo "============================================="
echo ""

cd "$(dirname "$0")"

# Check if scraper exists
if [ ! -f "node-interview-generator/kali_tools_scraper.py" ]; then
    echo "❌ kali_tools_scraper.py not found"
    exit 1
fi

# Check dependencies
if ! python3 -c "import requests, bs4" 2>/dev/null; then
    echo "📦 Installing dependencies..."
    pip3 install requests beautifulsoup4
fi

# Check for SPIRES (optional)
SPIRES_AVAILABLE=false
if python3 -c "import ontogpt" 2>/dev/null; then
    SPIRES_AVAILABLE=true
    echo "✅ SPIRES (ontogpt) available - will generate ontology"
else
    echo "⚠️  SPIRES (ontogpt) not installed - skipping ontology generation"
    echo "   Install with: pip install ontogpt"
fi

echo ""
echo "💰 COST: FREE (local processing only)"
echo "   • No cloud costs"
echo "   • No API costs"
echo "   • Just local CPU/network time"
echo ""
echo "🚀 Starting comprehensive Kali tools scrape..."
echo "   This will scrape all tools from kali.org/tools"
echo "   Estimated time: 30-60 minutes (with rate limiting)"
echo ""

# Run the scraper (no limit = scrape all)
cd node-interview-generator
python3 kali_tools_scraper.py
cd ..

# Copy to threat_content for SPIRES processing
echo ""
echo "📦 Copying to threat_content for SPIRES processing..."
mkdir -p node-interview-generator/output/threat_content
if [ -f "node-interview-generator/output/kali_tools/kali_tools_complete.json" ]; then
    cp node-interview-generator/output/kali_tools/kali_tools_complete.json \
       node-interview-generator/output/threat_content/kali_tools_inventory.json
    echo "✅ Copied to threat_content/kali_tools_inventory.json"
fi

# Run SPIRES ontology extraction if available
if [ "$SPIRES_AVAILABLE" = true ]; then
    echo ""
    echo "🧠 Running SPIRES ontology extraction for Kali tools..."
    if [ -f "spires_ontology_extractor.py" ]; then
        python3 spires_ontology_extractor.py --threats 2>&1 | grep -i "kali\|tool\|ontology" || true
        echo "✅ SPIRES ontology generated"
        echo "   Output: output/ontology/"
    else
        echo "⚠️  spires_ontology_extractor.py not found in current directory"
    fi
fi

echo ""
echo "✅ Complete!"
echo ""
echo "📊 Output files:"
echo "   • node-interview-generator/output/kali_tools/kali_tools_complete.json - Full inventory"
echo "   • node-interview-generator/output/kali_tools/kali_categories.json - Categories index"
echo "   • node-interview-generator/output/kali_tools/kali_tool_list.txt - Simple tool list"
echo "   • node-interview-generator/output/kali_tools/kali_git_repos.txt - Git repos"
if [ "$SPIRES_AVAILABLE" = true ]; then
    echo "   • output/ontology/ - SPIRES ontology (JSON, Cypher, LinkML)"
fi
echo ""
echo "📥 To view results:"
echo "   cat node-interview-generator/output/kali_tools/kali_tools_complete.json | jq '.tool_count'"
echo "   cat node-interview-generator/output/kali_tools/kali_tool_list.txt | wc -l"

