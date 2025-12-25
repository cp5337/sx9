//! Linear GraphQL Client

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use tracing::debug;

use super::{Comment, Issue, IssueCreateInput};

/// Linear API base URL
const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

/// Linear API client
pub struct Client {
    http: reqwest::Client,
    team_id: String,
}

impl Client {
    /// Create new Linear client
    pub fn new(api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            team_id: String::new(),
        })
    }

    /// Set team ID
    pub fn with_team(mut self, team_id: &str) -> Self {
        self.team_id = team_id.to_string();
        self
    }

    /// Execute GraphQL query
    async fn query(&self, query: &str, variables: Value) -> Result<Value> {
        let body = json!({
            "query": query,
            "variables": variables
        });

        let response = self
            .http
            .post(LINEAR_API_URL)
            .json(&body)
            .send()
            .await?;

        let result: Value = response.json().await?;

        if let Some(errors) = result.get("errors") {
            return Err(anyhow::anyhow!("GraphQL error: {:?}", errors));
        }

        Ok(result["data"].clone())
    }

    /// Get assigned issues
    pub async fn get_assigned_issues(&self) -> Result<Vec<Issue>> {
        let query = r#"
            query AssignedIssues($teamId: String!) {
                issues(filter: {
                    team: { id: { eq: $teamId } }
                    state: { type: { in: ["unstarted", "started"] } }
                    assignee: { isMe: { eq: true } }
                }) {
                    nodes {
                        id
                        identifier
                        title
                        description
                        priority
                        url
                        state {
                            id
                            name
                            type
                        }
                        team {
                            id
                        }
                        assignee {
                            id
                        }
                    }
                }
            }
        "#;

        let data = self.query(query, json!({ "teamId": self.team_id })).await?;

        let nodes = data["issues"]["nodes"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let issues: Vec<Issue> = nodes
            .into_iter()
            .filter_map(|node| serde_json::from_value(node).ok())
            .collect();

        Ok(issues)
    }

    /// Get issue by ID
    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        let query = r#"
            query GetIssue($id: String!) {
                issue(id: $id) {
                    id
                    identifier
                    title
                    description
                    priority
                    url
                    state {
                        id
                        name
                        type
                    }
                    team {
                        id
                    }
                    assignee {
                        id
                    }
                }
            }
        "#;

        let data = self.query(query, json!({ "id": issue_id })).await?;
        let issue: Issue = serde_json::from_value(data["issue"].clone())?;

        Ok(issue)
    }

    /// Create issue
    pub async fn create_issue(
        &self,
        title: &str,
        description: &str,
        parent_id: Option<&str>,
    ) -> Result<Issue> {
        let query = r#"
            mutation CreateIssue($input: IssueCreateInput!) {
                issueCreate(input: $input) {
                    success
                    issue {
                        id
                        identifier
                        title
                        description
                        url
                        team {
                            id
                        }
                    }
                }
            }
        "#;

        let mut input = json!({
            "title": title,
            "description": description,
            "teamId": self.team_id
        });

        if let Some(pid) = parent_id {
            input["parentId"] = json!(pid);
        }

        let data = self.query(query, json!({ "input": input })).await?;
        let issue: Issue = serde_json::from_value(data["issueCreate"]["issue"].clone())?;

        Ok(issue)
    }

    /// Update issue status
    pub async fn update_status(&self, issue_id: &str, state_name: &str) -> Result<()> {
        // First, get state ID by name
        let state_id = self.get_state_id(state_name).await?;

        let query = r#"
            mutation UpdateIssue($id: String!, $input: IssueUpdateInput!) {
                issueUpdate(id: $id, input: $input) {
                    success
                }
            }
        "#;

        let input = json!({ "stateId": state_id });
        self.query(query, json!({ "id": issue_id, "input": input })).await?;

        debug!("Updated issue {} to state {}", issue_id, state_name);
        Ok(())
    }

    /// Get state ID by name
    async fn get_state_id(&self, state_name: &str) -> Result<String> {
        let query = r#"
            query GetStates($teamId: String!) {
                workflowStates(filter: { team: { id: { eq: $teamId } } }) {
                    nodes {
                        id
                        name
                        type
                    }
                }
            }
        "#;

        let data = self.query(query, json!({ "teamId": self.team_id })).await?;
        let nodes = data["workflowStates"]["nodes"].as_array().unwrap();

        for node in nodes {
            if node["name"].as_str() == Some(state_name)
                || node["type"].as_str() == Some(state_name)
            {
                return Ok(node["id"].as_str().unwrap().to_string());
            }
        }

        // Map common names to types
        let state_type = match state_name {
            "todo" => "unstarted",
            "in_progress" => "started",
            "done" => "completed",
            _ => state_name,
        };

        for node in nodes {
            if node["type"].as_str() == Some(state_type) {
                return Ok(node["id"].as_str().unwrap().to_string());
            }
        }

        Err(anyhow::anyhow!("State not found: {}", state_name))
    }

    /// Add comment to issue
    pub async fn add_comment(&self, issue_id: &str, body: &str) -> Result<String> {
        let query = r#"
            mutation AddComment($issueId: String!, $body: String!) {
                commentCreate(input: { issueId: $issueId, body: $body }) {
                    success
                    comment {
                        id
                    }
                }
            }
        "#;

        let data = self
            .query(query, json!({ "issueId": issue_id, "body": body }))
            .await?;

        let comment_id = data["commentCreate"]["comment"]["id"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(comment_id)
    }

    /// Get comments for issue
    pub async fn get_comments(&self, issue_id: &str) -> Result<Vec<Comment>> {
        let query = r#"
            query GetComments($issueId: String!) {
                issue(id: $issueId) {
                    comments {
                        nodes {
                            id
                            body
                            createdAt
                            user {
                                id
                            }
                        }
                    }
                }
            }
        "#;

        let data = self.query(query, json!({ "issueId": issue_id })).await?;

        let nodes = data["issue"]["comments"]["nodes"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let comments: Vec<Comment> = nodes
            .into_iter()
            .filter_map(|node| {
                Some(Comment {
                    id: node["id"].as_str()?.to_string(),
                    body: node["body"].as_str()?.to_string(),
                    user_id: node["user"]["id"].as_str().map(|s| s.to_string()),
                    created_at: node["createdAt"].as_str()?.to_string(),
                })
            })
            .collect();

        Ok(comments)
    }

    /// Delete comment
    pub async fn delete_comment(&self, comment_id: &str) -> Result<()> {
        let query = r#"
            mutation DeleteComment($id: String!) {
                commentDelete(id: $id) {
                    success
                }
            }
        "#;

        self.query(query, json!({ "id": comment_id })).await?;
        Ok(())
    }
}
