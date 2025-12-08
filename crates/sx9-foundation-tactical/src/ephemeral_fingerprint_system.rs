//! CTAS Ephemeral Asset Fingerprinting System
//! 
//! Deploy ephemeral assets that adapt based on HD4 perspective:
//! 🛡️ DEFENSIVE: Pop up honeypots/traps to snare and fingerprint adversaries
//! ⚔️ OFFENSIVE: Deploy covert assets to fingerprint target defenses

use bevy::prelude::*;
use crate::hd4_perspective_system::{HD4Perspective, HD4Operation, HD4Phase};
use ctas_slotgraph_tools::{
    node_types::{TaskMetadata, TaskPriority, SlotGraphNode, SlotNodeType, NodeState, NodeStatus},
    cognigraph::universal_nodes::{UniversalNodeType, NodeFunctionExecutor}
};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Ephemeral asset for fingerprinting operations
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralAsset {
    pub asset_id: Uuid,
    pub asset_name: String,
    pub asset_type: EphemeralAssetType,
    pub deployment_perspective: HD4Perspective,
    pub target_signature: String,
    pub lifespan_seconds: u64,
    pub deployment_time: f64,
    pub fingerprint_data: FingerprintData,
    pub stealth_level: f32,
    pub interaction_count: u32,
    pub status: AssetStatus,
}

/// Types of ephemeral assets based on perspective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EphemeralAssetType {
    // 🛡️ DEFENSIVE Assets (catch adversaries)
    HoneypotWebServer,     // Fake web server to catch scanning
    DeceptiveDatabase,     // Fake DB with monitoring
    TrapEmailServer,       // Email honeypot for phishing detection
    CanaryFile,            // Monitored decoy files
    FakeCredentialStore,   // Honeypot credentials
    NetworkDecoyService,   // Fake network services
    FingerprintTrap,       // Specifically designed to fingerprint tools
    
    // ⚔️ OFFENSIVE Assets (test targets)
    CovertBeacon,          // C2 beacon to test detection
    PivotPoint,            // Staging area for lateral movement
    ExfilChannel,          // Data exfiltration testing
    DefenseProbe,          // Test target defense capabilities
    ReconAsset,            // Intelligence gathering point
    CoverAsset,            // Legitimate-looking cover for operations
    CounterfingerTrap,     // Fingerprint target's defensive tools
}

/// Asset deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetStatus {
    Deploying,
    Active,
    Triggered,
    Compromised,
    Expired,
    Recalled,
}

/// Fingerprint data collected by asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintData {
    pub interactions: Vec<FingerprintInteraction>,
    pub tool_signatures: Vec<String>,
    pub behavioral_patterns: Vec<String>,
    pub timing_patterns: Vec<f64>,
    pub source_ips: Vec<String>,
    pub user_agents: Vec<String>,
    pub attack_vectors: Vec<String>,
    pub confidence_score: f32,
}

/// Individual fingerprint interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintInteraction {
    pub interaction_id: Uuid,
    pub timestamp: f64,
    pub interaction_type: InteractionType,
    pub source_info: String,
    pub payload: String,
    pub signature_match: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    PortScan,
    VulnScan,
    ServiceProbe,
    LoginAttempt,
    FileAccess,
    NetworkConnection,
    ExploitAttempt,
    DataQuery,
}

/// System for managing ephemeral assets
pub fn ephemeral_asset_management_system(
    mut asset_query: Query<(Entity, &mut EphemeralAsset, &mut NodeState)>,
    hd4_query: Query<&HD4Operation>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut asset, mut node_state) in asset_query.iter_mut() {
        let current_time = time.elapsed_seconds_f64();
        
        // Check asset lifespan
        if current_time - asset.deployment_time > asset.lifespan_seconds as f64 {
            asset.status = AssetStatus::Expired;
            info!("⏰ Ephemeral asset '{}' expired after {} seconds", 
                asset.asset_name, asset.lifespan_seconds);
            
            // Generate final fingerprint report
            generate_fingerprint_report(&mut commands, entity, &asset);
            continue;
        }
        
        // Process asset based on perspective and type
        match asset.deployment_perspective {
            HD4Perspective::Defensive => process_defensive_asset(&mut asset, current_time),
            HD4Perspective::Offensive => process_offensive_asset(&mut asset, current_time),
        }
        
        // Update node state based on interactions
        if asset.interaction_count > 0 {
            node_state.status = NodeStatus::Executing;
            asset.status = AssetStatus::Triggered;
        }
        
        node_state.last_update = time.elapsed_seconds();
    }
}

/// Process defensive ephemeral assets (catching adversaries)
fn process_defensive_asset(asset: &mut EphemeralAsset, current_time: f64) {
    match asset.asset_type {
        EphemeralAssetType::HoneypotWebServer => {
            // Simulate web server interactions
            if fastrand::f32() < 0.3 { // 30% chance of interaction per cycle
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::PortScan,
                    source_info: "192.168.1.100".to_string(),
                    payload: "HTTP GET / User-Agent: Nmap Scripting Engine".to_string(),
                    signature_match: Some("Nmap".to_string()),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.tool_signatures.push("Nmap NSE".to_string());
                asset.interaction_count += 1;
                
                info!("🍯 Honeypot '{}' caught adversary probe - Nmap detected!", asset.asset_name);
            }
        }
        
        EphemeralAssetType::FingerprintTrap => {
            // Specialized fingerprinting trap
            if fastrand::f32() < 0.2 { // 20% chance of detailed fingerprinting
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::VulnScan,
                    source_info: "10.0.0.50".to_string(),
                    payload: "OpenVAS vulnerability scanner probe".to_string(),
                    signature_match: Some("OpenVAS".to_string()),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.behavioral_patterns.push("Sequential port enumeration".to_string());
                asset.fingerprint_data.timing_patterns.push(current_time);
                asset.interaction_count += 1;
                
                info!("🔍 Fingerprint trap '{}' identified adversary tool - OpenVAS!", asset.asset_name);
            }
        }
        
        EphemeralAssetType::DeceptiveDatabase => {
            // Fake database to catch SQL injection attempts
            if fastrand::f32() < 0.15 {
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::ExploitAttempt,
                    source_info: "172.16.0.25".to_string(),
                    payload: "'; DROP TABLE users; --".to_string(),
                    signature_match: Some("SQLMap".to_string()),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.attack_vectors.push("SQL Injection".to_string());
                asset.interaction_count += 1;
                
                info!("🗃️  Deceptive database '{}' caught SQL injection attempt!", asset.asset_name);
            }
        }
        
        _ => {
            // Generic defensive asset processing
            if fastrand::f32() < 0.1 {
                asset.interaction_count += 1;
                info!("🛡️  Defensive asset '{}' recorded suspicious activity", asset.asset_name);
            }
        }
    }
}

/// Process offensive ephemeral assets (testing targets)
fn process_offensive_asset(asset: &mut EphemeralAsset, current_time: f64) {
    match asset.asset_type {
        EphemeralAssetType::CovertBeacon => {
            // Test if target can detect our C2 beacon
            if fastrand::f32() < 0.4 { // 40% chance target responds
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::NetworkConnection,
                    source_info: "TARGET-SOC-SYSTEM".to_string(),
                    payload: "Connection blocked by endpoint protection".to_string(),
                    signature_match: Some("CrowdStrike Falcon".to_string()),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.tool_signatures.push("Endpoint Protection".to_string());
                asset.interaction_count += 1;
                
                info!("📡 Covert beacon '{}' detected target defense - CrowdStrike!", asset.asset_name);
            }
        }
        
        EphemeralAssetType::DefenseProbe => {
            // Probe target defense capabilities
            if fastrand::f32() < 0.25 {
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::ServiceProbe,
                    source_info: "TARGET-FIREWALL".to_string(),
                    payload: "Connection filtered by Palo Alto Networks".to_string(),
                    signature_match: Some("Palo Alto").to_string(),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.behavioral_patterns.push("Aggressive filtering".to_string());
                asset.interaction_count += 1;
                
                info!("🎯 Defense probe '{}' identified target firewall - Palo Alto!", asset.asset_name);
            }
        }
        
        EphemeralAssetType::CounterfingerTrap => {
            // Fingerprint target's defensive tools
            if fastrand::f32() < 0.35 {
                let interaction = FingerprintInteraction {
                    interaction_id: Uuid::new_v4(),
                    timestamp: current_time,
                    interaction_type: InteractionType::VulnScan,
                    source_info: "TARGET-SCANNER".to_string(),
                    payload: "Rapid7 Nexpose scan detected".to_string(),
                    signature_match: Some("Nexpose").to_string(),
                };
                
                asset.fingerprint_data.interactions.push(interaction);
                asset.fingerprint_data.tool_signatures.push("Rapid7 Nexpose".to_string());
                asset.interaction_count += 1;
                
                info!("🔄 Counter-fingerprint trap '{}' caught target scanner - Nexpose!", asset.asset_name);
            }
        }
        
        _ => {
            // Generic offensive asset processing
            if fastrand::f32() < 0.12 {
                asset.interaction_count += 1;
                info!("⚔️  Offensive asset '{}' gathered target intelligence", asset.asset_name);
            }
        }
    }
}

/// Generate fingerprint report when asset expires or is recalled
fn generate_fingerprint_report(
    commands: &mut Commands,
    parent_entity: Entity,
    asset: &EphemeralAsset,
) {
    let report_summary = match asset.deployment_perspective {
        HD4Perspective::Defensive => {
            format!("Defensive fingerprinting complete. Asset '{}' collected {} adversary interactions, identified {} tool signatures.",
                asset.asset_name, asset.interaction_count, asset.fingerprint_data.tool_signatures.len())
        }
        HD4Perspective::Offensive => {
            format!("Offensive reconnaissance complete. Asset '{}' gathered {} target responses, identified {} defense signatures.",
                asset.asset_name, asset.interaction_count, asset.fingerprint_data.tool_signatures.len())
        }
    };
    
    let confidence_score = if asset.interaction_count > 0 {
        (asset.interaction_count as f32 * 0.1).min(1.0)
    } else {
        0.0
    };
    
    let mut metadata = HashMap::new();
    metadata.insert("perspective".to_string(), format!("{:?}", asset.deployment_perspective));
    metadata.insert("asset_type".to_string(), format!("{:?}", asset.asset_type));
    metadata.insert("interactions".to_string(), asset.interaction_count.to_string());
    metadata.insert("signatures_detected".to_string(), asset.fingerprint_data.tool_signatures.len().to_string());
    metadata.insert("confidence_score".to_string(), confidence_score.to_string());
    
    let report_node = SlotGraphNode::new(
        SlotNodeType::Intelligence,
        Vec3::new(0.0, 0.0, 0.0),
        metadata,
    );
    
    commands.spawn((
        FingerprintReport {
            report_id: Uuid::new_v4(),
            parent_asset: parent_entity,
            perspective: asset.deployment_perspective,
            summary: report_summary,
            fingerprint_data: asset.fingerprint_data.clone(),
            confidence_score,
        },
        report_node,
        Name::new(format!("FingerprintReport-{}", asset.asset_name)),
    ));
}

/// System for deploying ephemeral assets based on HD4 phase
pub fn ephemeral_deployment_system(
    mut hd4_query: Query<(Entity, &HD4Operation, &mut NodeState)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, hd4_op, mut node_state) in hd4_query.iter_mut() {
        // Deploy assets during Hunt and Detect phases
        if matches!(hd4_op.current_phase, HD4Phase::Hunt | HD4Phase::Detect) {
            
            // Random deployment chance (simulating strategic deployment)
            if fastrand::f32() < 0.1 { // 10% chance per cycle
                let asset_type = match hd4_op.perspective {
                    HD4Perspective::Defensive => select_defensive_asset_type(),
                    HD4Perspective::Offensive => select_offensive_asset_type(),
                };
                
                deploy_ephemeral_asset(&mut commands, entity, hd4_op, asset_type, time.elapsed_seconds_f64());
            }
        }
    }
}

/// Select appropriate defensive asset type
fn select_defensive_asset_type() -> EphemeralAssetType {
    let options = vec![
        EphemeralAssetType::HoneypotWebServer,
        EphemeralAssetType::FingerprintTrap,
        EphemeralAssetType::DeceptiveDatabase,
        EphemeralAssetType::CanaryFile,
    ];
    
    options[fastrand::usize(0..options.len())].clone()
}

/// Select appropriate offensive asset type
fn select_offensive_asset_type() -> EphemeralAssetType {
    let options = vec![
        EphemeralAssetType::CovertBeacon,
        EphemeralAssetType::DefenseProbe,
        EphemeralAssetType::CounterfingerTrap,
        EphemeralAssetType::ReconAsset,
    ];
    
    options[fastrand::usize(0..options.len())].clone()
}

/// Deploy ephemeral asset
fn deploy_ephemeral_asset(
    commands: &mut Commands,
    parent_entity: Entity,
    hd4_op: &HD4Operation,
    asset_type: EphemeralAssetType,
    current_time: f64,
) {
    let asset_name = match hd4_op.perspective {
        HD4Perspective::Defensive => format!("DefensiveTrap-{:?}-{}", asset_type, Uuid::new_v4()),
        HD4Perspective::Offensive => format!("OffensiveProbe-{:?}-{}", asset_type, Uuid::new_v4()),
    };
    
    let bundle = EphemeralAssetBundle::new(
        asset_name.clone(),
        asset_type,
        hd4_op.perspective,
        hd4_op.target_context.primary_target.clone(),
        current_time,
    );
    
    commands.spawn((
        bundle,
        Name::new(asset_name.clone()),
    ));
    
    info!("🚀 Deployed ephemeral asset: {} ({:?} perspective)", 
        asset_name, hd4_op.perspective);
}

/// Component for fingerprint reports
#[derive(Component, Debug, Clone)]
pub struct FingerprintReport {
    pub report_id: Uuid,
    pub parent_asset: Entity,
    pub perspective: HD4Perspective,
    pub summary: String,
    pub fingerprint_data: FingerprintData,
    pub confidence_score: f32,
}

/// Bundle for creating ephemeral assets
#[derive(Bundle)]
pub struct EphemeralAssetBundle {
    pub asset: EphemeralAsset,
    pub slotgraph_node: SlotGraphNode,
    pub node_state: NodeState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl EphemeralAssetBundle {
    /// Create new ephemeral asset
    pub fn new(
        name: String,
        asset_type: EphemeralAssetType,
        perspective: HD4Perspective,
        target_signature: String,
        deployment_time: f64,
    ) -> Self {
        let asset_id = Uuid::new_v4();
        
        let mut metadata = HashMap::new();
        metadata.insert("asset_type".to_string(), format!("{:?}", asset_type));
        metadata.insert("perspective".to_string(), format!("{:?}", perspective));
        metadata.insert("target".to_string(), target_signature.clone());
        metadata.insert("deployment_time".to_string(), deployment_time.to_string());
        
        let slotgraph_node = SlotGraphNode::new(
            SlotNodeType::Asset,
            Vec3::new(0.0, 0.0, 0.0),
            metadata,
        );
        
        // Asset lifespan based on type and perspective
        let lifespan = match (&asset_type, perspective) {
            (EphemeralAssetType::HoneypotWebServer, _) => 300, // 5 minutes
            (EphemeralAssetType::CovertBeacon, _) => 180,      // 3 minutes
            (EphemeralAssetType::FingerprintTrap, _) => 600,   // 10 minutes
            _ => 240, // 4 minutes default
        };
        
        Self {
            asset: EphemeralAsset {
                asset_id,
                asset_name: name,
                asset_type,
                deployment_perspective: perspective,
                target_signature,
                lifespan_seconds: lifespan,
                deployment_time,
                fingerprint_data: FingerprintData {
                    interactions: vec![],
                    tool_signatures: vec![],
                    behavioral_patterns: vec![],
                    timing_patterns: vec![],
                    source_ips: vec![],
                    user_agents: vec![],
                    attack_vectors: vec![],
                    confidence_score: 0.0,
                },
                stealth_level: fastrand::f32(),
                interaction_count: 0,
                status: AssetStatus::Deploying,
            },
            slotgraph_node,
            node_state: NodeState {
                status: NodeStatus::Ready,
                last_update: 0.0,
                health_score: 1.0,
                execution_count: 0,
                failure_count: 0,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}