# Threat Intelligence → CTAS Integration Plan

**Date:** December 7, 2025  
**Status:** 📋 **READY FOR IMPLEMENTATION**  
**Goal:** Integrate comprehensive threat intelligence into CTAS-7

---

## 🎯 **OBJECTIVE**

Enable CTAS-7 to access, analyze, and visualize threat intelligence data from 27,606+ threat items stored in Supabase and CDN.

---

## 📊 **CURRENT THREAT INTELLIGENCE INFRASTRUCTURE**

### **Data Available (After Pipeline Completes)**
- 27,606 threat items (tools, techniques, rules)
- SPIRES ontology (50-70 MB)
- DSL conversion (40-60 MB)
- Task graph (45-65 MB)
- Dual-trivariate hashes (RFC-9001/9002)

### **Storage**
- Supabase (primary, 500-800 MB)
- Neo4j (graph relationships)
- Cloudflare R2 (CDN, 200-300 MB)
- GCP Cloud CDN (private, 100-150 MB)

---

## 🔧 **INTEGRATION STEPS**

### **Step 1: Create Threat Intel Service Layer** (2-3 hours)

**File:** `sx9-ops-main-platform/src/services/threatIntelService.ts`

```typescript
import { supabase } from '@/utils/supabaseClient';

export class ThreatIntelService {
  // MITRE ATT&CK
  async getTechniques(domain?: 'enterprise' | 'ics' | 'mobile'): Promise<Technique[]> {
    const { data } = await supabase
      .from('threat_tools')
      .select('*')
      .eq('type', 'technique')
      .eq('domain', domain || 'enterprise');
    return data || [];
  }
  
  async getTechniqueById(id: string): Promise<Technique | null> {
    const { data } = await supabase
      .from('threat_tools')
      .select('*')
      .eq('hash_id', id)
      .single();
    return data;
  }
  
  // Detection Rules
  async getSigmaRules(filters?: RuleFilters): Promise<SigmaRule[]> {
    const query = supabase.from('threat_detection_rules').select('*');
    if (filters?.level) query.eq('level', filters.level);
    if (filters?.status) query.eq('status', filters.status);
    const { data } = await query;
    return data || [];
  }
  
  // Task Graph
  async getTaskGraph(hd4Phase?: string): Promise<TaskGraph> {
    const query = supabase.from('threat_task_graph').select('*');
    if (hd4Phase) query.eq('hd4_phase', hd4Phase);
    const { data } = await query;
    return { nodes: data || [], edges: [] };
  }
  
  // SPIRES Ontology
  async queryOntology(term: string): Promise<OntologyTerm | null> {
    const { data } = await supabase
      .from('threat_ontology')
      .select('*')
      .ilike('term', `%${term}%`)
      .single();
    return data;
  }
  
  // Trivariate Hashes
  async lookupHash(hash: string): Promise<HashLookupResult | null> {
    const { data } = await supabase
      .from('threat_tools')
      .select('*')
      .or(`primary_hash.eq.${hash},secondary_hash.eq.${hash}`)
      .single();
    return data;
  }
  
  // CDN Access
  async getFromCDN(path: string): Promise<any> {
    // Fetch from Cloudflare R2 or GCP CDN
    const response = await fetch(`https://threat-intel.sx9.io/${path}`);
    return response.json();
  }
}
```

### **Step 2: Create Database Schemas** (1-2 hours)

**File:** `sx9-ops-main-platform/supabase/migrations/YYYYMMDD_create_threat_intel_schema.sql`

```sql
-- Threat Tools Table
CREATE TABLE IF NOT EXISTS threat_tools (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  hash_id TEXT UNIQUE NOT NULL,  -- Primary trivariate hash
  task_name TEXT NOT NULL,
  description TEXT,
  category TEXT,
  hd4_phase TEXT CHECK (hd4_phase IN ('Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate')),
  primitive_type TEXT,
  primary_hash TEXT,  -- RFC-9001 primary hash
  secondary_hash TEXT,  -- RFC-9001 secondary hash
  unicode_operation TEXT,  -- RFC-9002 Unicode operation
  task_graph_node JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Threat Ontology Table
CREATE TABLE IF NOT EXISTS threat_ontology (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  term TEXT NOT NULL,
  category TEXT,
  relationships JSONB,
  spires_metadata JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Threat Detection Rules Table
CREATE TABLE IF NOT EXISTS threat_detection_rules (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  rule_id TEXT UNIQUE NOT NULL,
  rule_type TEXT CHECK (rule_type IN ('sigma', 'yara', 'nuclei', 'wazuh')),
  title TEXT,
  description TEXT,
  level TEXT,
  status TEXT,
  mitre_refs TEXT[],
  rule_content JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Threat Task Graph Table
CREATE TABLE IF NOT EXISTS threat_task_graph (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  hash_id TEXT UNIQUE NOT NULL,
  task_name TEXT NOT NULL,
  hd4_phase TEXT,
  predecessors TEXT[],
  successors TEXT[],
  p_probability FLOAT,
  t_time FLOAT,
  h_hazard FLOAT,
  node_data JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_threat_tools_hash_id ON threat_tools(hash_id);
CREATE INDEX IF NOT EXISTS idx_threat_tools_hd4_phase ON threat_tools(hd4_phase);
CREATE INDEX IF NOT EXISTS idx_threat_ontology_term ON threat_ontology(term);
CREATE INDEX IF NOT EXISTS idx_threat_detection_rules_type ON threat_detection_rules(rule_type);
```

### **Step 3: Wire TaskGraph into VisualizationManager** (1 hour)

Already done! Just verify it's working.

### **Step 4: Add Threat Intel Components to Gallery** (2-3 hours)

**File:** `sx9-ops-main-platform/src/pages/Gallery.tsx`

Add 12 Threat Intel components:
1. Threat Intelligence Dashboard (Pro)
2. MITRE ATT&CK Navigator (Basic)
3. Detection Rules Manager (Pro)
4. Adversary Emulation (Pro)
5. Threat Graph Visualization (Pro)
6. IOC Enrichment (Basic)
7. Threat Hunting (Enterprise)
8. Threat Intelligence Correlation (Enterprise)
9. SPIRES Ontology Browser (Pro)
10. DSL Playbook Editor (Enterprise)
11. Trivariate Hash Explorer (Pro)
12. Threat Intelligence API (Enterprise)

### **Step 5: Create Threat Intel Dashboard** (3-4 hours)

**File:** `sx9-ops-main-platform/src/pages/ThreatIntel.tsx`

- Real-time threat feed
- Threat level summary
- Source aggregation
- Task graph visualization
- MITRE ATT&CK navigator

### **Step 6: Integrate with HD4PhaseContent** (1 hour)

Add Threat Intel tab to horizon tabs.

---

## 📋 **IMPLEMENTATION CHECKLIST**

- [ ] Create `threatIntelService.ts`
- [ ] Create database schemas (Supabase migrations)
- [ ] Load threat data to Supabase (after pipeline completes)
- [ ] Wire TaskGraph to VisualizationManager
- [ ] Add 12 Threat Intel components to Gallery
- [ ] Create Threat Intel dashboard page
- [ ] Add Threat Intel tab to HD4PhaseContent
- [ ] Test Supabase queries
- [ ] Test CDN access
- [ ] Test task graph visualization

---

## ⏱️ **ESTIMATED TIME: 12-15 hours**

---

## 📄 **See Also:**
- `THREAT_TOOLS_SPIRES_PLAN.md` (detailed plan)
- `STORAGE_STRATEGY.md` (storage plan)


