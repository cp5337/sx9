// sx9-gateway/src/cdn/mod.rs
//
// CDN Module - Local CDN services for SX9 Gateway
//
// Includes:
// - R2 Subscriber (CloudFlare R2 sync)
// - ChromaDB Service (vector search)
// - Threat Intel CDN (local files)
// - OSINT Data CDN (local database)

pub mod r2_subscriber;

// Re-export main types
pub use r2_subscriber::{R2Config, R2SubscriberService};
