#!/bin/bash
# ============================================================================
# CTAS-7 GCP Extraction Pipeline
# ============================================================================
# Runs the dual AI extraction on Google Cloud:
# 1. Vertex AI (Gemini 1.5 Flash) - Initial extraction
# 2. Gemini 1.5 Pro - Refinement + scholarly refs + test harness
# 3. GPU-accelerated embeddings
#
# Prerequisites:
#   - gcloud CLI authenticated
#   - GOOGLE_API_KEY or GEMINI_API_KEY set
#   - Project ID configured
# ============================================================================

set -e

# Configuration
PROJECT_ID="${GCP_PROJECT_ID:-cognetix-alpha}"
REGION="${GCP_REGION:-us-central1}"
BUCKET="${GCP_BUCKET:-ctas7-extraction-data}"

echo "=============================================="
echo "🚀 CTAS-7 GCP Extraction Pipeline"
echo "=============================================="
echo "Project: $PROJECT_ID"
echo "Region: $REGION"
echo "Bucket: $BUCKET"
echo ""

# Check for API key
if [ -z "$GOOGLE_API_KEY" ] && [ -z "$GEMINI_API_KEY" ]; then
    echo "⚠️  No API key found. Set GOOGLE_API_KEY or GEMINI_API_KEY"
    echo "   export GOOGLE_API_KEY='your-key-here'"
    exit 1
fi

API_KEY="${GOOGLE_API_KEY:-$GEMINI_API_KEY}"

echo "✅ API key found"
echo ""

# Run the extraction
echo "📄 Running extraction pipeline..."
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/04-abe-iac

python3 gcp_vertex_gemini_pipeline.py \
    --project "$PROJECT_ID" \
    --region "$REGION" \
    --api-key "$API_KEY" \
    --output "output/gcp_extraction" \
    "$@"

echo ""
echo "=============================================="
echo "✅ GCP Extraction Complete"
echo "=============================================="

