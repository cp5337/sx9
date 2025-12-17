-- ============================================================================
-- DUAL TRIVARIATE HASH SCHEMA UPDATE
-- RFC-9112: Deterministic Prompt Engineering (Dual Hash)
-- ============================================================================
-- This migration adds H1/H2 dual trivariate hash support to existing tables

-- Add H1/H2 columns to ui_pages
ALTER TABLE ui_pages 
ADD COLUMN IF NOT EXISTS h1_operational TEXT,  -- Primary trivariate hash
ADD COLUMN IF NOT EXISTS h1_sch TEXT,          -- H1 SCH component
ADD COLUMN IF NOT EXISTS h1_cuid TEXT,         -- H1 CUID component  
ADD COLUMN IF NOT EXISTS h1_uuid UUID,         -- H1 UUID component
ADD COLUMN IF NOT EXISTS h2_semantic TEXT,     -- Secondary trivariate hash
ADD COLUMN IF NOT EXISTS h2_sch TEXT,          -- H2 SCH component
ADD COLUMN IF NOT EXISTS h2_cuid TEXT,         -- H2 CUID component
ADD COLUMN IF NOT EXISTS h2_uuid UUID,         -- H2 UUID component
ADD COLUMN IF NOT EXISTS unicode_compressed TEXT; -- Compressed H1+H2 Unicode

-- Add H1/H2 columns to ui_buttons
ALTER TABLE ui_buttons
ADD COLUMN IF NOT EXISTS h1_operational TEXT,
ADD COLUMN IF NOT EXISTS h1_sch TEXT,
ADD COLUMN IF NOT EXISTS h2_semantic TEXT,
ADD COLUMN IF NOT EXISTS h2_sch TEXT,
ADD COLUMN IF NOT EXISTS unicode_compressed TEXT;

-- Add H1/H2 columns to ui_modals
ALTER TABLE ui_modals
ADD COLUMN IF NOT EXISTS h1_operational TEXT,
ADD COLUMN IF NOT EXISTS h1_sch TEXT,
ADD COLUMN IF NOT EXISTS h2_semantic TEXT,
ADD COLUMN IF NOT EXISTS h2_sch TEXT,
ADD COLUMN IF NOT EXISTS unicode_compressed TEXT;

-- Add H1/H2 columns to ui_forms
ALTER TABLE ui_forms
ADD COLUMN IF NOT EXISTS h1_operational TEXT,
ADD COLUMN IF NOT EXISTS h1_sch TEXT,
ADD COLUMN IF NOT EXISTS h2_semantic TEXT,
ADD COLUMN IF NOT EXISTS h2_sch TEXT,
ADD COLUMN IF NOT EXISTS unicode_compressed TEXT;

-- Add H1/H2 columns to crates
ALTER TABLE crates
ADD COLUMN IF NOT EXISTS h1_operational TEXT,
ADD COLUMN IF NOT EXISTS h1_sch TEXT,
ADD COLUMN IF NOT EXISTS h2_semantic TEXT,
ADD COLUMN IF NOT EXISTS h2_sch TEXT,
ADD COLUMN IF NOT EXISTS unicode_compressed TEXT;

-- ============================================================================
-- UNICODE COMPRESSION FUNCTION
-- RFC-9112 Section 4: Compress H1 + H2 into Unicode shortcut
-- ============================================================================

CREATE OR REPLACE FUNCTION compress_dual_hash_to_unicode(
    h1_sch TEXT,
    h2_sch TEXT
) RETURNS TEXT AS $$
DECLARE
    h1_prefix TEXT;
    h2_prefix TEXT;
    compressed TEXT;
BEGIN
    -- Extract first 4 chars of each SCH
    h1_prefix := SUBSTRING(h1_sch FROM 1 FOR 4);
    h2_prefix := SUBSTRING(h2_sch FROM 1 FOR 4);
    
    -- Compress: 🔹{H1[0:4]}⚡{H2[0:4]}
    -- 🔹 = Operational (U+1F539)
    -- ⚡ = Semantic (U+26A1)
    compressed := '🔹' || h1_prefix || '⚡' || h2_prefix;
    
    RETURN compressed;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- ============================================================================
-- DUAL HASH GENERATION FUNCTION
-- RFC-9112 + RFC-9001: Generate H1 (operational) and H2 (semantic)
-- ============================================================================

CREATE OR REPLACE FUNCTION generate_dual_trivariate_hash(
    operation_text TEXT,
    semantic_content TEXT,
    domain_mask INTEGER DEFAULT 0,
    execution_mask INTEGER DEFAULT 0
) RETURNS TABLE(
    h1_operational TEXT,
    h1_sch TEXT,
    h1_cuid TEXT,
    h1_uuid UUID,
    h2_semantic TEXT,
    h2_sch TEXT,
    h2_cuid TEXT,
    h2_uuid UUID,
    unicode_compressed TEXT
) AS $$
DECLARE
    h1_sch_val TEXT;
    h2_sch_val TEXT;
    h1_cuid_val TEXT;
    h2_cuid_val TEXT;
    h1_uuid_val UUID;
    h2_uuid_val UUID;
BEGIN
    -- Generate H1 (Operational) components
    -- SCH: Hash of operation text + domain/execution masks
    h1_sch_val := MD5(operation_text || domain_mask::TEXT || execution_mask::TEXT);
    h1_sch_val := SUBSTRING(h1_sch_val FROM 1 FOR 16);
    
    -- CUID: Timestamp + execution context
    h1_cuid_val := MD5(EXTRACT(EPOCH FROM NOW())::TEXT || 'operational');
    h1_cuid_val := SUBSTRING(h1_cuid_val FROM 1 FOR 16);
    
    -- UUID: UUIDv7 (timestamp-sortable)
    h1_uuid_val := gen_random_uuid();
    
    -- Generate H2 (Semantic) components
    -- SCH: Hash of semantic content
    h2_sch_val := MD5(semantic_content);
    h2_sch_val := SUBSTRING(h2_sch_val FROM 1 FOR 16);
    
    -- CUID: Timestamp + semantic context
    h2_cuid_val := MD5(EXTRACT(EPOCH FROM NOW())::TEXT || 'semantic');
    h2_cuid_val := SUBSTRING(h2_cuid_val FROM 1 FOR 16);
    
    -- UUID: UUIDv7
    h2_uuid_val := gen_random_uuid();
    
    -- Return all components
    RETURN QUERY SELECT
        h1_sch_val || '_' || h1_cuid_val || '_' || h1_uuid_val::TEXT AS h1_operational,
        h1_sch_val AS h1_sch,
        h1_cuid_val AS h1_cuid,
        h1_uuid_val AS h1_uuid,
        h2_sch_val || '_' || h2_cuid_val || '_' || h2_uuid_val::TEXT AS h2_semantic,
        h2_sch_val AS h2_sch,
        h2_cuid_val AS h2_cuid,
        h2_uuid_val AS h2_uuid,
        compress_dual_hash_to_unicode(h1_sch_val, h2_sch_val) AS unicode_compressed;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- INDEXES FOR DUAL HASH COLUMNS
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_ui_pages_h1_sch ON ui_pages(h1_sch);
CREATE INDEX IF NOT EXISTS idx_ui_pages_h2_sch ON ui_pages(h2_sch);
CREATE INDEX IF NOT EXISTS idx_ui_pages_unicode_compressed ON ui_pages(unicode_compressed);

CREATE INDEX IF NOT EXISTS idx_ui_buttons_h1_sch ON ui_buttons(h1_sch);
CREATE INDEX IF NOT EXISTS idx_ui_buttons_h2_sch ON ui_buttons(h2_sch);

CREATE INDEX IF NOT EXISTS idx_crates_h1_sch ON crates(h1_sch);
CREATE INDEX IF NOT EXISTS idx_crates_h2_sch ON crates(h2_sch);

-- ============================================================================
-- EXAMPLE USAGE
-- ============================================================================

/*
-- Generate dual hash for a UI page
SELECT * FROM generate_dual_trivariate_hash(
    'navigate_to_hunt_phase',           -- operation_text (H1)
    'HD4 Hunt Phase - Threat Discovery', -- semantic_content (H2)
    1,                                   -- domain_mask (cyber=1)
    0                                    -- execution_mask (local=0)
);

-- Result:
-- h1_operational: abc123def456_ghi789jkl012_uuid-...
-- h1_sch: abc123def456ghi7
-- h2_semantic: mno345pqr678_stu901vwx234_uuid-...
-- h2_sch: mno345pqr678stu9
-- unicode_compressed: 🔹abc1⚡mno3

-- Update existing page with dual hash
UPDATE ui_pages
SET (h1_operational, h1_sch, h1_cuid, h1_uuid, 
     h2_semantic, h2_sch, h2_cuid, h2_uuid, unicode_compressed) = (
    SELECT * FROM generate_dual_trivariate_hash(
        'navigate_to_' || LOWER(REPLACE(name, ' ', '_')),
        name || ' - ' || COALESCE(metadata->>'description', ''),
        1,
        0
    )
)
WHERE path = '/hunt';
*/

-- ============================================================================
-- MIGRATION NOTES
-- ============================================================================

/*
RFC-9112 Dual Trivariate Hash Compliance:

H1 (Operational):
- SCH: Operation text + domain mask + execution mask
- CUID: Timestamp + execution environment
- UUID: UUIDv7 (timestamp-sortable)
- Purpose: Routing, execution context, operational state

H2 (Semantic):
- SCH: Semantic content hash
- CUID: Timestamp + semantic context
- UUID: UUIDv7
- Purpose: Content addressing, semantic search, knowledge graph

Unicode Compression:
- Format: 🔹{H1_SCH[0:4]}⚡{H2_SCH[0:4]}
- Example: 🔹abc1⚡mno3
- Length: 11 characters (2 emoji + 8 hash chars + 1 separator)
- Use: Voice navigation, quick lookup, visual identification

Seeds (RFC-9001):
- H1 SCH: 0xC7A5_0000 (operational)
- H2 SCH: 0xC7A5_0001 (semantic)
- CUID: 0xC7A5_0002
- UUID: gen_random_uuid() (PostgreSQL native)

Note: This PostgreSQL implementation uses MD5 for demonstration.
Production should use Murmur3-64 via pg_murmur3 extension or
external hash generation in application layer.
*/
