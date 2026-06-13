use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    config::{RateLimitConfig, RateLimitRule, KeyType},
    rate_limiter::{RateLimiter, TokenBucket, TokenBucketError},
};

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
}

pub struct RateLimitMiddleware {
    config: RateLimitConfig,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl RateLimitMiddleware {
    pub fn new(config: RateLimitConfig, rate_limiter: Arc<Mutex<RateLimiter>>) -> Self {
        RateLimitMiddleware { config, rate_limiter }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RateLimitMiddleware
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = RateLimitConfig::default();
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
        Ok(RateLimitMiddleware::new(config, rate_limiter))
    }
}

pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let config = RateLimitConfig::default();
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new("redis://127.0.0.1:6379")));
    
    let path = request.uri().path().to_string();
    let rule = find_matching_rule(&config, &path);
    
    if let Some(rule) = rule {
        let key = generate_key(&rule, request.headers()).await;
        
        let mut rate_limiter = rate_limiter.lock().await;
        let mut backend = rate_limiter.backend.lock().await;
        
        if !backend.is_available().await {
            return Ok(next.run(request).await);
        }
        
        let result = rate_limiter
            .circuit_breaker
            .lock()
            .await
            .check_operation(async {
                let (tokens, last_refill) = backend.get_token_bucket(&key).await?;
                let mut bucket = TokenBucket::new(rule.limit, rule.limit as f64 / rule.window_seconds as f64)?;
                bucket.tokens = tokens;
                bucket.last_refill = last_refill;
                
                if bucket.try_consume(1).is_ok() {
                    backend.set_token_bucket(&key, bucket.remaining_tokens(), bucket.last_refill).await?;
                    Ok(bucket)
                } else {
                    Err(TokenBucketError::LimitExceeded)
                }
            })
            .await;
        
        match result {
            Ok(Some(bucket)) => {
                let mut response = next.run(request).await;
                add_rate_limit_headers(&mut response, rule.limit, bucket.remaining_tokens(), rule.window_seconds);
                Ok(response)
            }
            Ok(None) => Ok(next.run(request).await), // Fail-open
            Err(_) => {
                let mut response = Response::new("Rate limit exceeded".to_string());
                *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                add_rate_limit_headers(&mut response, rule.limit, 0, rule.window_seconds);
                Ok(response)
            }
        }
    } else {
        Ok(next.run(request).await)
    }
}

fn find_matching_rule(config: &RateLimitConfig, path: &str) -> Option<RateLimitRule> {
    for (pattern, rule) in &config.rules {
        if pattern == "/api/user/*" && path.starts_with("/api/user/") {
            return Some(rule.clone());
        } else if pattern == path {
            return Some(rule.clone());
        }
    }
    None
}

async fn generate_key(rule: &RateLimitRule, headers: &axum::http::HeaderMap) -> String {
    match rule.key_type {
        KeyType::Ip => {
            // In a real implementation, you would extract the IP from the request
            "ip:127.0.0.1".to_string()
        }
        KeyType::JwtSub => {
            if let Some(auth_header) = headers.get("Authorization") {
                if let Ok(token) = auth_header.to_str() {
                    if token.starts_with("Bearer ") {
                        let token = &token[7..];
                        if let Ok(claims) = decode_jwt(token).await {
                            return format!("jwt_sub:{}", claims.sub);
                        }
                    }
                }
            }
            "jwt_sub:anonymous".to_string()
        }
    }
}

async fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // In a real implementation, you would use a proper secret key
    let secret = "secret";
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

fn add_rate_limit_headers(response: &mut Response, limit: u64, remaining: u64, window_seconds: u64) {
    response.headers_mut().insert(
        "X-RateLimit-Limit",
        axum::http::HeaderValue::from_str(&limit.to_string()).unwrap(),
    );
    response.headers_mut().insert(
        "X-RateLimit-Remaining",
        axum::http::HeaderValue::from_str(&remaining.to_string()).unwrap(),
    );
    response.headers_mut().insert(
        "X-RateLimit-Reset",
        axum::http::HeaderValue::from_str(&window_seconds.to_string()).unwrap(),
    );
}
