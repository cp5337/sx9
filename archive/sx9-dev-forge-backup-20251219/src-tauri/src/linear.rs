//! Linear API client for SX9 Dev Forge

use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinearError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("No API key configured")]
    NoApiKey,
    #[error("API error: {0}")]
    Api(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearConfig {
    pub api_key: Option<String>,
    pub team_id: Option<String>,
}

impl Default for LinearConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            team_id: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearIssue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<IssueState>,
    pub priority: Option<i32>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueState {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueInput {
    pub title: String,
    pub description: Option<String>,
    pub team_id: String,
    pub project_id: Option<String>,
    pub priority: Option<i32>,
    pub labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearTeam {
    pub id: String,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearProject {
    pub id: String,
    pub name: String,
    pub state: String,
}

pub struct LinearClient {
    client: reqwest::Client,
    api_key: String,
}

impl LinearClient {
    const API_URL: &'static str = "https://api.linear.app/graphql";

    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    async fn query<T: for<'de> Deserialize<'de>>(&self, query: &str) -> Result<T, LinearError> {
        let body = serde_json::json!({ "query": query });
        
        let response = self.client
            .post(Self::API_URL)
            .headers(self.headers())
            .json(&body)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        
        if let Some(errors) = data.get("errors") {
            return Err(LinearError::Api(errors.to_string()));
        }

        Ok(serde_json::from_value(data["data"].clone())
            .map_err(|e| LinearError::Api(e.to_string()))?)
    }

    pub async fn list_teams(&self) -> Result<Vec<LinearTeam>, LinearError> {
        let query = r#"
            query {
                teams {
                    nodes {
                        id
                        name
                        key
                    }
                }
            }
        "#;

        #[derive(Deserialize)]
        struct Response {
            teams: Nodes<LinearTeam>,
        }
        #[derive(Deserialize)]
        struct Nodes<T> {
            nodes: Vec<T>,
        }

        let response: Response = self.query(query).await?;
        Ok(response.teams.nodes)
    }

    pub async fn list_projects(&self, team_id: &str) -> Result<Vec<LinearProject>, LinearError> {
        let query = format!(r#"
            query {{
                team(id: "{}") {{
                    projects {{
                        nodes {{
                            id
                            name
                            state
                        }}
                    }}
                }}
            }}
        "#, team_id);

        #[derive(Deserialize)]
        struct Response {
            team: TeamProjects,
        }
        #[derive(Deserialize)]
        struct TeamProjects {
            projects: Nodes<LinearProject>,
        }
        #[derive(Deserialize)]
        struct Nodes<T> {
            nodes: Vec<T>,
        }

        let response: Response = self.query(&query).await?;
        Ok(response.team.projects.nodes)
    }

    pub async fn create_issue(&self, input: CreateIssueInput) -> Result<LinearIssue, LinearError> {
        let labels_str = input.labels.iter()
            .map(|l| format!(r#""{}""#, l))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(r#"
            mutation {{
                issueCreate(input: {{
                    title: "{}"
                    description: "{}"
                    teamId: "{}"
                    {}
                    {}
                    labelIds: [{}]
                }}) {{
                    success
                    issue {{
                        id
                        identifier
                        title
                        description
                        url
                        state {{
                            id
                            name
                        }}
                        priority
                    }}
                }}
            }}
        "#,
            input.title.replace('"', r#"\""#),
            input.description.as_deref().unwrap_or("").replace('"', r#"\""#),
            input.team_id,
            input.project_id.map(|p| format!(r#"projectId: "{}""#, p)).unwrap_or_default(),
            input.priority.map(|p| format!("priority: {}", p)).unwrap_or_default(),
            labels_str
        );

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "issueCreate")]
            issue_create: IssueCreate,
        }
        #[derive(Deserialize)]
        struct IssueCreate {
            issue: LinearIssue,
        }

        let response: Response = self.query(&query).await?;
        Ok(response.issue_create.issue)
    }

    pub async fn add_comment(&self, issue_id: &str, body: &str) -> Result<(), LinearError> {
        let query = format!(r#"
            mutation {{
                commentCreate(input: {{
                    issueId: "{}"
                    body: "{}"
                }}) {{
                    success
                }}
            }}
        "#, issue_id, body.replace('"', r#"\""#));

        let _: serde_json::Value = self.query(&query).await?;
        Ok(())
    }

    pub async fn update_issue_state(&self, issue_id: &str, state_id: &str) -> Result<(), LinearError> {
        let query = format!(r#"
            mutation {{
                issueUpdate(id: "{}", input: {{
                    stateId: "{}"
                }}) {{
                    success
                }}
            }}
        "#, issue_id, state_id);

        let _: serde_json::Value = self.query(&query).await?;
        Ok(())
    }
}
