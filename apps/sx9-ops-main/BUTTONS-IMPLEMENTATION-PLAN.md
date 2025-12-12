# Buttons & Functions Implementation Plan

**Date:** 2025-12-07  
**Status:** ACTION PLAN  
**Total Gaps:** ~285 actions (95% of total)  
**Target:** Complete critical actions in 4-6 weeks

---

## 📊 Current State

### ✅ **Working** (15 actions - 5%)
- Phase navigation tabs (Overview, Kali Tools, Playbooks, Red Team, Phase Mapping, Tasks)
- Show CLI button
- Agent tabs (Natasha, Marcus, Elena, Cove, Kali)
- Chat + Terminal views
- Task loading and display

### ❌ **Needs Implementation** (285 actions - 95%)
- Data visualization suite (13 actions × 10 pages = 130 instances)
- Database connections (4 actions × 10 pages = 40 instances)
- Filter/sector controls (2 actions × 10 pages = 20 instances)
- Page-specific actions (95 unique actions)

---

## 🎯 Implementation Strategy

### **Phase 1: Shared Components** (Week 1-2)
**Goal:** Create reusable components to eliminate 130+ duplicate implementations

#### **1.1 DataVisualizationToolbar** (Priority: CRITICAL)
**Impact:** Eliminates 130 duplicate button implementations  
**Time:** 2-3 days

**Actions to implement:**
1. Network View
2. Task Graph
3. Sectors
4. Filters
5. Data Sources
6. GIS Layers
7. OSINT Nodes
8. Threat Intel
9. Infrastructure
10. GeoIP
11. Supabase
12. SurrealDB
13. GEE (KMZ)

**Implementation:**
```typescript
// src/components/shared/toolbars/DataVisualizationToolbar.tsx
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
  variant?: 'default' | 'compact' | 'minimal';
  enabledActions?: string[]; // Whitelist which actions to show
}
```

**Backend Integration:**
- Network View → SX9 Gateway WebSocket (port 18600)
- Task Graph → SurrealDB graph queries (port 8000)
- Supabase → PostgREST API (port 3000)
- SurrealDB → SurrealQL WebSocket (port 8000)
- OSINT Nodes → Neo4j Cypher queries (port 7474)
- Threat Intel → Supabase tasks table

**Deliverable:** Single reusable component used on all 10 pages

---

#### **1.2 DatabaseConnectionPanel** (Priority: HIGH)
**Impact:** Eliminates 40 duplicate database connection buttons  
**Time:** 1-2 days

**Databases:**
- Supabase (PostgREST API)
- SurrealDB (SurrealQL WebSocket)
- Sled KVR (Local file system)
- SlotGraph (Rust crate API)

**Implementation:**
```typescript
// src/components/shared/toolbars/DatabaseConnectionPanel.tsx
interface DatabaseConnectionPanelProps {
  databases: DatabaseConfig[];
  onConnect: (dbId: string) => Promise<void>;
  onDisconnect: (dbId: string) => Promise<void>;
  onRefresh: (dbId: string) => Promise<void>;
  showStatus?: boolean;
  compact?: boolean;
}
```

**Deliverable:** Unified database connection UI with status monitoring

---

#### **1.3 FilterPanel** (Priority: HIGH)
**Impact:** Eliminates 20 duplicate filter implementations  
**Time:** 1-2 days

**Features:**
- Phase filtering (Hunt, Detect, Disrupt, Disable, Dominate)
- Sector/region filtering
- Date range filtering
- Priority filtering
- Status filtering

**Implementation:**
```typescript
// src/components/shared/toolbars/FilterPanel.tsx
interface FilterPanelProps {
  filters: FilterConfig[];
  onFilterChange: (filters: ActiveFilters) => void;
  onReset: () => void;
  variant?: 'sidebar' | 'dropdown' | 'modal';
}
```

**Deliverable:** Reusable filter component with advanced options

---

### **Phase 2: Core Visualizations** (Week 2-3)
**Goal:** Implement critical visualization components

#### **2.1 NetworkView Component** (Priority: CRITICAL)
**Impact:** High user value, foundation for other views  
**Time:** 3-4 days

**Features:**
- Network topology visualization
- Node/edge rendering
- Interactive zoom/pan
- Node details on click
- Real-time updates via WebSocket

**Backend:**
- SX9 Gateway WebSocket (port 18600)
- Neo4j for graph data (port 7474)
- GLAF Server for analytics (port 18050)

**Technology:**
- D3.js or vis.js for graph rendering
- WebSocket client for real-time updates

**Deliverable:** Functional network topology viewer

---

#### **2.2 TaskGraph Component** (Priority: CRITICAL)
**Impact:** Essential for task dependency visualization  
**Time:** 3-4 days

**Features:**
- Task dependency graph
- Phase-based coloring
- Interactive node selection
- Task details panel
- Filter by phase/category

**Backend:**
- SurrealDB for task relationships (port 8000)
- Supabase for task data (port 3000)

**Technology:**
- D3.js for graph layout
- React for component integration

**Deliverable:** Task dependency graph with filtering

---

### **Phase 3: Database Integrations** (Week 3-4)
**Goal:** Wire up database connection buttons to actual services

#### **3.1 Supabase Integration** (Priority: HIGH)
**Time:** 1 day

**Actions:**
- Connection status indicator
- Query execution UI
- Table browser
- Query builder

**Backend:**
- PostgREST API (port 3000)
- Real-time subscriptions

**Deliverable:** Functional Supabase connection panel

---

#### **3.2 SurrealDB Integration** (Priority: HIGH)
**Time:** 1-2 days

**Actions:**
- WebSocket connection
- SurrealQL query interface
- Graph visualization
- Table/record browser

**Backend:**
- SurrealDB WebSocket (port 8000)
- SurrealQL query execution

**Deliverable:** Functional SurrealDB connection panel

---

#### **3.3 Sled KVR Integration** (Priority: MEDIUM)
**Time:** 1 day

**Actions:**
- Local file system access
- Key-value browser
- Search functionality
- Export/import

**Backend:**
- Rust API bridge (via WASM or HTTP)

**Deliverable:** Sled KVS browser interface

---

#### **3.4 SlotGraph Integration** (Priority: MEDIUM)
**Time:** 1-2 days

**Actions:**
- Graph engine connection
- Slot visualization
- Hash → slot mapping
- Query interface

**Backend:**
- SlotGraph Rust crate API

**Deliverable:** SlotGraph management interface

---

### **Phase 4: Page-Specific Actions** (Week 4-5)
**Goal:** Implement unique actions per page

#### **4.1 Dashboard Actions**
- [ ] Chat send button
- [ ] Info stream filtering
- [ ] Real-time updates

**Time:** 1-2 days

---

#### **4.2 Info Streams Actions**
- [ ] Priority filtering (Critical, High, Medium)
- [ ] List/Map view toggle
- [ ] Stream details modal

**Time:** 1-2 days

---

#### **4.3 Containers Actions**
- [ ] Simulate Exploit (7 instances)
- [ ] Container status monitoring
- [ ] Exploit result visualization

**Time:** 2-3 days

---

#### **4.4 Database Page Actions**
- [ ] Create New Project
- [ ] Backup Database
- [ ] View Analytics
- [ ] Bevy Interface

**Time:** 2-3 days

---

#### **4.5 Map Actions**
- [ ] Diagnostic
- [ ] Direct Test
- [ ] Show Test Results

**Time:** 1-2 days

---

### **Phase 5: Advanced Features** (Week 5-6)
**Goal:** Implement advanced visualization and integration features

#### **5.1 GIS Layers**
- [ ] Map layer toggles
- [ ] Layer management
- [ ] Custom layer upload

**Time:** 2-3 days

---

#### **5.2 OSINT Nodes**
- [ ] OSINT node visualization
- [ ] Node details panel
- [ ] Data source integration

**Time:** 2-3 days

---

#### **5.3 Threat Intel**
- [ ] MITRE ATT&CK mapping
- [ ] Threat actor profiles
- [ ] Attack pattern visualization

**Time:** 2-3 days

---

#### **5.4 Infrastructure View**
- [ ] Asset visualization
- [ ] Network topology
- [ ] Status monitoring

**Time:** 2-3 days

---

#### **5.5 GEE (KMZ) Integration**
- [ ] Google Earth Engine API
- [ ] KMZ file upload
- [ ] 3D visualization

**Time:** 3-4 days

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

## 📋 Implementation Checklist

### **Week 1-2: Shared Components**
- [ ] DataVisualizationToolbar.tsx
- [ ] DatabaseConnectionPanel.tsx
- [ ] FilterPanel.tsx
- [ ] Integrate toolbar on all 10 pages

### **Week 2-3: Core Visualizations**
- [ ] NetworkView.tsx
- [ ] TaskGraph.tsx
- [ ] Backend WebSocket clients
- [ ] Real-time update integration

### **Week 3-4: Database Integrations**
- [ ] Supabase connection panel
- [ ] SurrealDB connection panel
- [ ] Sled KVR interface
- [ ] SlotGraph interface

### **Week 4-5: Page-Specific**
- [ ] Dashboard chat send
- [ ] Info Streams filtering
- [ ] Containers exploit simulation
- [ ] Database project creation
- [ ] Map diagnostics

### **Week 5-6: Advanced Features**
- [ ] GIS Layers
- [ ] OSINT Nodes
- [ ] Threat Intel
- [ ] Infrastructure View
- [ ] GEE (KMZ) Integration

---

## 🔧 Technical Requirements

### **Backend Services**
- SX9 Gateway (port 18600) - WebSocket API
- Supabase (port 3000) - PostgREST API
- SurrealDB (port 8000) - SurrealQL WebSocket
- Neo4j (port 7474) - Cypher queries
- GLAF Server (port 18050) - Graph analytics
- Thalamic Filter (port 18114) - DistilBERT classification

### **Frontend Libraries**
- D3.js - Graph visualization
- vis.js - Alternative graph library
- Mapbox GL - Map rendering
- React Query - Data fetching
- WebSocket client - Real-time updates

### **Component Architecture**
```
src/components/
├── shared/
│   ├── toolbars/
│   │   ├── DataVisualizationToolbar.tsx
│   │   ├── DatabaseConnectionPanel.tsx
│   │   └── FilterPanel.tsx
│   └── visualizations/
│       ├── NetworkView.tsx
│       ├── TaskGraph.tsx
│       └── VisualizationManager.tsx
└── pages/
    ├── Dashboard/
    ├── HD4PhaseContent/
    └── ...
```

---

## 📈 Success Metrics

### **Phase 1 Success Criteria**
- ✅ DataVisualizationToolbar used on all 10 pages
- ✅ 130 duplicate implementations eliminated
- ✅ Consistent UI across all pages

### **Phase 2 Success Criteria**
- ✅ Network View displays topology
- ✅ Task Graph shows dependencies
- ✅ Real-time updates working

### **Phase 3 Success Criteria**
- ✅ All database buttons functional
- ✅ Connection status visible
- ✅ Query interfaces working

### **Phase 4 Success Criteria**
- ✅ All page-specific actions implemented
- ✅ User workflows complete

### **Phase 5 Success Criteria**
- ✅ OSINT nodes visualize on map/graph
- ✅ Threat Intel displays MITRE mappings
- ✅ Infrastructure view shows assets

---

## 💡 Key Principles

1. **Reusability First** - Build shared components before page-specific
2. **Backend Integration** - Connect to existing services (don't rebuild)
3. **Progressive Enhancement** - Get basic functionality working, then enhance
4. **User Value** - Prioritize actions users will actually use
5. **Consistency** - Use shared components for common actions

---

## 🔗 Related Documentation

- `BUTTONS-FUNCTIONS-WORK-REQUIRED.md` - Full gap analysis
- `IMPLEMENTATION-PLAN.md` - Previous implementation plan
- `META-COMPONENTS-GUIDE.md` - Meta components documentation
- `capability-audit/UI_CAPABILITY_MANIFEST.md` - Original manifest

---

## 🎯 Next Steps

1. **Start with DataVisualizationToolbar** (This Week)
   - Create component structure
   - Implement basic actions
   - Integrate on one page as proof of concept

2. **Wire Supabase Button** (Today)
   - Quick win to show progress
   - 1 hour implementation

3. **Plan Network View** (This Week)
   - Design component architecture
   - Identify backend integration points
   - Create mockup/prototype

---

**Ready to start? Begin with DataVisualizationToolbar - it's the highest impact, lowest effort component that will eliminate 130+ duplicate implementations.**



