use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct WazuhAlert {
    id: String,
    timestamp: String,
    rule_id: String,
    rule_description: String,
    agent_name: String,
    severity: u8,
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct ProcessedAlert {
    alert_id: String,
    usim_hash: String,
    severity: u8,
    timestamp: String,
    agent: String,
    rule: String,
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "ctas7-axon",
        "version": "7.3.1"
    }))
}

async fn process_alert(alert: web::Json<WazuhAlert>) -> impl Responder {
    log::info!("Processing Wazuh alert: {}", alert.id);
    
    // Generate USIM hash from alert data
    let alert_json = serde_json::to_string(&alert.0).unwrap_or_default();
    let usim_hash = format!("USIM_{:x}", md5::compute(&alert_json));
    
    let processed = ProcessedAlert {
        alert_id: alert.id.clone(),
        usim_hash,
        severity: alert.severity,
        timestamp: alert.timestamp.clone(),
        agent: alert.agent_name.clone(),
        rule: alert.rule_description.clone(),
    };
    
    log::info!("Generated USIM: {}", processed.usim_hash);
    
    HttpResponse::Ok().json(processed)
}

async fn list_alerts() -> impl Responder {
    // Mock response for now
    HttpResponse::Ok().json(serde_json::json!({
        "alerts": [],
        "count": 0,
        "message": "AXON ready to process Wazuh alerts"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    let port = env::var("AXON_PORT").unwrap_or_else(|_| "15176".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    log::info!("🛡️  CTAS-7 AXON v7.3.1 starting on {}", bind_addr);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/api/alerts", web::get().to(list_alerts))
            .route("/api/alerts/process", web::post().to(process_alert))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

