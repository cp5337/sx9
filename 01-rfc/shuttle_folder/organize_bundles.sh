#!/bin/bash
# Shuttle Folder Organization Script
# Organizes bundles and documentation into task-based directories

set -e

SHUTTLE_DIR="/Users/cp5337/Developer/sx9/01-rfc/shuttle_folder"
cd "$SHUTTLE_DIR"

echo "🚀 Organizing Shuttle Folder Bundles"
echo "====================================="
echo ""

# Create task directories
echo "📁 Creating task directories..."
mkdir -p tasks/01-nats-integration
mkdir -p tasks/02-threat-intel-pipeline
mkdir -p tasks/03-frontend-integration
mkdir -p tasks/04-rfc-updates
mkdir -p tasks/05-deployment

echo "✅ Task directories created"
echo ""

# Task 01: NATS Integration
echo "📦 Task 01: NATS Integration"
mv nats-bundle.zip tasks/01-nats-integration/ 2>/dev/null || echo "  ⚠️  nats-bundle.zip already moved"
echo "  ✅ Moved nats-bundle.zip"

# Task 02: Threat Intel Pipeline
echo "📦 Task 02: Threat Intel Pipeline"
mv sx9-threat-intel-neon-bundle.zip tasks/02-threat-intel-pipeline/ 2>/dev/null || echo "  ⚠️  sx9-threat-intel-neon-bundle.zip already moved"
mv threat-intel-pipeline.zip tasks/02-threat-intel-pipeline/ 2>/dev/null || echo "  ⚠️  threat-intel-pipeline.zip already moved"
echo "  ✅ Moved threat intel bundles"

# Task 03: Frontend Integration
echo "📦 Task 03: Frontend Integration"
mv CTAS_INTEGRATION_GUIDE.md tasks/03-frontend-integration/ 2>/dev/null || echo "  ⚠️  CTAS_INTEGRATION_GUIDE.md already moved"
mv RFC_9115_FRONTEND_ECS_INTEGRATION.md tasks/03-frontend-integration/ 2>/dev/null || echo "  ⚠️  RFC_9115_FRONTEND_ECS_INTEGRATION.md already moved"
mv SX9_PLASMA_DEFENDER_CODEBASE_MAPPING.md tasks/03-frontend-integration/ 2>/dev/null || echo "  ⚠️  SX9_PLASMA_DEFENDER_CODEBASE_MAPPING.md already moved"
mv SYNAPTIX9_MASTER_INTEGRATION.md tasks/03-frontend-integration/ 2>/dev/null || echo "  ⚠️  SYNAPTIX9_MASTER_INTEGRATION.md already moved"
echo "  ✅ Moved frontend documentation"

# Task 04: RFC Updates
echo "📦 Task 04: RFC Updates"
mv "Bundle and Corrections.zip" tasks/04-rfc-updates/ 2>/dev/null || echo "  ⚠️  Bundle and Corrections.zip already moved"
mv "Dynamic Domain Adaptation v4.0.docx" tasks/04-rfc-updates/ 2>/dev/null || echo "  ⚠️  Dynamic Domain Adaptation v4.0.docx already moved"
mv "LINEAR ISSUE → BNE ARCHAEOLOGICAL MINING → VOICE.md" tasks/04-rfc-updates/ 2>/dev/null || echo "  ⚠️  LINEAR ISSUE already moved"
mv RFC_9016_DELTA_ANGLE_FIX.md tasks/04-rfc-updates/ 2>/dev/null || echo "  ⚠️  RFC_9016_DELTA_ANGLE_FIX.md already moved"
echo "  ✅ Moved RFC documentation"

# Task 05: Deployment
echo "📦 Task 05: Deployment"
mv small_commad.zip tasks/05-deployment/ 2>/dev/null || echo "  ⚠️  small_commad.zip already moved"
mv ANTIGRAVITY_PROMPT.md tasks/05-deployment/ 2>/dev/null || echo "  ⚠️  ANTIGRAVITY_PROMPT.md already moved"
mv "Excellent — this SX9 Gateway + Cloudflare R2 archi.md" tasks/05-deployment/ 2>/dev/null || echo "  ⚠️  Gateway R2 archi already moved"
mv UNICODE_ADDRESSING_DEPLOYMENT_GUIDE.md tasks/05-deployment/ 2>/dev/null || echo "  ⚠️  UNICODE_ADDRESSING_DEPLOYMENT_GUIDE.md already moved"
echo "  ✅ Moved deployment files"

echo ""
echo "====================================="
echo "✅ Organization Complete!"
echo "====================================="
echo ""

# Show directory structure
echo "📊 Task Directory Structure:"
echo ""
tree -L 2 tasks/ 2>/dev/null || ls -R tasks/

echo ""
echo "📋 Next Steps:"
echo "  1. Extract bundles: cd tasks/01-nats-integration && unzip nats-bundle.zip"
echo "  2. Review organization plan: cat /Users/cp5337/.gemini/antigravity/brain/d1a087cd-9f2b-46a5-8b03-fa879210658e/shuttle_folder_organization.md"
echo "  3. Begin Task 01 (NATS Integration)"
echo ""
