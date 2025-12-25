use anyhow::Result;
use tracing::{info, error};

mod agent;
mod linear;
mod security;
mod mcp;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("sx9_linear_agent=debug")
        .init();

    info!("Starting SX9 Linear Agent");

    // Load configuration
    let config = load_config()?;

    // Initialize Linear client
    let linear_client = linear::Client::new(&config.linear_api_key)?
        .with_team(&config.team_id);

    // Initialize Slack client
    let slack_client = mcp::SlackMCP::new(&config.slack_bot_token)?;

    // Initialize Serena MCP
    let serena_mcp = mcp::SerenaClient::new(&config.serena_endpoint)?;

    // Run agent loop
    agent::run(linear_client, slack_client, serena_mcp).await?;

    Ok(())
}

fn load_config() -> Result<Config> {
    // Load from config/linear.toml or environment
    let config_path = std::env::current_dir()?.join("config/linear.toml");

    if config_path.exists() {
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    } else {
        // Fall back to environment variables
        Ok(Config {
            linear_api_key: std::env::var("LINEAR_API_KEY")?,
            slack_bot_token: std::env::var("SLACK_BOT_TOKEN")?,
            serena_endpoint: std::env::var("SERENA_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            team_id: std::env::var("LINEAR_TEAM_ID")?,
            project_id: std::env::var("LINEAR_PROJECT_ID").ok(),
        })
    }
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    linear_api_key: String,
    slack_bot_token: String,
    serena_endpoint: String,
    team_id: String,
    project_id: Option<String>,
}
