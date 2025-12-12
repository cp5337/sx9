-- Plasma Data Layer Schema Migration
-- RFC-9021 Cognitive Inference Engine Integration
-- Created: 2024-12-01
--
-- NVNN: Migration establishes Plasma threat intelligence tables with cognitive inference support

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- CORE TABLES
-- ============================================================================

-- Wazuh Manager Registry
-- NVNN: Managers coordinate agent deployments and sync operations
CREATE TABLE IF NOT EXISTS plasma_managers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  url TEXT NOT NULL,
  api_key_encrypted TEXT,
  enabled BOOLEAN DEFAULT true,
  agent_count INTEGER DEFAULT 0,
  last_sync TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Wazuh Agent Registry (maps to PTCC threat actors via trivariate_hash)
-- NVNN: Agents represent monitored endpoints with cognitive status tracking
CREATE TABLE IF NOT EXISTS plasma_agents (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  ip INET,
  status TEXT CHECK (status IN ('active', 'disconnected', 'never_connected', 'pending')) DEFAULT 'pending',
  os TEXT CHECK (os IN ('linux', 'windows', 'macos', 'unknown')) DEFAULT 'unknown',
  os_version TEXT,
  version TEXT,
  last_keep_alive TIMESTAMPTZ,
  groups TEXT[] DEFAULT '{}',
  manager_id UUID REFERENCES plasma_managers(id) ON DELETE SET NULL,
  register_date TIMESTAMPTZ DEFAULT NOW(),
  config_sum TEXT,
  merged_sum TEXT,
  alert_count INTEGER DEFAULT 0,
  -- RFC-9021 Cognitive fields
  trivariate_hash TEXT,  -- 48-char Base96 linking to PTCC/TETH data
  cognitive_status JSONB DEFAULT '{}',  -- Layer 1 thalamic output cache
  last_inference TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Threat Events (fed from PTCC attack_scenarios + OSINT)
-- NVNN: Threats store intelligence events with cognitive enrichment
CREATE TABLE IF NOT EXISTS plasma_threats (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  timestamp TIMESTAMPTZ DEFAULT NOW(),
  level TEXT CHECK (level IN ('critical', 'high', 'medium', 'low')) NOT NULL,
  source TEXT,
  target TEXT,
  description TEXT NOT NULL,
  indicators TEXT[] DEFAULT '{}',
  mitre TEXT[] DEFAULT '{}',
  confidence FLOAT CHECK (confidence >= 0 AND confidence <= 1) DEFAULT 0.5,
  scenario_id UUID,  -- Links to scenarios database
  -- RFC-9021 Cognitive fields
  trivariate_hash TEXT,
  thalamic_output JSONB,  -- Layer 1: gate_decision, pathway, priority, domains
  glaf_scores JSONB,  -- Layer 2c: h1_operational, h2_semantic, combined
  phi3_analysis JSONB,  -- Layer 4: summary, recommendations, techniques
  embedding_id TEXT,  -- ChromaDB document ID for vector search
  -- Metadata
  raw_data JSONB,
  source_feed TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Entity Reputation (IP, domain, hash tracking)
-- NVNN: Entities track IOCs with reputation scoring and cognitive correlation
CREATE TABLE IF NOT EXISTS plasma_entities (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  type TEXT CHECK (type IN ('ip', 'domain', 'hash', 'user', 'process')) NOT NULL,
  value TEXT NOT NULL,
  first_seen TIMESTAMPTZ DEFAULT NOW(),
  last_seen TIMESTAMPTZ DEFAULT NOW(),
  threat_count INTEGER DEFAULT 0,
  reputation FLOAT CHECK (reputation >= 0 AND reputation <= 1) DEFAULT 0.5,
  related TEXT[] DEFAULT '{}',
  tags TEXT[] DEFAULT '{}',
  -- RFC-9021 Cognitive fields
  trivariate_hash TEXT,
  embedding_id TEXT,  -- ChromaDB document ID
  gnn_cluster TEXT,  -- GNN-assigned cluster ID
  -- Metadata
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(type, value)
);

-- Tool Executions (offensive tool tracking)
-- NVNN: Tools track security assessment operations with status
CREATE TABLE IF NOT EXISTS plasma_tool_executions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  tool TEXT NOT NULL,
  status TEXT CHECK (status IN ('running', 'success', 'failed', 'queued')) DEFAULT 'queued',
  start_time TIMESTAMPTZ DEFAULT NOW(),
  end_time TIMESTAMPTZ,
  target TEXT,
  output TEXT,
  error TEXT,
  agent_id UUID REFERENCES plasma_agents(id) ON DELETE SET NULL,
  trivariate_hash TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- COGNITIVE INFERENCE TABLES (RFC-9021)
-- ============================================================================

-- Thalamic Filter Cache (Layer 1)
-- NVNN: Cache stores DistilBERT thalamic decisions for fast retrieval
CREATE TABLE IF NOT EXISTS plasma_thalamic_cache (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  input_hash TEXT NOT NULL UNIQUE,  -- SHA256 of input text
  gate_decision TEXT CHECK (gate_decision IN ('reflexive', 'full_processing')) NOT NULL,
  pathway TEXT CHECK (pathway IN ('threat_analysis', 'operational', 'informational', 'creative')) NOT NULL,
  priority TEXT CHECK (priority IN ('low', 'medium', 'high', 'critical')) NOT NULL,
  activated_domains TEXT[] DEFAULT '{}',
  inference_ms INTEGER,  -- Inference latency in milliseconds
  created_at TIMESTAMPTZ DEFAULT NOW(),
  expires_at TIMESTAMPTZ DEFAULT (NOW() + INTERVAL '1 hour')
);

-- GLAF Convergence Scores (Layer 2c)
-- NVNN: GLAF table persists H1/H2 convergence calculations
CREATE TABLE IF NOT EXISTS plasma_glaf_scores (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  threat_id UUID REFERENCES plasma_threats(id) ON DELETE CASCADE,
  trivariate_hash TEXT NOT NULL,
  h1_operational FLOAT CHECK (h1_operational >= 0 AND h1_operational <= 1),  -- Hawkes temporal score
  h2_semantic FLOAT CHECK (h2_semantic >= 0 AND h2_semantic <= 1),  -- Semantic similarity score
  combined_score FLOAT CHECK (combined_score >= 0 AND combined_score <= 1),
  fragment_count INTEGER DEFAULT 0,
  matroid_independent BOOLEAN DEFAULT true,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(threat_id)
);

-- Phi-3 Analysis Cache (Layer 4)
-- NVNN: Cache stores Phi-3 generated threat analyses
CREATE TABLE IF NOT EXISTS plasma_phi3_cache (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  threat_id UUID REFERENCES plasma_threats(id) ON DELETE CASCADE,
  context_hash TEXT NOT NULL,  -- Hash of unified context input
  summary TEXT NOT NULL,
  recommendations TEXT[] DEFAULT '{}',
  related_techniques TEXT[] DEFAULT '{}',  -- MITRE ATT&CK IDs
  model_confidence FLOAT CHECK (model_confidence >= 0 AND model_confidence <= 1),
  inference_ms INTEGER,
  token_count INTEGER,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  expires_at TIMESTAMPTZ DEFAULT (NOW() + INTERVAL '24 hours'),
  UNIQUE(threat_id)
);

-- ============================================================================
-- DATA BRIDGE VIEWS
-- ============================================================================

-- View linking PTCC threat actors to Plasma agents
-- NVNN: Bridge view joins Plasma agents with PTCC threat actor data
CREATE OR REPLACE VIEW plasma_actor_agents AS
SELECT
  pa.id as agent_id,
  pa.name,
  pa.ip,
  pa.status,
  pa.os,
  pa.alert_count,
  pa.trivariate_hash,
  pa.cognitive_status,
  pa.last_inference
FROM plasma_agents pa
WHERE pa.trivariate_hash IS NOT NULL;

-- View linking scenarios to Plasma threats with GLAF scores
-- NVNN: Bridge view enriches threats with convergence scores
CREATE OR REPLACE VIEW plasma_enriched_threats AS
SELECT
  pt.id as threat_id,
  pt.timestamp,
  pt.level,
  pt.source,
  pt.target,
  pt.description,
  pt.indicators,
  pt.mitre,
  pt.confidence,
  pt.trivariate_hash,
  pt.thalamic_output,
  pg.h1_operational,
  pg.h2_semantic,
  pg.combined_score as glaf_score,
  pp.summary as phi3_summary,
  pp.recommendations as phi3_recommendations
FROM plasma_threats pt
LEFT JOIN plasma_glaf_scores pg ON pt.id = pg.threat_id
LEFT JOIN plasma_phi3_cache pp ON pt.id = pp.threat_id;

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Performance indexes for real-time queries
CREATE INDEX IF NOT EXISTS idx_plasma_threats_timestamp ON plasma_threats(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_plasma_threats_level ON plasma_threats(level);
CREATE INDEX IF NOT EXISTS idx_plasma_threats_trivariate ON plasma_threats(trivariate_hash);
CREATE INDEX IF NOT EXISTS idx_plasma_agents_status ON plasma_agents(status);
CREATE INDEX IF NOT EXISTS idx_plasma_agents_trivariate ON plasma_agents(trivariate_hash);
CREATE INDEX IF NOT EXISTS idx_plasma_entities_type_value ON plasma_entities(type, value);
CREATE INDEX IF NOT EXISTS idx_plasma_entities_trivariate ON plasma_entities(trivariate_hash);
CREATE INDEX IF NOT EXISTS idx_plasma_thalamic_hash ON plasma_thalamic_cache(input_hash);
CREATE INDEX IF NOT EXISTS idx_plasma_thalamic_expires ON plasma_thalamic_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_plasma_glaf_trivariate ON plasma_glaf_scores(trivariate_hash);
CREATE INDEX IF NOT EXISTS idx_plasma_phi3_expires ON plasma_phi3_cache(expires_at);

-- GIN indexes for array/JSONB searches
CREATE INDEX IF NOT EXISTS idx_plasma_threats_indicators ON plasma_threats USING GIN(indicators);
CREATE INDEX IF NOT EXISTS idx_plasma_threats_mitre ON plasma_threats USING GIN(mitre);
CREATE INDEX IF NOT EXISTS idx_plasma_entities_tags ON plasma_entities USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_plasma_agents_groups ON plasma_agents USING GIN(groups);

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Update timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply update triggers
DROP TRIGGER IF EXISTS plasma_managers_updated_at ON plasma_managers;
CREATE TRIGGER plasma_managers_updated_at
  BEFORE UPDATE ON plasma_managers
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();

DROP TRIGGER IF EXISTS plasma_agents_updated_at ON plasma_agents;
CREATE TRIGGER plasma_agents_updated_at
  BEFORE UPDATE ON plasma_agents
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();

DROP TRIGGER IF EXISTS plasma_threats_updated_at ON plasma_threats;
CREATE TRIGGER plasma_threats_updated_at
  BEFORE UPDATE ON plasma_threats
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();

DROP TRIGGER IF EXISTS plasma_entities_updated_at ON plasma_entities;
CREATE TRIGGER plasma_entities_updated_at
  BEFORE UPDATE ON plasma_entities
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Function to clean expired caches
CREATE OR REPLACE FUNCTION plasma_cleanup_expired_caches()
RETURNS void AS $$
BEGIN
  DELETE FROM plasma_thalamic_cache WHERE expires_at < NOW();
  DELETE FROM plasma_phi3_cache WHERE expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- ROW LEVEL SECURITY (RLS)
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE plasma_managers ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_threats ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_entities ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_tool_executions ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_thalamic_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_glaf_scores ENABLE ROW LEVEL SECURITY;
ALTER TABLE plasma_phi3_cache ENABLE ROW LEVEL SECURITY;

-- Default policies (allow authenticated users)
-- In production, replace with more restrictive policies
CREATE POLICY "Allow authenticated read" ON plasma_managers FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_agents FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_threats FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_entities FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_tool_executions FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_thalamic_cache FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_glaf_scores FOR SELECT TO authenticated USING (true);
CREATE POLICY "Allow authenticated read" ON plasma_phi3_cache FOR SELECT TO authenticated USING (true);

-- Service role has full access
CREATE POLICY "Service role full access" ON plasma_managers FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_agents FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_threats FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_entities FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_tool_executions FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_thalamic_cache FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_glaf_scores FOR ALL TO service_role USING (true);
CREATE POLICY "Service role full access" ON plasma_phi3_cache FOR ALL TO service_role USING (true);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE plasma_managers IS 'Wazuh manager instances for agent coordination';
COMMENT ON TABLE plasma_agents IS 'Monitored endpoints with RFC-9021 cognitive status';
COMMENT ON TABLE plasma_threats IS 'Threat intelligence events with cognitive enrichment';
COMMENT ON TABLE plasma_entities IS 'IOC entities with reputation and embedding links';
COMMENT ON TABLE plasma_tool_executions IS 'Security tool execution tracking';
COMMENT ON TABLE plasma_thalamic_cache IS 'Layer 1 DistilBERT thalamic filter cache';
COMMENT ON TABLE plasma_glaf_scores IS 'Layer 2c GLAF matroid convergence scores';
COMMENT ON TABLE plasma_phi3_cache IS 'Layer 4 Phi-3 threat analysis cache';
COMMENT ON VIEW plasma_actor_agents IS 'Bridge view linking agents to PTCC threat actors';
COMMENT ON VIEW plasma_enriched_threats IS 'Threats enriched with cognitive inference data';
