use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    id: String,
    entity_type: String,
    name: String,
    hash: String,
    metadata: serde_json::Value,
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "ctas7-legion-ecs",
        "version": "7.3.1"
    }))
}

async fn list_entities() -> impl Responder {
    // Mock response
    HttpResponse::Ok().json(serde_json::json!({
        "entities": [],
        "count": 0,
        "message": "Legion ECS ready to track entities"
    }))
}

async fn create_entity(entity: web::Json<Entity>) -> impl Responder {
    log::info!("Creating entity: {} ({})", entity.name, entity.entity_type);
    HttpResponse::Ok().json(entity.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    let port = env::var("LEGION_PORT").unwrap_or_else(|_| "15177".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    log::info!("🎯 CTAS-7 Legion ECS v7.3.1 starting on {}", bind_addr);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/api/entities", web::get().to(list_entities))
            .route("/api/entities", web::post().to(create_entity))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

