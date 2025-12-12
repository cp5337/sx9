# Buttons & Functions Work Required

**Date:** 2025-01-27  
**Status:** All buttons/functions need implementation  
**Source:** `capability-audit/UI_CAPABILITY_MANIFEST.md`

---

## 📊 Summary

**Total Actions:** ~300+ buttons/functions across all pages  
**Status:** All unchecked `[ ]` - **ALL NEED IMPLEMENTATION**

---

## 🎯 Priority Actions by Page

### **Dashboard** (11 actions)
- [ ] **Send** - Chat message send button
- [ ] **Network View** - Network topology visualization
- [ ] **Task Graph** - Task dependency graph
- [ ] **Sectors** - Sector/region filtering
- [ ] **Filters** - Data filtering controls
- [ ] **Data Sources** - Data source selection
- [ ] **GIS Layers** - Map layer toggles
- [ ] **OSINT Nodes** - OSINT node visualization
- [ ] **Threat Intel** - Threat intelligence display
- [ ] **Infrastructure** - Infrastructure view
- [ ] **GeoIP** - GeoIP lookup
- [ ] **Supabase** - Supabase connection
- [ ] **SurrealDB** - SurrealDB connection
- [ ] **GEE (KMZ)** - Google Earth Engine integration

### **HD4 Phase Pages** (Hunt, Detect, Disrupt, Disable, Dominate)
Each phase has **~20 actions**:

#### **Core Phase Actions** (6 per phase)
- [ ] **Overview** - Phase overview tab ✅ (Working)
- [ ] **Kali Tools** - Kali tools integration ✅ (Working)
- [ ] **Playbooks** - Playbook management ✅ (Working)
- [ ] **Red Team** - Red team operations ✅ (Working)
- [ ] **Phase Mapping** - Phase relationship mapping ✅ (Working)
- [ ] **Tasks** - Task management ✅ (Working)

#### **Agent/CLI Actions** (3 per phase)
- [ ] **+** - Add agent/session
- [ ] **−** - Remove agent/session
- [ ] **Show CLI** - Toggle CLI panel ✅ (Working - just implemented)

#### **Data Visualization Actions** (11 per phase - **ALL NEED WORK**)
- [ ] **Network View** - Network topology
- [ ] **Task Graph** - Task dependency visualization
- [ ] **Sectors** - Sector filtering
- [ ] **Filters** - Advanced filtering
- [ ] **Data Sources** - Data source management
- [ ] **GIS Layers** - Map layer controls
- [ ] **OSINT Nodes** - OSINT visualization
- [ ] **Threat Intel** - Threat intelligence
- [ ] **Infrastructure** - Infrastructure view
- [ ] **GeoIP** - GeoIP services
- [ ] **Supabase** - Database connection
- [ ] **SurrealDB** - Graph database connection
- [ ] **GEE (KMZ)** - Google Earth Engine

### **Tasks Page** (13 actions)
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

### **Graph Visualization** (14 actions)
- [ ] **All Tasks** - Show all tasks
- [ ] **Hunt Phase** - Filter by phase
- [ ] **Detect Phase** - Filter by phase
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

### **Info Streams** (17 actions)
- [ ] **All (0)** - Show all streams
- [ ] **Critical (0)** - Critical priority filter
- [ ] **High (0)** - High priority filter
- [ ] **Medium (0)** - Medium priority filter
- [ ] **List View** - List display mode
- [ ] **Map View** - Map display mode
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

### **Containers** (14 actions)
- [ ] **Simulate Exploit** (7 instances) - Exploit simulation
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

### **Database** (17 actions)
- [ ] **Supabase** - Supabase connection/management
- [ ] **SurrealDB** - SurrealDB connection/management
- [ ] **Sled KVR** - Sled key-value store
- [ ] **SlotGraph** - SlotGraph integration
- [ ] **Bevy Interface** - Bevy ECS interface
- [ ] **Create New Project** - Project creation
- [ ] **Backup Database** - Database backup
- [ ] **View Analytics** - Analytics dashboard
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase** (duplicate)
- [ ] **SurrealDB** (duplicate)
- [ ] **GEE (KMZ)**

### **Map** (14 actions)
- [ ] **Diagnostic** - Map diagnostics
- [ ] **Direct Test** - Direct map test
- [ ] **Show Test** - Show test results
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

### **Settings** (13 actions)
- [ ] **Network View**
- [ ] **Task Graph**
- [ ] **Sectors**
- [ ] **Filters**
- [ ] **Data Sources**
- [ ] **GIS Layers**
- [ ] **OSINT Nodes**
- [ ] **Threat Intel**
- [ ] **Infrastructure**
- [ ] **GeoIP**
- [ ] **Supabase**
- [ ] **SurrealDB**
- [ ] **GEE (KMZ)**

---

## 🔄 Common Actions (Appear on Multiple Pages)

### **Data Visualization Suite** (13 actions - appears on ALL pages)
These are the most common and likely need a shared component:

1. **Network View** - Network topology visualization
2. **Task Graph** - Task dependency graph
3. **Sectors** - Sector/region filtering
4. **Filters** - Advanced filtering controls
5. **Data Sources** - Data source management
6. **GIS Layers** - Map layer toggles
7. **OSINT Nodes** - OSINT node visualization
8. **Threat Intel** - Threat intelligence display
9. **Infrastructure** - Infrastructure view
10. **GeoIP** - GeoIP lookup service
11. **Supabase** - Supabase database connection
12. **SurrealDB** - SurrealDB graph database
13. **GEE (KMZ)** - Google Earth Engine integration

**Recommendation:** Create a shared `DataVisualizationToolbar` component

---

## ✅ Currently Working

### **Phase Navigation** (Working)
- ✅ Overview tab
- ✅ Kali Tools tab
- ✅ Playbooks tab
- ✅ Red Team tab
- ✅ Phase Mapping tab
- ✅ Tasks tab
- ✅ Show CLI button (just implemented)

### **Agent Management** (Working)
- ✅ Agent tabs (Natasha, Marcus, Elena, Cove, Kali)
- ✅ Chat + Terminal split view
- ✅ Drag-and-drop for Kali tabs
- ✅ CLI persistence across tabs

### **Tasks** (Working)
- ✅ Task loading from Supabase
- ✅ Phase filtering
- ✅ Category expansion
- ✅ Task display

---

## 🎯 Implementation Priority

### **Priority 1: Core Functionality** (High Impact)
1. **Network View** - Critical for topology visualization
2. **Task Graph** - Essential for task dependencies
3. **Filters** - Needed for data management
4. **Data Sources** - Required for data integration

### **Priority 2: Database Connections** (Medium Impact)
1. **Supabase** - Already connected, needs UI
2. **SurrealDB** - Graph database integration
3. **Sled KVR** - Key-value store interface
4. **SlotGraph** - Graph engine integration

### **Priority 3: Visualization** (Medium Impact)
1. **GIS Layers** - Map layer controls
2. **OSINT Nodes** - OSINT visualization
3. **Threat Intel** - Threat display
4. **Infrastructure** - Infrastructure view

### **Priority 4: Advanced Features** (Lower Impact)
1. **GEE (KMZ)** - Google Earth Engine
2. **GeoIP** - GeoIP services
3. **Sectors** - Sector filtering
4. **Simulate Exploit** - Container exploit simulation

---

## 📝 Implementation Notes

### **Shared Components Needed**
1. **DataVisualizationToolbar** - Reusable toolbar for data viz actions
2. **DatabaseConnectionPanel** - Unified database connection UI
3. **FilterPanel** - Advanced filtering component
4. **NetworkViewComponent** - Network topology viewer
5. **TaskGraphComponent** - Task dependency graph

### **Backend Integration Points**
- **SX9 Gateway** (port 18600) - WebSocket API
- **Supabase** (port 3000) - PostgreSQL via PostgREST
- **SurrealDB** (port 8000) - Graph database
- **GLAF Server** (port 18050) - Graph analytics
- **Thalamic Filter** (port 18114) - DistilBERT classification

---

## 🔗 Related Files

- `capability-audit/UI_CAPABILITY_MANIFEST.md` - Full manifest
- `capability-audit/UI_CAPABILITY_MANIFEST.json` - JSON version
- `modal-inventory-foundation/inventory.json` - Playwright inventory
- `playwright-foundation-inventory.js` - Inventory script

---

## 📈 Progress Tracking

**Total Actions:** ~300+  
**Implemented:** ~15 (5%)  
**Remaining:** ~285 (95%)

**Next Steps:**
1. Create shared `DataVisualizationToolbar` component
2. Implement Network View
3. Implement Task Graph
4. Connect database buttons to actual services
5. Build filter panel component



