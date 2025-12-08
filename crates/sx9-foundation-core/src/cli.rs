//! CLI macros and commands for CTAS v7.0
//! 
//! Command-line interface for Linear integration and agent messaging

use anyhow::Result;
use clap::{Arg, Command};

use crate::{V7LinearBot, AgentMuxV7, symbols::generate_symbolic_hash};

/// V7 Linear macro implementation
pub async fn v7_linear_macro(
    issue_text: &str,
    cuid: &str,
    team_id: Option<&str>,
) -> Result<String> {
    let bot = V7LinearBot::new(None);
    let team_id = team_id.unwrap_or("team1234");
    
    let response = bot.create_issue(issue_text, team_id, cuid, None).await?;
    
    if response.issue_create.success {
        if let Some(issue) = response.issue_create.issue {
            Ok(format!(
                "✅ Issue created: {} - {}",
                issue.identifier, issue.title
            ))
        } else {
            Ok("✅ Issue created successfully".to_string())
        }
    } else {
        Ok("❌ Failed to create issue".to_string())
    }
}

/// V7 message macro implementation
pub async fn v7_message_macro(
    content: &str,
    cuid: &str,
    persona: &str,
    entropy: &str,
    ttl: &str,
    sch: &str,
) -> Result<String> {
    use crate::proto::SymbolicMessageV7;
    
    let mut mux = AgentMuxV7::new(None)?;
    let message = SymbolicMessageV7 {
        cuid: cuid.to_string(),
        persona: persona.to_string(),
        content: content.to_string(),
        entropy: entropy.to_string(),
        ttl: ttl.to_string(),
        sch: sch.to_string(),
    };
    
    let ack = mux.process_message(message).await?;
    
    Ok(format!("Message processed: {}", ack.status))
}

/// Create CLI application
pub fn create_cli_app() -> Command {
    Command::new("ctas-v7")
        .version("1.0.0")
        .about("CTAS v7.0 Core - Tachyon Cortex CLI")
        .subcommand(
            Command::new(":v7.linear")
                .about("Create Linear issue via V7 macro")
                .arg(
                    Arg::new("issue_text")
                        .help("Issue title/text")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("cuid")
                        .help("Persona CUID")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("team_id")
                        .help("Linear team ID")
                        .long("team")
                        .short('t'),
                ),
        )
        .subcommand(
            Command::new(":v7.message")
                .about("Send symbolic message via V7 macro")
                .arg(
                    Arg::new("content")
                        .help("Message content")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("cuid")
                        .help("Persona CUID")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("persona")
                        .help("Persona name")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::new("entropy")
                        .help("Entropy symbol")
                        .required(true)
                        .index(4),
                )
                .arg(
                    Arg::new("ttl")
                        .help("Time-to-live (e.g., 48h)")
                        .required(true)
                        .index(5),
                )
                .arg(
                    Arg::new("sch")
                        .help("SCH identifier")
                        .required(true)
                        .index(6),
                ),
        )
        .subcommand(
            Command::new("list-personas")
                .about("List all configured personas"),
        )
        .subcommand(
            Command::new("stats")
                .about("Show agent mux statistics"),
        )
        .subcommand(
            Command::new("generate-hash")
                .about("Generate symbolic hash for content")
                .arg(
                    Arg::new("cuid")
                        .help("Persona CUID")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("content")
                        .help("Content to hash")
                        .required(true)
                        .index(2),
                ),
        )
}

/// Execute CLI command
pub async fn execute_cli_command(matches: clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some((":v7.linear", sub_matches)) => {
            let issue_text = sub_matches.get_one::<String>("issue_text").unwrap();
            let cuid = sub_matches.get_one::<String>("cuid").unwrap();
            let team_id = sub_matches.get_one::<String>("team_id");
            
            let result = v7_linear_macro(issue_text, cuid, team_id.map(|s| s.as_str())).await?;
            println!("{}", result);
        }
        
        Some((":v7.message", sub_matches)) => {
            let content = sub_matches.get_one::<String>("content").unwrap();
            let cuid = sub_matches.get_one::<String>("cuid").unwrap();
            let persona = sub_matches.get_one::<String>("persona").unwrap();
            let entropy = sub_matches.get_one::<String>("entropy").unwrap();
            let ttl = sub_matches.get_one::<String>("ttl").unwrap();
            let sch = sub_matches.get_one::<String>("sch").unwrap();
            
            let result = v7_message_macro(content, cuid, persona, entropy, ttl, sch).await?;
            println!("{}", result);
        }
        
        Some(("list-personas", _)) => {
            let mux = AgentMuxV7::new(None)?;
            println!("📋 Configured Personas:");
            for persona in &mux.config.personas {
                println!(
                    "  {} {} ({}) - {} - Trust: {:?}",
                    persona.entropy_symbol.as_char(),
                    persona.cuid,
                    persona.persona,
                    persona.grpc_endpoint,
                    persona.trust_level
                );
            }
        }
        
        Some(("stats", _)) => {
            let mux = AgentMuxV7::new(None)?;
            let stats = mux.get_stats();
            println!("📊 Agent Mux Statistics:");
            println!("  Total Personas: {}", stats.total_personas);
            println!("  Active Sessions: {}", stats.active_sessions);
            println!("  Expired Sessions: {}", stats.expired_sessions);
            println!("  Uptime: {}", stats.uptime.format("%Y-%m-%d %H:%M:%S UTC"));
        }
        
        Some(("generate-hash", sub_matches)) => {
            let cuid = sub_matches.get_one::<String>("cuid").unwrap();
            let content = sub_matches.get_one::<String>("content").unwrap();
            
            use crate::symbols::EntropySymbol;
            let hash = generate_symbolic_hash(cuid, EntropySymbol::default(), content);
            println!("🔗 Symbolic Hash: {}", hash);
        }
        
        _ => {
            println!("Use --help to see available commands");
        }
    }
    
    Ok(())
}

