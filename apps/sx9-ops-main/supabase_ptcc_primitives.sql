-- ============================================================================
-- PTCC PRIMITIVE INTEGRATION SCHEMA
-- RFC-9016: Dual-Trivariate PTCC Integration
-- ============================================================================
-- This schema extends the existing Supabase schema with PTCC primitive support
-- for government demos and future dev-center integration.

-- ============================================================================
-- PTCC Primitives Table (32 Universal Primitives)
-- ============================================================================

CREATE TABLE IF NOT EXISTS ptcc_primitives (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    primitive_id INTEGER NOT NULL UNIQUE CHECK (primitive_id >= 0 AND primitive_id <= 31),
    primitive_name TEXT NOT NULL,
    category TEXT NOT NULL,
    unicode_rune CHAR(1) NOT NULL UNIQUE,
    h1_operational TEXT,
    h2_semantic TEXT,
    hd4_affinity TEXT,
    description TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for fast primitive lookup
CREATE INDEX IF NOT EXISTS idx_ptcc_primitive_id ON ptcc_primitives(primitive_id);
CREATE INDEX IF NOT EXISTS idx_ptcc_unicode ON ptcc_primitives(unicode_rune);
CREATE INDEX IF NOT EXISTS idx_ptcc_category ON ptcc_primitives(category);

-- ============================================================================
-- Component to Primitive Mapping
-- ============================================================================

CREATE TABLE IF NOT EXISTS component_primitives (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id TEXT NOT NULL,
    primitive_id INTEGER NOT NULL REFERENCES ptcc_primitives(primitive_id),
    is_primary BOOLEAN DEFAULT false,
    weight FLOAT DEFAULT 1.0 CHECK (weight >= 0.0 AND weight <= 1.0),
    execution_order INTEGER,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for component lookup
CREATE INDEX IF NOT EXISTS idx_component_primitives_component ON component_primitives(component_id);
CREATE INDEX IF NOT EXISTS idx_component_primitives_primitive ON component_primitives(primitive_id);

-- ============================================================================
-- Tool Chain Definitions
-- ============================================================================

CREATE TABLE IF NOT EXISTS tool_chains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chain_name TEXT NOT NULL,
    chain_notation TEXT NOT NULL,
    primitive_sequence INTEGER[] NOT NULL,
    h1_operational TEXT,
    h2_semantic TEXT,
    unicode_compressed TEXT,
    category TEXT,
    estimated_duration_ms INTEGER,
    success_rate FLOAT CHECK (success_rate >= 0.0 AND success_rate <= 1.0),
    mitre_techniques TEXT[],
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for chain lookup
CREATE INDEX IF NOT EXISTS idx_tool_chains_category ON tool_chains(category);
CREATE INDEX IF NOT EXISTS idx_tool_chains_name ON tool_chains(chain_name);

-- ============================================================================
-- Tool Chain Executions (Tracking)
-- ============================================================================

CREATE TABLE IF NOT EXISTS tool_chain_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chain_id UUID REFERENCES tool_chains(id),
    correlation_id TEXT NOT NULL UNIQUE,
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    status TEXT CHECK (status IN ('queued', 'running', 'success', 'failure', 'partial', 'cancelled')),
    current_stage INTEGER,
    total_stages INTEGER,
    results JSONB DEFAULT '{}',
    error TEXT,
    telemetry JSONB DEFAULT '{}'
);

-- Index for execution tracking
CREATE INDEX IF NOT EXISTS idx_chain_executions_chain ON tool_chain_executions(chain_id);
CREATE INDEX IF NOT EXISTS idx_chain_executions_correlation ON tool_chain_executions(correlation_id);
CREATE INDEX IF NOT EXISTS idx_chain_executions_status ON tool_chain_executions(status);

-- ============================================================================
-- Triggers for updated_at
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_ptcc_primitives_updated_at
    BEFORE UPDATE ON ptcc_primitives
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tool_chains_updated_at
    BEFORE UPDATE ON tool_chains
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SEED DATA: 32 PTCC Primitives (RFC-9016 Appendix A)
-- ============================================================================

INSERT INTO ptcc_primitives (primitive_id, primitive_name, category, unicode_rune, hd4_affinity, description) VALUES
-- Core CRUD (0x00-0x03)
(0, 'CREATE', 'CRUD', '\uE400', 'Hunt', 'Create new entities or resources'),
(1, 'READ', 'CRUD', '\uE401', 'Detect', 'Read or retrieve existing data'),
(2, 'UPDATE', 'CRUD', '\uE402', 'Disrupt', 'Modify existing entities'),
(3, 'DELETE', 'CRUD', '\uE403', 'Disable', 'Remove entities or resources'),

-- Communication (0x04-0x05)
(4, 'SEND', 'Communication', '\uE404', 'Dominate', 'Transmit data or commands'),
(5, 'RECEIVE', 'Communication', '\uE405', 'Detect', 'Receive data or responses'),

-- Data Processing (0x06-0x07)
(6, 'TRANSFORM', 'Data Processing', '\uE406', 'Disrupt', 'Transform or encode data'),
(7, 'VALIDATE', 'Data Processing', '\uE407', 'Detect', 'Validate data integrity'),

-- Control Flow (0x08-0x0B)
(8, 'BRANCH', 'Control Flow', '\uE408', 'Hunt', 'Conditional branching logic'),
(9, 'LOOP', 'Control Flow', '\uE409', 'Hunt', 'Iterative execution'),
(10, 'RETURN', 'Control Flow', '\uE40A', 'Dominate', 'Return from execution'),
(11, 'CALL', 'Control Flow', '\uE40B', 'Hunt', 'Invoke function or tool'),

-- Network Operations (0x0C-0x0F)
(12, 'CONNECT', 'Network', '\uE40C', 'Hunt', 'Establish network connection'),
(13, 'DISCONNECT', 'Network', '\uE40D', 'Disable', 'Terminate network connection'),
(14, 'ROUTE', 'Network', '\uE40E', 'Disrupt', 'Route network traffic'),
(15, 'FILTER', 'Network', '\uE40F', 'Detect', 'Filter network packets'),

-- Security (0x10-0x13)
(16, 'AUTHENTICATE', 'Security', '\uE410', 'Detect', 'Verify identity'),
(17, 'AUTHORIZE', 'Security', '\uE411', 'Detect', 'Check permissions'),
(18, 'ENCRYPT', 'Security', '\uE412', 'Disrupt', 'Encrypt data'),
(19, 'DECRYPT', 'Security', '\uE413', 'Disrupt', 'Decrypt data'),

-- Resource Management (0x14-0x17)
(20, 'ALLOCATE', 'Resource', '\uE414', 'Hunt', 'Allocate resources'),
(21, 'DEALLOCATE', 'Resource', '\uE415', 'Disable', 'Free resources'),
(22, 'LOCK', 'Resource', '\uE416', 'Disrupt', 'Lock resource access'),
(23, 'UNLOCK', 'Resource', '\uE417', 'Dominate', 'Unlock resource access'),

-- State Management (0x18-0x1B)
(24, 'SAVE', 'State', '\uE418', 'Dominate', 'Persist state'),
(25, 'RESTORE', 'State', '\uE419', 'Hunt', 'Restore previous state'),
(26, 'CHECKPOINT', 'State', '\uE41A', 'Detect', 'Create state checkpoint'),
(27, 'ROLLBACK', 'State', '\uE41B', 'Disrupt', 'Rollback to previous state'),

-- Coordination (0x1C-0x1F)
(28, 'COORDINATE', 'Coordination', '\uE41C', 'Dominate', 'Coordinate multiple operations'),
(29, 'SYNCHRONIZE', 'Coordination', '\uE41D', 'Dominate', 'Synchronize state across systems'),
(30, 'SIGNAL', 'Coordination', '\uE41E', 'Hunt', 'Send coordination signal'),
(31, 'WAIT', 'Coordination', '\uE41F', 'Detect', 'Wait for signal or condition')
ON CONFLICT (primitive_id) DO NOTHING;

-- ============================================================================
-- SEED DATA: Gallery Component Mappings
-- ============================================================================

-- Red Team Runner: CONNECT → AUTHENTICATE → SEND → CALL
INSERT INTO component_primitives (component_id, primitive_id, is_primary, weight, execution_order) VALUES
('redteam', 12, true, 1.0, 1),   -- CONNECT (primary)
('redteam', 16, false, 0.9, 2),  -- AUTHENTICATE
('redteam', 4, false, 0.8, 3),   -- SEND
('redteam', 11, false, 0.7, 4)   -- CALL
ON CONFLICT DO NOTHING;

-- Kali Tools Integration: CALL → CONNECT → various
INSERT INTO component_primitives (component_id, primitive_id, is_primary, weight, execution_order) VALUES
('kali', 11, true, 1.0, 1),      -- CALL (primary)
('kali', 12, false, 0.8, 2),     -- CONNECT
('kali', 18, false, 0.6, 3),     -- ENCRYPT
('kali', 7, false, 0.5, 4)       -- VALIDATE
ON CONFLICT DO NOTHING;

-- Task Management: CREATE → READ → UPDATE → SAVE
INSERT INTO component_primitives (component_id, primitive_id, is_primary, weight, execution_order) VALUES
('tasks', 0, true, 1.0, 1),      -- CREATE (primary)
('tasks', 1, false, 0.9, 2),     -- READ
('tasks', 2, false, 0.8, 3),     -- UPDATE
('tasks', 24, false, 0.7, 4)     -- SAVE
ON CONFLICT DO NOTHING;

-- Atomic Test Runner: READ → CALL → VALIDATE
INSERT INTO component_primitives (component_id, primitive_id, is_primary, weight, execution_order) VALUES
('atomic-runner', 1, true, 1.0, 1),   -- READ (primary)
('atomic-runner', 11, false, 0.9, 2), -- CALL
('atomic-runner', 7, false, 0.8, 3)   -- VALIDATE
ON CONFLICT DO NOTHING;

-- Database Connection Panel: CONNECT → AUTHENTICATE → READ
INSERT INTO component_primitives (component_id, primitive_id, is_primary, weight, execution_order) VALUES
('db-panel', 12, true, 1.0, 1),  -- CONNECT (primary)
('db-panel', 16, false, 0.9, 2), -- AUTHENTICATE
('db-panel', 1, false, 0.8, 3)   -- READ
ON CONFLICT DO NOTHING;

-- ============================================================================
-- SEED DATA: Example Tool Chains
-- ============================================================================

-- Basic Reconnaissance Chain
INSERT INTO tool_chains (chain_name, chain_notation, primitive_sequence, category, estimated_duration_ms, success_rate, mitre_techniques) VALUES
('Basic Recon', '🔗E40C→E401→E407', ARRAY[12, 1, 7], 'recon', 30000, 0.85, ARRAY['T1595.001', 'T1046'])
ON CONFLICT DO NOTHING;

-- Web Exploitation Chain
INSERT INTO tool_chains (chain_name, chain_notation, primitive_sequence, category, estimated_duration_ms, success_rate, mitre_techniques) VALUES
('Web Exploit', '🔗E40C→E410→E40B→E404', ARRAY[12, 16, 11, 4], 'exploit', 120000, 0.65, ARRAY['T1190', 'T1078'])
ON CONFLICT DO NOTHING;

-- Post-Exploitation Chain
INSERT INTO tool_chains (chain_name, chain_notation, primitive_sequence, category, estimated_duration_ms, success_rate, mitre_techniques) VALUES
('Post-Exploit', '🔗E401→E414→E418→E404', ARRAY[1, 20, 24, 4], 'post-exploit', 180000, 0.55, ARRAY['T1083', 'T1005', 'T1041'])
ON CONFLICT DO NOTHING;

-- Full Penetration Test Chain
INSERT INTO tool_chains (chain_name, chain_notation, primitive_sequence, category, estimated_duration_ms, success_rate, mitre_techniques) VALUES
('Full Pentest', '🔗E40C→E401→E40B→E404→E418', ARRAY[12, 1, 11, 4, 24], 'full', 300000, 0.70, ARRAY['T1595', 'T1046', 'T1190', 'T1041'])
ON CONFLICT DO NOTHING;

-- ============================================================================
-- VIEWS FOR GOVERNMENT DEMOS
-- ============================================================================

-- View: Component Primitive Composition
CREATE OR REPLACE VIEW v_component_primitive_composition AS
SELECT 
    cp.component_id,
    COUNT(*) as primitive_count,
    ARRAY_AGG(p.primitive_name ORDER BY cp.execution_order) as primitives,
    ARRAY_AGG(p.unicode_rune ORDER BY cp.execution_order) as unicode_runes,
    ARRAY_AGG(p.category ORDER BY cp.execution_order) as categories,
    STRING_AGG(p.hd4_affinity, '→' ORDER BY cp.execution_order) as hd4_flow
FROM component_primitives cp
JOIN ptcc_primitives p ON cp.primitive_id = p.primitive_id
GROUP BY cp.component_id;

-- View: Tool Chain Summary
CREATE OR REPLACE VIEW v_tool_chain_summary AS
SELECT 
    tc.id,
    tc.chain_name,
    tc.chain_notation,
    tc.category,
    tc.estimated_duration_ms,
    tc.success_rate,
    ARRAY_LENGTH(tc.primitive_sequence, 1) as stage_count,
    tc.mitre_techniques,
    COUNT(tce.id) as execution_count,
    AVG(EXTRACT(EPOCH FROM (tce.completed_at - tce.started_at)) * 1000)::INTEGER as avg_duration_ms
FROM tool_chains tc
LEFT JOIN tool_chain_executions tce ON tc.id = tce.chain_id AND tce.status = 'success'
GROUP BY tc.id, tc.chain_name, tc.chain_notation, tc.category, tc.estimated_duration_ms, tc.success_rate, tc.primitive_sequence, tc.mitre_techniques;

-- View: Primitive Usage Statistics
CREATE OR REPLACE VIEW v_primitive_usage_stats AS
SELECT 
    p.primitive_id,
    p.primitive_name,
    p.category,
    p.unicode_rune,
    COUNT(DISTINCT cp.component_id) as component_count,
    COUNT(DISTINCT tc.id) as tool_chain_count,
    SUM(CASE WHEN cp.is_primary THEN 1 ELSE 0 END) as primary_usage_count
FROM ptcc_primitives p
LEFT JOIN component_primitives cp ON p.primitive_id = cp.primitive_id
LEFT JOIN tool_chains tc ON p.primitive_id = ANY(tc.primitive_sequence)
GROUP BY p.primitive_id, p.primitive_name, p.category, p.unicode_rune
ORDER BY component_count DESC, tool_chain_count DESC;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE ptcc_primitives IS 'RFC-9016: 32 Universal PTCC Primitives for domain-agnostic operations';
COMMENT ON TABLE component_primitives IS 'Maps Gallery components to their constituent PTCC primitives';
COMMENT ON TABLE tool_chains IS 'Composable tool chains built from primitive sequences';
COMMENT ON TABLE tool_chain_executions IS 'Execution history and telemetry for tool chains';

COMMENT ON COLUMN ptcc_primitives.unicode_rune IS 'Unicode address U+E400-E41F for routing';
COMMENT ON COLUMN ptcc_primitives.h1_operational IS 'Operational hash (context-independent)';
COMMENT ON COLUMN ptcc_primitives.h2_semantic IS 'Semantic hash (meaning-based)';
COMMENT ON COLUMN ptcc_primitives.hd4_affinity IS 'HD4 phase affinity (Hunt/Detect/Disrupt/Disable/Dominate)';

COMMENT ON COLUMN tool_chains.chain_notation IS 'Unicode chain notation: 🔗E400→E401→E402';
COMMENT ON COLUMN tool_chains.primitive_sequence IS 'Array of primitive IDs in execution order';
COMMENT ON COLUMN tool_chains.mitre_techniques IS 'MITRE ATT&CK technique IDs covered by this chain';

-- ============================================================================
-- END OF PTCC PRIMITIVE INTEGRATION SCHEMA
-- ============================================================================
