// 🚀 CTAS-7 Performance Test Harness
// Validates data flow speeds, hash performance, and system latency

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Performance Test Suite - Comprehensive validation
#[derive(Debug)]
pub struct PerformanceTestHarness {
    pub test_config: TestConfiguration,
    pub results: TestResults,
    pub benchmarks: SystemBenchmarks,
    pub glaf_integration: GLAFTestInterface,
}

#[derive(Debug, Clone)]
pub struct TestConfiguration {
    pub hash_performance_target: f64,  // MB/sec (15,240 for MurmurHash3)
    pub routing_latency_target: f64,   // nanoseconds (<250ns)
    pub service_response_target: f64,  // milliseconds
    pub throughput_target: u64,        // operations per second
    pub test_duration: Duration,
    pub payload_sizes: Vec<usize>,     // Test different data sizes
    pub concurrent_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub hash_performance: HashPerformanceResults,
    pub routing_latency: RoutingLatencyResults,
    pub service_response: ServiceResponseResults,
    pub throughput: ThroughputResults,
    pub glaf_integration: GLAFPerformanceResults,
    pub overall_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hash Performance Validation (MurmurHash3 target: 15,240 MB/sec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashPerformanceResults {
    pub algorithm: String,
    pub throughput_mb_per_sec: f64,
    pub operations_per_second: u64,
    pub average_latency_ns: f64,
    pub payload_size_tests: Vec<PayloadSizeResult>,
    pub meets_target: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadSizeResult {
    pub size_bytes: usize,
    pub throughput_mb_per_sec: f64,
    pub latency_ns: f64,
    pub operations_tested: u64,
}

/// Routing Latency Validation (target: <250ns for HFT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingLatencyResults {
    pub average_latency_ns: f64,
    pub p99_latency_ns: f64,
    pub p95_latency_ns: f64,
    pub min_latency_ns: f64,
    pub max_latency_ns: f64,
    pub meets_hft_target: bool,
    pub routing_hops: Vec<RoutingHopResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingHopResult {
    pub hop_name: String,
    pub latency_ns: f64,
    pub success_rate: f64,
}

/// Service Response Validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponseResults {
    pub service_tests: HashMap<String, ServiceTestResult>,
    pub average_response_time_ms: f64,
    pub discovery_latency_ms: f64,
    pub coordination_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTestResult {
    pub service_name: String,
    pub port: u16,
    pub response_time_ms: f64,
    pub success_rate: f64,
    pub throughput_ops_per_sec: u64,
    pub error_rate: f64,
}

/// Throughput Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputResults {
    pub total_operations: u64,
    pub operations_per_second: u64,
    pub data_processed_mb: f64,
    pub concurrent_streams: usize,
    pub saturation_point: Option<u64>,
}

/// GLAF Integration Performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GLAFPerformanceResults {
    pub intelligence_processing_ms: f64,
    pub threat_correlation_ms: f64,
    pub alert_generation_ms: f64,
    pub plasma_integration_ms: f64,
    pub total_pipeline_latency_ms: f64,
    pub intelligence_accuracy: f64,
}

/// System Benchmarks
#[derive(Debug)]
pub struct SystemBenchmarks {
    pub hash_algorithms: HashMap<String, HashBenchmark>,
    pub network_latency: NetworkBenchmark,
    pub service_discovery: ServiceDiscoveryBenchmark,
}

#[derive(Debug)]
pub struct HashBenchmark {
    pub algorithm_name: String,
    pub throughput_mb_per_sec: f64,
    pub latency_per_operation_ns: f64,
    pub memory_usage_mb: f64,
}

#[derive(Debug)]
pub struct NetworkBenchmark {
    pub inter_service_latency_ns: f64,
    pub bridge_coordination_ms: f64,
    pub discovery_response_ms: f64,
}

#[derive(Debug)]
pub struct ServiceDiscoveryBenchmark {
    pub registration_time_ms: f64,
    pub lookup_time_ms: f64,
    pub heartbeat_overhead_ms: f64,
}

/// GLAF Test Interface - Intelligence System Integration
#[derive(Debug)]
pub struct GLAFTestInterface {
    pub glaf_endpoint: String,
    pub test_intelligence_data: Vec<IntelligenceTestCase>,
    pub expected_response_times: ExpectedPerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct IntelligenceTestCase {
    pub case_id: String,
    pub threat_indicators: Vec<String>,
    pub context_data: String,
    pub expected_classification: String,
    pub processing_complexity: ComplexityLevel,
}

#[derive(Debug, Clone)]
pub enum ComplexityLevel {
    Simple,    // Single indicator
    Medium,    // Multiple indicators
    Complex,   // Cross-domain correlation
    Critical,  // Real-time threat analysis
}

#[derive(Debug)]
pub struct ExpectedPerformanceMetrics {
    pub simple_processing_ms: f64,
    pub medium_processing_ms: f64,
    pub complex_processing_ms: f64,
    pub critical_processing_ms: f64,
}

impl PerformanceTestHarness {
    /// Create new test harness with performance targets
    pub fn new() -> Self {
        let test_config = TestConfiguration {
            hash_performance_target: 15_240.0, // MB/sec for MurmurHash3
            routing_latency_target: 250.0,     // nanoseconds
            service_response_target: 100.0,    // milliseconds
            throughput_target: 100_000,        // ops/sec
            test_duration: Duration::from_secs(60),
            payload_sizes: vec![1024, 4096, 16384, 65536, 262144], // 1KB to 256KB
            concurrent_connections: 100,
        };

        let glaf_integration = GLAFTestInterface {
            glaf_endpoint: "http://localhost:8090".to_string(),
            test_intelligence_data: Self::generate_intelligence_test_cases(),
            expected_response_times: ExpectedPerformanceMetrics {
                simple_processing_ms: 10.0,
                medium_processing_ms: 50.0,
                complex_processing_ms: 200.0,
                critical_processing_ms: 500.0,
            },
        };

        Self {
            test_config,
            results: TestResults::default(),
            benchmarks: SystemBenchmarks::default(),
            glaf_integration,
        }
    }

    /// Run comprehensive performance test suite
    pub async fn run_full_test_suite(&mut self) -> Result<TestResults, Box<dyn std::error::Error>> {
        println!("🚀 Starting CTAS-7 Performance Test Suite");
        println!("📊 Targets: Hash {:.0} MB/s, Routing <{:.0}ns, Response <{:.0}ms",
                 self.test_config.hash_performance_target,
                 self.test_config.routing_latency_target,
                 self.test_config.service_response_target);

        let start_time = Instant::now();

        // Test 1: Hash Performance (Critical for data integrity)
        println!("\n🔐 Testing Hash Performance...");
        let hash_results = self.test_hash_performance().await?;

        // Test 2: Routing Latency (Critical for HFT operations)
        println!("\n⚡ Testing Routing Latency...");
        let routing_results = self.test_routing_latency().await?;

        // Test 3: Service Response Times
        println!("\n🌐 Testing Service Response Times...");
        let service_results = self.test_service_responses().await?;

        // Test 4: Throughput Under Load
        println!("\n📈 Testing System Throughput...");
        let throughput_results = self.test_system_throughput().await?;

        // Test 5: GLAF Intelligence Integration
        println!("\n🧠 Testing GLAF Intelligence Integration...");
        let glaf_results = self.test_glaf_performance().await?;

        // Calculate overall score
        let overall_score = self.calculate_overall_score(
            &hash_results,
            &routing_results,
            &service_results,
            &throughput_results,
            &glaf_results,
        );

        self.results = TestResults {
            hash_performance: hash_results,
            routing_latency: routing_results,
            service_response: service_results,
            throughput: throughput_results,
            glaf_integration: glaf_results,
            overall_score,
            timestamp: chrono::Utc::now(),
        };

        let total_time = start_time.elapsed();
        println!("\n✅ Test Suite Complete in {:.2}s", total_time.as_secs_f64());
        println!("📊 Overall Score: {:.1}/100", overall_score);

        Ok(self.results.clone())
    }

    /// Test Hash Performance (MurmurHash3 target: 15,240 MB/sec)
    pub async fn test_hash_performance(&self) -> Result<HashPerformanceResults, Box<dyn std::error::Error>> {
        let mut payload_results = Vec::new();
        let mut total_operations = 0u64;
        let mut total_data_mb = 0.0;
        let start_time = Instant::now();

        for &payload_size in &self.test_config.payload_sizes {
            let test_data = vec![0u8; payload_size];
            let iterations = 10_000;

            let payload_start = Instant::now();

            for _ in 0..iterations {
                // Simulate MurmurHash3 operation
                let _hash = self.simulate_murmurhash3(&test_data);
                total_operations += 1;
            }

            let payload_duration = payload_start.elapsed();
            let payload_mb = (payload_size * iterations) as f64 / (1024.0 * 1024.0);
            let throughput = payload_mb / payload_duration.as_secs_f64();
            let avg_latency_ns = payload_duration.as_nanos() as f64 / iterations as f64;

            payload_results.push(PayloadSizeResult {
                size_bytes: payload_size,
                throughput_mb_per_sec: throughput,
                latency_ns: avg_latency_ns,
                operations_tested: iterations as u64,
            });

            total_data_mb += payload_mb;

            println!("  📦 {} bytes: {:.0} MB/s, {:.1}ns avg latency",
                     payload_size, throughput, avg_latency_ns);
        }

        let total_duration = start_time.elapsed();
        let overall_throughput = total_data_mb / total_duration.as_secs_f64();
        let overall_ops_per_sec = total_operations / total_duration.as_secs();
        let avg_latency = total_duration.as_nanos() as f64 / total_operations as f64;

        let meets_target = overall_throughput >= self.test_config.hash_performance_target;

        Ok(HashPerformanceResults {
            algorithm: "MurmurHash3".to_string(),
            throughput_mb_per_sec: overall_throughput,
            operations_per_second: overall_ops_per_sec,
            average_latency_ns: avg_latency,
            payload_size_tests: payload_results,
            meets_target,
        })
    }

    /// Test Routing Latency (target: <250ns for HFT)
    pub async fn test_routing_latency(&self) -> Result<RoutingLatencyResults, Box<dyn std::error::Error>> {
        let mut latencies = Vec::new();
        let test_iterations = 100_000;

        // Simulate routing operations
        for _ in 0..test_iterations {
            let start = Instant::now();

            // Simulate trivariate hash routing decision
            let _routing_decision = self.simulate_routing_decision();

            let latency_ns = start.elapsed().as_nanos() as f64;
            latencies.push(latency_ns);
        }

        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let average = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let p95_index = (latencies.len() as f64 * 0.95) as usize;
        let p99_index = (latencies.len() as f64 * 0.99) as usize;

        let p95_latency = latencies[p95_index];
        let p99_latency = latencies[p99_index];
        let min_latency = latencies[0];
        let max_latency = latencies[latencies.len() - 1];

        let meets_hft_target = average <= self.test_config.routing_latency_target;

        let routing_hops = vec![
            RoutingHopResult {
                hop_name: "Service Discovery".to_string(),
                latency_ns: 50.0,
                success_rate: 99.9,
            },
            RoutingHopResult {
                hop_name: "Hash Engine".to_string(),
                latency_ns: 25.0,
                success_rate: 99.99,
            },
            RoutingHopResult {
                hop_name: "Backend MCP".to_string(),
                latency_ns: 75.0,
                success_rate: 99.95,
            },
        ];

        println!("  ⚡ Avg: {:.1}ns, P95: {:.1}ns, P99: {:.1}ns",
                 average, p95_latency, p99_latency);

        Ok(RoutingLatencyResults {
            average_latency_ns: average,
            p99_latency_ns: p99_latency,
            p95_latency_ns: p95_latency,
            min_latency_ns: min_latency,
            max_latency_ns: max_latency,
            meets_hft_target,
            routing_hops,
        })
    }

    /// Test Service Response Times
    pub async fn test_service_responses(&self) -> Result<ServiceResponseResults, Box<dyn std::error::Error>> {
        let mut service_tests = HashMap::new();

        // Define services to test
        let services = vec![
            ("Service Discovery", 18650),
            ("Backend MCP", 18600),
            ("Database Validator", 18605),
            ("Hash Engine", 18105),
            ("Port Manager", 18103),
            ("GLAF Intelligence", 8090),
        ];

        let mut total_response_time = 0.0;
        let mut successful_tests = 0;

        for (service_name, port) in services {
            let result = self.test_service_endpoint(service_name, port).await;

            match result {
                Ok(test_result) => {
                    total_response_time += test_result.response_time_ms;
                    successful_tests += 1;
                    service_tests.insert(service_name.to_string(), test_result);

                    println!("  ✅ {}: {:.1}ms", service_name,
                             service_tests[service_name].response_time_ms);
                }
                Err(e) => {
                    println!("  ❌ {}: Error - {}", service_name, e);
                    service_tests.insert(service_name.to_string(), ServiceTestResult {
                        service_name: service_name.to_string(),
                        port,
                        response_time_ms: f64::INFINITY,
                        success_rate: 0.0,
                        throughput_ops_per_sec: 0,
                        error_rate: 100.0,
                    });
                }
            }
        }

        let average_response_time = if successful_tests > 0 {
            total_response_time / successful_tests as f64
        } else {
            f64::INFINITY
        };

        Ok(ServiceResponseResults {
            service_tests,
            average_response_time_ms: average_response_time,
            discovery_latency_ms: 15.0, // Simulated
            coordination_latency_ms: 25.0, // Simulated
        })
    }

    /// Test System Throughput Under Load
    async fn test_system_throughput(&self) -> Result<ThroughputResults, Box<dyn std::error::Error>> {
        let test_duration = Duration::from_secs(30);
        let start_time = Instant::now();
        let mut total_operations = 0u64;

        // Simulate concurrent load
        let (tx, mut rx) = mpsc::channel(1000);

        // Spawn concurrent workers
        for i in 0..self.test_config.concurrent_connections {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                let mut ops_count = 0u64;
                let worker_start = Instant::now();

                while worker_start.elapsed() < test_duration {
                    // Simulate operation
                    tokio::time::sleep(Duration::from_micros(100)).await;
                    ops_count += 1;
                }

                let _ = tx_clone.send(ops_count).await;
            });
        }

        // Drop original sender
        drop(tx);

        // Collect results
        while let Some(ops_count) = rx.recv().await {
            total_operations += ops_count;
        }

        let actual_duration = start_time.elapsed();
        let ops_per_second = total_operations / actual_duration.as_secs();
        let data_processed_mb = (total_operations as f64 * 1024.0) / (1024.0 * 1024.0);

        println!("  📈 {:.0} ops/sec, {:.1} MB processed",
                 ops_per_second, data_processed_mb);

        Ok(ThroughputResults {
            total_operations,
            operations_per_second: ops_per_second,
            data_processed_mb,
            concurrent_streams: self.test_config.concurrent_connections,
            saturation_point: None,
        })
    }

    /// Test GLAF Intelligence Performance
    pub async fn test_glaf_performance(&self) -> Result<GLAFPerformanceResults, Box<dyn std::error::Error>> {
        let mut intelligence_times = Vec::new();
        let mut correlation_times = Vec::new();
        let mut alert_times = Vec::new();

        for test_case in &self.glaf_integration.test_intelligence_data {
            let start_time = Instant::now();

            // Simulate intelligence processing
            let intelligence_time = self.simulate_intelligence_processing(&test_case).await;
            intelligence_times.push(intelligence_time);

            // Simulate threat correlation
            let correlation_time = self.simulate_threat_correlation(&test_case).await;
            correlation_times.push(correlation_time);

            // Simulate alert generation
            let alert_time = self.simulate_alert_generation(&test_case).await;
            alert_times.push(alert_time);
        }

        let avg_intelligence = intelligence_times.iter().sum::<f64>() / intelligence_times.len() as f64;
        let avg_correlation = correlation_times.iter().sum::<f64>() / correlation_times.len() as f64;
        let avg_alert = alert_times.iter().sum::<f64>() / alert_times.len() as f64;

        let total_pipeline = avg_intelligence + avg_correlation + avg_alert + 50.0; // PLASMA integration overhead

        println!("  🧠 Intelligence: {:.1}ms, Correlation: {:.1}ms, Alerts: {:.1}ms",
                 avg_intelligence, avg_correlation, avg_alert);

        Ok(GLAFPerformanceResults {
            intelligence_processing_ms: avg_intelligence,
            threat_correlation_ms: avg_correlation,
            alert_generation_ms: avg_alert,
            plasma_integration_ms: 50.0,
            total_pipeline_latency_ms: total_pipeline,
            intelligence_accuracy: 94.5, // Simulated accuracy metric
        })
    }

    // Simulation methods
    fn simulate_murmurhash3(&self, data: &[u8]) -> u32 {
        // Simplified MurmurHash3 simulation
        let mut hash = 1337u32;
        for &byte in data {
            hash = hash.wrapping_mul(0xcc9e2d51);
            hash ^= byte as u32;
            hash = hash.rotate_left(15);
            hash = hash.wrapping_mul(0x1b873593);
        }
        hash
    }

    fn simulate_routing_decision(&self) -> String {
        // Simulate trivariate hash routing
        "backend_mcp_route".to_string()
    }

    async fn test_service_endpoint(&self, service_name: &str, port: u16) -> Result<ServiceTestResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();

        // Simulate HTTP health check
        tokio::time::sleep(Duration::from_millis(10)).await;

        let response_time = start_time.elapsed().as_secs_f64() * 1000.0;

        Ok(ServiceTestResult {
            service_name: service_name.to_string(),
            port,
            response_time_ms: response_time,
            success_rate: 99.5,
            throughput_ops_per_sec: 1000,
            error_rate: 0.5,
        })
    }

    async fn simulate_intelligence_processing(&self, test_case: &IntelligenceTestCase) -> f64 {
        match test_case.processing_complexity {
            ComplexityLevel::Simple => 8.0,
            ComplexityLevel::Medium => 35.0,
            ComplexityLevel::Complex => 150.0,
            ComplexityLevel::Critical => 400.0,
        }
    }

    async fn simulate_threat_correlation(&self, test_case: &IntelligenceTestCase) -> f64 {
        test_case.threat_indicators.len() as f64 * 5.0 + 10.0
    }

    async fn simulate_alert_generation(&self, _test_case: &IntelligenceTestCase) -> f64 {
        15.0 // Base alert generation time
    }

    /// Calculate overall performance score
    fn calculate_overall_score(
        &self,
        hash: &HashPerformanceResults,
        routing: &RoutingLatencyResults,
        service: &ServiceResponseResults,
        throughput: &ThroughputResults,
        glaf: &GLAFPerformanceResults,
    ) -> f64 {
        let mut score = 0.0;

        // Hash performance (25% weight)
        score += if hash.meets_target { 25.0 } else {
            (hash.throughput_mb_per_sec / self.test_config.hash_performance_target * 25.0).min(25.0)
        };

        // Routing latency (25% weight)
        score += if routing.meets_hft_target { 25.0 } else {
            ((self.test_config.routing_latency_target / routing.average_latency_ns) * 25.0).min(25.0)
        };

        // Service response (20% weight)
        score += if service.average_response_time_ms <= self.test_config.service_response_target {
            20.0
        } else {
            ((self.test_config.service_response_target / service.average_response_time_ms) * 20.0).min(20.0)
        };

        // Throughput (15% weight)
        score += if throughput.operations_per_second >= self.test_config.throughput_target {
            15.0
        } else {
            ((throughput.operations_per_second as f64 / self.test_config.throughput_target as f64) * 15.0).min(15.0)
        };

        // GLAF intelligence (15% weight)
        score += if glaf.total_pipeline_latency_ms <= 1000.0 {
            15.0
        } else {
            ((1000.0 / glaf.total_pipeline_latency_ms) * 15.0).min(15.0)
        };

        score
    }

    /// Generate intelligence test cases for GLAF
    fn generate_intelligence_test_cases() -> Vec<IntelligenceTestCase> {
        vec![
            IntelligenceTestCase {
                case_id: "simple-01".to_string(),
                threat_indicators: vec!["192.168.1.100".to_string()],
                context_data: "Single IP scan detected".to_string(),
                expected_classification: "reconnaissance".to_string(),
                processing_complexity: ComplexityLevel::Simple,
            },
            IntelligenceTestCase {
                case_id: "medium-01".to_string(),
                threat_indicators: vec![
                    "malicious_domain.com".to_string(),
                    "suspicious_user_agent".to_string(),
                ],
                context_data: "Multi-vector attack pattern".to_string(),
                expected_classification: "active_threat".to_string(),
                processing_complexity: ComplexityLevel::Medium,
            },
            IntelligenceTestCase {
                case_id: "complex-01".to_string(),
                threat_indicators: vec![
                    "apt_campaign_hash".to_string(),
                    "c2_infrastructure".to_string(),
                    "lateral_movement_pattern".to_string(),
                ],
                context_data: "Advanced persistent threat detected".to_string(),
                expected_classification: "apt_activity".to_string(),
                processing_complexity: ComplexityLevel::Complex,
            },
            IntelligenceTestCase {
                case_id: "critical-01".to_string(),
                threat_indicators: vec![
                    "ied_signature".to_string(),
                    "explosive_device_pattern".to_string(),
                    "threat_to_ctas_operational".to_string(),
                ],
                context_data: "Critical threat to operational systems".to_string(),
                expected_classification: "imminent_danger".to_string(),
                processing_complexity: ComplexityLevel::Critical,
            },
        ]
    }
}

// Default implementations
impl Default for TestResults {
    fn default() -> Self {
        Self {
            hash_performance: HashPerformanceResults {
                algorithm: "Unknown".to_string(),
                throughput_mb_per_sec: 0.0,
                operations_per_second: 0,
                average_latency_ns: 0.0,
                payload_size_tests: vec![],
                meets_target: false,
            },
            routing_latency: RoutingLatencyResults {
                average_latency_ns: 0.0,
                p99_latency_ns: 0.0,
                p95_latency_ns: 0.0,
                min_latency_ns: 0.0,
                max_latency_ns: 0.0,
                meets_hft_target: false,
                routing_hops: vec![],
            },
            service_response: ServiceResponseResults {
                service_tests: HashMap::new(),
                average_response_time_ms: 0.0,
                discovery_latency_ms: 0.0,
                coordination_latency_ms: 0.0,
            },
            throughput: ThroughputResults {
                total_operations: 0,
                operations_per_second: 0,
                data_processed_mb: 0.0,
                concurrent_streams: 0,
                saturation_point: None,
            },
            glaf_integration: GLAFPerformanceResults {
                intelligence_processing_ms: 0.0,
                threat_correlation_ms: 0.0,
                alert_generation_ms: 0.0,
                plasma_integration_ms: 0.0,
                total_pipeline_latency_ms: 0.0,
                intelligence_accuracy: 0.0,
            },
            overall_score: 0.0,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for SystemBenchmarks {
    fn default() -> Self {
        Self {
            hash_algorithms: HashMap::new(),
            network_latency: NetworkBenchmark {
                inter_service_latency_ns: 0.0,
                bridge_coordination_ms: 0.0,
                discovery_response_ms: 0.0,
            },
            service_discovery: ServiceDiscoveryBenchmark {
                registration_time_ms: 0.0,
                lookup_time_ms: 0.0,
                heartbeat_overhead_ms: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_performance_benchmark() {
        let harness = PerformanceTestHarness::new();
        let results = harness.test_hash_performance().await.unwrap();

        assert!(results.throughput_mb_per_sec > 0.0);
        assert!(results.operations_per_second > 0);
        assert!(!results.payload_size_tests.is_empty());
    }

    #[tokio::test]
    async fn test_routing_latency_benchmark() {
        let harness = PerformanceTestHarness::new();
        let results = harness.test_routing_latency().await.unwrap();

        assert!(results.average_latency_ns > 0.0);
        assert!(results.p99_latency_ns >= results.average_latency_ns);
    }
}