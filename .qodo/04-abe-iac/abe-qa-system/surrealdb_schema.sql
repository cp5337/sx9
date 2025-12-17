-- ============================================================================
-- CTAS 7.3.1 SurrealDB Schema for Node Interview System
-- Automated Business Environment (ABE) Integration
-- ============================================================================

-- Define custom types for Unicode addressing
DEFINE FIELD unicode_range ON TABLE unicode_addresses TYPE string;
DEFINE FIELD component_range ON TABLE unicode_addresses TYPE string VALUE "U+E000-U+E3FF";  -- Components
DEFINE FIELD tool_range ON TABLE unicode_addresses TYPE string VALUE "U+E400-U+E7FF";      -- Tools
DEFINE FIELD escalation_range ON TABLE unicode_addresses TYPE string VALUE "U+E800-U+EBFF"; -- Escalations
DEFINE FIELD eei_range ON TABLE unicode_addresses TYPE string VALUE "U+EC00-U+EFFF";       -- EEIs

-- ============================================================================
-- Core Interview Table
-- ============================================================================
DEFINE TABLE node_interviews;

-- Core metadata fields
DEFINE FIELD metadata.interview_id ON TABLE node_interviews TYPE uuid;
DEFINE FIELD metadata.hash_id ON TABLE node_interviews TYPE string;        -- SCH-CUID-UUID-trivariate
DEFINE FIELD metadata.semantic_hash ON TABLE node_interviews TYPE string;  -- H3-block-semantic-auth
DEFINE FIELD metadata.unicode_assembly_ref ON TABLE node_interviews TYPE string; -- \u{EXXX} operation_reference
DEFINE FIELD metadata.created_at ON TABLE node_interviews TYPE datetime DEFAULT time::now();
DEFINE FIELD metadata.interview_version ON TABLE node_interviews TYPE string DEFAULT "7.3.1";
DEFINE FIELD metadata.hd4_phase ON TABLE node_interviews TYPE string ASSERT $value INSIDE ["hunt", "detect", "disable", "disrupt", "dominate"];

-- Identity fields
DEFINE FIELD identity.name ON TABLE node_interviews TYPE string;
DEFINE FIELD identity.type ON TABLE node_interviews TYPE string ASSERT $value INSIDE ["crate", "node", "tool"];
DEFINE FIELD identity.description ON TABLE node_interviews TYPE string;

-- Capabilities fields
DEFINE FIELD capabilities.primary ON TABLE node_interviews TYPE string;
DEFINE FIELD capabilities.operational ON TABLE node_interviews TYPE string;
DEFINE FIELD capabilities.integration ON TABLE node_interviews TYPE string;
DEFINE FIELD capabilities.performance ON TABLE node_interviews TYPE string;

-- Limitations fields
DEFINE FIELD limitations.vulnerabilities ON TABLE node_interviews TYPE string;
DEFINE FIELD limitations.dependencies ON TABLE node_interviews TYPE string;
DEFINE FIELD limitations.constraints ON TABLE node_interviews TYPE string;
DEFINE FIELD limitations.failure_modes ON TABLE node_interviews TYPE string;

-- Tactical profile arrays
DEFINE FIELD tactical_profile.ttps ON TABLE node_interviews TYPE array<string>;
DEFINE FIELD tactical_profile.toolchain_refs ON TABLE node_interviews TYPE array<string>;
DEFINE FIELD tactical_profile.attack_vectors ON TABLE node_interviews TYPE array<string>;

-- Intelligence fields
DEFINE FIELD intelligence.eei_priority ON TABLE node_interviews TYPE array<string>;
DEFINE FIELD intelligence.indicators ON TABLE node_interviews TYPE array<string>;
DEFINE FIELD intelligence.historical_reference ON TABLE node_interviews TYPE string;

-- Relationships
DEFINE FIELD relationships.dependencies ON TABLE node_interviews TYPE string;
DEFINE FIELD relationships.provides_to ON TABLE node_interviews TYPE string;
DEFINE FIELD relationships.coordinates_with ON TABLE node_interviews TYPE string;
DEFINE FIELD relationships.escalates_to ON TABLE node_interviews TYPE string;

-- Operational integration
DEFINE FIELD operational_integration.legion_ecs_entity ON TABLE node_interviews TYPE string;
DEFINE FIELD operational_integration.ground_station_mapping ON TABLE node_interviews TYPE string;
DEFINE FIELD operational_integration.memory_mesh_coordination ON TABLE node_interviews TYPE string;
DEFINE FIELD operational_integration.voice_activation ON TABLE node_interviews TYPE string;

-- Type-specific extensions (use flexible object for polymorphic storage)
DEFINE FIELD type_specific ON TABLE node_interviews TYPE object;

-- ============================================================================
-- Export Format Tables (Dual Export: TOML/JSON)
-- ============================================================================
DEFINE TABLE export_formats;
DEFINE FIELD interview_id ON TABLE export_formats TYPE uuid;
DEFINE FIELD operational_sch ON TABLE export_formats TYPE string;  -- TOML format
DEFINE FIELD semantic_sch ON TABLE export_formats TYPE string;     -- JSON format
DEFINE FIELD export_timestamp ON TABLE export_formats TYPE datetime DEFAULT time::now();

-- ============================================================================
-- GLAF (Graph Link Analysis Fabric) Relationship Graph
-- ============================================================================
DEFINE TABLE glaf_relationships;
DEFINE FIELD source_interview_id ON TABLE glaf_relationships TYPE uuid;
DEFINE FIELD target_interview_id ON TABLE glaf_relationships TYPE uuid;
DEFINE FIELD relationship_type ON TABLE glaf_relationships TYPE string ASSERT $value INSIDE [
    "depends_on", "provides_to", "coordinates_with", "escalates_to",
    "implements", "extends", "uses", "monitors", "configures"
];
DEFINE FIELD unicode_linkage ON TABLE glaf_relationships TYPE string;  -- Unicode reference linkage
DEFINE FIELD strength ON TABLE glaf_relationships TYPE float DEFAULT 1.0;
DEFINE FIELD created_at ON TABLE glaf_relationships TYPE datetime DEFAULT time::now();

-- ============================================================================
-- SlotGraph Operations (O(1) Performance)
-- ============================================================================
DEFINE TABLE slotgraph_cache;
DEFINE FIELD slot_id ON TABLE slotgraph_cache TYPE string;  -- Unicode slot reference
DEFINE FIELD interview_id ON TABLE slotgraph_cache TYPE uuid;
DEFINE FIELD cached_data ON TABLE slotgraph_cache TYPE object;     -- Cached interview subset
DEFINE FIELD access_count ON TABLE slotgraph_cache TYPE int DEFAULT 0;
DEFINE FIELD last_accessed ON TABLE slotgraph_cache TYPE datetime DEFAULT time::now();

-- ============================================================================
-- Unicode Component Registry
-- ============================================================================
DEFINE TABLE unicode_registry;
DEFINE FIELD unicode_code ON TABLE unicode_registry TYPE string;  -- \u{EXXX} format
DEFINE FIELD component_type ON TABLE unicode_registry TYPE string ASSERT $value INSIDE ["component", "tool", "escalation", "eei"];
DEFINE FIELD component_name ON TABLE unicode_registry TYPE string;
DEFINE FIELD component_description ON TABLE unicode_registry TYPE string;
DEFINE FIELD address_range ON TABLE unicode_registry TYPE string;
DEFINE FIELD allocation_status ON TABLE unicode_registry TYPE string ASSERT $value INSIDE ["allocated", "reserved", "available"];

-- ============================================================================
-- ABE Integration Tables
-- ============================================================================
DEFINE TABLE abe_processing_queue;
DEFINE FIELD interview_id ON TABLE abe_processing_queue TYPE uuid;
DEFINE FIELD crate_name ON TABLE abe_processing_queue TYPE string;
DEFINE FIELD processing_status ON TABLE abe_processing_queue TYPE string ASSERT $value INSIDE [
    "pending", "processing", "completed", "failed", "lightning_qa", "expert_qa", "linear_integration", "claude_automation"
];
DEFINE FIELD priority ON TABLE abe_processing_queue TYPE int DEFAULT 5;  -- 1=highest, 10=lowest
DEFINE FIELD gemini_model_used ON TABLE abe_processing_queue TYPE string;
DEFINE FIELD processing_start_time ON TABLE abe_processing_queue TYPE datetime;
DEFINE FIELD processing_end_time ON TABLE abe_processing_queue TYPE datetime;
DEFINE FIELD gpu_utilization ON TABLE abe_processing_queue TYPE float;

-- ============================================================================
-- Performance Monitoring & Analytics
-- ============================================================================
DEFINE TABLE interview_metrics;
DEFINE FIELD interview_id ON TABLE interview_metrics TYPE uuid;
DEFINE FIELD generation_time_ms ON TABLE interview_metrics TYPE int;
DEFINE FIELD gpu_cores_used ON TABLE interview_metrics TYPE int;
DEFINE FIELD memory_usage_mb ON TABLE interview_metrics TYPE int;
DEFINE FIELD gemini_api_calls ON TABLE interview_metrics TYPE int;
DEFINE FIELD lightning_qa_score ON TABLE interview_metrics TYPE float;
DEFINE FIELD expert_qa_score ON TABLE interview_metrics TYPE float;
DEFINE FIELD quality_assessment ON TABLE interview_metrics TYPE string ASSERT $value INSIDE ["excellent", "good", "acceptable", "needs_review"];

-- ============================================================================
-- Indexes for Performance (O(1) Operations)
-- ============================================================================
DEFINE INDEX interview_id_idx ON TABLE node_interviews COLUMNS metadata.interview_id UNIQUE;
DEFINE INDEX unicode_ref_idx ON TABLE node_interviews COLUMNS metadata.unicode_assembly_ref;
DEFINE INDEX type_idx ON TABLE node_interviews COLUMNS identity.type;
DEFINE INDEX hd4_phase_idx ON TABLE node_interviews COLUMNS metadata.hd4_phase;
DEFINE INDEX created_at_idx ON TABLE node_interviews COLUMNS metadata.created_at;

-- GLAF relationship indexes
DEFINE INDEX glaf_source_idx ON TABLE glaf_relationships COLUMNS source_interview_id;
DEFINE INDEX glaf_target_idx ON TABLE glaf_relationships COLUMNS target_interview_id;
DEFINE INDEX glaf_type_idx ON TABLE glaf_relationships COLUMNS relationship_type;

-- SlotGraph cache indexes
DEFINE INDEX slot_id_idx ON TABLE slotgraph_cache COLUMNS slot_id UNIQUE;
DEFINE INDEX slot_interview_idx ON TABLE slotgraph_cache COLUMNS interview_id;

-- Unicode registry indexes
DEFINE INDEX unicode_code_idx ON TABLE unicode_registry COLUMNS unicode_code UNIQUE;
DEFINE INDEX component_type_idx ON TABLE unicode_registry COLUMNS component_type;

-- ABE processing indexes
DEFINE INDEX abe_status_idx ON TABLE abe_processing_queue COLUMNS processing_status;
DEFINE INDEX abe_priority_idx ON TABLE abe_processing_queue COLUMNS priority;
DEFINE INDEX abe_crate_idx ON TABLE abe_processing_queue COLUMNS crate_name;

-- ============================================================================
-- Triggers for Automated Operations
-- ============================================================================

-- Automatically generate Unicode references for new interviews
DEFINE EVENT unicode_assignment ON TABLE node_interviews WHEN $event = "CREATE" THEN {
    LET $unicode_ref = (SELECT VALUE unicode_code FROM unicode_registry WHERE allocation_status = "available" AND component_type = $after.identity.type LIMIT 1);
    IF $unicode_ref != NONE {
        UPDATE unicode_registry SET allocation_status = "allocated" WHERE unicode_code = $unicode_ref;
        UPDATE node_interviews SET metadata.unicode_assembly_ref = $unicode_ref WHERE id = $after.id;
    };
};

-- Update SlotGraph cache when interviews change
DEFINE EVENT slotgraph_update ON TABLE node_interviews WHEN $event = "UPDATE" THEN {
    UPDATE slotgraph_cache SET
        cached_data = $after,
        last_accessed = time::now()
    WHERE interview_id = $after.metadata.interview_id;
};

-- Auto-queue for ABE processing
DEFINE EVENT abe_queue_new_interview ON TABLE node_interviews WHEN $event = "CREATE" THEN {
    INSERT INTO abe_processing_queue {
        interview_id: $after.metadata.interview_id,
        crate_name: $after.identity.name,
        processing_status: "pending",
        priority: 5
    };
};

-- ============================================================================
-- Functions for Interview Operations
-- ============================================================================

-- Function to generate trivariate hash
DEFINE FUNCTION fn::generate_trivariate_hash($interview_id: uuid, $semantic_data: string) {
    RETURN string::concat("SCH-", string::slice($interview_id, 0, 8), "-UUID-", crypto::sha256($semantic_data));
};

-- Function to allocate Unicode address
DEFINE FUNCTION fn::allocate_unicode_address($component_type: string) -> string {
    LET $available = SELECT VALUE unicode_code FROM unicode_registry
                    WHERE allocation_status = "available" AND component_type = $component_type
                    LIMIT 1;
    IF $available != NONE {
        UPDATE unicode_registry SET allocation_status = "allocated" WHERE unicode_code = $available[0];
        RETURN $available[0];
    };
    RETURN NONE;
};

-- Function for O(1) SlotGraph lookup
DEFINE FUNCTION fn::slotgraph_lookup($slot_id: string) -> object {
    UPDATE slotgraph_cache SET
        access_count += 1,
        last_accessed = time::now()
    WHERE slot_id = $slot_id;

    RETURN (SELECT VALUE cached_data FROM slotgraph_cache WHERE slot_id = $slot_id)[0];
};

-- Function to create GLAF relationship
DEFINE FUNCTION fn::create_glaf_relationship($source_id: uuid, $target_id: uuid, $rel_type: string, $unicode_link: string) {
    INSERT INTO glaf_relationships {
        source_interview_id: $source_id,
        target_interview_id: $target_id,
        relationship_type: $rel_type,
        unicode_linkage: $unicode_link,
        strength: 1.0
    };
};

-- ============================================================================
-- Initial Unicode Registry Population
-- ============================================================================

-- Populate Unicode addresses for components (U+E000-U+E3FF)
INSERT INTO unicode_registry [
    { unicode_code: "\\u{E001}", component_type: "component", component_name: "foundation-core", address_range: "U+E000-U+E3FF", allocation_status: "available" },
    { unicode_code: "\\u{E002}", component_type: "component", component_name: "tactical-analyzer", address_range: "U+E000-U+E3FF", allocation_status: "available" },
    { unicode_code: "\\u{E003}", component_type: "component", component_name: "intelligence-engine", address_range: "U+E000-U+E3FF", allocation_status: "available" }
];

-- Populate Unicode addresses for tools (U+E400-U+E7FF)
INSERT INTO unicode_registry [
    { unicode_code: "\\u{E401}", component_type: "tool", component_name: "memgraph-connector", address_range: "U+E400-U+E7FF", allocation_status: "available" },
    { unicode_code: "\\u{E402}", component_type: "tool", component_name: "playwright-tester", address_range: "U+E400-U+E7FF", allocation_status: "available" },
    { unicode_code: "\\u{E403}", component_type: "tool", component_name: "caldera-integration", address_range: "U+E400-U+E7FF", allocation_status: "available" }
];

-- Populate Unicode addresses for escalations (U+E800-U+EBFF)
INSERT INTO unicode_registry [
    { unicode_code: "\\u{E801}", component_type: "escalation", component_name: "high-priority-alert", address_range: "U+E800-U+EBFF", allocation_status: "available" },
    { unicode_code: "\\u{E802}", component_type: "escalation", component_name: "security-incident", address_range: "U+E800-U+EBFF", allocation_status: "available" },
    { unicode_code: "\\u{E803}", component_type: "escalation", component_name: "system-failure", address_range: "U+E800-U+EBFF", allocation_status: "available" }
];

-- Populate Unicode addresses for EEIs (U+EC00-U+EFFF)
INSERT INTO unicode_registry [
    { unicode_code: "\\u{EC01}", component_type: "eei", component_name: "threat-assessment", address_range: "U+EC00-U+EFFF", allocation_status: "available" },
    { unicode_code: "\\u{EC02}", component_type: "eei", component_name: "capability-analysis", address_range: "U+EC00-U+EFFF", allocation_status: "available" },
    { unicode_code: "\\u{EC03}", component_type: "eei", component_name: "operational-pattern", address_range: "U+EC00-U+EFFF", allocation_status: "available" }
];

-- ============================================================================
-- Views for Common Queries
-- ============================================================================

-- View for interview dashboard
DEFINE TABLE interview_dashboard AS
SELECT
    metadata.interview_id,
    identity.name,
    identity.type,
    metadata.hd4_phase,
    metadata.created_at,
    (SELECT processing_status FROM abe_processing_queue WHERE interview_id = metadata.interview_id) AS abe_status
FROM node_interviews;

-- View for GLAF relationship graph
DEFINE TABLE glaf_graph AS
SELECT
    source_interview_id,
    target_interview_id,
    relationship_type,
    unicode_linkage,
    strength,
    (SELECT identity.name FROM node_interviews WHERE metadata.interview_id = source_interview_id) AS source_name,
    (SELECT identity.name FROM node_interviews WHERE metadata.interview_id = target_interview_id) AS target_name
FROM glaf_relationships;

-- ============================================================================
-- Sample Data Insertion (for testing)
-- ============================================================================

-- Sample interview for testing
INSERT INTO node_interviews {
    metadata: {
        interview_id: uuid(),
        hash_id: fn::generate_trivariate_hash(uuid(), "test_semantic_data"),
        semantic_hash: "H3-test-auth-block",
        unicode_assembly_ref: "\\u{E001}",
        interview_version: "7.3.1",
        hd4_phase: "hunt"
    },
    identity: {
        name: "ctas7-foundation-tactical",
        type: "crate",
        description: "I am the foundation tactical crate, I provide core tactical analysis capabilities"
    },
    capabilities: {
        primary: "I transform raw intelligence into tactical assessments",
        operational: "I operate through multi-threaded analysis pipelines",
        integration: "I integrate with CTAS7 core via secure channels",
        performance: "I achieve < 100ms analysis time with GPU acceleration"
    },
    limitations: {
        vulnerabilities: "I am vulnerable to memory exhaustion attacks",
        dependencies: "I require ctas7-core and tokio runtime",
        constraints: "I am limited by available GPU memory",
        failure_modes: "I fail when GPU drivers are incompatible"
    },
    tactical_profile: {
        ttps: ["Tactic: Collection - Automated intelligence gathering"],
        toolchain_refs: ["primary_tool: SurrealDB - Graph relationship storage"],
        attack_vectors: ["vector: Buffer overflow - Memory boundary exploitation"]
    },
    intelligence: {
        eei_priority: ["High: Real-time threat indicators", "Medium: Historical pattern analysis"],
        indicators: ["Observable: Memory spike patterns - Resource monitoring"],
        historical_reference: "I mirror CTAS previous tactical frameworks and appear in operational defense scenarios"
    }
};