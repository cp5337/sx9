# Implementation Plan: Closing Button/Function Gaps

**Date:** 2025-01-27  
**Status:** Implementation Roadmap  
**Total Gaps:** ~285 actions (95% of total)  
**Target:** Complete all critical actions in 4-6 weeks

---

## 📊 Current State Analysis

### **Working** (15 actions - 5%)
- ✅ Phase navigation tabs (Overview, Kali Tools, Playbooks, Red Team, Phase Mapping, Tasks)
- ✅ Show CLI button
- ✅ Agent tabs (Natasha, Marcus, Elena, Cove, Kali)
- ✅ Chat + Terminal views
- ✅ Task loading and display

### **Needs Implementation** (285 actions - 95%)
- ❌ Data visualization suite (13 actions × 10 pages = 130 instances)
- ❌ Database connections (4 actions × 10 pages = 40 instances)
- ❌ Filter/sector controls (2 actions × 10 pages = 20 instances)
- ❌ Page-specific actions (95 unique actions)

---

## 🎯 Implementation Strategy

### **Phase 1: Shared Components** (Week 1-2)
**Goal:** Create reusable components for common actions

#### 1.1 DataVisualizationToolbar Component
**Priority:** CRITICAL  
**Impact:** Eliminates 130 duplicate implementations  
**Time:** 2-3 days

**Actions to implement:**
- Network View
- Task Graph
- Sectors
- Filters
- Data Sources
- GIS Layers
- OSINT Nodes
- Threat Intel
- Infrastructure
- GeoIP
- Supabase
- SurrealDB
- GEE (KMZ)

**Implementation:**
```typescript
// src/components/shared/DataVisualizationToolbar.tsx
interface DataVisualizationToolbarProps {
  onNetworkView?: () => void;
  onTaskGraph?: () => void;
  onSectors?: () => void;
  onFilters?: () => void;
  onDataSources?: () => void;
  onGisLayers?: () => void;
  onOsintNodes?: () => void;
  onThreatIntel?: () => void;
  onInfrastructure?: () => void;
  onGeoIP?: () => void;
  onSupabase?: () => void;
  onSurrealDB?: () => void;
  onGEE?: () => void;
  enabledActions?: string[]; // Whitelist which actions to show
}
```

**Backend Integration:**
- Network View → SX9 Gateway WebSocket (port 18600)
- Task Graph → SurrealDB graph queries (port 8000)
- Supabase → PostgREST API (port 3000)
- SurrealDB → SurrealDB WebSocket (port 8000)
- GIS Layers → Mapbox GL integration
- OSINT Nodes → GLAF graph server (port 18050)

#### 1.2 DatabaseConnectionPanel Component
**Priority:** HIGH  
**Impact:** Eliminates 40 duplicate implementations  
**Time:** 1-2 days

**Actions:**
- Supabase connection/status
- SurrealDB connection/status
- Sled KVR interface
- SlotGraph integration
- Connection health monitoring

**Implementation:**
```typescript
// src/components/shared/DatabaseConnectionPanel.tsx
interface DatabaseConnectionPanelProps {
  databases: ('supabase' | 'surrealdb' | 'sled' | 'slotgraph')[];
  onConnect?: (db: string) => void;
  onDisconnect?: (db: string) => void;
  showStatus?: boolean;
}
```

#### 1.3 FilterPanel Component
**Priority:** HIGH  
**Impact:** Eliminates 20 duplicate implementations  
**Time:** 1-2 days

**Features:**
- Advanced filtering UI
- Sector/region selection
- Data source filtering
- Time range selection
- HD4 phase filtering

---

### **Phase 2: Core Visualizations** (Week 2-3)
**Goal:** Implement critical visualization components

#### 2.1 NetworkView Component
**Priority:** CRITICAL  
**Time:** 3-4 days

**Features:**
- Network topology visualization
- Node/edge rendering
- Interactive exploration
- Integration with SX9 Gateway for real-time data

**Tech Stack:**
- React Flow or Cytoscape.js for graph rendering
- WebSocket connection to SX9 Gateway
- SurrealDB for graph data

#### 2.2 TaskGraph Component
**Priority:** CRITICAL  
**Time:** 3-4 days

**Features:**
- Task dependency graph
- HD4 phase visualization
- Task execution flow
- Interactive task selection

**Data Source:**
- Supabase `ctas_tasks` table
- Task relationships (predecessors/successors)
- HD4 phase grouping

#### 2.3 GISLayers Component
**Priority:** HIGH  
**Time:** 2-3 days

**Features:**
- Mapbox GL integration
- Layer toggles (OSINT, Threat Intel, Infrastructure)
- GeoIP visualization
- GEE (KMZ) import/display

**Integration:**
- Mapbox token (already configured)
- GeoIP service
- Google Earth Engine API

---

### **Phase 3: Database Integrations** (Week 3-4)
**Goal:** Connect all database buttons to actual services

#### 3.1 Supabase Integration
**Priority:** HIGH  
**Time:** 1 day

**Status:** ✅ Already connected (PostgREST on port 3000)  
**Needs:** UI status indicator and connection management

#### 3.2 SurrealDB Integration
**Priority:** HIGH  
**Time:** 2-3 days

**Features:**
- WebSocket connection (port 8000)
- Graph query interface
- Real-time data streaming
- Query builder UI

#### 3.3 Sled KVR Interface
**Priority:** MEDIUM  
**Time:** 1-2 days

**Features:**
- Key-value store browser
- Search and filter
- Export functionality

#### 3.4 SlotGraph Integration
**Priority:** MEDIUM  
**Time:** 2-3 days

**Features:**
- SlotGraph engine connection
- Graph operations
- Query interface

---

### **Phase 4: Page-Specific Actions** (Week 4-5)
**Goal:** Implement unique actions per page

#### 4.1 Dashboard Actions
**Priority:** HIGH  
**Time:** 2-3 days

- [ ] **Send** - Chat message send (Natasha integration)
- [ ] Connect all DataVisualizationToolbar actions

#### 4.2 Info Streams Actions
**Priority:** HIGH  
**Time:** 2-3 days

- [ ] **All/Critical/High/Medium** - Priority filtering
- [ ] **List View / Map View** - View mode toggle
- [ ] Connect DataVisualizationToolbar

#### 4.3 Containers Actions
**Priority:** MEDIUM  
**Time:** 2-3 days

- [ ] **Simulate Exploit** (7 instances) - Container exploit simulation
- [ ] Integration with Kali Plasma agent
- [ ] Connect DataVisualizationToolbar

#### 4.4 Database Page Actions
**Priority:** HIGH  
**Time:** 2-3 days

- [ ] **Create New Project** - Project creation modal
- [ ] **Backup Database** - Database backup functionality
- [ ] **View Analytics** - Analytics dashboard
- [ ] **Bevy Interface** - Bevy ECS integration

#### 4.5 Map Page Actions
**Priority:** MEDIUM  
**Time:** 1-2 days

- [ ] **Diagnostic** - Map diagnostics
- [ ] **Direct Test** - Direct map test
- [ ] **Show Test** - Test results display

---

### **Phase 5: OSINT & Intelligence** (Week 5-6)
**Goal:** Implement intelligence-related features

#### 5.1 OSINT Nodes Component
**Priority:** HIGH  
**Time:** 3-4 days

**Features:**
- OSINT node visualization
- GLAF graph integration
- Real-time OSINT feed
- Node relationship mapping

**Integration:**
- GLAF graph server (port 18050)
- OSINT correlation engine
- Neo4j for graph data

#### 5.2 Threat Intel Component
**Priority:** HIGH  
**Time:** 3-4 days

**Features:**
- Threat intelligence display
- MITRE ATT&CK mapping
- Threat correlation
- Real-time threat feeds

**Integration:**
- Plasma Defender ANN
- Threat content fetcher
- ChromaDB for semantic search

#### 5.3 Infrastructure View
**Priority:** MEDIUM  
**Time:** 2-3 days

**Features:**
- Infrastructure topology
- Asset inventory
- Network mapping
- Health monitoring

---

## 🔧 Technical Implementation Details

### **Shared Component Architecture**

```
src/components/shared/
├── DataVisualizationToolbar.tsx    # 13 common actions
├── DatabaseConnectionPanel.tsx     # Database connections
├── FilterPanel.tsx                 # Advanced filtering
├── NetworkView.tsx                  # Network topology
├── TaskGraph.tsx                   # Task dependencies
└── GISLayers.tsx                   # Map layers
```

### **Backend Service Integration**

| Action | Backend Service | Port | Protocol |
|--------|----------------|------|----------|
| Network View | SX9 Gateway | 18600 | WebSocket |
| Task Graph | SurrealDB | 8000 | WebSocket |
| Supabase | PostgREST | 3000 | REST |
| SurrealDB | SurrealDB | 8000 | WebSocket |
| OSINT Nodes | GLAF Server | 18050 | HTTP |
| Threat Intel | Plasma Defender | 18180 | HTTP |
| GIS Layers | Mapbox GL | - | API |
| GEE (KMZ) | Google Earth Engine | - | API |

### **State Management**

**Recommended:** React Context + Zustand for:
- Database connection state
- Filter state (shared across pages)
- View mode state (List/Map/Network)
- Selected data sources

---

## 📅 Implementation Timeline

### **Week 1: Foundation**
- Day 1-2: DataVisualizationToolbar component
- Day 3-4: DatabaseConnectionPanel component
- Day 5: FilterPanel component
- **Deliverable:** 3 shared components ready

### **Week 2: Core Visualizations**
- Day 1-3: NetworkView component
- Day 4-5: TaskGraph component
- **Deliverable:** 2 critical visualization components

### **Week 3: Database & GIS**
- Day 1: Supabase UI integration
- Day 2-3: SurrealDB integration
- Day 4-5: GISLayers component
- **Deliverable:** All database connections working

### **Week 4: Page-Specific Actions**
- Day 1-2: Dashboard actions
- Day 3: Info Streams actions
- Day 4-5: Containers actions
- **Deliverable:** Major pages functional

### **Week 5: Intelligence Features**
- Day 1-3: OSINT Nodes component
- Day 4-5: Threat Intel component
- **Deliverable:** Intelligence features operational

### **Week 6: Polish & Integration**
- Day 1-2: Infrastructure view
- Day 3-4: Map page actions
- Day 5: End-to-end testing
- **Deliverable:** All critical actions implemented

---

## 🎯 Success Metrics

### **Phase 1 Success Criteria**
- ✅ DataVisualizationToolbar appears on all 10 pages
- ✅ All 13 common actions wired to backend
- ✅ DatabaseConnectionPanel shows connection status

### **Phase 2 Success Criteria**
- ✅ Network View displays topology
- ✅ Task Graph shows dependencies
- ✅ GIS Layers toggle on map

### **Phase 3 Success Criteria**
- ✅ All database buttons show connection status
- ✅ SurrealDB queries work
- ✅ Supabase operations functional

### **Phase 4 Success Criteria**
- ✅ Dashboard chat send works
- ✅ Info Streams filtering works
- ✅ Container exploit simulation works

### **Phase 5 Success Criteria**
- ✅ OSINT nodes visualize on map/graph
- ✅ Threat Intel displays MITRE mappings
- ✅ Infrastructure view shows assets

---

## 🚀 Quick Wins (Do First)

### **Immediate Actions** (This Week)

1. **Create DataVisualizationToolbar** (2-3 days)
   - Single component, used everywhere
   - Eliminates 130 duplicate implementations
   - High visual impact

2. **Wire Supabase Button** (1 hour)
   - Already connected, just needs UI indicator
   - Shows immediate progress

3. **Implement Network View** (3-4 days)
   - Critical visualization
   - High user value
   - Foundation for other views

4. **Task Graph Component** (3-4 days)
   - Uses existing task data
   - Visual impact
   - Foundation for dependencies

---

## 📝 Implementation Checklist

### **Shared Components**
- [ ] DataVisualizationToolbar.tsx
- [ ] DatabaseConnectionPanel.tsx
- [ ] FilterPanel.tsx
- [ ] NetworkView.tsx
- [ ] TaskGraph.tsx
- [ ] GISLayers.tsx

### **Backend Integrations**
- [ ] SX9 Gateway WebSocket client
- [ ] SurrealDB WebSocket client
- [ ] GLAF Server HTTP client
- [ ] Plasma Defender HTTP client
- [ ] Mapbox GL integration
- [ ] Google Earth Engine API

### **Page-Specific**
- [ ] Dashboard chat send
- [ ] Info Streams filtering
- [ ] Containers exploit simulation
- [ ] Database project creation
- [ ] Map diagnostics

### **Testing**
- [ ] Component unit tests
- [ ] Integration tests
- [ ] E2E tests for critical flows
- [ ] Performance testing

---

## 🔗 Related Documentation

- `BUTTONS-FUNCTIONS-WORK-REQUIRED.md` - Full gap analysis
- `capability-audit/UI_CAPABILITY_MANIFEST.md` - Original manifest
- `docs/SX9-GATEWAY-TASK-GRAPH.md` - Gateway architecture
- `docs/PLASMA-TECHNICAL-ASSESSMENT.md` - Plasma Defender integration

---

## 💡 Key Principles

1. **Reusability First** - Build shared components before page-specific
2. **Backend Integration** - Connect to existing services (don't rebuild)
3. **Progressive Enhancement** - Get basic functionality working, then enhance
4. **User Value** - Prioritize actions users will actually use
5. **Consistency** - Use same patterns across all pages

---

## 🎯 Next Immediate Steps

1. **Today:** Create DataVisualizationToolbar component skeleton
2. **This Week:** Implement Network View and Task Graph
3. **Next Week:** Wire all database connections
4. **Week 3:** Complete page-specific actions
5. **Week 4+:** Intelligence features and polish

**Estimated Total Time:** 4-6 weeks for all critical actions  
**Quick Win Time:** 1 week for shared components + Network View



