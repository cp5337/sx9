//! SSE streaming response handler
//!
//! This module handles Server-Sent Events (SSE) streaming responses from the Claude API.

use futures::stream::Stream;
use pin_project_lite::pin_project;
use serde::Deserialize;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::messages::{ContentBlock, StopReason};
use crate::{ClaudeError, Result};

pin_project! {
    /// A stream of message events from a streaming response
    pub struct MessageStream {
        #[pin]
        response: reqwest::Response,
        buffer: String,
        done: bool,
    }
}

impl MessageStream {
    /// Create a new message stream from a response
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self {
            response,
            buffer: String::new(),
            done: false,
        }
    }

    /// Collect all events into a complete response
    pub async fn collect_text(&mut self) -> Result<String> {
        use futures::StreamExt;

        let mut text = String::new();

        while let Some(event) = self.next().await {
            match event? {
                StreamEvent::ContentBlockDelta { delta, .. } => {
                    if let ContentDelta::TextDelta { text: t } = delta {
                        text.push_str(&t);
                    }
                }
                StreamEvent::MessageStop => break,
                _ => {}
            }
        }

        Ok(text)
    }

    /// Parse SSE data from the buffer
    fn parse_sse_event(&mut self) -> Option<Result<StreamEvent>> {
        // Find the next complete SSE event (ends with double newline)
        if let Some(event_end) = self.buffer.find("\n\n") {
            let event_data = self.buffer[..event_end].to_string();
            self.buffer = self.buffer[event_end + 2..].to_string();

            // Parse the event
            let mut event_type = None;
            let mut data = None;

            for line in event_data.lines() {
                if let Some(rest) = line.strip_prefix("event: ") {
                    event_type = Some(rest.to_string());
                } else if let Some(rest) = line.strip_prefix("data: ") {
                    data = Some(rest.to_string());
                }
            }

            match (event_type.as_deref(), data) {
                (Some("message_start"), Some(d)) => {
                    match serde_json::from_str::<MessageStartWrapper>(&d) {
                        Ok(wrapper) => Some(Ok(StreamEvent::MessageStart {
                            message_id: wrapper.message.id,
                            model: wrapper.message.model,
                            usage: wrapper.message.usage,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                (Some("content_block_start"), Some(d)) => {
                    match serde_json::from_str::<ContentBlockStartWrapper>(&d) {
                        Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockStart {
                            index: wrapper.index,
                            content_block: wrapper.content_block,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                (Some("content_block_delta"), Some(d)) => {
                    match serde_json::from_str::<ContentBlockDeltaWrapper>(&d) {
                        Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockDelta {
                            index: wrapper.index,
                            delta: wrapper.delta,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                (Some("content_block_stop"), Some(d)) => {
                    match serde_json::from_str::<ContentBlockStopWrapper>(&d) {
                        Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockStop {
                            index: wrapper.index,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                (Some("message_delta"), Some(d)) => {
                    match serde_json::from_str::<MessageDeltaWrapper>(&d) {
                        Ok(wrapper) => Some(Ok(StreamEvent::MessageDelta {
                            stop_reason: wrapper.delta.stop_reason,
                            usage: wrapper.usage,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                (Some("message_stop"), _) => Some(Ok(StreamEvent::MessageStop)),
                (Some("ping"), _) => Some(Ok(StreamEvent::Ping)),
                (Some("error"), Some(d)) => {
                    match serde_json::from_str::<ErrorWrapper>(&d) {
                        Ok(wrapper) => Some(Err(ClaudeError::Api {
                            status: 0,
                            error_type: wrapper.error.error_type,
                            message: wrapper.error.message,
                        })),
                        Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
                    }
                }
                _ => None, // Unknown event type, skip
            }
        } else {
            None
        }
    }
}

impl Stream for MessageStream {
    type Item = Result<StreamEvent>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if *this.done {
            return Poll::Ready(None);
        }

        // Check if we have a complete event in the buffer
        if let Some(event) = {
            let buffer = &mut *this.buffer;
            let done = &mut *this.done;

            if let Some(event_end) = buffer.find("\n\n") {
                let event_data = buffer[..event_end].to_string();
                *buffer = buffer[event_end + 2..].to_string();

                parse_sse_event_data(&event_data, done)
            } else {
                None
            }
        } {
            return Poll::Ready(Some(event));
        }

        // Need to read more data
        // Note: This is a simplified implementation. In production, you'd want
        // to properly poll the response body stream.
        Poll::Pending
    }
}

fn parse_sse_event_data(event_data: &str, done: &mut bool) -> Option<Result<StreamEvent>> {
    let mut event_type = None;
    let mut data = None;

    for line in event_data.lines() {
        if let Some(rest) = line.strip_prefix("event: ") {
            event_type = Some(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("data: ") {
            data = Some(rest.to_string());
        }
    }

    match (event_type.as_deref(), data) {
        (Some("message_start"), Some(d)) => {
            match serde_json::from_str::<MessageStartWrapper>(&d) {
                Ok(wrapper) => Some(Ok(StreamEvent::MessageStart {
                    message_id: wrapper.message.id,
                    model: wrapper.message.model,
                    usage: wrapper.message.usage,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        (Some("content_block_start"), Some(d)) => {
            match serde_json::from_str::<ContentBlockStartWrapper>(&d) {
                Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockStart {
                    index: wrapper.index,
                    content_block: wrapper.content_block,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        (Some("content_block_delta"), Some(d)) => {
            match serde_json::from_str::<ContentBlockDeltaWrapper>(&d) {
                Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockDelta {
                    index: wrapper.index,
                    delta: wrapper.delta,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        (Some("content_block_stop"), Some(d)) => {
            match serde_json::from_str::<ContentBlockStopWrapper>(&d) {
                Ok(wrapper) => Some(Ok(StreamEvent::ContentBlockStop {
                    index: wrapper.index,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        (Some("message_delta"), Some(d)) => {
            match serde_json::from_str::<MessageDeltaWrapper>(&d) {
                Ok(wrapper) => Some(Ok(StreamEvent::MessageDelta {
                    stop_reason: wrapper.delta.stop_reason,
                    usage: wrapper.usage,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        (Some("message_stop"), _) => {
            *done = true;
            Some(Ok(StreamEvent::MessageStop))
        }
        (Some("ping"), _) => Some(Ok(StreamEvent::Ping)),
        (Some("error"), Some(d)) => {
            *done = true;
            match serde_json::from_str::<ErrorWrapper>(&d) {
                Ok(wrapper) => Some(Err(ClaudeError::Api {
                    status: 0,
                    error_type: wrapper.error.error_type,
                    message: wrapper.error.message,
                })),
                Err(e) => Some(Err(ClaudeError::Stream(e.to_string()))),
            }
        }
        _ => None,
    }
}

/// Events emitted by the streaming API
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Message started
    MessageStart {
        message_id: String,
        model: String,
        usage: InputUsage,
    },

    /// Content block started
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },

    /// Content block delta (incremental update)
    ContentBlockDelta {
        index: usize,
        delta: ContentDelta,
    },

    /// Content block stopped
    ContentBlockStop {
        index: usize,
    },

    /// Message delta (final metadata)
    MessageDelta {
        stop_reason: Option<StopReason>,
        usage: OutputUsage,
    },

    /// Message completed
    MessageStop,

    /// Ping (keep-alive)
    Ping,
}

/// Content delta types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentDelta {
    /// Text content delta
    TextDelta { text: String },

    /// Input JSON delta for tool use
    InputJsonDelta { partial_json: String },
}

/// Input token usage
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct InputUsage {
    pub input_tokens: u32,
}

/// Output token usage
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct OutputUsage {
    pub output_tokens: u32,
}

// Internal wrapper types for parsing

#[derive(Deserialize)]
struct MessageStartWrapper {
    message: MessageStartMessage,
}

#[derive(Deserialize)]
struct MessageStartMessage {
    id: String,
    model: String,
    usage: InputUsage,
}

#[derive(Deserialize)]
struct ContentBlockStartWrapper {
    index: usize,
    content_block: ContentBlock,
}

#[derive(Deserialize)]
struct ContentBlockDeltaWrapper {
    index: usize,
    delta: ContentDelta,
}

#[derive(Deserialize)]
struct ContentBlockStopWrapper {
    index: usize,
}

#[derive(Deserialize)]
struct MessageDeltaWrapper {
    delta: DeltaPayload,
    usage: OutputUsage,
}

#[derive(Deserialize)]
struct DeltaPayload {
    stop_reason: Option<StopReason>,
}

#[derive(Deserialize)]
struct ErrorWrapper {
    error: ErrorPayload,
}

#[derive(Deserialize)]
struct ErrorPayload {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}
