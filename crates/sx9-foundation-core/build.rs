//! Build script for CTAS-7 Foundation Core
//! Handles dependency resolution and fallback downloads

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Check for foundation crates and download if missing
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap();

    let foundation_crates: &[(&str, &str)] = &[];

    for (crate_name, git_url) in foundation_crates {
        let crate_path = workspace_root.join(crate_name);

        if !crate_path.exists() {
            println!("cargo:warning=Missing dependency crate: {}", crate_name);

            // Attempt to clone from git if fallback is enabled
            if env::var("CTAS7_FALLBACK_DOWNLOAD").unwrap_or_else(|_| "true".to_string()) == "true" {
                println!("cargo:warning=Attempting to download {} from {}", crate_name, git_url);

                match clone_dependency(git_url, &crate_path) {
                    Ok(_) => println!("cargo:warning=Successfully downloaded {}", crate_name),
                    Err(e) => {
                        println!("cargo:warning=Failed to download {}: {}", crate_name, e);
                        create_stub_crate(&crate_path, crate_name);
                    }
                }
            } else {
                create_stub_crate(&crate_path, crate_name);
            }
        }
    }

    // Generate XSD schema if enabled
    if env::var("CTAS7_GENERATE_XSD").unwrap_or_else(|_| "true".to_string()) == "true" {
        generate_xsd_schema(&manifest_dir);
    }

    // Generate status reporting template
    generate_status_template(&manifest_dir);
}

fn clone_dependency(git_url: &str, target_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["clone", git_url, &target_path.to_string_lossy()])
        .output()?;

    if !output.status.success() {
        return Err(format!("Git clone failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}

fn create_stub_crate(crate_path: &Path, crate_name: &str) {
    println!("cargo:warning=Creating stub crate for {}", crate_name);

    if let Err(e) = fs::create_dir_all(crate_path.join("src")) {
        println!("cargo:warning=Failed to create stub directory: {}", e);
        return;
    }

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "Stub crate for {}"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
"#,
        crate_name, crate_name
    );

    let lib_rs = format!(
        r#"//! Stub implementation for {}
//! This is a temporary stub until the actual crate is available

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StubStruct {{
    pub message: String,
}}

impl Default for StubStruct {{
    fn default() -> Self {{
        Self {{
            message: "Stub implementation - replace with actual {}".to_string(),
        }}
    }}
}}
"#,
        crate_name, crate_name
    );

    let _ = fs::write(crate_path.join("Cargo.toml"), cargo_toml);
    let _ = fs::write(crate_path.join("src/lib.rs"), lib_rs);
}

fn generate_xsd_schema(manifest_dir: &str) {
    let schemas_dir = Path::new(manifest_dir).join("schemas");
    if let Err(_) = fs::create_dir_all(&schemas_dir) {
        return;
    }

    let xsd_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ctas="http://ctas.cyber.gov/orchestration/core"
           targetNamespace="http://ctas.cyber.gov/orchestration/core"
           elementFormDefault="qualified">

  <!-- CTAS-7 Orchestration Schema -->
  <xs:element name="CTASOrchestration" type="ctas:OrchestrationType"/>

  <xs:complexType name="OrchestrationType">
    <xs:sequence>
      <xs:element name="metadata" type="ctas:MetadataType"/>
      <xs:element name="orchestrators" type="ctas:OrchestratorsType"/>
      <xs:element name="dependencies" type="ctas:DependenciesType"/>
      <xs:element name="status" type="ctas:StatusType"/>
    </xs:sequence>
    <xs:attribute name="version" type="xs:string" use="required"/>
  </xs:complexType>

  <xs:complexType name="OrchestratorType">
    <xs:sequence>
      <xs:element name="id" type="xs:string"/>
      <xs:element name="type" type="ctas:OrchestratorTypeEnum"/>
      <xs:element name="state" type="ctas:OrchestrationStateEnum"/>
      <xs:element name="health" type="ctas:HealthMetricsType"/>
      <xs:element name="resources" type="ctas:ResourceMetricsType"/>
    </xs:sequence>
  </xs:complexType>

  <xs:simpleType name="OrchestratorTypeEnum">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Service"/>
      <xs:enumeration value="Asset"/>
      <xs:enumeration value="Crate"/>
      <xs:enumeration value="QualityAssurance"/>
      <xs:enumeration value="Shipyard"/>
    </xs:restriction>
  </xs:simpleType>

  <xs:simpleType name="OrchestrationStateEnum">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Initializing"/>
      <xs:enumeration value="Running"/>
      <xs:enumeration value="Stopping"/>
      <xs:enumeration value="Stopped"/>
      <xs:enumeration value="Error"/>
    </xs:restriction>
  </xs:simpleType>

</xs:schema>"#;

    let _ = fs::write(schemas_dir.join("ctas7-orchestration.xsd"), xsd_content);
}

fn generate_status_template(manifest_dir: &str) {
    let status_template = r#"[orchestration]
timestamp = "2024-01-01T00:00:00Z"
total_orchestrators = 0
system_health = "Healthy"

# Foundation crates status
[foundation_crates]
data_foundation = { available = false, version = "0.1.0", status = "stub" }
interface_foundation = { available = false, version = "0.1.0", status = "stub" }
tactical_foundation = { available = false, version = "0.1.0", status = "stub" }

# Orchestrator instances
[[orchestrators]]
id = "example-orchestrator"
type = "Service"
state = "Running"
health = "Healthy"
operations = 0
cpu_percent = 0.0
memory_mb = 0
xsd_valid = true
last_updated = "2024-01-01T00:00:00Z"

# Ontology integration status
[ontology]
frontend_connected = false
entities_synced = 0
last_sync = "2024-01-01T00:00:00Z"
namespaces = ["ctas-core", "nist-csf", "mitre-attack", "cwe-cve"]
"#;

    let _ = fs::write(Path::new(manifest_dir).join("ctas7-status-template.toml"), status_template);
}