//! Conversation context window management
//!
//! This module provides a sliding window of conversation messages
//! to maintain context within token limits.

use crate::messages::{Message, MessageContent};

/// A sliding window of conversation context
#[derive(Debug, Clone)]
pub struct ContextWindow {
    /// Maximum number of messages to keep
    max_messages: usize,

    /// Maximum estimated tokens (rough approximation)
    max_tokens: usize,

    /// The messages in the context
    messages: Vec<Message>,

    /// System prompt (always included)
    system: Option<String>,
}

impl Default for ContextWindow {
    fn default() -> Self {
        Self {
            max_messages: 50,
            max_tokens: 100_000,
            messages: Vec::new(),
            system: None,
        }
    }
}

impl ContextWindow {
    /// Create a new context window with default limits
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of messages
    pub fn with_max_messages(mut self, max: usize) -> Self {
        self.max_messages = max;
        self
    }

    /// Set the maximum token budget
    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = max;
        self
    }

    /// Set the system prompt
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Add a user message
    pub fn add_user(&mut self, content: impl Into<String>) {
        self.messages.push(Message::user(content));
        self.trim();
    }

    /// Add an assistant message
    pub fn add_assistant(&mut self, content: impl Into<String>) {
        self.messages.push(Message::assistant(content));
        self.trim();
    }

    /// Add a message
    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
        self.trim();
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Get the system prompt
    pub fn system(&self) -> Option<&str> {
        self.system.as_deref()
    }

    /// Estimate total tokens in the context
    pub fn estimated_tokens(&self) -> usize {
        let system_tokens = self
            .system
            .as_ref()
            .map(|s| estimate_tokens(s))
            .unwrap_or(0);

        let message_tokens: usize = self
            .messages
            .iter()
            .map(|m| match &m.content {
                MessageContent::Text(t) => estimate_tokens(t),
                MessageContent::Blocks(blocks) => blocks
                    .iter()
                    .map(|b| {
                        if let Some(text) = b.as_text() {
                            estimate_tokens(text)
                        } else {
                            100 // Rough estimate for non-text blocks
                        }
                    })
                    .sum(),
            })
            .sum();

        system_tokens + message_tokens
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Trim the context to fit within limits
    fn trim(&mut self) {
        // Trim by message count
        while self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }

        // Trim by token count (rough approximation)
        while self.estimated_tokens() > self.max_tokens && self.messages.len() > 1 {
            self.messages.remove(0);
        }
    }

    /// Get the last N messages
    pub fn last_n(&self, n: usize) -> &[Message] {
        let start = self.messages.len().saturating_sub(n);
        &self.messages[start..]
    }

    /// Check if context is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get the number of messages
    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

/// Rough token estimation (chars / 4)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::Role;

    #[test]
    fn test_context_window_basic() {
        let mut ctx = ContextWindow::new();

        ctx.add_user("Hello");
        ctx.add_assistant("Hi there!");

        assert_eq!(ctx.len(), 2);
        assert_eq!(ctx.messages()[0].role, Role::User);
        assert_eq!(ctx.messages()[1].role, Role::Assistant);
    }

    #[test]
    fn test_context_window_trim() {
        let mut ctx = ContextWindow::new().with_max_messages(2);

        ctx.add_user("Message 1");
        ctx.add_assistant("Response 1");
        ctx.add_user("Message 2");

        assert_eq!(ctx.len(), 2);
        // First message should be trimmed
    }
}
