use async_nats::{Client, Message};
use futures::StreamExt;
use log::{error, info, warn};
use pest::Parser;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct RouterParser;

pub struct Reactor {
    nats: Client,
    last_atlas_tick: Arc<RwLock<Instant>>,
}

impl Reactor {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let nats = async_nats::connect(url).await?;
        Ok(Self {
            nats,
            last_atlas_tick: Arc::new(RwLock::new(Instant::now())),
        })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("🌀 Starting Smart Crate Reactor (Zero Trust Mode)...");

        // 1. Subscribe to Ops Heartbeat (ATLAS)
        let mut atlas_sub = self.nats.subscribe("atlas.tick").await?;
        let tick_state = self.last_atlas_tick.clone();

        tokio::spawn(async move {
            while let Some(_) = atlas_sub.next().await {
                let mut lock = tick_state.write().unwrap();
                *lock = Instant::now();
            }
        });

        // 2. Subscribe to Application Data
        let mut router_sub = self.nats.subscribe("sx9.core.router.>").await?;

        while let Some(msg) = router_sub.next().await {
            // Factor 1: System Check (Ops)
            if !self.is_system_healthy() {
                warn!("🔒 SYSTEM LOCK: ATLAS Heartbeat missing (>100ms). Dropping packet.");
                continue;
            }

            // Factor 2: Context Check (Dev)
            self.process_message(msg).await;
        }

        Ok(())
    }

    fn is_system_healthy(&self) -> bool {
        let lock = self.last_atlas_tick.read().unwrap();
        lock.elapsed() < Duration::from_millis(100)
    }

    async fn process_message(&self, msg: Message) {
        let payload_str = String::from_utf8_lossy(&msg.payload);

        // Zero Trust: Must parse via Grammar
        match RouterParser::parse(Rule::main, &payload_str) {
            Ok(_) => {
                info!("✅ Valid Routing Command: {}", payload_str);
                // Implementation would execute route logic here...
            }
            Err(e) => {
                error!("❌ Context Failure: Invalid DSL or Hash. {}", e);
            }
        }
    }
}
