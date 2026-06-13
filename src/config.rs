use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub rules: HashMap<String, RateLimitRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitRule {
    pub limit: u64,
    pub window_seconds: u64,
    pub key_type: KeyType,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum KeyType {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "jwt_sub")]
    JwtSub,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut rules = HashMap::new();
        rules.insert(
            "/api/public".to_string(),
            RateLimitRule {
                limit: 100,
                window_seconds: 60,
                key_type: KeyType::Ip,
            },
        );
        rules.insert(
            "/api/auth".to_string(),
            RateLimitRule {
                limit: 10,
                window_seconds: 60,
                key_type: KeyType::Ip,
            },
        );
        rules.insert(
            "/api/user/*".to_string(),
            RateLimitRule {
                limit: 1000,
                window_seconds: 60,
                key_type: KeyType::JwtSub,
            },
        );
        
        RateLimitConfig { rules }
    }
}
