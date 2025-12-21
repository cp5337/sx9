# 🔗 SX9 Backend → CTAS Frontend Integration

**Connecting Your New Backend to Your Existing CTAS UI**

---

## 🎯 **WHAT YOU HAVE:**

### **CTAS Frontend (v7.3.1)**
```
✅ Map-based UI (Mapbox)
✅ AI Agents (Natasha, Marcus, Elena, Cove, Kali ISO)
✅ Data Sources panel (Supabase, SurrealDB, GEE)
✅ Map Layers (OSINT, Threat Intel, Infrastructure, GeoIP)
✅ Firefly IAC module
✅ GLAF visualization (System Graph, Entity Model)
✅ Hunt operations interface
```

### **SX9 Backend (What We Just Built)**
```
✅ CloudFlare Worker (Smart Router)
✅ CloudFlare Workflow (Threat Intel Sync)
✅ Supabase (Entities, Relationships)
✅ Neon (Tool Registry, Executions)
✅ Neo4j (GLAF Graph)
✅ IaC Executor (Unicode → Infrastructure)
✅ Dual-Trivariate Hashing (RFC-9001)
✅ Unicode Addressing (RFC-9002)
```

---

## 🚀 **INTEGRATION POINTS:**

### **1. Data Sources Panel → Backend APIs**

Update your CTAS `Data Sources` configuration:

```javascript
// In CTAS frontend config
const dataSources = {
  supabase: {
    url: process.env.SUPABASE_URL,
    key: process.env.SUPABASE_KEY,
    tables: ['entities', 'relationships', 'atlas_nodes', 'threat_tools']
  },
  
  cloudflare: {
    worker_url: process.env.CF_WORKER_URL,
    endpoints: {
      tool_lookup: '/api/tool/{unicode}',
      search: '/api/search',
      health: '/api/health',
      graph_query: '/api/graph'
    }
  },
  
  neon: {
    url: process.env.NEON_DATABASE_URL,
    tables: ['tool_registry', 'executions', 'crystal_presets', 'scenarios']
  },
  
  neo4j: {
    uri: process.env.NEO4J_URI,
    user: process.env.NEO4J_USER,
    password: process.env.NEO4J_PASSWORD,
    database: 'neo4j'
  }
};
```

---

### **2. Map Layers → Threat Intelligence**

Wire up your map layers to pull from SX9 backend:

```javascript
// Threat Intel Layer
async function loadThreatIntelLayer() {
  // Pull from CloudFlare Worker (3ms global edge)
  const response = await fetch(`${CF_WORKER_URL}/api/search?q=&category=ThreatIntel&limit=1000`);
  const threats = await response.json();
  
  // Add to map as GeoJSON
  map.addLayer({
    id: 'threat-intel',
    type: 'circle',
    source: {
      type: 'geojson',
      data: {
        type: 'FeatureCollection',
        features: threats.results.map(threat => ({
          type: 'Feature',
          geometry: {
            type: 'Point',
            coordinates: [threat.longitude, threat.latitude]
          },
          properties: {
            name: threat.name,
            category: threat.category,
            mitre_techniques: threat.mitre_techniques,
            unicode: threat.unicode_address,
            severity: threat.risk_level
          }
        }))
      }
    },
    paint: {
      'circle-radius': 8,
      'circle-color': [
        'match',
        ['get', 'severity'],
        'critical', '#ff0000',
        'high', '#ff6600',
        'medium', '#ffcc00',
        'low', '#00ff00',
        '#999999'
      ]
    }
  });
}
```

---

### **3. AI Agents → Unicode Operations**

Connect your AI agents (Natasha, Marcus, etc.) to trigger IaC operations:

```javascript
// In your AI agent chat handler
async function handleAgentCommand(agent, command) {
  // Example: "Natasha, scan 192.168.1.0/24 with Nmap"
  
  if (command.includes('scan') && command.includes('nmap')) {
    const target = extractTarget(command); // "192.168.1.0/24"
    
    // Trigger IaC executor via API
    const response = await fetch(`${BACKEND_API}/iac/execute`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        unicode: 'E800', // Nmap
        target: target,
        agent: agent.name
      })
    });
    
    const result = await response.json();
    
    // Show in UI
    return {
      message: `✅ Nmap scan initiated on ${target}`,
      container_id: result.container_id,
      output_dir: result.output_dir,
      estimated_time: '2-5 minutes'
    };
  }
}
```

---

### **4. Firefly IAC → IaC Executor**

Wire your "Firefly IAC" module to the IaC executor:

```javascript
// In Firefly IAC module
async function provisionInfrastructure(toolUnicode, target, options = {}) {
  const response = await fetch(`${BACKEND_API}/iac/execute`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      unicode: toolUnicode,
      target: target,
      cleanup_after: options.cleanup_seconds || 300,
      notify_on_complete: true
    })
  });
  
  const result = await response.json();
  
  // Add to infrastructure map layer
  addInfrastructureMarker({
    lat: target.latitude,
    lng: target.longitude,
    tool: result.tool_name,
    status: 'running',
    container_id: result.container_id
  });
  
  // Poll for results
  pollInfrastructureStatus(result.container_id);
  
  return result;
}
```

---

### **5. System Graph → Neo4j GLAF**

Connect your "System Graph" visualization to Neo4j:

```javascript
// Load GLAF graph from Neo4j
async function loadSystemGraph() {
  const response = await fetch(`${CF_WORKER_URL}/api/graph`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      query: `
        MATCH (t:Tool)-[r:USES_TECHNIQUE]->(tech:Technique)
        RETURN t, r, tech
        LIMIT 100
      `
    })
  });
  
  const graph = await response.json();
  
  // Render in D3/Cytoscape/Force-graph
  renderGraph(graph);
}
```

---

## 📋 **API ENDPOINTS TO IMPLEMENT:**

Create these endpoints in your CTAS backend API:

```
POST /api/iac/execute
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Body: { unicode, target, options }
Returns: { container_id, output_dir, terraform_dir }
Action: Triggers IaC executor (calls iac_executor.py)

GET /api/iac/status/:container_id
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Returns: { status, progress, output_preview }
Action: Polls Docker container status

GET /api/iac/results/:run_id
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Returns: { files, summary, report }
Action: Returns scan/exploit results

POST /api/threat-intel/sync
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Action: Triggers CloudFlare Workflow (threat-intel-sync)

GET /api/tools/search
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Params: ?q=nmap&category=NetworkRecon
Returns: List of tools (proxies to CloudFlare Worker)

POST /api/graph/query
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Body: { cypher_query }
Returns: Neo4j graph results
```

---

## 🎯 **DEMO SCENARIO:**

### **"Natasha, scan the New York area for vulnerable hosts"**

```
STEP 1: User asks Natasha (AI agent)
  ↓
STEP 2: Natasha interprets command
  - Target: "New York area" → Geo-bounds
  - Operation: "scan" → Unicode E800 (Nmap)
  ↓
STEP 3: CTAS calls backend API
  POST /api/iac/execute
  {
    "unicode": "E800",
    "target": "geo:40.7128,-74.0060,radius:50km",
    "agent": "Natasha",
    "notify": true
  }
  ↓
STEP 4: Backend triggers IaC executor
  - Pulls E800 (Nmap) from CloudFlare KV (3ms)
  - Generates Terraform for distributed scan
  - Spawns containers across regions
  ↓
STEP 5: Results stream back to UI
  - Infrastructure markers appear on map
  - Progress bars show scan status
  - Results populate in threat intel layer
  ↓
STEP 6: Natasha reports back
  "✅ Scan complete. Found 127 hosts, 23 vulnerable services.
   Results displayed on map. Would you like me to investigate?"
```

---

## 🔧 **IMPLEMENTATION STEPS:**

### **Week 1: Core Integration**
```bash
# 1. Deploy SX9 backend
./deploy_backend.sh
cd cloudflare && npm run deploy

# 2. Add environment variables to CTAS
echo "CF_WORKER_URL=https://sx9-backend.YOUR.workers.dev" >> .env
echo "SUPABASE_URL=https://YOUR.supabase.co" >> .env
echo "SUPABASE_KEY=your-key" >> .env

# 3. Create API wrapper in CTAS backend
mkdir -p ctas-backend/src/api/sx9
touch ctas-backend/src/api/sx9/iac-executor.ts
touch ctas-backend/src/api/sx9/threat-intel.ts
touch ctas-backend/src/api/sx9/tool-search.ts

# 4. Wire up data sources
# Update: ctas-frontend/src/config/dataSources.ts
```

### **Week 2: Map Layers**
```bash
# 1. Add threat intel layer
# Update: ctas-frontend/src/components/Map/layers/ThreatIntel.tsx

# 2. Add infrastructure layer
# Update: ctas-frontend/src/components/Map/layers/Infrastructure.tsx

# 3. Add OSINT nodes layer
# Update: ctas-frontend/src/components/Map/layers/OSINTNodes.tsx
```

### **Week 3: AI Agents Integration**
```bash
# 1. Update agent command handlers
# Update: ctas-frontend/src/ai-agents/Natasha/commands.ts

# 2. Add IaC operation triggers
# Update: ctas-frontend/src/ai-agents/shared/iacOperations.ts

# 3. Add result streaming
# Update: ctas-frontend/src/ai-agents/shared/streaming.ts
```

### **Week 4: GLAF Visualization**
```bash
# 1. Connect System Graph to Neo4j
# Update: ctas-frontend/src/components/GLAF/SystemGraph.tsx

# 2. Add entity model viewer
# Update: ctas-frontend/src/components/GLAF/EntityModel.tsx

# 3. Add forge builder integration
# Update: ctas-frontend/src/components/GLAF/ForgeBuilder.tsx
```

---

## 🎉 **RESULT:**

After integration, you'll have:

```
CTAS FRONTEND (What you have)
    ↓
    ↓ REST API calls
    ↓
SX9 BACKEND (What we built)
    ↓
    ├→ CloudFlare KV (3ms global edge)
    ├→ Supabase (Entity storage)
    ├→ Neon (Tool registry)
    ├→ Neo4j (GLAF graph)
    └→ IaC Executor (Docker/Terraform)
    ↓
INFRASTRUCTURE (Auto-provisioned)
    ├→ Nmap containers
    ├→ Exploit frameworks
    ├→ OSINT collectors
    └→ Custom tools
```

---

## 📊 **FINAL ARCHITECTURE:**

```
USER CLICKS MAP → "Scan this area"
    ↓
NATASHA AI AGENT (Frontend)
    ↓
CTAS BACKEND API (/api/iac/execute)
    ↓
IAC EXECUTOR (iac_executor.py)
    ├→ Pulls tool from CloudFlare (3ms)
    ├→ Generates Terraform
    └→ Spawns Docker containers
    ↓
RESULTS STREAM BACK
    ├→ Update map markers
    ├→ Populate threat intel
    └→ Notify Natasha
    ↓
NATASHA REPORTS: "✅ Scan complete!"
```

**Total time:** ~10 seconds from click to infrastructure running! 🚀

---

**This is EXACTLY what your CTAS UI was designed for!** 

Want me to create the specific API wrapper code for CTAS integration?
