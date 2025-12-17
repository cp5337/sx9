//! OSSEC Integration for Plasma Defender
//!
//! Provides:
//! - MITRE ATT&CK technique mapping
//! - Alert parsing and processing
//! - Rule-to-ECS entity conversion
//! - HD4 phase determination from alert severity

pub mod alert_parser;
pub mod mitre_map;
pub mod ossec_agent;

pub use alert_parser::{OssecAlertParser, ParsedAlert};
pub use mitre_map::{Hd4Level, MitreMapping, TacticPrimitive};
pub use ossec_agent::OssecAgent;
