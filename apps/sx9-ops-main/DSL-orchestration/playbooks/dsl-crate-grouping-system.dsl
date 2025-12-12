<?xml version="1.0" encoding="UTF-8"?>
<ctasCrateGrouping xmlns="http://ctas.org/schemas/grouping"
                   xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                   version="1.0"
                   groupingType="operational-intelligence"
                   mode="comprehensive">

  <!-- Grouping Metadata -->
  <metadata>
    <groupingId>ctas-crate-grouping-001</groupingId>
    <timestamp>2025-08-20T07:00:00Z</timestamp>
    <author>CTAS-Orchestrator</author>
    <description>Comprehensive XSD-driven crate grouping for operational intelligence</description>
    <version>1.0</version>
    <status>active</status>
  </metadata>

  <!-- Grouping Configuration -->
  <groupingConfig>
    <mode>operational_intelligence</mode>
    <scope>all_96_crates</scope>
    <outputFormats>
      <format>json</format>
      <format>xml</format>
      <format>rdf</format>
      <format>typescript</format>
    </outputFormats>
    <outputDirectory>./XSD-QA-5/results/crate-grouping</outputDirectory>
  </groupingConfig>

  <!-- Foundation Group -->
  <crateGroup groupId="foundation" groupName="Foundation" groupType="core_infrastructure">
    <description>Core infrastructure and foundation crates</description>
    <criteria>
      <criterion>core_infrastructure</criterion>
      <criterion>base_functionality</criterion>
      <criterion>system_foundation</criterion>
    </criteria>
    <crates>
      <crate>ctas-core</crate>
      <crate>ctas-tie</crate>
      <crate>ctas-port-manager</crate>
      <crate>ctas-hook-system</crate>
      <crate>ctas-qa-system</crate>
      <crate>ctas-standards-enforcement</crate>
      <crate>ctas-xsd-environment</crate>
      <crate>ctas-forge-hashing</crate>
      <crate>ctas-genetic-hash</crate>
      <crate>ctas-hashflow</crate>
      <crate>ctas-hashing-engine</crate>
      <crate>ctas-hash-affixation</crate>
    </crates>
    <operationalCapabilities>
      <capability>system_foundation</capability>
      <capability>core_infrastructure</capability>
      <capability>base_functionality</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Intelligence Group -->
  <crateGroup groupId="intelligence" groupName="Intelligence" groupType="intelligence_processing">
    <description>Intelligence and analysis crates</description>
    <criteria>
      <criterion>intelligence_processing</criterion>
      <criterion>analysis_capabilities</criterion>
      <criterion>threat_intelligence</criterion>
    </criteria>
    <crates>
      <crate>ctas-intelligence-hub</crate>
      <crate>ctas-document-intel</crate>
      <crate>ctas-threat-vector-db</crate>
      <crate>cyber-intelligence-platform</crate>
      <crate>ctas-gnn-standalone</crate>
      <crate>ctas-gnn-core</crate>
      <crate>ctas-gnn-utils</crate>
      <crate>ctas-gnn-models</crate>
      <crate>ctas-gnn-training</crate>
      <crate>ctas-gnn-inference</crate>
      <crate>ctas-gnn-visualization</crate>
      <crate>ctas-gnn-integration</crate>
    </crates>
    <operationalCapabilities>
      <capability>intelligence_processing</capability>
      <capability>analysis_capabilities</capability>
      <capability>threat_intelligence</capability>
      <capability>machine_learning</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Operations Group -->
  <crateGroup groupId="operations" groupName="Operations" groupType="operational_workflows">
    <description>Operational and workflow crates</description>
    <criteria>
      <criterion>operational_workflows</criterion>
      <criterion>tactical_operations</criterion>
      <criterion>mission_execution</criterion>
    </criteria>
    <crates>
      <crate>ctas-tie</crate>
      <crate>ctas-port-manager</crate>
      <crate>ctas-hook-system</crate>
      <crate>ctas-persona-dashboard</crate>
      <crate>ctas-honeypot-engine</crate>
      <crate>ctas-fratricide-detection</crate>
      <crate>Deception-Platform</crate>
      <crate>Financial-Intelligence-Blockchain</crate>
      <crate>Financial-Warfare-Detection</crate>
      <crate>ctas-scenario-engine</crate>
      <crate>ctas-mission-planner</crate>
      <crate>ctas-tactical-coordinator</crate>
    </crates>
    <operationalCapabilities>
      <capability>operational_workflows</capability>
      <capability>tactical_operations</capability>
      <capability>mission_execution</capability>
      <capability>deception_operations</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Specialized Group -->
  <crateGroup groupId="specialized" groupName="Specialized" groupType="domain_specific">
    <description>Specialized and domain-specific crates</description>
    <criteria>
      <criterion>domain_specific</criterion>
      <criterion>specialized_capabilities</criterion>
      <criterion>niche_functionality</criterion>
    </criteria>
    <crates>
      <crate>ctas-gis-engine</crate>
      <crate>ctas-geospatial-analysis</crate>
      <crate>ctas-map-integration</crate>
      <crate>ctas-location-services</crate>
      <crate>ctas-spatial-intelligence</crate>
      <crate>ctas-financial-intelligence</crate>
      <crate>ctas-blockchain-analysis</crate>
      <crate>ctas-cryptocurrency-tracking</crate>
      <crate>ctas-economic-warfare</crate>
      <crate>ctas-deception-engine</crate>
      <crate>ctas-honeypot-manager</crate>
      <crate>ctas-threat-deception</crate>
    </crates>
    <operationalCapabilities>
      <capability>domain_specific</capability>
      <capability>specialized_capabilities</capability>
      <capability>niche_functionality</capability>
      <capability>geospatial_intelligence</capability>
      <capability>financial_intelligence</capability>
      <capability>deception_operations</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Testing Group -->
  <crateGroup groupId="testing" groupName="Testing" groupType="quality_assurance">
    <description>Testing and quality assurance crates</description>
    <criteria>
      <criterion>quality_assurance</criterion>
      <criterion>testing_framework</criterion>
      <criterion>integration_testing</criterion>
    </criteria>
    <crates>
      <crate>ctas-qa-system</crate>
      <crate>ctas-standards-enforcement</crate>
      <crate>ctas-xsd-environment</crate>
      <crate>integration_test</crate>
      <crate>ctas-test-framework</crate>
      <crate>ctas-integration-tests</crate>
      <crate>ctas-unit-tests</crate>
      <crate>ctas-performance-tests</crate>
      <crate>ctas-security-tests</crate>
    </crates>
    <operationalCapabilities>
      <capability>quality_assurance</capability>
      <capability>testing_framework</capability>
      <capability>integration_testing</capability>
      <capability>standards_enforcement</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- AI-CLI Group -->
  <crateGroup groupId="ai-cli" groupName="AI-CLI" groupType="artificial_intelligence">
    <description>Artificial Intelligence and Command Line Interface crates</description>
    <criteria>
      <criterion>artificial_intelligence</criterion>
      <criterion>command_line_interface</criterion>
      <criterion>automation</criterion>
    </criteria>
    <crates>
      <crate>AI-CLI</crate>
      <crate>ctas-ai-engine</crate>
      <crate>ctas-cli-interface</crate>
      <crate>ctas-automation-engine</crate>
      <crate>ctas-ai-coordinator</crate>
      <crate>ctas-intelligent-automation</crate>
    </crates>
    <operationalCapabilities>
      <capability>artificial_intelligence</capability>
      <capability>command_line_interface</capability>
      <capability>automation</capability>
      <capability>intelligent_coordination</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Tools Group -->
  <crateGroup groupId="tools" groupName="Tools" groupType="utility_tools">
    <description>Utility tools and helper crates</description>
    <criteria>
      <criterion>utility_tools</criterion>
      <criterion>helper_functions</criterion>
      <criterion>development_tools</criterion>
    </criteria>
    <crates>
      <crate>tools</crate>
      <crate>ctas-utility-tools</crate>
      <crate>ctas-development-tools</crate>
      <crate>ctas-helper-functions</crate>
      <crate>ctas-build-tools</crate>
      <crate>ctas-deployment-tools</crate>
    </crates>
    <operationalCapabilities>
      <capability>utility_tools</capability>
      <capability>helper_functions</capability>
      <capability>development_tools</capability>
      <capability>build_automation</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Infrastructure Group -->
  <crateGroup groupId="infrastructure" groupName="Infrastructure" groupType="utility_support">
    <description>Infrastructure and utility crates</description>
    <criteria>
      <criterion>utility_functions</criterion>
      <criterion>infrastructure_support</criterion>
      <criterion>system_utilities</criterion>
    </criteria>
    <crates>
      <crate>ctas-frontend-port-block</crate>
      <crate>ctas-repo-prompt</crate>
      <crate>ctas-document-intel</crate>
      <crate>ctas-unified-intelligence</crate>
      <crate>ctas-threat-vector-db</crate>
      <crate>cyber-intelligence-platform</crate>
      <crate>ctas-persona-dashboard</crate>
      <crate>ctas-honeypot-engine</crate>
      <crate>ctas-fratricide-detection</crate>
      <crate>Deception-Platform</crate>
      <crate>Financial-Intelligence-Blockchain</crate>
      <crate>Financial-Warfare-Detection</crate>
    </crates>
    <operationalCapabilities>
      <capability>utility_functions</capability>
      <capability>infrastructure_support</capability>
      <capability>system_utilities</capability>
    </operationalCapabilities>
  </crateGroup>

  <!-- Operational Intelligence Mapping -->
  <operationalIntelligenceMapping>
    <mappingType>threat_emulation_capability</mappingType>
    <description>Map crate groups to operational intelligence capabilities</description>
    
    <intelligenceCapabilities>
      <capability name="threat_emulation">
        <description>Adversary emulation and threat simulation</description>
        <groupMapping>
          <group>intelligence</group>
          <group>operations</group>
          <group>specialized</group>
        </groupMapping>
        <crateMapping>map_to_threat_emulation</crateMapping>
        <assessment>assess_emulation_capability</assessment>
      </capability>
      
      <capability name="intelligence_fusion">
        <description>Intelligence correlation and fusion</description>
        <groupMapping>
          <group>intelligence</group>
          <group>foundation</group>
        </groupMapping>
        <crateMapping>map_to_intelligence_fusion</crateMapping>
        <assessment>assess_fusion_capability</assessment>
      </capability>
      
      <capability name="countermeasures">
        <description>Defensive countermeasures and response</description>
        <groupMapping>
          <group>operations</group>
          <group>specialized</group>
        </groupMapping>
        <crateMapping>map_to_countermeasures</crateMapping>
        <assessment>assess_countermeasure_capability</assessment>
      </capability>
      
      <capability name="forensics">
        <description>Digital forensics and investigation</description>
        <groupMapping>
          <group>intelligence</group>
          <group>specialized</group>
        </groupMapping>
        <crateMapping>map_to_forensics</crateMapping>
        <assessment>assess_forensic_capability</assessment>
      </capability>
      
      <capability name="investigation">
        <description>Threat investigation and attribution</description>
        <groupMapping>
          <group>intelligence</group>
          <group>operations</group>
        </groupMapping>
        <crateMapping>map_to_investigation</crateMapping>
        <assessment>assess_investigation_capability</assessment>
      </capability>
    </intelligenceCapabilities>
  </operationalIntelligenceMapping>

  <!-- Frontend Component Registry Mapping -->
  <frontendComponentMapping>
    <mappingType>component_registry_integration</mappingType>
    <description>Map crate groups to frontend component registry</description>
    
    <componentMappings>
      <mapping name="foundation_dashboard">
        <description>Foundation system status dashboard</description>
        <group>foundation</group>
        <uiComponent>FoundationStatusDashboard</uiComponent>
        <realTimeData>true</realTimeData>
        <dashboardIntegration>qa5_master_dashboard</dashboardIntegration>
      </mapping>
      
      <mapping name="intelligence_dashboard">
        <description>Intelligence processing dashboard</description>
        <group>intelligence</group>
        <uiComponent>IntelligenceProcessingDashboard</uiComponent>
        <realTimeData>true</realTimeData>
        <dashboardIntegration>operational_intelligence_dashboard</dashboardIntegration>
      </mapping>
      
      <mapping name="operations_dashboard">
        <description>Operational workflows dashboard</description>
        <group>operations</group>
        <uiComponent>OperationsWorkflowDashboard</uiComponent>
        <realTimeData>true</realTimeData>
        <dashboardIntegration>tactical_operations_dashboard</dashboardIntegration>
      </mapping>
      
      <mapping name="specialized_dashboard">
        <description>Specialized capabilities dashboard</description>
        <group>specialized</group>
        <uiComponent>SpecializedCapabilitiesDashboard</uiComponent>
        <realTimeData>true</realTimeData>
        <dashboardIntegration>domain_specific_dashboard</dashboardIntegration>
      </mapping>
      
      <mapping name="testing_dashboard">
        <description>Testing and QA dashboard</description>
        <group>testing</group>
        <uiComponent>TestingQADashboard</uiComponent>
        <realTimeData>true</realTimeData>
        <dashboardIntegration>qa5_master_dashboard</dashboardIntegration>
      </mapping>
    </componentMappings>
  </frontendComponentMapping>

  <!-- Database Integration -->
  <databaseIntegration>
    <integrationType>multi_database</integrationType>
    <description>Store crate grouping data in multiple databases</description>
    
    <databases>
      <database name="supabase">
        <type>postgresql</type>
        <purpose>operational_data</purpose>
        <tables>
          <table>crate_groups</table>
          <table>operational_capabilities</table>
          <table>component_mappings</table>
        </tables>
      </database>
      
      <database name="surreal">
        <type>graph</type>
        <purpose>graph_analysis</purpose>
        <tables>
          <table>crate_relationships</table>
          <table>group_dependencies</table>
          <table>operational_graph</table>
        </tables>
      </database>
      
      <database name="sqlite">
        <type>relational</type>
        <purpose>local_storage</purpose>
        <tables>
          <table>crate_grouping</table>
          <table>group_operations</table>
        </tables>
      </database>
    </databases>
  </databaseIntegration>

  <!-- Validation and Quality Checks -->
  <validation>
    <checks>
      <check name="grouping_completeness">
        <description>Verify all 96 crates are grouped</description>
        <command>python scripts/validate_grouping.py</command>
        <required>true</required>
      </check>
      
      <check name="operational_mapping">
        <description>Validate operational intelligence mapping</description>
        <command>python scripts/validate_operational_mapping.py</command>
        <required>true</required>
      </check>
      
      <check name="frontend_mapping">
        <description>Validate frontend component mapping</description>
        <command>npm test component-mapping</command>
        <required>true</required>
      </check>
    </checks>
  </validation>

  <!-- Report Generation -->
  <reporting>
    <reports>
      <report name="crate-grouping">
        <format>markdown</format>
        <file>crate-grouping-report.md</file>
        <sections>
          <section>executive-summary</section>
          <section>group-overview</section>
          <section>operational-mapping</section>
          <section>frontend-integration</section>
          <section>database-integration</section>
          <section>validation-results</section>
          <section>recommendations</section>
        </sections>
      </report>
      
      <report name="operational-capabilities">
        <format>json</format>
        <file>operational-capabilities-report.json</file>
      </report>
      
      <report name="frontend-components">
        <format>typescript</format>
        <file>frontend-component-mappings.ts</file>
      </report>
    </reports>
  </reporting>

  <!-- Execution Configuration -->
  <execution>
    <parallel>true</parallel>
    <timeout>3600</timeout>
    <retryAttempts>3</retryAttempts>
    <logging>
      <level>info</level>
      <file>crate-grouping.log</file>
    </logging>
  </execution>

</ctasCrateGrouping>
