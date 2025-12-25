use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Slack MCP Client
/// Integrates with Slack for notifications and updates
pub struct SlackMCP {
    bot_token: String,
    client: reqwest::Client,
}

impl SlackMCP {
    pub fn new(bot_token: &str) -> Result<Self> {
        Ok(Self {
            bot_token: bot_token.to_string(),
            client: reqwest::Client::new(),
        })
    }

    /// Send message to Slack channel (alias for post_message)
    pub async fn send_message(&self, channel: &str, message: &str) -> Result<()> {
        self.post_message(channel, message).await
    }

    /// Post message to Slack channel
    pub async fn post_message(&self, channel: &str, message: &str) -> Result<()> {
        let payload = serde_json::json!({
            "channel": channel,
            "text": message
        });

        self.client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(&self.bot_token)
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }

    /// Post rich message with blocks
    pub async fn post_blocks(&self, channel: &str, blocks: Vec<SlackBlock>) -> Result<()> {
        let payload = serde_json::json!({
            "channel": channel,
            "blocks": blocks
        });

        self.client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(&self.bot_token)
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }

    /// Notify about Linear issue update
    pub async fn notify_issue_update(
        &self,
        channel: &str,
        issue_id: &str,
        status: &str,
        assignee: Option<&str>,
    ) -> Result<()> {
        let message = format!(
            "📋 Linear Issue Updated: `{}`\nStatus: *{}*{}",
            issue_id,
            status,
            assignee.map(|a| format!("\nAssignee: {}", a)).unwrap_or_default()
        );

        self.post_message(channel, &message).await
    }

    /// Notify about code generation completion
    pub async fn notify_code_generated(
        &self,
        channel: &str,
        issue_id: &str,
        files_count: usize,
    ) -> Result<()> {
        let message = format!(
            "✅ Code Generated for `{}`\nFiles created: {}",
            issue_id, files_count
        );

        self.post_message(channel, &message).await
    }

    /// Notify about QA gate results
    pub async fn notify_qa_results(
        &self,
        channel: &str,
        issue_id: &str,
        passed: bool,
        details: &str,
    ) -> Result<()> {
        let emoji = if passed { "✅" } else { "❌" };
        let message = format!(
            "{} QA Gate for `{}`\n{}",
            emoji, issue_id, details
        );

        self.post_message(channel, &message).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: Option<SlackText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackText {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
}
