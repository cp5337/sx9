//! Plasma Agent - The only visible process on Kali Plasma
//!
//! This is the single userspace binary that:
//! 1. Verifies operator biometrics
//! 2. Loads eBPF tool programs
//! 3. Connects to CDN tunnel
//! 4. Dispatches commands to eBPF
//! 5. Reads results from ring buffers
//! 6. Filters and forwards through NATS

use anyhow::{Result, Context};
use tracing::{info, warn, error};
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use futures_util::StreamExt;
use base64::{Engine as _, engine::general_purpose};

use sx9_atlas_bus::{
    AtlasBus, PlasmaState, Polycrystal, ThyristorConfig,
    CrystalFamily, VotingPolicy,
};

mod biometric;
mod ebpf;
mod tunnel;
mod filter;
mod entropy;
mod toolchain;

use biometric::BiometricGate;
use ebpf::EbpfToolManager;
use tunnel::CdnTunnel;
use filter::ResultFilter;
use entropy::EntropyHarvester;

/// ANN Advisory from Plasma-Defender
#[derive(Debug, Clone, serde::Deserialize)]
struct AnnAdvisory {
    confidence: f32,
    recommendation: String,
    reason_trace: Vec<String>,
}

/// Send tool result to Plasma-Defender for ANN analysis
async fn send_to_plasma_defender(
    result: &ebpf::ToolResult,
    operator: &Operator,
    tunnel: &CdnTunnel,
) -> Result<Option<AnnAdvisory>> {
    use futures_util::StreamExt;
    
    // Publish result to Plasma-Defender via NATS
    let nats_client = tunnel.get_nats_client();
    let payload = serde_json::json!({
        "operator_id": hex::encode(&operator.id[..8]),
        "tool": result.tool,
        "result": general_purpose::STANDARD.encode(&result.payload),
        "success": result.success,
        "timestamp": result.timestamp,
    });
    
    nats_client.publish(
        "sx9.tool.result.ann",
        serde_json::to_vec(&payload)?.into()
    ).await?;
    
    // Subscribe to ANN advisory (with timeout)
    let mut subscriber = nats_client.subscribe("sx9.plasma.ann.advisory").await?;
    
    // Wait for advisory (timeout after 1 second)
    tokio::select! {
        msg = subscriber.next() => {
            if let Some(msg) = msg {
                let advisory: AnnAdvisory = serde_json::from_slice(&msg.payload)?;
                Ok(Some(advisory))
            } else {
                Ok(None)
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
            // Timeout - no advisory received
            Ok(None)
        }
    }
}

/// Global plasma state
static PLASMA: PlasmaState = PlasmaState::new();

/// Global tick counter
static TICK: AtomicU64 = AtomicU64::new(0);

/// Running flag
static RUNNING: AtomicBool = AtomicBool::new(true);

/// Operator configuration
#[derive(Clone)]
pub struct Operator {
    /// Operator ID (hash of biometrics)
    pub id: [u8; 32],
    /// Polycrystal configuration for this operator
    pub polycrystal: Polycrystal,
    /// Thyristor configuration
    pub thyristor_config: ThyristorConfig,
    /// Allowed tools
    pub allowed_tools: Vec<String>,
    /// CDN endpoints
    pub cdn_endpoints: Vec<String>,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Operator")
            .field("id", &hex::encode(&self.id[..8]))
            .field("allowed_tools", &self.allowed_tools)
            .field("cdn_endpoints", &self.cdn_endpoints)
            .finish()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing (minimal, to /dev/null in production)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("=== Kali Plasma Agent ===");
    info!("Initializing...");
    
    // Step 1: Biometric verification
    info!("[1/5] Verifying operator biometrics...");
    let operator = verify_operator().await
        .context("Biometric verification failed")?;
    
    info!("Operator verified: {:?}", hex::encode(&operator.id[..8]));
    
    // Step 2: Initialize plasma state
    info!("[2/5] Initializing plasma state...");
    PLASMA.prime();
    PLASMA.set_entropy(entropy::initial_entropy());
    
    // Step 3: Load eBPF tools
    info!("[3/5] Loading eBPF tools...");
    let tool_manager = EbpfToolManager::new(&operator)
        .context("Failed to load eBPF tools")?;
    
    info!("Loaded {} tools", tool_manager.tool_count());
    
    // Step 4: Connect to CDN tunnel
    info!("[4/5] Connecting to CDN tunnel...");
    let tunnel = CdnTunnel::connect(&operator).await
        .context("Failed to connect to CDN")?;
    
    info!("Connected to CDN");
    
    // Step 5: Start main loop
    info!("[5/5] Starting main loop...");
    
    // Channels for internal communication (unused for now, but ready for future use)
    let (_cmd_tx, mut _cmd_rx) = mpsc::channel::<ebpf::ToolCommand>(1024);
    let (_result_tx, mut _result_rx) = mpsc::channel::<ebpf::ToolResult>(1024);
    
    // Clone operator for spawned tasks
    let operator_for_integrity = operator.clone();
    
    // Spawn entropy harvester
    let _entropy_handle = tokio::spawn(async move {
        let harvester = EntropyHarvester::new();
        harvester.run(&PLASMA).await
    });
    
    // Spawn integrity checker
    let _integrity_handle = tokio::spawn(async move {
        integrity_loop(&operator_for_integrity).await
    });
    
    // Main loop
    while RUNNING.load(Ordering::Relaxed) {
        let tick = TICK.fetch_add(1, Ordering::Relaxed);
        
        tokio::select! {
            // Incoming command from CDN
            cmd = tunnel.recv() => {
                match cmd {
                    Ok(cmd) => {
                        // Resonate through polycrystal
                        let (passed, result) = PLASMA.resonate_poly(
                            &operator.polycrystal,
                            &cmd.payload,
                            tick,
                            &operator.thyristor_config,
                        );
                        
                        if passed {
                            info!("Command passed resonance: {:?}", cmd.tool);
                            
                            // Dispatch to eBPF
                            if let Err(e) = tool_manager.dispatch(&cmd) {
                                error!("Failed to dispatch command: {}", e);
                            }
                        } else {
                            warn!(
                                "Command rejected: ring_strength={:.3}, fired={}/{}",
                                result.ring_strength,
                                result.fired_count,
                                result.total_count
                            );
                        }
                    }
                    Err(e) => {
                        error!("Tunnel recv error: {}", e);
                    }
                }
            }
            
            // Results from eBPF ring buffer
            result = tool_manager.read_result() => {
                match result {
                    Ok(result) => {
                        // Send to Plasma-Defender for ANN analysis
                        let ann_advisory = send_to_plasma_defender(&result, &operator, &tunnel).await;
                        
                        // Filter based on ANN recommendation
                        let filtered = match ann_advisory {
                            Ok(Some(advisory)) => {
                                match advisory.recommendation.as_str() {
                                    "proceed" => {
                                        info!("ANN: proceed (confidence: {:.2})", advisory.confidence);
                                        ResultFilter::filter(&result)
                                    }
                                    "block" => {
                                        warn!("ANN: block (confidence: {:.2})", advisory.confidence);
                                        // Drop result, trip canary
                                        if let Err(e) = tunnel.send_canary("ann_blocked").await {
                                            error!("Failed to send canary: {}", e);
                                        }
                                        continue; // Skip sending result
                                    }
                                    "escalate" => {
                                        warn!("ANN: escalate (confidence: {:.2})", advisory.confidence);
                                        // Send to high-priority channel
                                        ResultFilter::filter(&result)
                                    }
                                    _ => ResultFilter::filter(&result),
                                }
                            }
                            Ok(None) => {
                                // No ANN advisory, use default filter
                                ResultFilter::filter(&result)
                            }
                            Err(e) => {
                                warn!("Plasma-Defender ANN error: {}, using default filter", e);
                                ResultFilter::filter(&result)
                            }
                        };
                        
                        // Send back through tunnel
                        if let Err(e) = tunnel.send(&filtered).await {
                            error!("Failed to send result: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Result read error: {}", e);
                    }
                }
            }
            
            // Self-destruct signals
            _ = tokio::signal::ctrl_c() => {
                warn!("Interrupt received, initiating secure shutdown...");
                RUNNING.store(false, Ordering::Relaxed);
            }
        }
    }
    
    // Secure shutdown
    info!("Secure shutdown...");
    secure_shutdown().await;
    
    Ok(())
}

/// Verify operator biometrics
async fn verify_operator() -> Result<Operator> {
    // Load biometric gate from TPM
    let gate = BiometricGate::load_from_tpm()
        .context("Failed to load biometric gate from TPM")?;
    
    // Capture biometrics
    let fingerprint = biometric::capture_fingerprint().await
        .context("Failed to capture fingerprint")?;
    
    let face = biometric::capture_face().await
        .context("Failed to capture face")?;
    
    let voice = biometric::capture_voice().await
        .context("Failed to capture voice")?;
    
    // Verify
    if !gate.verify(&fingerprint, &face, &voice) {
        // Self-destruct
        error!("Biometric verification failed!");
        self_destruct(SelfDestructReason::BiometricFail).await;
    }
    
    // Load operator config
    let operator = Operator::load(&gate)
        .context("Failed to load operator config")?;
    
    Ok(operator)
}

/// Integrity checking loop
async fn integrity_loop(operator: &Operator) {
    loop {
        // Check plasma state validity
        if !plasma_state_valid() {
            error!("Plasma state invalid!");
            self_destruct(SelfDestructReason::GateTamper).await;
        }
        
        // Check entropy level
        if PLASMA.entropy() < 1000 {
            warn!("Entropy drought detected!");
            // Don't self-destruct immediately, but increase alertness
        }
        
        // Check SDT gate
        if PLASMA.sdt_state() == sx9_atlas_bus::SdtState::Off {
            warn!("SDT gate unexpectedly off");
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Check plasma state validity
fn plasma_state_valid() -> bool {
    let snapshot = PLASMA.snapshot();
    
    // Basic sanity checks
    snapshot.delta_angle < 65535 &&
    snapshot.trigger_count < 1_000_000 // Reasonable limit
}

/// Self-destruct reasons
#[derive(Debug, Clone, Copy)]
pub enum SelfDestructReason {
    BiometricFail,
    TokenRemoved,
    CanaryTrip,
    EntropyDrought,
    GateTamper,
    ManualTrigger,
}

/// Self-destruct procedure
async fn self_destruct(reason: SelfDestructReason) -> ! {
    error!("SELF-DESTRUCT INITIATED: {:?}", reason);
    
    // 1. Stop all operations
    RUNNING.store(false, Ordering::SeqCst);
    
    // 2. Wipe BPF maps (would be implemented with aya)
    // ebpf::wipe_all_maps();
    
    // 3. Clear plasma state
    PLASMA.reset();
    PLASMA.supersede();
    
    // 4. Overwrite sensitive memory
    // In production: volatile memset of all pages
    
    // 5. Clear TPM (would require TPM library)
    // tpm::clear();
    
    // 6. Halt
    error!("Halting...");
    std::process::exit(1);
}

/// Secure shutdown (graceful)
async fn secure_shutdown() {
    // Clear sensitive state
    PLASMA.reset();
    
    // Unload eBPF programs
    // ebpf::unload_all();
    
    info!("Secure shutdown complete");
}

impl Operator {
    /// Load operator from biometric gate
    fn load(gate: &BiometricGate) -> Result<Self> {
        // In production, this would load from encrypted config
        // sealed to the operator's biometrics
        
        // Default polycrystal: tripwire for sensitive ops
        let mut polycrystal = Polycrystal::new(VotingPolicy::Any);
        polycrystal.add(sx9_atlas_bus::Crystal::new(CrystalFamily::Silent));
        polycrystal.add(sx9_atlas_bus::Crystal::new(CrystalFamily::GroundStation));
        
        Ok(Self {
            id: gate.operator_hash(),
            polycrystal,
            thyristor_config: ThyristorConfig::STRICT,
            allowed_tools: vec![
                "nmap".to_string(),
                "masscan".to_string(),
            ],
            cdn_endpoints: vec![
                "nats://cdn-edge-1.sx9.io:4222".to_string(),
            ],
        })
    }
}

