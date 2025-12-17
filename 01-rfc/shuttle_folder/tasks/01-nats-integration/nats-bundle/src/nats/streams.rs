//! JetStream Stream Configuration
//!
//! Defines durable streams for message persistence.

use anyhow::Result;
use async_nats::jetstream::{self, stream};
use std::time::Duration;

/// Stream names
pub mod names {
    pub const KALI: &str = "KALI";
    pub const HASH: &str = "HASH";
    pub const CRATE: &str = "CRATE";
    pub const CDN: &str = "CDN";
    pub const IAC: &str = "IAC";
    pub const TELEMETRY: &str = "TELEMETRY";
    pub const GATEWAY: &str = "GATEWAY";
}

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub name: &'static str,
    pub subjects: Vec<&'static str>,
    pub max_msgs: i64,
    pub max_bytes: i64,
    pub max_age: Duration,
    pub max_msg_size: i32,
    pub storage: Storage,
    pub retention: Retention,
}

#[derive(Debug, Clone, Copy)]
pub enum Storage {
    File,
    Memory,
}

#[derive(Debug, Clone, Copy)]
pub enum Retention {
    Limits,
    Interest,
    WorkQueue,
}

/// Consumer configuration
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    pub stream: &'static str,
    pub name: &'static str,
    pub filter_subject: Option<&'static str>,
    pub ack_wait: Duration,
    pub max_deliver: i64,
    pub durable: bool,
}

/// All stream configurations
pub const STREAMS: &[StreamConfig] = &[
    // ═══════════════════════════════════════════════════════════════
    // KALI - Tool execution and chains
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::KALI,
        subjects: vec!["sx9.kali.>"],
        max_msgs: 100_000,
        max_bytes: 1024 * 1024 * 1024, // 1GB
        max_age: Duration::from_secs(24 * 60 * 60), // 24h
        max_msg_size: 1024 * 1024, // 1MB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // HASH - Hashing operations
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::HASH,
        subjects: vec!["sx9.hash.>"],
        max_msgs: 1_000_000,
        max_bytes: 500 * 1024 * 1024, // 500MB
        max_age: Duration::from_secs(60 * 60), // 1h
        max_msg_size: 64 * 1024, // 64KB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // CRATE - Smart crate lifecycle
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::CRATE,
        subjects: vec!["sx9.crate.>"],
        max_msgs: 10_000,
        max_bytes: 100 * 1024 * 1024, // 100MB
        max_age: Duration::from_secs(7 * 24 * 60 * 60), // 7d
        max_msg_size: 256 * 1024, // 256KB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // CDN - Content distribution
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::CDN,
        subjects: vec!["sx9.cdn.>"],
        max_msgs: 50_000,
        max_bytes: 5 * 1024 * 1024 * 1024, // 5GB
        max_age: Duration::from_secs(72 * 60 * 60), // 72h
        max_msg_size: 10 * 1024 * 1024, // 10MB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // IAC - Infrastructure triggers
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::IAC,
        subjects: vec!["sx9.iac.>"],
        max_msgs: 10_000,
        max_bytes: 100 * 1024 * 1024, // 100MB
        max_age: Duration::from_secs(30 * 24 * 60 * 60), // 30d
        max_msg_size: 1024 * 1024, // 1MB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // TELEMETRY - Audit and tracing
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::TELEMETRY,
        subjects: vec!["sx9.telemetry.>"],
        max_msgs: 1_000_000,
        max_bytes: 10 * 1024 * 1024 * 1024, // 10GB
        max_age: Duration::from_secs(30 * 24 * 60 * 60), // 30d
        max_msg_size: 64 * 1024, // 64KB
        storage: Storage::File,
        retention: Retention::Limits,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // GATEWAY - Request/response
    // ═══════════════════════════════════════════════════════════════
    StreamConfig {
        name: names::GATEWAY,
        subjects: vec!["sx9.gateway.>"],
        max_msgs: 100_000,
        max_bytes: 1024 * 1024 * 1024, // 1GB
        max_age: Duration::from_secs(60 * 60), // 1h
        max_msg_size: 1024 * 1024, // 1MB
        storage: Storage::File,
        retention: Retention::Limits,
    },
];

/// Default consumer configurations
pub const CONSUMERS: &[ConsumerConfig] = &[
    ConsumerConfig {
        stream: names::KALI,
        name: "kali-executor",
        filter_subject: Some("sx9.kali.exec.>"),
        ack_wait: Duration::from_secs(300),
        max_deliver: 3,
        durable: true,
    },
    ConsumerConfig {
        stream: names::KALI,
        name: "kali-chain",
        filter_subject: Some("sx9.kali.chain.>"),
        ack_wait: Duration::from_secs(600),
        max_deliver: 3,
        durable: true,
    },
    ConsumerConfig {
        stream: names::HASH,
        name: "hash-engine",
        filter_subject: Some("sx9.hash.compute.>"),
        ack_wait: Duration::from_secs(5),
        max_deliver: 3,
        durable: true,
    },
    ConsumerConfig {
        stream: names::CDN,
        name: "cdn-store",
        filter_subject: Some("sx9.cdn.store.>"),
        ack_wait: Duration::from_secs(30),
        max_deliver: 3,
        durable: true,
    },
    ConsumerConfig {
        stream: names::IAC,
        name: "iac-controller",
        filter_subject: Some("sx9.iac.trigger.>"),
        ack_wait: Duration::from_secs(120),
        max_deliver: 3,
        durable: true,
    },
];

/// Initialize all streams and consumers
pub async fn init_streams(js: &jetstream::Context) -> Result<()> {
    for stream_config in STREAMS {
        create_or_update_stream(js, stream_config).await?;
    }
    
    for consumer_config in CONSUMERS {
        create_or_update_consumer(js, consumer_config).await?;
    }
    
    Ok(())
}

/// Create or update a single stream
pub async fn create_or_update_stream(
    js: &jetstream::Context,
    config: &StreamConfig,
) -> Result<stream::Stream> {
    let storage = match config.storage {
        Storage::File => stream::StorageType::File,
        Storage::Memory => stream::StorageType::Memory,
    };
    
    let retention = match config.retention {
        Retention::Limits => stream::RetentionPolicy::Limits,
        Retention::Interest => stream::RetentionPolicy::Interest,
        Retention::WorkQueue => stream::RetentionPolicy::WorkQueue,
    };
    
    let stream_config = stream::Config {
        name: config.name.to_string(),
        subjects: config.subjects.iter().map(|s| s.to_string()).collect(),
        max_messages: config.max_msgs,
        max_bytes: config.max_bytes,
        max_age: config.max_age,
        max_message_size: config.max_msg_size,
        storage,
        retention,
        ..Default::default()
    };
    
    let stream = js.get_or_create_stream(stream_config).await?;
    tracing::info!("✅ Stream {} ready", config.name);
    
    Ok(stream)
}

/// Create or update a consumer
pub async fn create_or_update_consumer(
    js: &jetstream::Context,
    config: &ConsumerConfig,
) -> Result<()> {
    let stream = js.get_stream(config.stream).await?;
    
    let mut consumer_config = jetstream::consumer::pull::Config {
        durable_name: if config.durable {
            Some(config.name.to_string())
        } else {
            None
        },
        ack_wait: config.ack_wait,
        max_deliver: config.max_deliver,
        ..Default::default()
    };
    
    if let Some(filter) = config.filter_subject {
        consumer_config.filter_subject = filter.to_string();
    }
    
    stream.get_or_create_consumer(config.name, consumer_config).await?;
    tracing::info!("✅ Consumer {} on {} ready", config.name, config.stream);
    
    Ok(())
}

/// Get stream by name
pub fn get_stream_config(name: &str) -> Option<&'static StreamConfig> {
    STREAMS.iter().find(|s| s.name == name)
}

/// Get consumer by name
pub fn get_consumer_config(name: &str) -> Option<&'static ConsumerConfig> {
    CONSUMERS.iter().find(|c| c.name == name)
}
