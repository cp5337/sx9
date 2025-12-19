#!/bin/bash
# Batch convert all RFCs to LaTeX for copyright filing

OUTPUT_DIR="02-sx9-latex/copyright-filing"
mkdir -p "$OUTPUT_DIR"

echo "🔒 Converting RFCs for Copyright Filing"
echo "========================================"

# Find all non-deprecated RFCs
count=0
find 01-rfc -name "RFC-*.md" | grep -v DEPRECATED | sort | while read rfc_file; do
    rfc_num=$(basename "$rfc_file" | sed 's/RFC-\([0-9A-Z]*\).*/\1/')
    
    echo "[$((++count))] Converting RFC-$rfc_num..."
    python3 sx9-conda/rfc_latex_generator.py \
        --rfc "$rfc_num" \
        --output "$OUTPUT_DIR" 2>&1 | grep -E "✅|❌"
done

echo ""
echo "✅ All RFCs converted for copyright filing"
echo "📁 Output: $OUTPUT_DIR"
echo "📝 Next: Review LaTeX files and push to Overleaf"
