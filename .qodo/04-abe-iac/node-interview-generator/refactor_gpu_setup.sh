#!/bin/bash
# Setup script for GPU machine refactoring with Vertex AI Gemini
# Run this on your GCP GPU instance

set -e

echo "🚀 Setting up GPU machine for Vertex AI Gemini refactoring"
echo "=========================================================="
echo ""

# Auto-detect project from gcloud config
PROJECT_ID=$(gcloud config get-value project 2>/dev/null || echo "")
if [ -z "$PROJECT_ID" ]; then
    # Try environment variable
    PROJECT_ID="${GOOGLE_CLOUD_PROJECT:-}"
    if [ -z "$PROJECT_ID" ]; then
        echo "⚠️  Could not auto-detect project. Setting default..."
        PROJECT_ID="gen-lang-client-0290627006"  # Default from existing code
    fi
fi

REGION="${GCP_REGION:-us-central1}"

echo "📋 Configuration:"
echo "   Project: $PROJECT_ID"
echo "   Region: $REGION"
echo ""

# Install Python dependencies
echo "📦 Installing Python dependencies..."
pip install --upgrade pip
pip install google-cloud-aiplatform vertexai google-cloud-storage

echo "✅ Dependencies installed"
echo ""

# Authenticate with GCP (if not already)
echo "🔐 Checking GCP authentication..."
if ! gcloud auth application-default print-access-token &>/dev/null; then
    echo "   Authenticating with GCP..."
    gcloud auth application-default login
else
    echo "   ✅ Already authenticated"
fi

echo ""

# Set project
echo "🔧 Setting GCP project..."
gcloud config set project "$PROJECT_ID"

# Enable Vertex AI API
echo "🔌 Enabling Vertex AI API..."
gcloud services enable aiplatform.googleapis.com --project="$PROJECT_ID"

echo ""
echo "✅ Setup complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Navigate to the refactoring directory:"
echo "      cd /path/to/04-abe-iac/node-interview-generator"
echo ""
echo "   2. Run the refactoring script:"
echo "      python refactor_with_vertex_gemini.py \\"
echo "        --project-id $PROJECT_ID \\"
echo "        --region $REGION \\"
echo "        --model gemini-2.0-flash-exp"
echo ""
echo "   3. Review the refactored code in: refactored_output/"
echo ""
echo "💰 Cost estimate:"
echo "   • Gemini 2.0 Flash: ~\$0.10-0.50 per refactoring"
echo "   • Gemini 1.5 Pro: ~\$1.00-2.00 per refactoring"
echo "   • Processing time: 5-15 minutes"
echo ""

