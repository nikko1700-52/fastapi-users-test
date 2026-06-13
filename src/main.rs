mod config;
mod rate_limiter;

use axum::{
    routing::get,
    Router,
};
use config::RateLimitConfig;
use rate_limiter::{RateLimiter, RateLimitMiddleware};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("rate_limiter=debug")
        .init();

    // Load configuration
    let config = RateLimitConfig::default();
    
    // Initialize rate limiter
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
    
    // Build our application with a route
    let app = Router::new()
        .route("/api/public", get(|| async { "Public API" }))
        .route("/api/auth", get(|| async { "Auth API" }))
        .route("/api/user/:id", get(|| async { "User API" }))
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter.clone(),
            |rate_limiter: Arc<Mutex<RateLimiter>>, request, next| {
                let middleware = RateLimitMiddleware::new(config.clone(), rate_limiter);
                async move {
                    rate_limiter::middleware::rate_limit_middleware(request, next).await
                }
            },
        ));
    
    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
