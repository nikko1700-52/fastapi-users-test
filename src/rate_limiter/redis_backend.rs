use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{warn, error};

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis connection error: {0}")]
    ConnectionError(String),
    #[error("Redis command error: {0}")]
    CommandError(String),
    #[error("Redis is unavailable")]
    Unavailable,
}

pub struct RedisBackend {
    client: redis::Client,
    connection_manager: Option<redis::aio::ConnectionManager>,
}

impl RedisBackend {
    pub fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).expect("Failed to create Redis client");
        RedisBackend {
            client,
            connection_manager: None,
        }
    }
    
    pub async fn get_connection(&mut self) -> Result<&mut redis::aio::ConnectionManager, RedisError> {
        if self.connection_manager.is_none() {
            let conn = self.client
                .get_tokio_connection_manager()
                .await
                .map_err(|e| RedisError::ConnectionError(e.to_string()))?;
            self.connection_manager = Some(conn);
        }
        
        Ok(self.connection_manager.as_mut().unwrap())
    }
    
    pub async fn get_token_bucket(&mut self, key: &str) -> Result<(u64, u64), RedisError> {
        let mut conn = self.get_connection().await?;
        
        let result: (u64, u64) = conn
            .hgetall(key)
            .await
            .map_err(|e| RedisError::CommandError(e.to_string()))?;
        
        Ok(result)
    }
    
    pub async fn set_token_bucket(&mut self, key: &str, tokens: u64, last_refill: u64) -> Result<(), RedisError> {
        let mut conn = self.get_connection().await?;
        
        conn.hset_multiple(key, &[("tokens", tokens), ("last_refill", last_refill)])
            .await
            .map_err(|e| RedisError::CommandError(e.to_string()))?;
        
        conn.expire(key, 60).await
            .map_err(|e| RedisError::CommandError(e.to_string()))?;
        
        Ok(())
    }
    
    pub async fn is_available(&mut self) -> bool {
        let mut conn = match self.get_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                warn!("Redis unavailable: {}", e);
                return false;
            }
        };
        
        match conn.ping().await {
            Ok(_) => true,
            Err(e) => {
                warn!("Redis ping failed: {}", e);
                false
            }
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
