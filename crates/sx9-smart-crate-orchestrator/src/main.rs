//! CTAS-7 Smart Crate Orchestrator CLI
//!
//! Command-line interface for autonomous crate generation following
//! Tesla/SpaceX engineering standards.

use std::collections::HashMap;
use std::path::PathBuf;
use sx9_foundation_manifold::core::async_runtime::tokio;
use sx9_foundation_manifold::core::diagnostics::anyhow::{Context, Result};
use sx9_foundation_manifold::core::diagnostics::tracing::{error, info, Level};
use sx9_foundation_manifold::core::diagnostics::tracing_subscriber::{EnvFilter, FmtSubscriber};
use sx9_foundation_manifold::core::interface::clap::{Parser, Subcommand, ValueEnum};
use sx9_smart_crate_orchestrator::dsl::Sx9Playbook;
use sx9_smart_crate_orchestrator::playbook_orchestrator::PlaybookOrchestrator;
use sx9_smart_crate_orchestrator::{
    CrateSpecification, Mission, OperatorMode, PlaybookFeature, SecurityLevel,
    SmartCrateOrchestrator,
};

/// Tesla/SpaceX-grade Smart Crate Orchestrator CLI
#[derive(Parser)]
#[command(
    name = "sco",
    about = "Smart Crate Orchestrator - Autonomous Rust crate generation",
    version,
    long_about = "
CTAS-7 Smart Crate Orchestrator provides autonomous generation of Rust crates
with cryptographic provenance, hermetic builds, and zero-trust validation.

Features:
- Template-based code generation with security validation
- SLSA provenance attestation for supply chain security
- Persona-based configuration (Developer/Specialist modes)
- XSD playbook integration for operational compliance
- Comprehensive dependency resolution and validation

Examples:
  sco generate --name my-service --mission data-ingestion --mode developer
  sco validate --crate-path ./generated/my-service
  sco template list --template-dir ./templates
    "
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Template directory path
    #[arg(
        short,
        long,
        global = true,
        default_value = "./templates",
        help = "Path to template files directory"
    )]
    template_dir: PathBuf,

    /// Output directory for generated crates
    #[arg(
        short,
        long,
        global = true,
        default_value = "./generated",
        help = "Output directory for generated crates"
    )]
    output_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new smart crate from specifications
    Generate {
        /// Crate name (must follow Rust naming conventions)
        #[arg(long, help = "Crate name following Rust conventions")]
        name: String,

        /// Human-readable description
        #[arg(long, help = "Crate description for documentation")]
        description: Option<String>,

        /// Operational mode for the generated crate
        #[arg(
            long,
            value_enum,
            default_value = "developer",
            help = "Target operational mode"
        )]
        mode: CliOperatorMode,

        /// Mission classification for capability selection
        #[arg(
            long,
            value_enum,
            default_value = "data-ingestion",
            help = "Mission type for capability selection"
        )]
        mission: CliMission,

        /// XSD playbook features to enable
        #[arg(
            long,
            value_enum,
            action = clap::ArgAction::Append,
            help = "XSD playbook features to enable"
        )]
        features: Vec<CliPlaybookFeature>,

        /// Security compliance level
        #[arg(
            long,
            value_enum,
            default_value = "development",
            help = "Security compliance level"
        )]
        security_level: CliSecurityLevel,

        /// Custom environment variables (KEY=VALUE format)
        #[arg(
            long,
            action = clap::ArgAction::Append,
            help = "Custom environment variables (KEY=VALUE)"
        )]
        env: Vec<String>,
    },

    /// Validate an existing crate
    Validate {
        /// Path to crate directory
        #[arg(long, help = "Path to crate directory for validation")]
        crate_path: PathBuf,

        /// Perform comprehensive security analysis
        #[arg(long, help = "Enable comprehensive security analysis")]
        security_check: bool,
    },

    /// Template management operations
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },

    /// Generate provenance attestation for existing crate
    Attest {
        /// Path to crate directory
        #[arg(long, help = "Path to crate directory")]
        crate_path: PathBuf,

        /// Output path for attestation file
        #[arg(long, help = "Output path for attestation file")]
        output: PathBuf,
    },

    /// Execute a TOML playbook
    Execute {
        /// Path to playbook file (.toml)
        #[arg(long, help = "Path to playbook file")]
        playbook_path: PathBuf,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates
    List,
    /// Validate template structure
    Validate,
    /// Generate template documentation
    Document {
        /// Output format for documentation
        #[arg(
            long,
            value_enum,
            default_value = "markdown",
            help = "Documentation output format"
        )]
        format: DocumentationFormat,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum CliOperatorMode {
    Developer,
    Specialist,
    TestHarness,
}

#[derive(ValueEnum, Clone, Debug)]
enum CliMission {
    DataIngestion,
    NeuralInference,
    CryptographicOperations,
    NetworkRouting,
    SystemMonitoring,
}

#[derive(ValueEnum, Clone, Debug)]
enum CliPlaybookFeature {
    XsdP1,
    XsdP2,
    XsdP3,
}

#[derive(ValueEnum, Clone, Debug)]
enum CliSecurityLevel {
    Development,
    Staging,
    Production,
    Classified,
}

#[derive(ValueEnum, Clone, Debug)]
enum DocumentationFormat {
    Markdown,
    Html,
    Json,
}

#[sx9_foundation_core::async_runtime::tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize structured logging
    initialize_logging(cli.verbose)?;

    info!("Starting CTAS-7 Smart Crate Orchestrator");

    match cli.command {
        Commands::Generate {
            name,
            description,
            mode,
            mission,
            features,
            security_level,
            env,
        } => {
            generate_crate(
                cli.template_dir,
                cli.output_dir,
                name,
                description,
                mode,
                mission,
                features,
                security_level,
                env,
            )
            .await
        }
        Commands::Validate {
            crate_path,
            security_check,
        } => validate_crate(crate_path, security_check).await,
        Commands::Template { action } => handle_template_command(action).await,
        Commands::Attest { crate_path, output } => generate_attestation(crate_path, output).await,
        Commands::Execute { playbook_path } => execute_playbook(playbook_path).await,
    }
}

/// Initialize structured logging with appropriate levels
fn initialize_logging(verbose: bool) -> Result<()> {
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("sx9_smart_crate_orchestrator=info".parse()?),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(verbose)
        .with_line_number(verbose)
        .finish();

    sx9_foundation_core::diagnostics::tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    Ok(())
}

/// Generate a new smart crate from specifications
async fn generate_crate(
    template_dir: PathBuf,
    output_dir: PathBuf,
    name: String,
    description: Option<String>,
    mode: CliOperatorMode,
    mission: CliMission,
    features: Vec<CliPlaybookFeature>,
    security_level: CliSecurityLevel,
    env_vars: Vec<String>,
) -> Result<()> {
    info!("Generating crate: {}", name);

    // Parse environment variables
    let environment = parse_environment_variables(env_vars)?;

    // Create crate specification
    let spec = CrateSpecification {
        name: name.clone(),
        description: description.unwrap_or_else(|| format!("CTAS-7 autonomous crate: {}", name)),
        mode: convert_operator_mode(mode),
        mission: convert_mission(mission),
        features: features.into_iter().map(convert_playbook_feature).collect(),
        environment,
        security_level: convert_security_level(security_level),
    };

    // Initialize orchestrator
    let mut orchestrator = SmartCrateOrchestrator::new(&template_dir, &output_dir)
        .await
        .context("Failed to initialize orchestrator")?;

    // Execute orchestration pipeline
    let result = orchestrator
        .orchestrate(spec)
        .await
        .context("Orchestration pipeline failed")?;

    println!("✅ Crate generated successfully!");
    println!("📁 Path: {:?}", result.crate_path);
    println!("🔒 Attestation: {:?}", result.attestation.signature.len());
    println!("🆔 ID: {}", result.metadata.id);

    Ok(())
}

/// Validate an existing crate
async fn validate_crate(_crate_path: PathBuf, _security_check: bool) -> Result<()> {
    println!("🔍 Crate validation (implementation pending)");
    Ok(())
}

/// Handle template management commands
async fn handle_template_command(action: TemplateCommands) -> Result<()> {
    match action {
        TemplateCommands::List => {
            println!("📋 Available templates (implementation pending)");
        }
        TemplateCommands::Validate => {
            println!("✅ Template validation (implementation pending)");
        }
        TemplateCommands::Document { format: _ } => {
            println!("📚 Template documentation (implementation pending)");
        }
    }
    Ok(())
}

/// Generate provenance attestation
async fn generate_attestation(_crate_path: PathBuf, _output: PathBuf) -> Result<()> {
    Ok(())
}

/// Execute a TOML playbook
async fn execute_playbook(playbook_path: PathBuf) -> Result<()> {
    info!("Executing playbook from: {:?}", playbook_path);

    // Read playbook file
    let content = sx9_foundation_core::async_runtime::tokio::fs::read_to_string(&playbook_path)
        .await
        .context("Failed to read playbook file")?;

    // Parse TOML
    let playbook: Sx9Playbook =
        toml::from_str(&content).context("Failed to parse playbook TOML")?;

    info!("Loaded playbook: {} (RFC-9011-B)", playbook.name);

    // Execute
    let mut orchestrator = PlaybookOrchestrator::new();
    let results = orchestrator
        .execute_modern_playbook(&playbook)
        .await
        .map_err(|e| {
            sx9_foundation_manifold::core::diagnostics::anyhow::anyhow!("Execution failed: {}", e)
        })?;

    // Report results
    println!("\n📊 Execution Results:");
    for (step_id, status) in results {
        println!(
            "  • {}: {:?} (Error: {:?})",
            step_id, status.status, status.error_message
        );
    }

    Ok(())
}

/// Parse environment variables from KEY=VALUE format
fn parse_environment_variables(env_vars: Vec<String>) -> Result<HashMap<String, String>> {
    let mut environment = HashMap::new();

    for env_var in env_vars {
        let parts: Vec<&str> = env_var.splitn(2, '=').collect();
        if parts.len() != 2 {
            sx9_foundation_manifold::core::diagnostics::anyhow::bail!(
                "Invalid environment variable format: {}",
                env_var
            );
        }
        environment.insert(parts[0].to_string(), parts[1].to_string());
    }

    Ok(environment)
}

/// Convert CLI operator mode to library type
fn convert_operator_mode(mode: CliOperatorMode) -> OperatorMode {
    match mode {
        CliOperatorMode::Developer => OperatorMode::Developer,
        CliOperatorMode::Specialist => OperatorMode::Specialist,
        CliOperatorMode::TestHarness => OperatorMode::TestHarness,
    }
}

/// Convert CLI mission to library type
fn convert_mission(mission: CliMission) -> Mission {
    match mission {
        CliMission::DataIngestion => Mission::DataIngestion,
        CliMission::NeuralInference => Mission::Analysis, // Mapped to Analysis
        CliMission::CryptographicOperations => Mission::Security, // Mapped to Security
        CliMission::NetworkRouting => Mission::Communication, // Mapped to Communication
        CliMission::SystemMonitoring => Mission::Analysis, // Mapped to Analysis
    }
}

/// Convert CLI playbook feature to library type
fn convert_playbook_feature(feature: CliPlaybookFeature) -> PlaybookFeature {
    match feature {
        CliPlaybookFeature::XsdP1 => PlaybookFeature::XsdP1,
        CliPlaybookFeature::XsdP2 => PlaybookFeature::XsdP2,
        CliPlaybookFeature::XsdP3 => PlaybookFeature::XsdP3,
    }
}

/// Convert CLI security level to library type
fn convert_security_level(level: CliSecurityLevel) -> SecurityLevel {
    match level {
        CliSecurityLevel::Development => SecurityLevel::Development,
        CliSecurityLevel::Staging => SecurityLevel::Staging,
        CliSecurityLevel::Production => SecurityLevel::Production,
        CliSecurityLevel::Classified => SecurityLevel::Classified,
    }
}
