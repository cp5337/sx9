//! Linear GraphQL client implementation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use tracing::{debug, error, info};

use ctas7_agentic_core::{Result, AgentError, Priority};
use crate::auth::LinearAuth;

/// Linear API client
#[derive(Debug, Clone)]
pub struct LinearClient {
    client: Client,
    auth: LinearAuth,
    endpoint: String,
}

impl LinearClient {
    /// Create new Linear client
    pub fn new(auth: LinearAuth) -> Self {
        Self {
            client: Client::new(),
            auth,
            endpoint: "https://api.linear.app/graphql".to_string(),
        }
    }

    /// Create new client with custom endpoint
    pub fn with_endpoint(auth: LinearAuth, endpoint: String) -> Self {
        Self {
            client: Client::new(),
            auth,
            endpoint,
        }
    }

    /// Execute GraphQL query
    pub async fn execute_query(&self, query: &str, variables: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let payload = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", self.auth.bearer_token())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AgentError::NetworkError {
                operation: "graphql_request".to_string(),
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(AgentError::NetworkError {
                operation: "graphql_request".to_string(),
                reason: format!("HTTP {}: {}", response.status(), response.status()),
            });
        }

        let result: GraphQLResponse = response
            .json()
            .await
            .map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?;

        if let Some(errors) = result.errors {
            return Err(AgentError::NetworkError {
                operation: "graphql_query".to_string(),
                reason: format!("GraphQL errors: {:?}", errors),
            });
        }

        result.data.ok_or_else(|| AgentError::NetworkError {
            operation: "graphql_query".to_string(),
            reason: "No data in GraphQL response".to_string(),
        })
    }

    /// Create issue in Linear workspace
    pub async fn create_issue(&self, request: CreateIssueRequest) -> Result<LinearIssue> {
        let query = r#"
            mutation CreateIssue($title: String!, $teamId: String!, $description: String, $priority: Int) {
                issueCreate(input: {
                    title: $title,
                    teamId: $teamId,
                    description: $description,
                    priority: $priority
                }) {
                    success
                    issue {
                        id
                        identifier
                        title
                        description
                        url
                        priority
                        createdAt
                        team {
                            id
                            name
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "title": request.title,
            "teamId": request.team_id,
            "description": request.description,
            "priority": request.priority.map(|p| p as u8)
        });

        debug!("Creating Linear issue: {}", request.title);

        let response = self.execute_query(query, Some(variables)).await?;

        let issue_data = response
            .get("issueCreate")
            .and_then(|ic| ic.get("issue"))
            .ok_or_else(|| AgentError::NetworkError {
                operation: "create_issue".to_string(),
                reason: "Invalid response structure".to_string(),
            })?;

        let issue: LinearIssue = serde_json::from_value(issue_data.clone())
            .map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?;

        info!("Created Linear issue: {} ({})", issue.identifier, issue.title);
        Ok(issue)
    }

    /// Update issue
    pub async fn update_issue(&self, issue_id: &str, updates: UpdateIssueRequest) -> Result<LinearIssue> {
        let query = r#"
            mutation UpdateIssue($issueId: String!, $title: String, $description: String, $priority: Int) {
                issueUpdate(input: {
                    id: $issueId,
                    title: $title,
                    description: $description,
                    priority: $priority
                }) {
                    success
                    issue {
                        id
                        identifier
                        title
                        description
                        url
                        priority
                        updatedAt
                        team {
                            id
                            name
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "issueId": issue_id,
            "title": updates.title,
            "description": updates.description,
            "priority": updates.priority.map(|p| p as u8)
        });

        debug!("Updating Linear issue: {}", issue_id);

        let response = self.execute_query(query, Some(variables)).await?;

        let issue_data = response
            .get("issueUpdate")
            .and_then(|iu| iu.get("issue"))
            .ok_or_else(|| AgentError::NetworkError {
                operation: "update_issue".to_string(),
                reason: "Invalid response structure".to_string(),
            })?;

        let issue: LinearIssue = serde_json::from_value(issue_data.clone())
            .map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?;

        info!("Updated Linear issue: {}", issue.identifier);
        Ok(issue)
    }

    /// Add comment to issue
    pub async fn add_comment(&self, issue_id: &str, body: &str) -> Result<LinearComment> {
        let query = r#"
            mutation CreateComment($issueId: String!, $body: String!) {
                commentCreate(input: {
                    issueId: $issueId,
                    body: $body
                }) {
                    success
                    comment {
                        id
                        body
                        createdAt
                        user {
                            id
                            name
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "issueId": issue_id,
            "body": body
        });

        debug!("Adding comment to Linear issue: {}", issue_id);

        let response = self.execute_query(query, Some(variables)).await?;

        let comment_data = response
            .get("commentCreate")
            .and_then(|cc| cc.get("comment"))
            .ok_or_else(|| AgentError::NetworkError {
                operation: "add_comment".to_string(),
                reason: "Invalid response structure".to_string(),
            })?;

        let comment: LinearComment = serde_json::from_value(comment_data.clone())
            .map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?;

        info!("Added comment to Linear issue: {}", issue_id);
        Ok(comment)
    }

    /// Get team information
    pub async fn get_team(&self, team_id: &str) -> Result<LinearTeam> {
        let query = r#"
            query GetTeam($teamId: String!) {
                team(id: $teamId) {
                    id
                    name
                    key
                    description
                }
            }
        "#;

        let variables = serde_json::json!({
            "teamId": team_id
        });

        let response = self.execute_query(query, Some(variables)).await?;

        let team_data = response
            .get("team")
            .ok_or_else(|| AgentError::NetworkError {
                operation: "get_team".to_string(),
                reason: "Team not found".to_string(),
            })?;

        let team: LinearTeam = serde_json::from_value(team_data.clone())
            .map_err(|e| AgentError::SerializationError {
                reason: e.to_string(),
            })?;

        Ok(team)
    }
}

/// GraphQL request structure
#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: Option<serde_json::Value>,
}

/// GraphQL response structure
#[derive(Debug, Deserialize)]
struct GraphQLResponse {
    data: Option<serde_json::Value>,
    errors: Option<Vec<serde_json::Value>>,
}

/// Create issue request
#[derive(Debug, Clone)]
pub struct CreateIssueRequest {
    pub title: String,
    pub team_id: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

/// Update issue request
#[derive(Debug, Clone)]
pub struct UpdateIssueRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

/// Linear issue representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIssue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub priority: Option<u8>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub team: LinearTeam,
}

/// Linear team representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearTeam {
    pub id: String,
    pub name: String,
    pub key: Option<String>,
    pub description: Option<String>,
}

/// Linear comment representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearComment {
    pub id: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub user: LinearUser,
}

/// Linear user representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearUser {
    pub id: String,
    pub name: String,
}