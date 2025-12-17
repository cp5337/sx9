//! Nginx Template Implementations
//! Specific server block generators for different operational contexts

use crate::environment::{XSDEnvironmentAnnotation, OperationalContext, OSILayer};
use crate::nginx_generator::{
    XSDNginxGenerator, NginxContextTemplate, OSILayerConfig, AnalysisDepth,
    LoggingProfile, RateLimitConfig, SSLConfig, ProxyBehavior, PortBlock, ConfigGenerationError
};
use std::collections::HashMap;

// Macro for easier hashmap creation
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}

impl XSDNginxGenerator {
    pub(crate) fn initialize_context_templates(&mut self) {
        // DevOps Context Template
        self.context_templates.insert(OperationalContext::DevOps, NginxContextTemplate {
            base_config: r#"
# DevOps Context - CI/CD and Development Tools
upstream jenkins_cluster {
    least_conn;
    server jenkins1:8080 max_fails=3 fail_timeout=30s;
    server jenkins2:8080 max_fails=3 fail_timeout=30s;
}

upstream gitlab_cluster {
    server gitlab:80;
}

upstream monitoring_stack {
    server grafana:3000;
    server prometheus:9090;
}
"#.to_string(),
            security_headers: hashmap![
                "X-Frame-Options".to_string() => "SAMEORIGIN".to_string(),
                "X-Content-Type-Options".to_string() => "nosniff".to_string(),
                "Strict-Transport-Security".to_string() => "max-age=31536000".to_string()
            ],
            logging_profile: LoggingProfile::Enhanced,
            rate_limiting: RateLimitConfig::Moderate,
            ssl_config: SSLConfig::Standard,
            proxy_behavior: ProxyBehavior::LoadBalancing,
        });

        // DeceptionOps Context Template
        self.context_templates.insert(OperationalContext::DeceptionOps, NginxContextTemplate {
            base_config: r#"
# DeceptionOps Context - Honeypots and Threat Hunting
# Every request is suspicious - log everything
log_format threat_intel '$remote_addr - $remote_user [$time_local] '
                        '"$request" $status $body_bytes_sent '
                        '"$http_referer" "$http_user_agent" '
                        '"$http_x_forwarded_for" "$request_body" '
                        '$request_time $upstream_response_time '
                        '"$http_authorization" "$args"';
"#.to_string(),
            security_headers: hashmap![
                "Server".to_string() => "Apache/2.4.41".to_string(),
                "X-Powered-By".to_string() => "PHP/7.4.0".to_string()
            ],
            logging_profile: LoggingProfile::Paranoid,
            rate_limiting: RateLimitConfig::Permissive,
            ssl_config: SSLConfig::Minimal,
            proxy_behavior: ProxyBehavior::Honeypot,
        });

        // LLMOps Context Template
        self.context_templates.insert(OperationalContext::LLMOps, NginxContextTemplate {
            base_config: r#"
# LLMOps Context - AI/ML Model Serving
upstream anthropic_mcp {
    server mcp-anthropic:8000;
}

upstream openai_mcp {
    server mcp-openai:8001;
}

upstream local_llm_cluster {
    least_conn;
    server ollama1:11434 weight=3;
    server ollama2:11434 weight=2;
    server ollama3:11434 weight=1;
}

# Large request bodies for model uploads
client_max_body_size 10G;
proxy_read_timeout 300s;
proxy_send_timeout 300s;
"#.to_string(),
            security_headers: hashmap![
                "X-API-Rate-Limit".to_string() => "1000".to_string(),
                "X-Model-Context-Length".to_string() => "128000".to_string()
            ],
            logging_profile: LoggingProfile::AIOptimized,
            rate_limiting: RateLimitConfig::AIModels,
            ssl_config: SSLConfig::Enhanced,
            proxy_behavior: ProxyBehavior::AILoadBalancing,
        });
    }

    pub(crate) fn initialize_osi_configs(&mut self) {
        // Layer 7 - Application Layer Configuration
        self.osi_layer_configs.insert(OSILayer::Application, OSILayerConfig {
            monitoring_capabilities: vec![
                "HTTP method analysis".to_string(),
                "User-Agent fingerprinting".to_string(),
                "Application-specific protocol parsing".to_string(),
                "API endpoint monitoring".to_string(),
                "Payload analysis".to_string(),
            ],
            analysis_depth: AnalysisDepth::ApplicationLevel,
            response_capabilities: vec![
                "Dynamic response generation".to_string(),
                "Context-aware proxying".to_string(),
                "Application-layer filtering".to_string(),
            ],
        });

        // Layer 4 - Transport Layer Configuration
        self.osi_layer_configs.insert(OSILayer::Transport, OSILayerConfig {
            monitoring_capabilities: vec![
                "TCP connection state tracking".to_string(),
                "Port utilization analysis".to_string(),
                "Connection pattern analysis".to_string(),
                "Load balancing decisions".to_string(),
            ],
            analysis_depth: AnalysisDepth::FlowLevel,
            response_capabilities: vec![
                "Connection limiting".to_string(),
                "Port-based routing".to_string(),
                "Transport-layer load balancing".to_string(),
            ],
        });

        // Layer 3 - Network Layer Configuration
        self.osi_layer_configs.insert(OSILayer::Network, OSILayerConfig {
            monitoring_capabilities: vec![
                "IP geolocation tracking".to_string(),
                "Routing path analysis".to_string(),
                "Network topology mapping".to_string(),
                "IP reputation analysis".to_string(),
            ],
            analysis_depth: AnalysisDepth::PacketLevel,
            response_capabilities: vec![
                "IP-based filtering".to_string(),
                "Geographic routing".to_string(),
                "Network-aware load balancing".to_string(),
            ],
        });
    }

    pub(crate) fn generate_deception_server_block(
        &self,
        port_block: &PortBlock,
        _annotation: &XSDEnvironmentAnnotation,
    ) -> Result<String, ConfigGenerationError> {
        Ok(format!(r#"
server {{
    listen {};
    server_name _;

    # Deception-specific headers to appear vulnerable
    add_header Server "Apache/2.4.41 (Ubuntu)";
    add_header X-Powered-By "PHP/7.4.0";

    # Log everything for threat intelligence
    access_log /var/log/nginx/deception_{}.log threat_intel;
    error_log /var/log/nginx/deception_{}_error.log warn;

    # Honeypot behavior - respond to common attack vectors
    location / {{
        # Serve convincing but fake responses
        return 200 '<html><title>Admin Panel</title><body>Please login...</body></html>';
    }}

    location /admin {{
        # Fake admin interface
        return 200 '{{"status":"ok","version":"1.0","admin":true}}';
    }}

    location /api/users {{
        # Fake API endpoint
        return 200 '{{"users":[{{"id":1,"name":"admin"}},{{"id":2,"name":"user"}}]}}';
    }}
}}
"#, port_block.base_port, port_block.id, port_block.id))
    }

    pub(crate) fn generate_llm_server_block(
        &self,
        port_block: &PortBlock,
        _annotation: &XSDEnvironmentAnnotation,
    ) -> Result<String, ConfigGenerationError> {
        Ok(format!(r#"
server {{
    listen {};
    server_name _;

    # LLM-optimized headers
    add_header X-API-Version "v1";
    add_header X-Model-Context-Length "128000";
    add_header X-Rate-Limit-Requests "1000";

    # Large request bodies for model operations
    client_max_body_size 10G;
    client_body_timeout 300s;

    # LLM-specific logging
    log_format llm_access '$remote_addr - $remote_user [$time_local] '
                          '"$request" $status $body_bytes_sent '
                          '"$http_user_agent" $request_time '
                          '"$upstream_response_time" "$http_x_api_key"';

    access_log /var/log/nginx/llm_{}.log llm_access;

    # Route to different model providers
    location /anthropic/ {{
        proxy_pass http://mcp-anthropic:8000/;
        proxy_set_header Host $host;
        proxy_read_timeout 300s;
    }}

    location /openai/ {{
        proxy_pass http://mcp-openai:8001/;
        proxy_set_header Host $host;
        proxy_read_timeout 300s;
    }}

    location /local/ {{
        proxy_pass http://local_llm_cluster/;
        proxy_set_header Host $host;
        proxy_read_timeout 600s;
    }}
}}
"#, port_block.base_port, port_block.id))
    }

    pub(crate) fn generate_devops_server_block(
        &self,
        port_block: &PortBlock,
        _annotation: &XSDEnvironmentAnnotation,
    ) -> Result<String, ConfigGenerationError> {
        Ok(format!(r#"
server {{
    listen {};
    server_name _;

    # DevOps security headers
    add_header X-Frame-Options "SAMEORIGIN";
    add_header X-Content-Type-Options "nosniff";

    access_log /var/log/nginx/devops_{}.log;

    location /jenkins/ {{
        proxy_pass http://jenkins_cluster/;
        proxy_set_header Host $host;
    }}

    location /gitlab/ {{
        proxy_pass http://gitlab_cluster/;
        proxy_set_header Host $host;
    }}

    location /monitoring/ {{
        proxy_pass http://monitoring_stack/;
        proxy_set_header Host $host;
    }}
}}
"#, port_block.base_port, port_block.id))
    }
}

