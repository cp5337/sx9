use axum::{routing::get, Router};
use firefly_core::native_calculate;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🔥 Axum Server listening on {} using Firefly Nexus", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    let result = native_calculate(10);
    format!("Axum Server reports: Firefly calculated {}", result)
}
