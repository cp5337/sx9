#!/bin/bash
# Install SX9 Threat Intelligence Dependencies via Conda
# This script creates a conda environment and installs all required packages

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_NAME="sx9-threat-intel"
ENV_FILE="environment.yml"

echo "🔧 Installing SX9 Threat Intelligence Dependencies"
echo "=================================================="
echo ""

# Check if conda is installed
if ! command -v conda &> /dev/null; then
    echo "❌ Conda not found. Please install Miniconda or Anaconda first."
    echo "   Download from: https://docs.conda.io/en/latest/miniconda.html"
    exit 1
fi

echo "✅ Conda found: $(conda --version)"
echo ""

# Check if environment exists
if conda env list | grep -q "^${ENV_NAME} "; then
    echo "⚠️  Environment '${ENV_NAME}' already exists."
    read -p "   Remove and recreate? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🗑️  Removing existing environment..."
        conda env remove -n "$ENV_NAME" -y
    else
        echo "📦 Updating existing environment..."
        conda env update -n "$ENV_NAME" -f "$ENV_FILE" --prune
        echo "✅ Environment updated!"
        exit 0
    fi
fi

# Create environment from file
echo "📦 Creating conda environment: ${ENV_NAME}"
conda env create -f "$ENV_FILE"

echo ""
echo "✅ Environment created successfully!"
echo ""
echo "🚀 To activate the environment:"
echo "   conda activate ${ENV_NAME}"
echo ""
echo "📋 Installed packages:"
echo "   • mmh3 (Murmur3-64 for RFC-9001 hashing)"
echo "   • neo4j (Neo4j driver)"
echo "   • supabase (Supabase client)"
echo "   • ontogpt (SPIRES ontology)"
echo "   • google-cloud-aiplatform (Vertex AI)"
echo "   • vertexai (Vertex AI SDK)"
echo ""
echo "💡 Next steps:"
echo "   1. Activate: conda activate ${ENV_NAME}"
echo "   2. Run pipeline: ./execute_full_pipeline.sh"
echo ""



