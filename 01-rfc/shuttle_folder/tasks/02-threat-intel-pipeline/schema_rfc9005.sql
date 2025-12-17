-- ═══════════════════════════════════════════════════════════════════════════
-- SX9 NEON SCHEMA — RFC-9005 Unified Schema v1.2
-- ═══════════════════════════════════════════════════════════════════════════
-- 
-- Primary ACID database for CTAS-7.3.1 / SX9
-- Compliance: RFC-9001, RFC-9002, RFC-9003, RFC-9005
-- 
-- Replaces: Supabase (direct migration path)
-- Syncs to: Sled (cache), Sledis (pubsub), SlotGraph (graph)
--
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- EXTENSIONS
-- ─────────────────────────────────────────────────────────────────────────────

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ─────────────────────────────────────────────────────────────────────────────
-- ENUMS (RFC-9003, RFC-9005)
-- ─────────────────────────────────────────────────────────────────────────────

-- Operation classification (RFC-9003)
CREATE TYPE operation_class_enum AS ENUM (
    'intelligence', 'defensive', 'offensive', 'administrative'
);

-- Entity states
CREATE TYPE entity_state_enum AS ENUM (
    'draft', 'active', 'deprecated', 'archived'
);

-- ATLAS node status
CREATE TYPE atlas_status_enum AS ENUM (
    'dormant', 'initializing', 'active', 'burst', 'cooling', 'error'
);

-- IAC manifold status
CREATE TYPE manifold_status_enum AS ENUM (
    'dormant', 'spawning', 'active', 'scaling', 'teardown', 'error'
);

-- IAC manifold types
CREATE TYPE manifold_type_enum AS ENUM (
    'abe_customer_env', 'plasma_burst_compute', 'cuda_parallel_cluster',
    'gee_paygo_instance', 'conda_scientific_env', 'kali_penetration_env',
    'cdn_edge_node', 'validation_cluster', 'monte_carlo_cluster'
);

-- ═══════════════════════════════════════════════════════════════════════════
-- CORE TABLES
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- ENTITIES (Unified) — RFC-9005 Section 3.1
-- ─────────────────────────────────────────────────────────────────────────────
-- All tools, techniques, tasks, components are rows here.
-- entity_type discriminates.

CREATE TABLE entities (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- RFC-9001: Trivariate Hash Identity
    trivariate_hash TEXT NOT NULL,          -- Format: SCH-CUID-UUID (48 chars)
    sch_hash TEXT,                          -- Murmur3-128 (24 chars) - Semantic Content Hash
    cuid TEXT,                              -- Base96 (16 chars) - Content Unique ID

    -- RFC-9002: Unicode Addressing
    unicode_address TEXT,                   -- U+E000-EFFF
    unicode_class CHAR(1),                  -- A-H class designation

    -- RFC-9003: Operation Classification
    operation_class operation_class_enum,
    escalation_tier INTEGER DEFAULT 1 CHECK (escalation_tier BETWEEN 1 AND 7),

    -- Entity Identity
    name TEXT NOT NULL,
    entity_type TEXT NOT NULL CHECK (entity_type IN (
        -- Core types
        'component', 'tool', 'technique', 'tactic',
        -- CTAS types
        'node', 'task', 'playbook', 'chain',
        -- Infrastructure
        'daemon', 'atlas_node', 'iac_manifold', 'crate',
        -- Ontology types
        'actor', 'object', 'event', 'concept', 'attribute',
        -- Code types
        'function', 'module', 'header', 'footer', 'comment',
        -- Escalation
        'escalation', 'eei'
    )),
    description TEXT,

    -- Source tracking (for tools)
    source TEXT,                            -- kali, atomic, caldera, nuclei, sigma, etc.
    source_id TEXT,                         -- Original ID from source (T1234, nmap, etc.)

    -- HD4 Phase (for operational entities)
    hd4_phase TEXT CHECK (hd4_phase IN ('Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate')),
    
    -- PTCC Primitive (RFC-9100) 
    ptcc_primitive INTEGER CHECK (ptcc_primitive BETWEEN 0 AND 31),

    -- Polymorphic Capabilities
    capabilities JSONB DEFAULT '{}',
    limitations JSONB DEFAULT '{}',

    -- Tactical Profile
    tactical JSONB DEFAULT '{
        "ttps": [],
        "toolchain_refs": [],
        "attack_vectors": []
    }',

    -- GLAF-Compatible Relationships (denormalized for fast access)
    relationships JSONB DEFAULT '{
        "dependencies": [],
        "provides_to": [],
        "coordinates_with": [],
        "escalates_to": []
    }',

    -- Type-Specific Extensions
    type_extensions JSONB DEFAULT '{}',
    -- For tools: {"command": "nmap", "default_args": ["-sV"], "installed": true}
    -- For techniques: {"tactic": "reconnaissance", "platforms": ["linux", "windows"]}
    -- For tasks: {"skill_categories": [], "kali_tools": []}

    -- State Management
    current_state entity_state_enum DEFAULT 'draft',
    compilation_status TEXT DEFAULT 'pending',
    last_compiled_at TIMESTAMPTZ,

    -- Cross-System Sync
    sled_key TEXT,                          -- Key in Sled cache
    surreal_node_id TEXT,                   -- Legacy SurrealDB reference
    slot_graph_id TEXT,                     -- SlotGraph node ID
    hash_slot INTEGER,                      -- Hash slot for sharding

    -- RFC Compliance Tracking
    rfc_version TEXT DEFAULT '9001-9002-9003-9005',
    schema_version TEXT DEFAULT '1.2',

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance Indexes
CREATE UNIQUE INDEX idx_entities_trivariate ON entities (trivariate_hash);
CREATE INDEX idx_entities_sch ON entities (sch_hash);
CREATE INDEX idx_entities_cuid ON entities (cuid);
CREATE INDEX idx_entities_unicode ON entities (unicode_address);
CREATE INDEX idx_entities_type ON entities (entity_type);
CREATE INDEX idx_entities_class ON entities (operation_class);
CREATE INDEX idx_entities_tier ON entities (escalation_tier);
CREATE INDEX idx_entities_state ON entities (current_state);
CREATE INDEX idx_entities_source ON entities (source);
CREATE INDEX idx_entities_source_id ON entities (source_id);
CREATE INDEX idx_entities_hd4 ON entities (hd4_phase);
CREATE INDEX idx_entities_created ON entities (created_at DESC);

-- ─────────────────────────────────────────────────────────────────────────────
-- RELATIONSHIPS — RFC-9005 Section 3.2
-- ─────────────────────────────────────────────────────────────────────────────

CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    target_entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,

    relationship_type TEXT NOT NULL CHECK (relationship_type IN (
        -- Standard relationships
        'depends_on', 'provides_to', 'coordinates_with', 'escalates_to',
        'implements', 'extends', 'uses', 'monitors', 'configures',
        -- Tool→Technique mappings
        'covers_technique', 'detects_technique', 'exploits_technique',
        -- Task mappings
        'task_covers', 'task_uses_tool',
        -- ATLAS cognitive links
        'atlas_cognitive_link', 'neural_mux_routing',
        -- IAC relationships
        'iac_manifold_spawn', 'iac_manifold_teardown',
        -- Ontology relationships
        'is_a', 'part_of', 'related_to', 'derived_from'
    )),

    -- Source attribution
    mapping_source TEXT,                    -- atomic, caldera, nuclei, manual
    confidence DECIMAL(3,2) DEFAULT 1.00,   -- 0.00 - 1.00

    -- Neural Mux Routing
    unicode_linkage TEXT,
    neural_weight DECIMAL(5,4) DEFAULT 1.0,
    routing_latency_ns INTEGER,

    -- Escalation Context
    escalation_tier INTEGER DEFAULT 1,
    escalation_approved BOOLEAN DEFAULT false,
    escalation_approver TEXT,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Prevent duplicate relationships
    UNIQUE(source_entity_id, target_entity_id, relationship_type)
);

CREATE INDEX idx_rel_source ON relationships (source_entity_id);
CREATE INDEX idx_rel_target ON relationships (target_entity_id);
CREATE INDEX idx_rel_type ON relationships (relationship_type);
CREATE INDEX idx_rel_unicode ON relationships (unicode_linkage);

-- ─────────────────────────────────────────────────────────────────────────────
-- ATLAS COGNITIVE NODES — RFC-9005 Section 3.3
-- ─────────────────────────────────────────────────────────────────────────────

CREATE TABLE atlas_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,

    -- Cognitive Performance
    cognitive_tick_rate_us INTEGER DEFAULT 1000,   -- 1ms = 1000μs
    neural_mux_latency_ns INTEGER DEFAULT 250,     -- Target: <250ns

    -- Learning Capabilities
    l_star_learning_enabled BOOLEAN DEFAULT true,
    voice_orchestration_enabled BOOLEAN DEFAULT false,

    -- IAC Capabilities
    iac_manifold_capabilities TEXT[] DEFAULT '{}',
    cuda_compute_slots INTEGER DEFAULT 0,
    burst_compute_allocation JSONB DEFAULT '{}',

    -- Operational State
    last_cognitive_tick TIMESTAMPTZ,
    node_status atlas_status_enum DEFAULT 'dormant',

    -- Metrics
    ticks_processed BIGINT DEFAULT 0,
    average_tick_latency_us DECIMAL(10,2),

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_atlas_entity ON atlas_nodes (entity_id);
CREATE INDEX idx_atlas_status ON atlas_nodes (node_status);

-- ─────────────────────────────────────────────────────────────────────────────
-- IAC MANIFOLDS — RFC-9005 Section 3.4
-- ─────────────────────────────────────────────────────────────────────────────

CREATE TABLE iac_manifolds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    manifold_id UUID NOT NULL UNIQUE,
    atlas_node_id UUID NOT NULL REFERENCES atlas_nodes(id) ON DELETE CASCADE,

    manifold_type manifold_type_enum NOT NULL,

    -- Infrastructure Paths
    terraform_module_path TEXT,
    kubernetes_manifest_path TEXT,

    -- Compute Configuration
    compute_requirements JSONB DEFAULT '{}',
    gpu_allocation INTEGER DEFAULT 0,
    memory_gb INTEGER DEFAULT 0,

    -- Unicode Trigger (RFC-9103)
    spawn_trigger_unicode TEXT,   -- U+EA00-EAFF range

    -- Lifecycle
    current_status manifold_status_enum DEFAULT 'dormant',
    spawn_time_ms INTEGER,
    teardown_time_ms INTEGER,
    last_spawn_at TIMESTAMPTZ,
    last_teardown_at TIMESTAMPTZ,

    -- Cost Tracking
    cost_per_minute DECIMAL(10,4),
    total_runtime_minutes INTEGER DEFAULT 0,
    total_cost DECIMAL(12,2) DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_manifold_atlas ON iac_manifolds (atlas_node_id);
CREATE INDEX idx_manifold_type ON iac_manifolds (manifold_type);
CREATE INDEX idx_manifold_status ON iac_manifolds (current_status);
CREATE INDEX idx_manifold_trigger ON iac_manifolds (spawn_trigger_unicode);

-- ─────────────────────────────────────────────────────────────────────────────
-- PLAYBOOKS / CHAINS
-- ─────────────────────────────────────────────────────────────────────────────

CREATE TABLE playbooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID REFERENCES entities(id) ON DELETE SET NULL,
    
    name TEXT NOT NULL,
    description TEXT,
    
    -- Chain definition
    tool_ids UUID[] DEFAULT '{}',           -- Ordered list of tool entity IDs
    technique_ids TEXT[] DEFAULT '{}',      -- Technique source_ids covered (T1234)
    
    -- Classification
    hd4_phase TEXT CHECK (hd4_phase IN ('Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate')),
    category TEXT,
    playbook_type TEXT DEFAULT 'offensive' CHECK (playbook_type IN ('offensive', 'defensive', 'investigative')),
    
    -- Automation
    automation_level INTEGER DEFAULT 50 CHECK (automation_level BETWEEN 0 AND 100),
    requires_approval BOOLEAN DEFAULT false,
    
    -- Execution
    default_timeout_secs INTEGER DEFAULT 300,
    parallel_execution BOOLEAN DEFAULT false,
    
    -- RFC-9001 Hashes
    trivariate_hash TEXT,
    unicode_address TEXT,
    
    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_playbooks_hd4 ON playbooks (hd4_phase);
CREATE INDEX idx_playbooks_type ON playbooks (playbook_type);

-- ═══════════════════════════════════════════════════════════════════════════
-- EXECUTION TRACKING
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- EXEC SESSIONS
-- ─────────────────────────────────────────────────────────────────────────────

CREATE TABLE exec_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- What's being executed
    tool_entity_id UUID REFERENCES entities(id),
    playbook_id UUID REFERENCES playbooks(id),
    
    -- Target
    target TEXT NOT NULL,
    args JSONB DEFAULT '[]',
    
    -- Environment
    exec_env TEXT DEFAULT 'orbstack' CHECK (exec_env IN ('orbstack', 'docker', 'native', 'remote')),
    container_image TEXT DEFAULT 'kalilinux/kali-rolling',
    
    -- Status
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'complete', 'failed', 'timeout', 'cancelled')),
    
    -- Results
    exit_code INTEGER,
    stdout TEXT,
    stderr TEXT,
    duration_ms INTEGER,
    
    -- Correlation
    correlation_id UUID DEFAULT gen_random_uuid(),
    parent_session_id UUID REFERENCES exec_sessions(id),
    
    -- Timestamps
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_exec_tool ON exec_sessions (tool_entity_id);
CREATE INDEX idx_exec_playbook ON exec_sessions (playbook_id);
CREATE INDEX idx_exec_status ON exec_sessions (status);
CREATE INDEX idx_exec_correlation ON exec_sessions (correlation_id);

-- ═══════════════════════════════════════════════════════════════════════════
-- CONVENIENCE VIEWS
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- v_tools — All tools with their techniques
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE VIEW v_tools AS
SELECT 
    e.id,
    e.name,
    e.source,
    e.source_id,
    e.description,
    e.hd4_phase,
    e.ptcc_primitive,
    e.unicode_address,
    e.trivariate_hash,
    e.sch_hash,
    e.cuid,
    e.type_extensions->>'command' as command,
    e.type_extensions->>'default_args' as default_args,
    (e.type_extensions->>'installed')::boolean as installed,
    COALESCE(
        ARRAY_AGG(DISTINCT r.target_entity_id) FILTER (WHERE r.target_entity_id IS NOT NULL),
        '{}'
    ) as technique_ids,
    e.created_at,
    e.updated_at
FROM entities e
LEFT JOIN relationships r ON e.id = r.source_entity_id 
    AND r.relationship_type IN ('covers_technique', 'exploits_technique')
WHERE e.entity_type = 'tool'
GROUP BY e.id;

-- ─────────────────────────────────────────────────────────────────────────────
-- v_techniques — All MITRE ATT&CK techniques
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE VIEW v_techniques AS
SELECT 
    e.id,
    e.source_id as technique_id,        -- T1234.001
    e.name,
    e.type_extensions->>'tactic' as tactic,
    e.description,
    e.type_extensions->>'detection' as detection,
    e.type_extensions->'platforms' as platforms,
    e.unicode_address,
    e.trivariate_hash,
    COALESCE(
        ARRAY_AGG(DISTINCT r.source_entity_id) FILTER (WHERE r.source_entity_id IS NOT NULL),
        '{}'
    ) as tool_ids,
    e.created_at
FROM entities e
LEFT JOIN relationships r ON e.id = r.target_entity_id 
    AND r.relationship_type IN ('covers_technique', 'exploits_technique')
WHERE e.entity_type = 'technique'
GROUP BY e.id;

-- ─────────────────────────────────────────────────────────────────────────────
-- v_tasks — CTAS tasks with techniques and tools
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE VIEW v_tasks AS
SELECT 
    e.id,
    e.source_id as task_id,
    e.name as task_name,
    e.hd4_phase,
    e.type_extensions->>'primitive_type' as primitive_type,
    e.ptcc_primitive,
    e.trivariate_hash,
    e.unicode_address,
    e.type_extensions as extensions,
    e.created_at
FROM entities e
WHERE e.entity_type IN ('node', 'task');

-- ─────────────────────────────────────────────────────────────────────────────
-- v_tool_technique_map — Flattened tool→technique mappings
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE VIEW v_tool_technique_map AS
SELECT 
    r.id as mapping_id,
    tool.id as tool_id,
    tool.name as tool_name,
    tool.source as tool_source,
    tech.id as technique_id,
    tech.source_id as technique_code,
    tech.name as technique_name,
    r.mapping_source,
    r.confidence,
    r.created_at
FROM relationships r
JOIN entities tool ON r.source_entity_id = tool.id AND tool.entity_type = 'tool'
JOIN entities tech ON r.target_entity_id = tech.id AND tech.entity_type = 'technique'
WHERE r.relationship_type IN ('covers_technique', 'exploits_technique', 'detects_technique');

-- ─────────────────────────────────────────────────────────────────────────────
-- v_stats — Dashboard statistics
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE VIEW v_stats AS
SELECT
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'tool') as total_tools,
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'technique') as total_techniques,
    (SELECT COUNT(*) FROM entities WHERE entity_type IN ('node', 'task')) as total_tasks,
    (SELECT COUNT(*) FROM relationships WHERE relationship_type LIKE '%technique%') as total_mappings,
    (SELECT COUNT(*) FROM playbooks) as total_playbooks,
    (SELECT COUNT(DISTINCT source) FROM entities WHERE entity_type = 'tool') as tool_sources,
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'tool' AND source = 'kali') as kali_tools,
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'tool' AND source = 'atomic') as atomic_tests,
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'tool' AND source = 'nuclei') as nuclei_templates,
    (SELECT COUNT(*) FROM entities WHERE entity_type = 'tool' AND source = 'sigma') as sigma_rules,
    (SELECT COUNT(*) FROM exec_sessions WHERE status = 'running') as active_sessions;

-- ═══════════════════════════════════════════════════════════════════════════
-- FUNCTIONS
-- ═══════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────
-- Updated timestamp trigger
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER entities_updated
    BEFORE UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER playbooks_updated
    BEFORE UPDATE ON playbooks
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- ─────────────────────────────────────────────────────────────────────────────
-- Get tools for a technique
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION get_tools_for_technique(tech_code TEXT)
RETURNS TABLE (
    tool_id UUID,
    tool_name TEXT,
    tool_source TEXT,
    confidence DECIMAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT e.id, e.name, e.source, r.confidence
    FROM entities e
    JOIN relationships r ON e.id = r.source_entity_id
    JOIN entities tech ON r.target_entity_id = tech.id
    WHERE tech.source_id = tech_code
      AND tech.entity_type = 'technique'
      AND e.entity_type = 'tool'
      AND r.relationship_type IN ('covers_technique', 'exploits_technique')
    ORDER BY r.confidence DESC;
END;
$$ LANGUAGE plpgsql;

-- ─────────────────────────────────────────────────────────────────────────────
-- Get techniques for a tool
-- ─────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION get_techniques_for_tool(t_id UUID)
RETURNS TABLE (
    technique_id UUID,
    technique_code TEXT,
    technique_name TEXT,
    tactic TEXT,
    confidence DECIMAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        tech.id, 
        tech.source_id, 
        tech.name, 
        tech.type_extensions->>'tactic',
        r.confidence
    FROM entities tech
    JOIN relationships r ON tech.id = r.target_entity_id
    WHERE r.source_entity_id = t_id
      AND tech.entity_type = 'technique'
      AND r.relationship_type IN ('covers_technique', 'exploits_technique')
    ORDER BY r.confidence DESC;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════
-- ROW LEVEL SECURITY (Optional - enable for multi-tenant)
-- ═══════════════════════════════════════════════════════════════════════════

-- Uncomment to enable RLS:
-- ALTER TABLE entities ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE relationships ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE atlas_nodes ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE iac_manifolds ENABLE ROW LEVEL SECURITY;

-- ═══════════════════════════════════════════════════════════════════════════
-- DONE
-- ═══════════════════════════════════════════════════════════════════════════

-- Run this to verify:
-- SELECT * FROM v_stats;
