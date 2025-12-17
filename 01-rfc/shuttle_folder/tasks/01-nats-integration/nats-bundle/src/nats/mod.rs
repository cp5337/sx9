//! SX9 Foundation NATS Integration
//!
//! Provides unified NATS/JetStream access for all SX9 services.
//!
//! # Feature Flag
//!
//! Enable with `features = ["messaging"]` in Cargo.toml
//!
//! # Usage
//!
//! ```rust,ignore
//! use sx9_foundation_core::nats::{connect, subjects, streams};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = connect().await?;
//!     
//!     // Publish
//!     client.publish(subjects::TICK_SYNC, "ping".into()).await?;
//!     
//!     // Subscribe
//!     let mut sub = client.subscribe(subjects::TICK_SYNC).await?;
//!     while let Some(msg) = sub.next().await {
//!         println!("Received: {:?}", msg.payload);
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod client;
mod messages;
mod streams;
mod subjects;

pub use client::{connect, connect_with_options, jetstream, NatsConfig};
pub use messages::{NatsHeader, NatsMessage};
pub use streams::{StreamConfig, ConsumerConfig, STREAMS};
pub use subjects::*;

/// NATS connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Reconnecting,
}

/// Re-export async_nats types for convenience
pub mod types {
    pub use async_nats::{
        Client, 
        Message, 
        Subject,
        PublishError,
        SubscribeError,
        jetstream::{
            self,
            Context as JetStreamContext,
            consumer::Consumer,
            stream::Stream,
        },
    };
}
