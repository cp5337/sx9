# UI Refactor Plan - Professional Production System

**Date:** 2025-12-07  
**Status:** ACTION PLAN  
**Priority:** HIGH - Fix AI-generated slop, make it production-ready

---

## 🎯 **CORE PRINCIPLES**

1. **Less is More** - Remove redundant buttons, consolidate into shared panels
2. **Real Data** - Wire up to actual backends, no hardcoded bullshit
3. **Professional Design** - Industry standard form factors, proper colors/fonts
4. **CLI First** - Highly capable CLI more important than button overload
5. **Horizon Tabs** - Focus on existing tabs, they're self-explanatory

---

## 🔥 **IMMEDIATE FIXES** (This Week)

### **1. Database Management Page** (Priority: CRITICAL)

**Current Issues:**
- Hardcoded bullshit data
- Buttons that may not be needed
- Unprofessional styling

**Actions:**
- [ ] Wire up to real databases (Supabase, SurrealDB, Sled, SlotGraph, Neo4j)
- [ ] Create professional status/control cards (follow gallery card form factors)
- [ ] Reassess buttons: "Create New Project", "Backup Database", "View Analytics" - do we need these or something smarter?
- [ ] Check `ctas-health-dashboard` for wiring patterns
- [ ] Show real connection status, query stats, health metrics

**Card Spec:**
```typescript
interface DatabaseStatusCard {
  name: string;
  type: 'supabase' | 'surreal' | 'sled' | 'slotgraph' | 'neo4j';
  status: 'connected' | 'disconnected' | 'error';
  stats: {
    queries: number;
    latency: number;
    health: 'healthy' | 'degraded' | 'unhealthy';
  };
  controls: {
    connect?: () => void;
    disconnect?: () => void;
    refresh?: () => void;
    query?: () => void;
  };
}
```

---

### **2. Styling Cleanup** (Priority: CRITICAL)

**Current Issues:**
- AI slop styling doesn't match anything
- Ugly bright green → needs dark green
- Wrong fonts
- Too much padding
- Double labels
- Big cards for nothing

**Actions:**
- [ ] Tune down green to dark green (`#065f46` or similar)
- [ ] Fix fonts to match system (check existing working components)
- [ ] Reduce padding (follow gallery card form factors)
- [ ] Remove double labels
- [ ] Make cards appropriate size (not huge for nothing)
- [ ] Match industry standard form factors

**Color Palette:**
- Dark Green: `#065f46` (instead of bright green)
- Use existing system colors from working components
- Match gallery card styling

---

### **3. Containers Page** (Priority: HIGH)

**Current Issues:**
- Not showing real containers
- Hardcoded data

**Actions:**
- [ ] Wire up to Docker/OrbStack API
- [ ] Show real containers: Alpine, Neo4j x2, etc.
- [ ] Virtual Kali = thin clients (show as such)
- [ ] Show container status, resources, logs
- [ ] Connect to backend gateway (port 18600)
- [ ] Show real container health

**Container Types:**
- **Alpine** - Base container
- **Neo4j x2** - Graph databases
- **Virtual Kali** - Thin clients (commercially optimized, downloadable)
- **Red Team Tasks** - Can be chained, output to CDN
- **Test Containers** - Already created one

**Display:**
- Container name, status, image, ports
- Resource usage (CPU, memory)
- Quick actions (start, stop, logs, shell)

---

### **4. Right Panel - Stats & Drawers** (Priority: HIGH)

**Current Issues:**
- Redundant database connections
- No drawer system for map layers
- Missing tool stats

**Actions:**
- [ ] Show tool stats (usage, health, performance)
- [ ] Remove redundant database connection indicators
- [ ] Create drawer system for map layers (when doing GIS)
- [ ] Consolidate connection status into single indicator
- [ ] Add collapsible sections for different tool categories

**Drawer System:**
```typescript
interface MapLayerDrawer {
  layers: MapLayer[];
  onToggle: (layerId: string) => void;
  onConfigure: (layerId: string) => void;
  collapsed?: boolean;
}
```

---

### **5. Main Pages - Button Overload** (Priority: CRITICAL)

**Current Issues:**
- Too many buttons
- Repeating side panel
- Wrong colors
- Huge buttons
- Taking away from GIS space and horizon tabs

**Actions:**
- [ ] Remove redundant buttons (they're in side panel already)
- [ ] Consolidate into shared side panel
- [ ] Fix colors to match system
- [ ] Reduce button sizes
- [ ] Focus on horizon tabs (Overview, Kali Tools, Playbooks, Red Team, Phase Mapping, Tasks)
- [ ] Keep phase mapping for now (may not make it)
- [ ] Free up GIS space

**Strategy:**
- Move common actions to shared side panel
- Only show page-specific actions on main page
- Horizon tabs are self-explanatory and will be populated with downloaded data

---

### **6. AI CLI Configuration** (Priority: HIGH)

**Current Issues:**
- Not configured properly
- Should be place for authenticating agents, MCPs, CLI

**Actions:**
- [ ] Configure AI CLI as authentication hub
- [ ] Wire up agent authentication (Natasha, Marcus, Elena, Cove, Kali)
- [ ] Wire up MCP server authentication
- [ ] Wire up CLI authentication
- [ ] This worked at one time with custom GPT - restore that functionality
- [ ] Make CLI highly capable (more important than buttons)

**CLI Features:**
- Agent authentication
- MCP server management
- CLI session management
- Command history
- Auto-completion
- Natural language commands

---

### **7. Streams Page** (Priority: MEDIUM)

**Current Issues:**
- Shit styling
- Wrong colors
- Wrong fonts

**Actions:**
- [ ] Fix colors (use system colors)
- [ ] Fix fonts (match system)
- [ ] Create Vercel prompt for professional components
- [ ] Make it look production-ready

**Vercel Prompt:**
```
Create a professional info streams component for a cybersecurity operations platform:
- Dark theme with proper contrast
- Real-time stream updates
- Priority filtering (Critical, High, Medium)
- List/Map view toggle
- Proper typography and spacing
- Industry standard form factors
- Match existing system design language
```

---

### **8. Plasma Dashboard** (Priority: MEDIUM)

**Current Issues:**
- Not showing operational status
- Need to build agents display

**Actions:**
- [ ] Design operational dashboard
- [ ] Show agents (status, health, activity)
- [ ] Show streams (real-time updates)
- [ ] Show system health
- [ ] Show performance metrics
- [ ] Make it look professional

**Dashboard Spec:**
- Agent status cards
- Stream visualization
- Health metrics
- Performance graphs
- Real-time updates

---

### **9. Settings Page** (Priority: MEDIUM)

**Current Issues:**
- Looks like prototype slop
- Too much padding
- Double labels
- Big cards for nothing

**Actions:**
- [ ] Create real settings page OR get Vercel prompt
- [ ] Follow gallery card form factors
- [ ] Reduce padding
- [ ] Remove double labels
- [ ] Make cards appropriate size
- [ ] Industry standard form factors

**Vercel Prompt:**
```
Create a professional settings page for a cybersecurity operations platform:
- Dark theme
- Industry standard form factors
- Proper spacing (not too much padding)
- Single labels (no double labels)
- Appropriate card sizes
- Match gallery card styling
- Settings categories: Notifications, Security, Appearance, Data Management, Storage, Performance
```

---

### **10. Gallery Page** (Priority: MEDIUM)

**Current Issues:**
- Too much padding
- Redundant menuing
- Rounded corners/upgrades/loaded icon legend takes up room and looks like shit
- Search bar probably not needed with granular side panel
- Cards need to be real components from task analysis
- When picked, should be playbooks/tool chains (iTunes tools/albums concept)

**Actions:**
- [ ] Reduce padding
- [ ] Remove redundant menuing
- [ ] Eliminate rounded corners/upgrades/loaded icon legend
- [ ] Remove search bar (or make it conditional - only after XX cards)
- [ ] Make cards real components derived from task analysis
- [ ] When component picked → show playbooks/tool chains
- [ ] Implement iTunes tools/albums concept (curated, high-performance playbooks)

**Gallery Concept:**
- Cards = Components (derived from task analysis)
- Click card → Show playbooks/tool chains for that component
- Playbooks = Curated, high-performance tool chains
- iTunes concept = Tools are like songs, playbooks are like albums

---

### **11. Scripts Page** (Priority: MEDIUM)

**Current Issues:**
- Not live
- Not wired to databases
- Missing GNN/ANN and two models health check/stats

**Actions:**
- [ ] Make script windows live
- [ ] Wire in databases (Supabase, SurrealDB, Neo4j)
- [ ] Wire in GNN health check and stats
- [ ] Wire in ANN health check and stats
- [ ] Wire in two models (Phi-3, DistilBERT) health check and stats
- [ ] Show real-time execution status

---

### **12. Kill Chain Tab** (Priority: LOW)

**Current Issues:**
- Should switch context for "whiny operator who can't understand a framework"

**Actions:**
- [ ] Make Kill Chain tab switch to simplified context
- [ ] Show framework-agnostic view
- [ ] Make it self-explanatory for non-technical operators

---

### **13. Task Graphs & Analysis Graphs** (Priority: MEDIUM)

**Current Issues:**
- Need way to invoke Neo4j browser
- Need endpoints for data analysis workbench

**Actions:**
- [ ] Create endpoint to invoke Neo4j browser (viewport or similar)
- [ ] Create endpoints for data analysis workbench
- [ ] Support nonagen graph system (in development)
- [ ] Make graphs interactive and exportable

**Endpoints Needed:**
- `/api/graph/neo4j-browser` - Invoke Neo4j browser
- `/api/graph/analysis-workbench` - Data analysis workbench
- `/api/graph/export` - Export graph data
- `/api/graph/nonagen` - Nonagen graph system support

---

### **14. Raptor Page** (Priority: LOW)

**Current Issues:**
- Should be vacant for now
- Needs proper title

**Actions:**
- [ ] Keep vacant
- [ ] Change title to: "REMOTELY ALLOCATED POLYMORPHIC TECHNICAL OPERATIONS RESOURCE" (all caps)
- [ ] Add placeholder content explaining what it will be

---

### **15. Demo Report** (Priority: LOW)

**Current Issues:**
- Can't be saved
- Was for another project (component library)

**Actions:**
- [ ] Remove or repurpose
- [ ] If keeping, make it saveable
- [ ] Clarify purpose

---

## 📋 **IMPLEMENTATION CHECKLIST**

### **Week 1: Critical Fixes**
- [ ] Database Management - Wire up real data, create status cards
- [ ] Styling Cleanup - Fix colors, fonts, padding
- [ ] Main Pages - Remove redundant buttons, consolidate to side panel
- [ ] Containers Page - Show real containers

### **Week 2: Core Features**
- [ ] AI CLI - Configure authentication hub
- [ ] Right Panel - Stats, drawers, remove redundancy
- [ ] Streams - Fix styling, get Vercel components
- [ ] Plasma - Build operational dashboard

### **Week 3: Polish**
- [ ] Settings - Real page or Vercel prompt
- [ ] Gallery - Reduce padding, remove redundancy, implement iTunes concept
- [ ] Scripts - Wire up databases, GNN/ANN/models
- [ ] Task Graphs - Neo4j browser integration

### **Week 4: Final Touches**
- [ ] Kill Chain - Simplified context
- [ ] Raptor - Proper title, placeholder
- [ ] Demo Report - Remove or repurpose
- [ ] End-to-end testing

---

## 🎨 **DESIGN SPECIFICATIONS**

### **Color Palette**
- Dark Green: `#065f46` (replaces bright green)
- Use existing system colors from working components
- Match gallery card colors

### **Typography**
- Match existing system fonts
- No double labels
- Proper hierarchy

### **Spacing**
- Reduce padding (follow gallery card form factors)
- Industry standard form factors
- Not too much whitespace

### **Card Form Factors**
- Follow gallery card styling
- Appropriate sizes (not huge for nothing)
- Professional appearance
- Status and control surfaces

---

## 🔗 **BACKEND INTEGRATIONS**

### **Databases**
- Supabase (PostgREST API - port 3000)
- SurrealDB (SurrealQL WebSocket - port 8000)
- Sled KVS (Local file system)
- SlotGraph (Rust crate API)
- Neo4j (Cypher queries - port 7474)

### **Services**
- SX9 Gateway (WebSocket - port 18600)
- Docker/OrbStack API (Container management)
- GLAF Server (Graph analytics - port 18050)
- Thalamic Filter (DistilBERT - port 18114)
- GNN Health Check
- ANN Health Check
- Phi-3 Health Check
- DistilBERT Health Check

---

## 💡 **KEY INSIGHTS**

1. **Less Buttons, More CLI** - Highly capable CLI is more important
2. **Real Data, Not Hardcoded** - Wire everything up to actual backends
3. **Professional Design** - Industry standard, not AI slop
4. **Horizon Tabs Focus** - They're self-explanatory and important
5. **Shared Side Panel** - Consolidate redundant actions
6. **iTunes Concept** - Components → Playbooks/Tool Chains (curated, high-performance)

---

## 🚀 **NEXT STEPS**

1. **Start with Database Management** - Wire up real data, create professional cards
2. **Fix Styling** - Colors, fonts, padding (this week)
3. **Remove Redundancy** - Consolidate buttons into shared side panel
4. **Configure AI CLI** - Make it the authentication hub
5. **Build Real Components** - From task analysis, not hardcoded

**Ready to start? Begin with Database Management and Styling Cleanup - these are the most visible issues.**



