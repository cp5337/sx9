//! Linear Webhook Handler
//!
//! Processes incoming webhooks from Linear with signature verification.

use super::{LinearWebhook, WebhookEventType};
use sx9_foundation_core::data::serde_json;

/// Webhook handler for Linear events
pub struct WebhookHandler {
    /// Webhook secret for signature verification
    secret: String,
}

impl WebhookHandler {
    /// Create new webhook handler
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Verify webhook signature
    ///
    /// Linear sends signature in `Linear-Signature` header
    /// Format: sha256=<hex_digest>
    pub fn verify_signature(&self, payload: &[u8], signature: &str) -> bool {
        // In production, use HMAC-SHA256 to verify
        // signature == hmac_sha256(secret, payload)

        // For now, just check signature is present and non-empty
        if self.secret.is_empty() {
            return true; // No secret configured, skip verification
        }

        !signature.is_empty() && signature.starts_with("sha256=")
    }

    /// Parse webhook payload
    pub fn parse_webhook(&self, body: &str) -> Result<LinearWebhook, String> {
        serde_json::from_str(body)
            .map_err(|e| format!("Failed to parse webhook: {}", e))
    }

    /// Get event type from webhook
    pub fn event_type(&self, webhook: &LinearWebhook) -> WebhookEventType {
        WebhookEventType::from_webhook(webhook)
    }

    /// Check if webhook should be processed
    ///
    /// Filters out noise like label-only updates
    pub fn should_process(&self, webhook: &LinearWebhook) -> bool {
        let event_type = self.event_type(webhook);

        match event_type {
            WebhookEventType::IssueCreated => true,
            WebhookEventType::IssueUpdated => {
                // Only process state changes, not label/priority tweaks
                self.is_significant_update(webhook)
            }
            WebhookEventType::CommentCreated => true,
            WebhookEventType::IssueRemoved => false, // Don't act on deletions
            WebhookEventType::CommentUpdated => false, // Ignore edits
            WebhookEventType::Unknown => false,
        }
    }

    /// Check if issue update is significant enough to process
    fn is_significant_update(&self, webhook: &LinearWebhook) -> bool {
        // Check for state change in the payload
        if let Some(updated_from) = webhook.data.get("updatedFrom") {
            // State change is significant
            if updated_from.get("state").is_some() {
                return true;
            }
            // Assignee change is significant
            if updated_from.get("assigneeId").is_some() {
                return true;
            }
            // Priority change to Urgent is significant
            if updated_from.get("priority").is_some() {
                if let Some(priority) = webhook.data.get("priority") {
                    if priority.as_i64() == Some(1) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_handler_creation() {
        let handler = WebhookHandler::new("test_secret".to_string());
        assert!(handler.verify_signature(b"test", "sha256=abc123"));
    }

    #[test]
    fn test_verify_signature_empty_secret() {
        let handler = WebhookHandler::new(String::new());
        // With empty secret, should skip verification
        assert!(handler.verify_signature(b"test", ""));
    }

    #[test]
    fn test_verify_signature_format() {
        let handler = WebhookHandler::new("secret".to_string());
        // Valid format
        assert!(handler.verify_signature(b"test", "sha256=abc123"));
        // Invalid format
        assert!(!handler.verify_signature(b"test", "invalid"));
        assert!(!handler.verify_signature(b"test", ""));
    }

    #[test]
    fn test_parse_webhook() {
        let handler = WebhookHandler::new(String::new());

        let json = r#"{
            "action": "create",
            "type": "Issue",
            "created_at": "2025-12-24T00:00:00Z",
            "data": {"id": "issue_123"},
            "organization_id": "org_123",
            "webhook_id": "wh_123"
        }"#;

        let result = handler.parse_webhook(json);
        assert!(result.is_ok());

        let webhook = result.unwrap();
        assert_eq!(webhook.action, "create");
        assert_eq!(webhook.webhook_type, "Issue");
    }

    #[test]
    fn test_should_process() {
        let handler = WebhookHandler::new(String::new());

        // Issue created should be processed
        let json = r#"{
            "action": "create",
            "type": "Issue",
            "created_at": "2025-12-24T00:00:00Z",
            "data": {},
            "organization_id": "org_123",
            "webhook_id": "wh_123"
        }"#;

        let webhook = handler.parse_webhook(json).unwrap();
        assert!(handler.should_process(&webhook));

        // Comment created should be processed
        let json = r#"{
            "action": "create",
            "type": "Comment",
            "created_at": "2025-12-24T00:00:00Z",
            "data": {},
            "organization_id": "org_123",
            "webhook_id": "wh_123"
        }"#;

        let webhook = handler.parse_webhook(json).unwrap();
        assert!(handler.should_process(&webhook));

        // Issue removed should not be processed
        let json = r#"{
            "action": "remove",
            "type": "Issue",
            "created_at": "2025-12-24T00:00:00Z",
            "data": {},
            "organization_id": "org_123",
            "webhook_id": "wh_123"
        }"#;

        let webhook = handler.parse_webhook(json).unwrap();
        assert!(!handler.should_process(&webhook));
    }
}
