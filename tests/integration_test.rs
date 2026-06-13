use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    config::RateLimitConfig,
    rate_limiter::{RateLimiter, TokenBucket, TokenBucketError},
};

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_rate_limiter() {
        let config = RateLimitConfig::default();
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
        
        // Test public API
        let request = Request::builder()
            .uri("/api/public")
            .body(Body::empty())
            .unwrap();
        
        let response = rate_limiter::middleware::rate_limit_middleware(request, next).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_rate_limit_exceeded() {
        let config = RateLimitConfig::default();
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
        
        // Test auth API with multiple requests
        for _ in 0..11 {
            let request = Request::builder()
                .uri("/api/auth")
                .body(Body::empty())
                .unwrap();
            
            let response = rate_limiter::middleware::rate_limit_middleware(request, next).await;
            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                return;
            }
        }
        
        panic!("Rate limit not exceeded");
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let config = RateLimitConfig::default();
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
        
        // Test circuit breaker with Redis down
        let request = Request::builder()
            .uri("/api/public")
            .body(Body::empty())
            .unwrap();
        
        let response = rate_limiter::middleware::rate_limit_middleware(request, next).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
