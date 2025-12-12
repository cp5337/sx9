#!/bin/bash
# CTAS v7.3.1 Gemini EA & DevSecOps Architecture Generator
# Uses Gemini 2M context window for comprehensive architecture analysis

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"
EA_DIR="$REPO_ROOT/docs/enterprise-architecture"
GEMINI_API_KEY="${GEMINI_API_KEY:-$(cat ~/.gemini_api_key 2>/dev/null || echo '')}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

if [ -z "$GEMINI_API_KEY" ]; then
    echo "❌ GEMINI_API_KEY not found"
    echo "   Set via: export GEMINI_API_KEY=your_key"
    echo "   Or save to: ~/.gemini_api_key"
    exit 1
fi

echo "🤖 Gemini EA & DevSecOps Generator"
echo "===================================="
echo "📊 Context: 2M tokens"
echo "🏗️  Output: EA diagrams, DevSecOps flows"
echo ""

# Get commit context
COMMIT_HASH=$(git rev-parse --short HEAD)
COMMIT_MSG=$(git log -1 --pretty=%B)

# Gather codebase context
echo "📚 Gathering codebase context..."
COMPONENTS=$(find "$REPO_ROOT/src/components" -name "*.tsx" 2>/dev/null | wc -l | xargs)
PAGES=$(find "$REPO_ROOT/src/pages" -name "*.tsx" 2>/dev/null | wc -l | xargs)
HOOKS=$(find "$REPO_ROOT/src/hooks" -name "*.ts" 2>/dev/null | wc -l | xargs)

# Get recent changes
RECENT_FILES=$(git diff --name-only HEAD~5 HEAD 2>/dev/null | head -20 || echo "")

# Create comprehensive context for Gemini
CONTEXT=$(cat << CTXEOF
# CTAS v7.3.1 Architecture Analysis Request

## Project Overview
- **Name:** Convergent Threat Analysis System (CTAS)
- **Version:** 7.3.1
- **Commit:** $COMMIT_HASH
- **Latest Change:** $COMMIT_MSG

## Codebase Statistics
- Components: $COMPONENTS
- Pages: $PAGES
- Hooks: $HOOKS

## Recent Changes
$RECENT_FILES

## Architecture Requirements

### 1. Enterprise Architecture (EA) Diagrams
Generate comprehensive EA diagrams covering:
- **Business Layer:** Capabilities, stakeholders, value streams
- **Application Layer:** Applications, services, integration points
- **Technology Layer:** Infrastructure, platforms, tools
- **Data Layer:** Entities, flows, storage
- **Security Layer:** Controls, compliance, threat model

Output formats: Mermaid, PlantUML, ArchiMate

### 2. DevSecOps Architecture
Generate DevSecOps flow diagrams showing:
- **CI/CD Pipeline:** Git → Build → Test → Deploy
- **Security Gates:** SAST, DAST, SCA, secrets scanning
- **Infrastructure as Code:** Terraform, Docker, OrbStack
- **Monitoring:** Wazuh, AXON, HFT analytics
- **Incident Response:** Detection → Analysis → Response → Recovery

### 3. Component Diagrams
Detailed component breakdowns:
- **PLASMA Dashboard:** Agents, Threats, HFT panels
- **Neural Mux:** AI orchestration, routing
- **Foundation v7.3.1:** Trivariate hashing, Unicode assembly
- **Smart Crates:** Modular Rust components

### 4. Data Flow Diagrams
End-to-end data flows:
- Wazuh → AXON → PLASMA → Visualization
- OSINT → Neural Mux → Foundation → Storage
- User Input → Processing → Database → Response

### 5. Security Architecture
Security controls and threat model:
- Trivariate hashing (SCH+CUID+UUID)
- QEK obfuscation
- Ephemeral execution
- PGP signing + Blockchain anchoring
- DoD compliance (NIST 800-53, FIPS 140-3)

## Specific Focus Areas

### PLASMA Integration
- 3-panel layout (Agents, Threats, HFT)
- Wazuh agent monitoring
- AXON threat intelligence
- Real-time streaming architecture

### Foundation v7.3.1
- 48-character Base96 hashing
- Deterministic execution
- Content addressing
- Knowledge registry (USIM/MARC)

### DevSecOps Pipeline
- Post-commit hooks (documentation, ABE, presentations)
- Automated testing
- Security scanning
- Deployment automation

## Output Requirements

For each diagram type, provide:
1. **Mermaid syntax** (for GitHub/docs)
2. **PlantUML syntax** (for detailed diagrams)
3. **ArchiMate notation** (for EA compliance)
4. **Narrative description** (for stakeholders)
5. **Implementation notes** (for developers)

Generate diagrams that are:
- **Comprehensive:** Cover all major components
- **Accurate:** Reflect actual codebase structure
- **Professional:** Suitable for executive presentation
- **Actionable:** Include implementation guidance

CTXEOF
)

echo "🤖 Sending request to Gemini 2M..."

# Call Gemini API
RESPONSE=$(curl -s -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key=$GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d "{
    \"contents\":[{
      \"parts\":[{
        \"text\": $(echo "$CONTEXT" | jq -Rs .)
      }]
    }],
    \"generationConfig\": {
      \"temperature\": 0.7,
      \"topK\": 40,
      \"topP\": 0.95,
      \"maxOutputTokens\": 8192
    }
  }")

# Extract generated content
GENERATED=$(echo "$RESPONSE" | jq -r '.candidates[0].content.parts[0].text' 2>/dev/null || echo "")

if [ -z "$GENERATED" ] || [ "$GENERATED" = "null" ]; then
    echo "❌ Failed to generate content"
    echo "Response: $RESPONSE"
    exit 1
fi

echo "✅ Content generated"
echo ""

# Save full output
mkdir -p "$EA_DIR"/{diagrams,models,devsecops}
OUTPUT_FILE="$EA_DIR/GEMINI-EA-ANALYSIS-$TIMESTAMP.md"
cat > "$OUTPUT_FILE" << OUTEOF
# CTAS v7.3.1 Enterprise Architecture Analysis

**Generated by:** Gemini 2M
**Date:** $(date)
**Commit:** $COMMIT_HASH

---

$GENERATED

---

*Generated by Gemini EA Generator - CTAS v7.3.1*
OUTEOF

echo "📄 Full analysis saved: $(basename $OUTPUT_FILE)"

# Extract and save individual diagram types
echo ""
echo "📊 Extracting diagrams..."

# Extract Mermaid diagrams
MERMAID_COUNT=$(echo "$GENERATED" | grep -c '```mermaid' || echo "0")
if [ "$MERMAID_COUNT" -gt 0 ]; then
    echo "$GENERATED" | awk '/```mermaid/,/```/' | sed '/```/d' > "$EA_DIR/diagrams/mermaid-$TIMESTAMP.mmd"
    echo "✅ Mermaid diagrams: $MERMAID_COUNT"
fi

# Extract PlantUML diagrams
PLANTUML_COUNT=$(echo "$GENERATED" | grep -c '```plantuml' || echo "0")
if [ "$PLANTUML_COUNT" -gt 0 ]; then
    echo "$GENERATED" | awk '/```plantuml/,/```/' | sed '/```/d' > "$EA_DIR/diagrams/plantuml-$TIMESTAMP.puml"
    echo "✅ PlantUML diagrams: $PLANTUML_COUNT"
fi

# Extract DevSecOps flows
echo "$GENERATED" | grep -A 50 "DevSecOps" > "$EA_DIR/devsecops/flow-$TIMESTAMP.md" 2>/dev/null || true

echo ""
echo "===================================="
echo "✅ Gemini EA Generation Complete"
echo ""
echo "📊 Generated Artifacts:"
echo "   📄 Full analysis: $(basename $OUTPUT_FILE)"
[ "$MERMAID_COUNT" -gt 0 ] && echo "   🎨 Mermaid diagrams: $MERMAID_COUNT"
[ "$PLANTUML_COUNT" -gt 0 ] && echo "   🎨 PlantUML diagrams: $PLANTUML_COUNT"
echo ""
echo "📂 Location: $EA_DIR"
echo "===================================="

