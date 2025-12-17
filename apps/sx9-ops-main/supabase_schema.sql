-- ============================================================================
-- SX9 OPS-MAIN SUPABASE SCHEMA
-- Modal Inventory + DSL Orchestration Integration
-- ============================================================================
-- Version: 1.0
-- Date: 2025-12-14
-- Compliance: RFC-9001 (Trivariate Hashing), RFC-9005 (Unified Schema)
-- Purpose: Store modal inventory and DSL orchestration data in Supabase

-- ============================================================================
-- 1. MODAL INVENTORY TABLES
-- ============================================================================

-- UI Pages Table
CREATE TABLE IF NOT EXISTS ui_pages (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL UNIQUE,
    sch_hash TEXT,  -- Murmur3-64 (16 chars)
    
    -- Page Metadata
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    unicode_shortcut TEXT,  -- e.g., "🔹d1ba"
    
    -- USIM Integration
    usim_header TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_scanned_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Indexes
    CONSTRAINT ui_pages_path_key UNIQUE (path)
);

-- UI Buttons Table
CREATE TABLE IF NOT EXISTS ui_buttons (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL,
    sch_hash TEXT,
    
    -- Foreign Key to Page
    page_id UUID REFERENCES ui_pages(id) ON DELETE CASCADE,
    
    -- Button Metadata
    text TEXT NOT NULL,
    selector TEXT,
    enabled BOOLEAN DEFAULT true,
    unicode_shortcut TEXT,
    
    -- Position/Order
    position INTEGER,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- UI Modals Table
CREATE TABLE IF NOT EXISTS ui_modals (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL UNIQUE,
    sch_hash TEXT,
    
    -- Foreign Key to Page
    page_id UUID REFERENCES ui_pages(id) ON DELETE CASCADE,
    
    -- Modal Metadata
    name TEXT NOT NULL,
    title TEXT,
    selector TEXT,
    unicode_shortcut TEXT,
    
    -- USIM Integration
    usim_header TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- UI Forms Table
CREATE TABLE IF NOT EXISTS ui_forms (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT NOT NULL UNIQUE,
    sch_hash TEXT,
    
    -- Foreign Key to Page/Modal
    page_id UUID REFERENCES ui_pages(id) ON DELETE CASCADE,
    modal_id UUID REFERENCES ui_modals(id) ON DELETE CASCADE,
    
    -- Form Metadata
    name TEXT NOT NULL,
    selector TEXT,
    action TEXT,
    method TEXT,
    unicode_shortcut TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Constraints
    CONSTRAINT ui_forms_parent_check CHECK (
        (page_id IS NOT NULL AND modal_id IS NULL) OR
        (page_id IS NULL AND modal_id IS NOT NULL)
    )
);

-- UI Screenshots Table
CREATE TABLE IF NOT EXISTS ui_screenshots (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Foreign Key to Page
    page_id UUID REFERENCES ui_pages(id) ON DELETE CASCADE,
    
    -- Screenshot Metadata
    filename TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    screenshot_type TEXT, -- 'initial', 'interaction', 'error'
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- ============================================================================
-- 2. DSL ORCHESTRATION TABLES
-- ============================================================================

-- Crate Groups Table
CREATE TABLE IF NOT EXISTS crate_groups (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Group Identification
    group_id TEXT NOT NULL UNIQUE,
    group_name TEXT NOT NULL,
    group_type TEXT NOT NULL,
    
    -- Description
    description TEXT,
    
    -- Criteria (stored as JSONB array)
    criteria JSONB DEFAULT '[]'::jsonb,
    
    -- Operational Capabilities (stored as JSONB array)
    operational_capabilities JSONB DEFAULT '[]'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Crates Table
CREATE TABLE IF NOT EXISTS crates (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- RFC-9001: Trivariate Hash
    trivariate_hash TEXT,
    sch_hash TEXT,
    
    -- Crate Identification
    crate_name TEXT NOT NULL UNIQUE,
    crate_version TEXT,
    
    -- Foreign Key to Group
    group_id UUID REFERENCES crate_groups(id) ON DELETE SET NULL,
    
    -- Crate Metadata
    description TEXT,
    repository_path TEXT,
    port INTEGER,
    
    -- Smart Crate Compliance
    smart_crate_version TEXT,
    tesla_grade BOOLEAN DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Operational Capabilities Table
CREATE TABLE IF NOT EXISTS operational_capabilities (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Capability Identification
    capability_name TEXT NOT NULL UNIQUE,
    capability_type TEXT,
    
    -- Description
    description TEXT,
    
    -- Group Mapping (JSONB array of group_ids)
    group_mapping JSONB DEFAULT '[]'::jsonb,
    
    -- Crate Mapping
    crate_mapping TEXT,
    
    -- Assessment
    assessment TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Component Mappings Table (Frontend Integration)
CREATE TABLE IF NOT EXISTS component_mappings (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Mapping Identification
    mapping_name TEXT NOT NULL UNIQUE,
    
    -- Foreign Key to Group
    group_id UUID REFERENCES crate_groups(id) ON DELETE CASCADE,
    
    -- Component Details
    ui_component TEXT NOT NULL,
    dashboard_integration TEXT,
    real_time_data BOOLEAN DEFAULT false,
    
    -- Description
    description TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Crate Relationships Table (Graph Data)
CREATE TABLE IF NOT EXISTS crate_relationships (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Foreign Keys
    source_crate_id UUID REFERENCES crates(id) ON DELETE CASCADE,
    target_crate_id UUID REFERENCES crates(id) ON DELETE CASCADE,
    
    -- Relationship Type
    relationship_type TEXT NOT NULL, -- 'depends_on', 'integrates_with', 'extends', etc.
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Constraints
    CONSTRAINT crate_relationships_unique UNIQUE (source_crate_id, target_crate_id, relationship_type)
);

-- ============================================================================
-- 3. INDEXES
-- ============================================================================

-- Modal Inventory Indexes
CREATE INDEX IF NOT EXISTS idx_ui_pages_sch_hash ON ui_pages(sch_hash);
CREATE INDEX IF NOT EXISTS idx_ui_pages_unicode ON ui_pages(unicode_shortcut);
CREATE INDEX IF NOT EXISTS idx_ui_buttons_page_id ON ui_buttons(page_id);
CREATE INDEX IF NOT EXISTS idx_ui_buttons_sch_hash ON ui_buttons(sch_hash);
CREATE INDEX IF NOT EXISTS idx_ui_modals_page_id ON ui_modals(page_id);
CREATE INDEX IF NOT EXISTS idx_ui_modals_sch_hash ON ui_modals(sch_hash);
CREATE INDEX IF NOT EXISTS idx_ui_forms_page_id ON ui_forms(page_id);
CREATE INDEX IF NOT EXISTS idx_ui_forms_modal_id ON ui_forms(modal_id);
CREATE INDEX IF NOT EXISTS idx_ui_screenshots_page_id ON ui_screenshots(page_id);

-- DSL Orchestration Indexes
CREATE INDEX IF NOT EXISTS idx_crate_groups_group_id ON crate_groups(group_id);
CREATE INDEX IF NOT EXISTS idx_crate_groups_group_type ON crate_groups(group_type);
CREATE INDEX IF NOT EXISTS idx_crates_group_id ON crates(group_id);
CREATE INDEX IF NOT EXISTS idx_crates_crate_name ON crates(crate_name);
CREATE INDEX IF NOT EXISTS idx_crates_sch_hash ON crates(sch_hash);
CREATE INDEX IF NOT EXISTS idx_operational_capabilities_name ON operational_capabilities(capability_name);
CREATE INDEX IF NOT EXISTS idx_component_mappings_group_id ON component_mappings(group_id);
CREATE INDEX IF NOT EXISTS idx_crate_relationships_source ON crate_relationships(source_crate_id);
CREATE INDEX IF NOT EXISTS idx_crate_relationships_target ON crate_relationships(target_crate_id);

-- ============================================================================
-- 4. FUNCTIONS
-- ============================================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 5. TRIGGERS
-- ============================================================================

-- Triggers for updated_at
CREATE TRIGGER update_ui_pages_updated_at BEFORE UPDATE ON ui_pages
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ui_buttons_updated_at BEFORE UPDATE ON ui_buttons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ui_modals_updated_at BEFORE UPDATE ON ui_modals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ui_forms_updated_at BEFORE UPDATE ON ui_forms
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_crate_groups_updated_at BEFORE UPDATE ON crate_groups
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_crates_updated_at BEFORE UPDATE ON crates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_operational_capabilities_updated_at BEFORE UPDATE ON operational_capabilities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_component_mappings_updated_at BEFORE UPDATE ON component_mappings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- 6. ROW LEVEL SECURITY (RLS)
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE ui_pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE ui_buttons ENABLE ROW LEVEL SECURITY;
ALTER TABLE ui_modals ENABLE ROW LEVEL SECURITY;
ALTER TABLE ui_forms ENABLE ROW LEVEL SECURITY;
ALTER TABLE ui_screenshots ENABLE ROW LEVEL SECURITY;
ALTER TABLE crate_groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE crates ENABLE ROW LEVEL SECURITY;
ALTER TABLE operational_capabilities ENABLE ROW LEVEL SECURITY;
ALTER TABLE component_mappings ENABLE ROW LEVEL SECURITY;
ALTER TABLE crate_relationships ENABLE ROW LEVEL SECURITY;

-- Create policies (allow authenticated users to read/write)
CREATE POLICY "Allow authenticated users to read ui_pages" ON ui_pages
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write ui_pages" ON ui_pages
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read ui_buttons" ON ui_buttons
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write ui_buttons" ON ui_buttons
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read ui_modals" ON ui_modals
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write ui_modals" ON ui_modals
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read ui_forms" ON ui_forms
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write ui_forms" ON ui_forms
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read ui_screenshots" ON ui_screenshots
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write ui_screenshots" ON ui_screenshots
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read crate_groups" ON crate_groups
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write crate_groups" ON crate_groups
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read crates" ON crates
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write crates" ON crates
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read operational_capabilities" ON operational_capabilities
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write operational_capabilities" ON operational_capabilities
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read component_mappings" ON component_mappings
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write component_mappings" ON component_mappings
    FOR ALL USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to read crate_relationships" ON crate_relationships
    FOR SELECT USING (auth.role() = 'authenticated');

CREATE POLICY "Allow authenticated users to write crate_relationships" ON crate_relationships
    FOR ALL USING (auth.role() = 'authenticated');

-- ============================================================================
-- 7. VIEWS
-- ============================================================================

-- View: Complete Page Inventory
CREATE OR REPLACE VIEW v_page_inventory AS
SELECT 
    p.id,
    p.name,
    p.path,
    p.url,
    p.trivariate_hash,
    p.sch_hash,
    p.unicode_shortcut,
    p.usim_header,
    COUNT(DISTINCT b.id) AS button_count,
    COUNT(DISTINCT m.id) AS modal_count,
    COUNT(DISTINCT f.id) AS form_count,
    COUNT(DISTINCT s.id) AS screenshot_count,
    p.created_at,
    p.updated_at,
    p.last_scanned_at
FROM ui_pages p
LEFT JOIN ui_buttons b ON b.page_id = p.id
LEFT JOIN ui_modals m ON m.page_id = p.id
LEFT JOIN ui_forms f ON f.page_id = p.id
LEFT JOIN ui_screenshots s ON s.page_id = p.id
GROUP BY p.id;

-- View: Crate Group Summary
CREATE OR REPLACE VIEW v_crate_group_summary AS
SELECT 
    cg.id,
    cg.group_id,
    cg.group_name,
    cg.group_type,
    cg.description,
    COUNT(c.id) AS crate_count,
    ARRAY_AGG(c.crate_name ORDER BY c.crate_name) AS crates,
    cg.operational_capabilities,
    cg.created_at,
    cg.updated_at
FROM crate_groups cg
LEFT JOIN crates c ON c.group_id = cg.id
GROUP BY cg.id;

-- View: Operational Intelligence Map
CREATE OR REPLACE VIEW v_operational_intelligence_map AS
SELECT 
    oc.id,
    oc.capability_name,
    oc.capability_type,
    oc.description,
    oc.group_mapping,
    oc.crate_mapping,
    oc.assessment,
    COUNT(DISTINCT cm.id) AS component_mapping_count
FROM operational_capabilities oc
LEFT JOIN component_mappings cm ON cm.group_id IN (
    SELECT id FROM crate_groups WHERE group_id = ANY(
        SELECT jsonb_array_elements_text(oc.group_mapping)
    )
)
GROUP BY oc.id;

-- ============================================================================
-- 8. SAMPLE DATA INSERTION FUNCTIONS
-- ============================================================================

-- Function to insert modal inventory from JSON
CREATE OR REPLACE FUNCTION insert_modal_inventory(inventory_json JSONB)
RETURNS void AS $$
DECLARE
    page_record JSONB;
    page_id UUID;
    button_record JSONB;
BEGIN
    -- Loop through pages
    FOR page_record IN SELECT * FROM jsonb_array_elements(inventory_json->'pages')
    LOOP
        -- Insert page
        INSERT INTO ui_pages (
            name,
            path,
            url,
            trivariate_hash,
            sch_hash,
            unicode_shortcut,
            usim_header,
            last_scanned_at
        ) VALUES (
            page_record->>'name',
            page_record->>'path',
            page_record->>'url',
            page_record->>'hash',
            page_record->>'sch',
            page_record->>'unicode',
            page_record->>'usim',
            (inventory_json->>'timestamp')::timestamptz
        )
        ON CONFLICT (path) DO UPDATE SET
            trivariate_hash = EXCLUDED.trivariate_hash,
            sch_hash = EXCLUDED.sch_hash,
            unicode_shortcut = EXCLUDED.unicode_shortcut,
            last_scanned_at = EXCLUDED.last_scanned_at,
            updated_at = NOW()
        RETURNING id INTO page_id;
        
        -- Insert buttons for this page
        FOR button_record IN SELECT * FROM jsonb_array_elements(page_record->'buttons')
        LOOP
            INSERT INTO ui_buttons (
                page_id,
                text,
                selector,
                enabled,
                trivariate_hash,
                sch_hash,
                position
            ) VALUES (
                page_id,
                button_record->>'text',
                button_record->>'selector',
                (button_record->>'enabled')::boolean,
                button_record->>'hash',
                button_record->>'hash',
                (button_record->>'position')::integer
            )
            ON CONFLICT DO NOTHING;
        END LOOP;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- END OF SCHEMA
-- ============================================================================
