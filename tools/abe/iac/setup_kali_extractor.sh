#!/bin/bash
# Quick Setup: Kali CLI Dual LLM Extractor
# Sets up environment and runs extraction with Vertex AI + OpenAI

set -e

echo "🔧 Setting up Kali CLI Dual LLM Extractor..."

# Load OpenAI key from vault
export OPENAI_API_KEY=$(grep "OPENAI_API_KEY" /Users/cp5337/Developer/sx9/sx9-conda/ctas7-secure-env.txt | cut -d'=' -f2)

# Set Google Cloud project for Vertex AI
export GOOGLE_CLOUD_PROJECT="your-project-id"  # TODO: Update with actual project ID
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"  # TODO: Update

# Install dependencies
echo "📦 Installing dependencies..."
pip install -q requests beautifulsoup4 openai google-cloud-aiplatform

# Optional: Install crawl4ai for advanced scraping
# pip install crawl4ai

echo "✅ Setup complete!"
echo ""
echo "🚀 Running extraction..."
echo "   - Tier 1: Regex extraction from Kali.org"
echo "   - Tier 2: GitLab debian/control parsing"
echo "   - Tier 3: Vertex AI Gemini 2.0 Flash + OpenAI GPT-4o-mini"
echo ""

# Run extractor
python3 kali_cli_dual_llm_extractor.py

echo ""
echo "✅ Extraction complete!"
echo "📁 Check output: kali_tools_with_commands.json"
