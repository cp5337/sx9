//! Sledis command handling
//!
//! Parses and executes Redis commands against the Sledis store.

use super::{RespValue, SledisError, SledisResult, SledisStore, SledisValue};

/// Command types supported by Sledis
#[derive(Debug, Clone)]
pub enum SledisCommand {
    // String commands
    Get { key: String },
    Set { key: String, value: String, ex: Option<u64> },
    Del { keys: Vec<String> },
    Exists { key: String },
    Expire { key: String, seconds: u64 },
    Ttl { key: String },

    // Hash commands
    HGet { key: String, field: String },
    HSet { key: String, field: String, value: String },
    HDel { key: String, field: String },
    HGetAll { key: String },

    // List commands
    LPush { key: String, value: String },
    RPush { key: String, value: String },
    LPop { key: String },
    RPop { key: String },
    LRange { key: String, start: i64, stop: i64 },

    // Set commands
    SAdd { key: String, member: String },
    SRem { key: String, member: String },
    SMembers { key: String },
    SIsMember { key: String, member: String },

    // SX9 extensions
    HashSet { key: String, value: String, hash: String, ex: Option<u64> },
    HashGet { key: String },

    // Admin commands
    Ping,
    FlushDb,
    DbSize,
    Info,
}

impl SledisCommand {
    /// Parse command from RESP array
    pub fn parse(args: &[String]) -> SledisResult<Self> {
        if args.is_empty() {
            return Err(SledisError::Protocol("Empty command".to_string()));
        }

        let cmd = args[0].to_uppercase();
        match cmd.as_str() {
            // String commands
            "GET" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::Get { key: args[1].clone() })
            }
            "SET" => {
                require_args_min(&args, 3)?;
                let mut ex = None;
                if args.len() >= 5 && args[3].to_uppercase() == "EX" {
                    ex = Some(args[4].parse().map_err(|_| {
                        SledisError::Protocol("Invalid EX value".to_string())
                    })?);
                }
                Ok(SledisCommand::Set {
                    key: args[1].clone(),
                    value: args[2].clone(),
                    ex,
                })
            }
            "DEL" => {
                require_args_min(&args, 2)?;
                Ok(SledisCommand::Del {
                    keys: args[1..].to_vec(),
                })
            }
            "EXISTS" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::Exists { key: args[1].clone() })
            }
            "EXPIRE" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::Expire {
                    key: args[1].clone(),
                    seconds: args[2].parse().map_err(|_| {
                        SledisError::Protocol("Invalid seconds value".to_string())
                    })?,
                })
            }
            "TTL" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::Ttl { key: args[1].clone() })
            }

            // Hash commands
            "HGET" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::HGet {
                    key: args[1].clone(),
                    field: args[2].clone(),
                })
            }
            "HSET" => {
                require_args(&args, 4)?;
                Ok(SledisCommand::HSet {
                    key: args[1].clone(),
                    field: args[2].clone(),
                    value: args[3].clone(),
                })
            }
            "HDEL" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::HDel {
                    key: args[1].clone(),
                    field: args[2].clone(),
                })
            }
            "HGETALL" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::HGetAll { key: args[1].clone() })
            }

            // List commands
            "LPUSH" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::LPush {
                    key: args[1].clone(),
                    value: args[2].clone(),
                })
            }
            "RPUSH" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::RPush {
                    key: args[1].clone(),
                    value: args[2].clone(),
                })
            }
            "LPOP" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::LPop { key: args[1].clone() })
            }
            "RPOP" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::RPop { key: args[1].clone() })
            }
            "LRANGE" => {
                require_args(&args, 4)?;
                Ok(SledisCommand::LRange {
                    key: args[1].clone(),
                    start: args[2].parse().map_err(|_| {
                        SledisError::Protocol("Invalid start value".to_string())
                    })?,
                    stop: args[3].parse().map_err(|_| {
                        SledisError::Protocol("Invalid stop value".to_string())
                    })?,
                })
            }

            // Set commands
            "SADD" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::SAdd {
                    key: args[1].clone(),
                    member: args[2].clone(),
                })
            }
            "SREM" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::SRem {
                    key: args[1].clone(),
                    member: args[2].clone(),
                })
            }
            "SMEMBERS" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::SMembers { key: args[1].clone() })
            }
            "SISMEMBER" => {
                require_args(&args, 3)?;
                Ok(SledisCommand::SIsMember {
                    key: args[1].clone(),
                    member: args[2].clone(),
                })
            }

            // SX9 extensions
            "HASHSET" => {
                require_args_min(&args, 4)?;
                let mut ex = None;
                if args.len() >= 6 && args[4].to_uppercase() == "EX" {
                    ex = Some(args[5].parse().map_err(|_| {
                        SledisError::Protocol("Invalid EX value".to_string())
                    })?);
                }
                Ok(SledisCommand::HashSet {
                    key: args[1].clone(),
                    value: args[2].clone(),
                    hash: args[3].clone(),
                    ex,
                })
            }
            "HASHGET" => {
                require_args(&args, 2)?;
                Ok(SledisCommand::HashGet { key: args[1].clone() })
            }

            // Admin commands
            "PING" => Ok(SledisCommand::Ping),
            "FLUSHDB" => Ok(SledisCommand::FlushDb),
            "DBSIZE" => Ok(SledisCommand::DbSize),
            "INFO" => Ok(SledisCommand::Info),

            _ => Err(SledisError::Protocol(format!("Unknown command: {}", cmd))),
        }
    }

    /// Execute command against store
    pub fn execute(&self, store: &SledisStore) -> SledisResult<RespValue> {
        match self {
            // String commands
            SledisCommand::Get { key } => match store.get(key)? {
                Some(SledisValue::String(s)) => Ok(RespValue::bulk(s)),
                Some(SledisValue::Integer(i)) => Ok(RespValue::bulk(i.to_string())),
                Some(_) => Err(SledisError::WrongType),
                None => Ok(RespValue::null()),
            },
            SledisCommand::Set { key, value, ex } => {
                store.set(key, SledisValue::String(value.clone()), *ex)?;
                Ok(RespValue::ok())
            }
            SledisCommand::Del { keys } => {
                let keys_ref: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                let count = store.del(&keys_ref)?;
                Ok(RespValue::int(count as i64))
            }
            SledisCommand::Exists { key } => {
                let exists = store.exists(key)?;
                Ok(RespValue::int(if exists { 1 } else { 0 }))
            }
            SledisCommand::Expire { key, seconds } => {
                let success = store.expire(key, *seconds)?;
                Ok(RespValue::int(if success { 1 } else { 0 }))
            }
            SledisCommand::Ttl { key } => match store.ttl(key)? {
                Some(ttl) => Ok(RespValue::int(ttl)),
                None => Ok(RespValue::int(-2)), // Key does not exist
            },

            // Hash commands
            SledisCommand::HGet { key, field } => match store.hget(key, field)? {
                Some(value) => Ok(RespValue::bulk(value)),
                None => Ok(RespValue::null()),
            },
            SledisCommand::HSet { key, field, value } => {
                let is_new = store.hset(key, field, value)?;
                Ok(RespValue::int(if is_new { 1 } else { 0 }))
            }
            SledisCommand::HDel { key, field } => {
                let removed = store.hdel(key, field)?;
                Ok(RespValue::int(if removed { 1 } else { 0 }))
            }
            SledisCommand::HGetAll { key } => {
                let hash = store.hgetall(key)?;
                let mut items = Vec::new();
                for (k, v) in hash {
                    items.push(RespValue::bulk(k));
                    items.push(RespValue::bulk(v));
                }
                Ok(RespValue::array(items))
            }

            // List commands
            SledisCommand::LPush { key, value } => {
                let len = store.lpush(key, value)?;
                Ok(RespValue::int(len as i64))
            }
            SledisCommand::RPush { key, value } => {
                let len = store.rpush(key, value)?;
                Ok(RespValue::int(len as i64))
            }
            SledisCommand::LPop { key } => match store.lpop(key)? {
                Some(value) => Ok(RespValue::bulk(value)),
                None => Ok(RespValue::null()),
            },
            SledisCommand::RPop { key } => match store.rpop(key)? {
                Some(value) => Ok(RespValue::bulk(value)),
                None => Ok(RespValue::null()),
            },
            SledisCommand::LRange { key, start, stop } => {
                let items = store.lrange(key, *start, *stop)?;
                Ok(RespValue::array(
                    items.into_iter().map(RespValue::bulk).collect(),
                ))
            }

            // Set commands
            SledisCommand::SAdd { key, member } => {
                let is_new = store.sadd(key, member)?;
                Ok(RespValue::int(if is_new { 1 } else { 0 }))
            }
            SledisCommand::SRem { key, member } => {
                let removed = store.srem(key, member)?;
                Ok(RespValue::int(if removed { 1 } else { 0 }))
            }
            SledisCommand::SMembers { key } => {
                let members = store.smembers(key)?;
                Ok(RespValue::array(
                    members.into_iter().map(RespValue::bulk).collect(),
                ))
            }
            SledisCommand::SIsMember { key, member } => {
                let is_member = store.sismember(key, member)?;
                Ok(RespValue::int(if is_member { 1 } else { 0 }))
            }

            // SX9 extensions
            SledisCommand::HashSet { key, value, hash, ex } => {
                store.set_with_hash(key, SledisValue::String(value.clone()), hash, *ex)?;
                Ok(RespValue::ok())
            }
            SledisCommand::HashGet { key } => match store.get_hash(key)? {
                Some(hash) => Ok(RespValue::bulk(hash)),
                None => Ok(RespValue::null()),
            },

            // Admin commands
            SledisCommand::Ping => Ok(RespValue::SimpleString("PONG".to_string())),
            SledisCommand::FlushDb => {
                store.flushdb()?;
                Ok(RespValue::ok())
            }
            SledisCommand::DbSize => {
                let size = store.dbsize()?;
                Ok(RespValue::int(size as i64))
            }
            SledisCommand::Info => Ok(RespValue::bulk(format!(
                "# Sledis\nversion:1.0.0\nport:{}\nkeys:{}\n",
                super::SLEDIS_PORT,
                store.dbsize().unwrap_or(0)
            ))),
        }
    }
}

fn require_args(args: &[String], expected: usize) -> SledisResult<()> {
    if args.len() != expected {
        return Err(SledisError::Protocol(format!(
            "Wrong number of arguments: expected {}, got {}",
            expected - 1,
            args.len() - 1
        )));
    }
    Ok(())
}

fn require_args_min(args: &[String], min: usize) -> SledisResult<()> {
    if args.len() < min {
        return Err(SledisError::Protocol(format!(
            "Wrong number of arguments: expected at least {}, got {}",
            min - 1,
            args.len() - 1
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        let cmd = SledisCommand::parse(&["GET".to_string(), "mykey".to_string()]).unwrap();
        match cmd {
            SledisCommand::Get { key } => assert_eq!(key, "mykey"),
            _ => panic!("Expected GET command"),
        }
    }

    #[test]
    fn test_parse_set_with_ex() {
        let cmd = SledisCommand::parse(&[
            "SET".to_string(),
            "mykey".to_string(),
            "myvalue".to_string(),
            "EX".to_string(),
            "60".to_string(),
        ])
        .unwrap();
        match cmd {
            SledisCommand::Set { key, value, ex } => {
                assert_eq!(key, "mykey");
                assert_eq!(value, "myvalue");
                assert_eq!(ex, Some(60));
            }
            _ => panic!("Expected SET command"),
        }
    }

    #[test]
    fn test_parse_hashset() {
        let cmd = SledisCommand::parse(&[
            "HASHSET".to_string(),
            "mykey".to_string(),
            "myvalue".to_string(),
            "abc123def456".to_string(),
        ])
        .unwrap();
        match cmd {
            SledisCommand::HashSet { key, value, hash, ex } => {
                assert_eq!(key, "mykey");
                assert_eq!(value, "myvalue");
                assert_eq!(hash, "abc123def456");
                assert_eq!(ex, None);
            }
            _ => panic!("Expected HASHSET command"),
        }
    }
}
