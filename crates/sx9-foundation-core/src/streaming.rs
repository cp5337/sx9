//! CTAS-7 Streaming Event Ingestion
//!
//! Uses canonical 64-bit MurmurHash3 from hash64 module.
//! All outputs are Base96 encoded per RFC-9001/RFC-9002.

use legion::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use serde::{Serialize, Deserialize};

use crate::hash64::{murmur3_64_base96, seeds};
use crate::components::*;

/// Streaming event for ingestion
#[derive(Clone, Debug)]
pub struct StreamingEvent {
    pub raw: Vec<u8>,
    pub sch: String,
    pub uuid: String,
    pub tail: String,
    pub pos: Option<Position>,
    pub ttl_secs: u64,
    pub tags: Vec<String>,
    pub meta: NodeMeta,
}

/// Tail policy for SCH suffix handling
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TailPolicy {
    /// Append tail to SCH
    Enforced,
    /// Suppress tail from SCH
    Suppressed,
}

/// Compute MurmurHash3 64-bit as Base96 (11 chars for full 64-bit entropy)
pub fn murmur_base96(data: &[u8]) -> String {
    murmur3_64_base96(data, seeds::SLOT, 11)
}

/// Compute MurmurHash3 64-bit as hex (16 chars)
pub fn murmur_hex(data: &[u8]) -> String {
    crate::hash64::murmur3_64_hex(data, seeds::SLOT)
}

/// Ingest a streaming event into the world
pub fn ingest_event(world: &mut World, evt: StreamingEvent, policy: TailPolicy) -> Entity {
    let sch_id = match policy {
        TailPolicy::Enforced => format!("{}{}", evt.sch, evt.tail),
        TailPolicy::Suppressed => evt.sch.clone(),
    };
    let trip = TriptyxId {
        sch: sch_id,
        uuid: evt.uuid.clone(),
        cuid: evt.uuid.clone(),
    };
    let pos = evt.pos.unwrap_or(Position {
        lat: 0.0,
        lon: 0.0,
        alt_m: 0.0,
        sigma_m: 99999.0,
    });
    world.push((
        trip,
        pos,
        Convergence::new(),
        ActivityState::Investigating,
        OodaPhase::Sense,
        Velocity { mps: 0.0 },
        evt.meta,
    ))
}

/// Ephemeral marker for TTL-based cleanup
#[derive(Clone, Debug)]
pub struct Ephemeral {
    pub expires_at: time::OffsetDateTime,
}

/// TTL cleanup system - removes expired entities
#[system]
#[read_component(Ephemeral)]
pub fn ttl_cleanup(world: &SubWorld, cmd: &mut CommandBuffer) {
    let now = time::OffsetDateTime::now_utc();
    let mut query = <(Entity, &Ephemeral)>::query();

    for (entity, eph) in query.iter(world) {
        if eph.expires_at <= now {
            cmd.remove(*entity);
        }
    }
}
