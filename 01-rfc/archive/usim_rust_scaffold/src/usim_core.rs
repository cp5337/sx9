
use serde::Serialize;
use uuid::Uuid;
use murmur3::murmur3_32::MurmurHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize)]
pub struct UsimRecord {
    pub cuid: String,
    pub uuid: String,
    pub sch: u32,
    pub ttl: u64,
    pub raw: String,
}

pub fn usimify_record(input: String) -> UsimRecord {
    let cuid = Uuid::new_v4().to_string();
    let uuid = Uuid::new_v4().to_string();

    let mut hasher = MurmurHasher::default();
    input.hash(&mut hasher);
    let sch = hasher.finish() as u32;

    UsimRecord {
        cuid,
        uuid,
        sch,
        ttl: 86400,
        raw: input,
    }
}
