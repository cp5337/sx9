//! Monte Carlo Scenario Test Harness
//! Generates realistic tactical scenarios with live data streams for max capacity testing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloHarness {
    pub scenario_id: String,
    pub scenario_type: ScenarioType,
    pub parameters: ScenarioParameters,
    pub simulation_state: SimulationState,
    pub data_generators: Vec<DataGenerator>,
    pub live_streams: HashMap<String, LiveStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioType {
    CyberAttack { attack_vectors: Vec<String>, intensity: f64 },
    TacticalOperations { force_size: u32, terrain: String },
    MultiDomainBattle { domains: Vec<String>, complexity: f64 },
    IntelligenceGathering { sources: u32, confidence_variance: f64 },
    EmergencyResponse { incident_type: String, urgency: f64 },
    StressTest { load_multiplier: f64, duration_hours: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParameters {
    pub duration_hours: f64,
    pub agent_count: u32,
    pub message_frequency: f64, // messages per second
    pub threat_probability: f64,
    pub decision_complexity: f64,
    pub data_volume_gb: f64,
    pub concurrent_users: u32,
    pub chaos_factor: f64, // 0.0 = deterministic, 1.0 = pure chaos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub started_at: DateTime<Utc>,
    pub current_time: DateTime<Utc>,
    pub elapsed_seconds: f64,
    pub events_generated: u64,
    pub agents_active: u32,
    pub ooda_loops_running: u32,
    pub system_load: f64,
    pub threat_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGenerator {
    pub generator_id: String,
    pub generator_type: GeneratorType,
    pub output_rate: f64, // events per second
    pub data_pattern: DataPattern,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorType {
    ThreatIntelligence,
    AgentCommunication,
    TelemetryData,
    DatabaseEvents,
    UserInteractions,
    ExternalFeeds,
    SyntheticThreats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPattern {
    Steady,
    Burst { burst_duration: f64, burst_intensity: f64 },
    Seasonal { period_minutes: f64, amplitude: f64 },
    Random { min_rate: f64, max_rate: f64 },
    Correlated { correlation_strength: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStream {
    pub stream_id: String,
    pub source: String,
    pub destination: String,
    pub data_type: String,
    pub current_rate: f64,
    pub total_events: u64,
    pub last_event: DateTime<Utc>,
    pub health_status: String,
}

pub struct MonteCarloEngine {
    scenarios: RwLock<HashMap<String, MonteCarloHarness>>,
    stream_broadcaster: broadcast::Sender<SimulationEvent>,
    metrics_collector: MetricsCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEvent {
    pub event_id: String,
    pub scenario_id: String,
    pub event_type: SimulationEventType,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationEventType {
    ThreatDetected,
    AgentMessage,
    DatabaseChange,
    SystemAlert,
    UserAction,
    NetworkEvent,
    PerformanceMetric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub generator_id: String,
    pub priority: u8,
    pub correlation_id: String,
    pub confidence: f64,
    pub synthetic: bool,
}

#[derive(Debug, Clone)]
pub struct MetricsCollector {
    pub events_per_second: f64,
    pub system_throughput: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_connections: u32,
    pub error_rate: f64,
}

impl MonteCarloEngine {
    pub async fn new() -> Self {
        let (stream_broadcaster, _) = broadcast::channel(10000);

        Self {
            scenarios: RwLock::new(HashMap::new()),
            stream_broadcaster,
            metrics_collector: MetricsCollector {
                events_per_second: 0.0,
                system_throughput: 0.0,
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                active_connections: 0,
                error_rate: 0.0,
            },
        }
    }

    pub async fn create_scenario(&self, scenario_type: ScenarioType, custom_params: Option<ScenarioParameters>) -> String {
        let scenario_id = Uuid::new_v4().to_string();

        let parameters = custom_params.unwrap_or_else(|| self.generate_default_parameters(&scenario_type));

        let data_generators = self.create_data_generators(&scenario_type, &parameters).await;

        let harness = MonteCarloHarness {
            scenario_id: scenario_id.clone(),
            scenario_type,
            parameters,
            simulation_state: SimulationState {
                started_at: Utc::now(),
                current_time: Utc::now(),
                elapsed_seconds: 0.0,
                events_generated: 0,
                agents_active: 0,
                ooda_loops_running: 0,
                system_load: 0.0,
                threat_level: "GREEN".to_string(),
            },
            data_generators,
            live_streams: HashMap::new(),
        };

        self.scenarios.write().await.insert(scenario_id.clone(), harness);
        scenario_id
    }

    fn generate_default_parameters(&self, scenario_type: &ScenarioType) -> ScenarioParameters {
        match scenario_type {
            ScenarioType::CyberAttack { intensity, .. } => ScenarioParameters {
                duration_hours: 2.0,
                agent_count: 15,
                message_frequency: intensity * 50.0,
                threat_probability: 0.8,
                decision_complexity: 0.9,
                data_volume_gb: 5.0,
                concurrent_users: 100,
                chaos_factor: 0.3,
            },
            ScenarioType::TacticalOperations { force_size, .. } => ScenarioParameters {
                duration_hours: 6.0,
                agent_count: *force_size / 10,
                message_frequency: 25.0,
                threat_probability: 0.4,
                decision_complexity: 0.7,
                data_volume_gb: 10.0,
                concurrent_users: 200,
                chaos_factor: 0.2,
            },
            ScenarioType::StressTest { load_multiplier, duration_hours } => ScenarioParameters {
                duration_hours: *duration_hours,
                agent_count: (50.0 * load_multiplier) as u32,
                message_frequency: 100.0 * load_multiplier,
                threat_probability: 0.6,
                decision_complexity: 0.8,
                data_volume_gb: 20.0 * load_multiplier,
                concurrent_users: (500.0 * load_multiplier) as u32,
                chaos_factor: 0.5,
            },
            _ => ScenarioParameters {
                duration_hours: 1.0,
                agent_count: 10,
                message_frequency: 10.0,
                threat_probability: 0.3,
                decision_complexity: 0.5,
                data_volume_gb: 1.0,
                concurrent_users: 50,
                chaos_factor: 0.1,
            },
        }
    }

    async fn create_data_generators(&self, scenario_type: &ScenarioType, params: &ScenarioParameters) -> Vec<DataGenerator> {
        let mut generators = Vec::new();

        // Threat Intelligence Generator
        generators.push(DataGenerator {
            generator_id: format!("threat-intel-{}", Uuid::new_v4()),
            generator_type: GeneratorType::ThreatIntelligence,
            output_rate: params.message_frequency * 0.1,
            data_pattern: DataPattern::Burst { burst_duration: 30.0, burst_intensity: 5.0 },
            correlation_id: None,
        });

        // Agent Communication Generator
        generators.push(DataGenerator {
            generator_id: format!("agent-comm-{}", Uuid::new_v4()),
            generator_type: GeneratorType::AgentCommunication,
            output_rate: params.message_frequency * 0.6,
            data_pattern: DataPattern::Steady,
            correlation_id: None,
        });

        // High-frequency telemetry
        generators.push(DataGenerator {
            generator_id: format!("telemetry-{}", Uuid::new_v4()),
            generator_type: GeneratorType::TelemetryData,
            output_rate: params.message_frequency * 2.0,
            data_pattern: DataPattern::Seasonal { period_minutes: 5.0, amplitude: 0.3 },
            correlation_id: None,
        });

        // Synthetic threats for stress testing
        if matches!(scenario_type, ScenarioType::StressTest { .. }) {
            generators.push(DataGenerator {
                generator_id: format!("synthetic-threats-{}", Uuid::new_v4()),
                generator_type: GeneratorType::SyntheticThreats,
                output_rate: params.message_frequency * 0.3,
                data_pattern: DataPattern::Random { min_rate: 1.0, max_rate: 50.0 },
                correlation_id: None,
            });
        }

        generators
    }

    pub async fn start_scenario(&self, scenario_id: &str) -> anyhow::Result<()> {
        let mut scenarios = self.scenarios.write().await;

        if let Some(scenario) = scenarios.get_mut(scenario_id) {
            scenario.simulation_state.started_at = Utc::now();

            // Start data generation loops
            for generator in &scenario.data_generators {
                self.start_data_generator(scenario_id, generator).await?;
            }

            // Start metrics collection
            self.start_metrics_collection(scenario_id).await?;

            tracing::info!("🚀 Monte Carlo scenario started: {}", scenario_id);
        }

        Ok(())
    }

    async fn start_data_generator(&self, scenario_id: &str, generator: &DataGenerator) -> anyhow::Result<()> {
        let generator_clone = generator.clone();
        let scenario_id_clone = scenario_id.to_string();
        let broadcaster = self.stream_broadcaster.clone();

        tokio::spawn(async move {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            let mut event_counter = 0u64;

            loop {
                // Calculate current rate based on pattern
                let current_rate = calculate_rate_from_pattern(&generator_clone.data_pattern, &mut rng);

                // Generate event
                let event = SimulationEvent {
                    event_id: Uuid::new_v4().to_string(),
                    scenario_id: scenario_id_clone.clone(),
                    event_type: match generator_clone.generator_type {
                        GeneratorType::ThreatIntelligence => SimulationEventType::ThreatDetected,
                        GeneratorType::AgentCommunication => SimulationEventType::AgentMessage,
                        GeneratorType::TelemetryData => SimulationEventType::PerformanceMetric,
                        GeneratorType::DatabaseEvents => SimulationEventType::DatabaseChange,
                        GeneratorType::UserInteractions => SimulationEventType::UserAction,
                        GeneratorType::ExternalFeeds => SimulationEventType::NetworkEvent,
                        GeneratorType::SyntheticThreats => SimulationEventType::ThreatDetected,
                    },
                    timestamp: Utc::now(),
                    data: generate_event_data(&generator_clone.generator_type, event_counter, &mut rng),
                    metadata: EventMetadata {
                        generator_id: generator_clone.generator_id.clone(),
                        priority: rng.gen_range(1..=5),
                        correlation_id: generator_clone.correlation_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
                        confidence: rng.gen_range(0.5..1.0),
                        synthetic: true,
                    },
                };

                let _ = broadcaster.send(event);
                event_counter += 1;

                // Sleep based on rate
                let sleep_duration = 1.0 / current_rate;
                tokio::time::sleep(tokio::time::Duration::from_secs_f64(sleep_duration)).await;
            }
        });

        Ok(())
    }

    async fn start_metrics_collection(&self, scenario_id: &str) -> anyhow::Result<()> {
        let scenario_id_clone = scenario_id.to_string();
        let broadcaster = self.stream_broadcaster.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                let metrics_event = SimulationEvent {
                    event_id: Uuid::new_v4().to_string(),
                    scenario_id: scenario_id_clone.clone(),
                    event_type: SimulationEventType::PerformanceMetric,
                    timestamp: Utc::now(),
                    data: serde_json::json!({
                        "cpu_usage": thread_rng().gen_range(10.0..90.0),
                        "memory_usage_mb": thread_rng().gen_range(100.0..2000.0),
                        "events_per_second": thread_rng().gen_range(50.0..500.0),
                        "active_connections": thread_rng().gen_range(10..1000),
                        "system_throughput": thread_rng().gen_range(1.0..100.0)
                    }),
                    metadata: EventMetadata {
                        generator_id: "metrics-collector".to_string(),
                        priority: 2,
                        correlation_id: Uuid::new_v4().to_string(),
                        confidence: 1.0,
                        synthetic: true,
                    },
                };

                let _ = broadcaster.send(metrics_event);
            }
        });

        Ok(())
    }

    pub async fn get_scenario_status(&self, scenario_id: &str) -> Option<MonteCarloHarness> {
        self.scenarios.read().await.get(scenario_id).cloned()
    }

    pub async fn get_live_metrics(&self) -> serde_json::Value {
        let scenarios = self.scenarios.read().await;
        let scenario_count = scenarios.len();
        let total_agents: u32 = scenarios.values().map(|s| s.simulation_state.agents_active).sum();

        serde_json::json!({
            "active_scenarios": scenario_count,
            "total_agents": total_agents,
            "system_metrics": {
                "events_per_second": self.metrics_collector.events_per_second,
                "system_throughput": self.metrics_collector.system_throughput,
                "memory_usage_mb": self.metrics_collector.memory_usage_mb,
                "cpu_usage_percent": self.metrics_collector.cpu_usage_percent,
                "active_connections": self.metrics_collector.active_connections,
                "error_rate": self.metrics_collector.error_rate
            },
            "timestamp": Utc::now()
        })
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<SimulationEvent> {
        self.stream_broadcaster.subscribe()
    }
}

fn calculate_rate_from_pattern(pattern: &DataPattern, rng: &mut impl Rng) -> f64 {
    match pattern {
        DataPattern::Steady => 1.0,
        DataPattern::Burst { burst_intensity, .. } => {
            if rng.gen_bool(0.1) { *burst_intensity } else { 1.0 }
        },
        DataPattern::Seasonal { amplitude, .. } => {
            1.0 + amplitude * (Utc::now().timestamp() as f64 / 60.0).sin()
        },
        DataPattern::Random { min_rate, max_rate } => {
            rng.gen_range(*min_rate..*max_rate)
        },
        DataPattern::Correlated { correlation_strength } => {
            1.0 + correlation_strength * rng.gen_range(-1.0..1.0)
        },
    }
}

fn generate_event_data(generator_type: &GeneratorType, counter: u64, rng: &mut impl Rng) -> serde_json::Value {
    match generator_type {
        GeneratorType::ThreatIntelligence => {
            let threat_types = ["malware", "phishing", "ddos", "apt", "insider"];
            let severities = ["low", "medium", "high", "critical"];
            let selected_threat = threat_types[rng.gen_range(0..5)];
            let selected_severity = severities[rng.gen_range(0..4)];
            serde_json::json!({
                "threat_id": format!("THREAT-{:06}", counter),
                "threat_type": selected_threat,
                "severity": selected_severity,
                "source_ip": format!("{}.{}.{}.{}",
                    rng.gen_range(1..255), rng.gen_range(1..255),
                    rng.gen_range(1..255), rng.gen_range(1..255)),
                "indicators": rng.gen_range(1..10),
                "confidence": rng.gen_range(0.3..1.0)
            })
        },
        GeneratorType::AgentCommunication => {
            let message_types = ["status", "request", "response", "alert"];
            let selected_type = message_types[rng.gen_range(0..4)];
            serde_json::json!({
                "agent_id": format!("AGENT-{}", rng.gen_range(1..20)),
                "message_type": selected_type,
                "content": format!("Agent message #{}", counter),
                "priority": rng.gen_range(1..5),
                "channel": format!("tactical-{}", rng.gen_range(1..5))
            })
        },
        GeneratorType::TelemetryData => {
            let metrics = ["cpu_usage", "memory_usage", "network_io", "disk_io"];
            let selected_metric = metrics[rng.gen_range(0..4)];
            serde_json::json!({
                "metric_name": selected_metric,
                "value": rng.gen_range(0.0..100.0),
                "unit": "percent",
                "timestamp": Utc::now(),
                "node_id": format!("NODE-{}", rng.gen_range(1..50))
            })
        },
        _ => serde_json::json!({
            "event_type": "generic",
            "counter": counter,
            "random_value": rng.gen_range(0..1000),
            "timestamp": Utc::now()
        })
    }
}

// REST API endpoints
pub async fn create_scenario_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let scenario_type = serde_json::from_value(request["scenario_type"].clone())
        .unwrap_or(ScenarioType::StressTest { load_multiplier: 1.0, duration_hours: 1.0 });

    let engine = MonteCarloEngine::new().await;
    let scenario_id = engine.create_scenario(scenario_type, None).await;

    axum::Json(serde_json::json!({
        "status": "created",
        "scenario_id": scenario_id,
        "endpoints": {
            "start": format!("/monte-carlo/{}/start", scenario_id),
            "status": format!("/monte-carlo/{}/status", scenario_id),
            "metrics": "/monte-carlo/metrics"
        }
    }))
}

pub async fn start_scenario_endpoint(
    axum::extract::Path(scenario_id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let engine = MonteCarloEngine::new().await;

    match engine.start_scenario(&scenario_id).await {
        Ok(_) => axum::Json(serde_json::json!({
            "status": "started",
            "scenario_id": scenario_id,
            "message": "Monte Carlo simulation is now generating live data streams"
        })),
        Err(e) => axum::Json(serde_json::json!({
            "status": "error",
            "error": e.to_string()
        }))
    }
}

pub async fn get_scenario_status_endpoint(
    axum::extract::Path(scenario_id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let engine = MonteCarloEngine::new().await;

    match engine.get_scenario_status(&scenario_id).await {
        Some(scenario) => axum::Json(serde_json::to_value(scenario).unwrap()),
        None => axum::Json(serde_json::json!({
            "status": "not_found",
            "scenario_id": scenario_id
        }))
    }
}

pub async fn get_live_metrics_endpoint() -> axum::Json<serde_json::Value> {
    let engine = MonteCarloEngine::new().await;
    engine.get_live_metrics().await
}

pub async fn start_test_scenario() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "starting",
        "message": "Monte Carlo test scenario initializing",
        "scenario_types": [
            "stress_test",
            "capacity_test",
            "load_test",
            "failover_test"
        ]
    }))
}

pub async fn get_test_harness_status() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "active",
        "active_scenarios": 0,
        "total_events_generated": 0,
        "harness_capabilities": {
            "threat_intelligence": true,
            "agent_communication": true,
            "telemetry_data": true,
            "performance_metrics": true
        }
    }))
}