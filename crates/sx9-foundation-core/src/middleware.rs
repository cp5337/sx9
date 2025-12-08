use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn, error};

/// Logging middleware
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let headers = request.headers().clone();
    
    // Log request
    info!(
        "Request: {} {} - Headers: {:?}",
        method,
        uri,
        headers
    );
    
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    // Log response
    info!(
        "Response: {} {} - Status: {} - Duration: {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    response
}

/// Authentication middleware
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    
    // Check for API key in headers
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            if auth_value.starts_with("Bearer ") {
                let token = &auth_value[7..]; // Remove "Bearer " prefix
                
                // TODO: Implement proper JWT validation
                if validate_token(token) {
                    return next.run(request).await;
                } else {
                    warn!("Invalid token provided");
                    return Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(axum::body::Body::from("Unauthorized"))
                        .unwrap();
                }
            }
        }
    }
    
    // For now, allow requests without authentication in development
    // TODO: Implement proper authentication for production
    next.run(request).await
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Response {
    let client_ip = get_client_ip(&request);
    
    // TODO: Implement proper rate limiting with Redis or in-memory store
    // For now, just log the request
    info!("Rate limit check for IP: {}", client_ip);
    
    next.run(request).await
}

/// CORS middleware
pub async fn cors_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    // Add CORS headers
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
    
    response
}

/// Error handling middleware
pub async fn error_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().path().to_string();
    
    match next.run(request).await {
        response if response.status().is_success() => response,
        response => {
            error!(
                "Error response: {} {} - Status: {}",
                method,
                uri,
                response.status()
            );
            response
        }
    }
}

/// Health check middleware - skip logging for health checks
pub async fn health_check_middleware(
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri().path();
    
    if uri == "/health" {
        // Skip detailed logging for health checks
        next.run(request).await
    } else {
        // Apply full middleware stack for other requests
        let response = logging_middleware(request, next).await;
        response
    }
}

/// Get client IP address
fn get_client_ip(request: &Request) -> String {
    // Check for forwarded headers first
    if let Some(forwarded) = request.headers().get("X-Forwarded-For") {
        if let Ok(ip) = forwarded.to_str() {
            return ip.split(',').next().unwrap_or("unknown").trim().to_string();
        }
    }
    
    // Check for real IP header
    if let Some(real_ip) = request.headers().get("X-Real-IP") {
        if let Ok(ip) = real_ip.to_str() {
            return ip.to_string();
        }
    }
    
    // Fallback to remote address
    if let Some(addr) = request.extensions().get::<std::net::SocketAddr>() {
        return addr.ip().to_string();
    }
    
    "unknown".to_string()
}

/// Validate JWT token
fn validate_token(token: &str) -> bool {
    // TODO: Implement proper JWT validation
    // For now, just check if token is not empty
    !token.is_empty()
}

/// Metrics middleware
pub async fn metrics_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().path().to_string();
    
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    // TODO: Update metrics in shared state
    info!(
        "Metrics: {} {} - Status: {} - Duration: {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    response
}
