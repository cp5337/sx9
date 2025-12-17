#!/usr/bin/env bash
# CTAS7 Local QA Runner (No Docker Required)
set -euo pipefail

echo "🔬 CTAS7 Local QA System"
echo "========================="
echo ""

# Create results directory
mkdir -p qa-results-local

CRATES=(
    "ctas7-foundation-core"
    "ctas7-foundation-interface"
    "ctas7-qa-analyzer"
    "ctas7-groundstations-hft"
    "ctas7-demo-platform"
    "ctas7-repoagent"
    "ctas7-linear-cli"
)

echo "📊 Running Clone Analysis on 7 crates..."
echo ""

for crate in "${CRATES[@]}"; do
    echo "📦 Analyzing: $crate"
    ./target/release/clone_checker "$crate" > "qa-results-local/${crate}-clone.txt" 2>&1 || true
    echo "   ✅ Report saved: qa-results-local/${crate}-clone.txt"
done

echo ""
echo "📋 Generating Summary..."

cat > qa-results-local/SUMMARY.md << 'EOF'
# CTAS7 QA Results Summary

**Generated:** $(date)
**Method:** Local clone analysis

## Results by Crate

EOF

for crate in "${CRATES[@]}"; do
    if [ -f "qa-results-local/${crate}-clone.txt" ]; then
        clones=$(grep "Total Clones:" "qa-results-local/${crate}-clone.txt" | awk '{print $4}' || echo "N/A")
        status=$(grep "PASSED\|WARNING\|ERROR\|CRITICAL" "qa-results-local/${crate}-clone.txt" | head -1 || echo "Unknown")
        
        echo "### $crate" >> qa-results-local/SUMMARY.md
        echo "- **Clones:** $clones" >> qa-results-local/SUMMARY.md
        echo "- **Status:** $status" >> qa-results-local/SUMMARY.md
        echo "" >> qa-results-local/SUMMARY.md
    fi
done

echo ""
echo "✅ QA Complete! Results saved in ./qa-results-local/"
echo ""
echo "📄 Files created:"
ls -lh qa-results-local/
echo ""
echo "📊 Quick Summary:"
cat qa-results-local/SUMMARY.md

