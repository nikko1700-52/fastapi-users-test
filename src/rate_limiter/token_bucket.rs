use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBucketError {
    #[error("Token bucket limit exceeded")]
    LimitExceeded,
    #[error("Invalid token bucket configuration")]
    InvalidConfiguration,
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    last_refill: u64,
    refill_rate: f64, // tokens per second
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: f64) -> Result<Self, TokenBucketError> {
        if capacity == 0 || refill_rate <= 0.0 {
            return Err(TokenBucketError::InvalidConfiguration);
        }
        
        Ok(TokenBucket {
            capacity,
            tokens: capacity,
            last_refill: current_timestamp(),
            refill_rate,
        })
    }
    
    pub fn try_consume(&mut self, tokens: u64) -> Result<(), TokenBucketError> {
        self.refill();
        
        if self.tokens >= tokens {
            self.tokens -= tokens;
            Ok(())
        } else {
            Err(TokenBucketError::LimitExceeded)
        }
    }
    
    pub fn remaining_tokens(&self) -> u64 {
        self.tokens
    }
    
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
    
    fn refill(&mut self) {
        let now = current_timestamp();
        let elapsed = now - self.last_refill;
        
        if elapsed > 0 {
            let tokens_to_add = (elapsed as f64 * self.refill_rate) as u64;
            self.tokens = std::cmp::min(self.capacity, self.tokens + tokens_to_add);
            self.last_refill = now;
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
