# Gemini Code Assist Custom Commands for CTAS v7.3.1

This directory contains custom Gemini Code Assist commands optimized for CTAS development using Gemini 2M's massive context window.

## Setup

1. **Install Gemini Code Assist** (VS Code or JetBrains)
2. **Set API Key:**
   ```bash
   export GEMINI_API_KEY=your_api_key_here
   # Or save to file:
   echo "your_api_key_here" > ~/.gemini_api_key
   ```

3. **Load Custom Commands:**
   - Copy `custom-commands.json` to your IDE's Gemini config directory
   - Or use the commands directly in Gemini Code Assist

## Available Commands

### 🏗️ Architecture & Design

#### `/ea` - Enterprise Architecture Diagrams
Generates comprehensive EA diagrams using Gemini 2M context.

**Usage:**
```
/ea
```

**Generates:**
- Business layer capabilities
- Application architecture
- Technology stack diagrams
- Data flow models
- Security architecture

**Output:** Mermaid, PlantUML, ArchiMate formats

---

#### `/devsecops` - DevSecOps Pipeline
Creates DevSecOps flow diagrams.

**Usage:**
```
/devsecops
```

**Includes:**
- CI/CD pipeline stages
- Security gates (SAST, DAST, SCA)
- Infrastructure as Code
- Monitoring & alerting
- Incident response flows

---

#### `/dataflow` - Data Flow Diagrams
Maps data flows through the system.

**Usage:**
```
/dataflow
```

**Shows:**
- Wazuh → AXON → PLASMA flow
- OSINT → Neural Mux → Foundation
- Frontend ↔ Backend ↔ Database

---

#### `/integrations` - Integration Map
Comprehensive system integration mapping.

**Usage:**
```
/integrations
```

**Details:**
- All external systems
- Ports & protocols
- Authentication methods
- Error handling strategies

---

### 🔍 Analysis & Audit

#### `/analyze-components` - Component Analysis
Deep dive into React component architecture.

**Usage:**
```
/analyze-components
```

**Analyzes:**
- Component hierarchy
- Data flow patterns
- State management
- Performance bottlenecks
- Refactoring opportunities

---

#### `/security-audit` - Security Audit
Comprehensive security analysis.

**Usage:**
```
/security-audit
```

**Covers:**
- Trivariate hashing implementation
- QEK obfuscation
- Ephemeral execution model
- DoD compliance (NIST 800-53, FIPS 140-3)
- Vulnerability assessment

---

#### `/performance` - Performance Analysis
System performance optimization.

**Usage:**
```
/performance
```

**Identifies:**
- React rendering issues
- Bundle size optimization
- API efficiency
- Database query optimization
- Streaming performance

---

#### `/deps-audit` - Dependency Audit
npm package security and updates.

**Usage:**
```
/deps-audit
```

**Checks:**
- Security vulnerabilities
- Outdated packages
- Unused dependencies
- License compliance

---

### 📝 Documentation

#### `/api-docs` - API Documentation
Generate OpenAPI 3.0 documentation.

**Usage:**
```
/api-docs
```

**Documents:**
- REST endpoints
- WebSocket connections
- SSE streams
- Authentication
- Error codes

---

#### `/deploy-docs` - Deployment Guide
Comprehensive deployment documentation.

**Usage:**
```
/deploy-docs
```

**Includes:**
- Local setup
- Docker deployment
- Environment variables
- Service dependencies
- Troubleshooting

---

#### `/usim` - USIM Headers
Generate USIM headers for documentation.

**Usage:**
```
/usim
```

**Creates:**
- Trivariate hash
- Unicode symbols
- Metadata
- Knowledge registry links

---

### 🛠️ Development

#### `/test-strategy` - Testing Strategy
Generate comprehensive test plans.

**Usage:**
```
/test-strategy
```

**Provides:**
- Unit test cases
- Integration tests
- E2E tests (Playwright)
- Security tests
- Performance tests

---

#### `/refactor` - Refactoring Suggestions
Code improvement recommendations.

**Usage:**
```
/refactor
```

**Suggests:**
- Code duplication fixes
- Complex function simplification
- State management improvements
- Type safety enhancements

---

#### `/commit-msg` - Commit Messages
Generate conventional commit messages.

**Usage:**
```
/commit-msg
```

**Format:**
```
feat(plasma): Add collapsible panels with glyph rails

- Implement left/right panel collapse functionality
- Add hover feedback on glyphs
- Update styling to match CTAS dark theme

Closes #123
```

---

#### `/adr` - Architecture Decision Records
Document architectural decisions.

**Usage:**
```
/adr
```

**Template:**
- Context
- Decision
- Consequences
- Alternatives
- Implementation notes

---

## Marcus (Gemini 2M) - CTAS AI Architect

Marcus is assigned to Gemini 2M for:
- **Enterprise Architecture** - Full codebase analysis
- **DevSecOps** - Pipeline design and security
- **IAC Integration** - N-DEx, NIEM compliance
- **Large Context Tasks** - 2M token window for comprehensive analysis

### Marcus Specializations

1. **Architecture Analysis**
   - Full system context in single prompt
   - Cross-component dependency mapping
   - Performance bottleneck identification

2. **DevSecOps Design**
   - End-to-end pipeline visualization
   - Security gate placement
   - Compliance verification

3. **Integration Strategy**
   - N-DEx/NIEM mapping
   - External system integration
   - API design patterns

4. **Documentation Generation**
   - Comprehensive EA diagrams
   - Technical specifications
   - Deployment guides

## Example Workflows

### 1. New Feature Development

```bash
# 1. Analyze current architecture
/analyze-components

# 2. Check security implications
/security-audit

# 3. Generate tests
/test-strategy

# 4. Document decision
/adr

# 5. Create commit message
/commit-msg
```

### 2. System Documentation

```bash
# 1. Generate EA diagrams
/ea

# 2. Create DevSecOps flow
/devsecops

# 3. Document APIs
/api-docs

# 4. Create deployment guide
/deploy-docs
```

### 3. Performance Optimization

```bash
# 1. Analyze performance
/performance

# 2. Get refactoring suggestions
/refactor

# 3. Check dependencies
/deps-audit

# 4. Update documentation
/api-docs
```

## Integration with Git Hooks

The post-commit hook automatically:
1. Calls `/ea` for architecture updates
2. Generates deployment docs
3. Creates USIM headers
4. Updates EA models

## Tips for Best Results

1. **Provide Context:** Include relevant files in your prompt
2. **Be Specific:** Clearly state what you want to achieve
3. **Iterate:** Use follow-up prompts to refine results
4. **Validate:** Always review generated code/diagrams
5. **Document:** Save important outputs to `docs/` folder

## Model Configuration

- **Model:** `gemini-2.0-flash-exp`
- **Context Window:** 2M tokens
- **Temperature:** 0.7 (balanced creativity/accuracy)
- **Max Output:** 8192 tokens

## File Locations

```
.gemini/
├── custom-commands.json    # Command definitions
├── README.md               # This file
└── examples/               # Example outputs
    ├── ea-diagrams/
    ├── devsecops-flows/
    └── api-docs/
```

## Support

For issues or suggestions:
- **Repository:** github.com/cp5337/sb1-snwqto-ctas_6
- **Version:** v7.3.1
- **Marcus (Gemini):** EA & DevSecOps Lead

---

*Powered by Gemini 2M - CTAS v7.3.1*

