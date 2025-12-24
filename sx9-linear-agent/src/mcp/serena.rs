use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Serena MCP Client
/// Integrates with Serena for AI-powered code generation and analysis
pub struct SerenaClient {
    endpoint: String,
    client: reqwest::Client,
}

impl SerenaClient {
    pub fn new(endpoint: &str) -> Result<Self> {
        Ok(Self {
            endpoint: endpoint.to_string(),
            client: reqwest::Client::new(),
        })
    }

    /// Generate code from specification
    pub async fn generate_code(&self, spec: &CodeSpec) -> Result<GeneratedCode> {
        let response = self
            .client
            .post(format!("{}/generate", self.endpoint))
            .json(spec)
            .send()
            .await?;

        let code: GeneratedCode = response.json().await?;
        Ok(code)
    }

    /// Analyze code quality
    pub async fn analyze_code(&self, code: &str) -> Result<CodeAnalysis> {
        let response = self
            .client
            .post(format!("{}/analyze", self.endpoint))
            .json(&serde_json::json!({ "code": code }))
            .send()
            .await?;

        let analysis: CodeAnalysis = response.json().await?;
        Ok(analysis)
    }

    /// Get code suggestions
    pub async fn get_suggestions(&self, context: &CodeContext) -> Result<Vec<Suggestion>> {
        let response = self
            .client
            .post(format!("{}/suggest", self.endpoint))
            .json(context)
            .send()
            .await?;

        let suggestions: Vec<Suggestion> = response.json().await?;
        Ok(suggestions)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSpec {
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub language: String,
    pub framework: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedCode {
    pub files: Vec<CodeFile>,
    pub tests: Vec<CodeFile>,
    pub documentation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeFile {
    pub path: String,
    pub content: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub quality_score: f32,
    pub issues: Vec<CodeIssue>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeIssue {
    pub severity: String,
    pub message: String,
    pub line: Option<usize>,
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeContext {
    pub file_path: String,
    pub code: String,
    pub cursor_position: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub title: String,
    pub description: String,
    pub code_snippet: Option<String>,
}
