use std::sync::Arc;
use tokio::sync::Mutex;

pub mod token_bucket;
pub mod redis_backend;
pub mod middleware;
pub mod circuit_breaker;

pub use token_bucket::TokenBucket;
pub use redis_backend::RedisBackend;
pub use middleware::RateLimitMiddleware;
pub use circuit_breaker::CircuitBreaker;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub backend: Arc<Mutex<RedisBackend>>,
    pub circuit_breaker: Arc<Mutex<CircuitBreaker>>,
}

impl RateLimiter {
    pub fn new(redis_url: &str) -> Self {
        let backend = RedisBackend::new(redis_url);
        let circuit_breaker = CircuitBreaker::new(5, 30);
        
        RateLimiter {
            backend: Arc::new(Mutex::new(backend)),
            circuit_breaker: Arc::new(Mutex::new(circuit_breaker)),
        }
    }
}
