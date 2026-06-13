use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    reset_timeout: u64,
    current_failures: usize,
    state: CircuitBreakerState,
    last_failure_time: u64,
}

#[derive(Debug, PartialEq)]
enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: u64) -> Self {
        CircuitBreaker {
            failure_threshold,
            reset_timeout,
            current_failures: 0,
            state: CircuitBreakerState::Closed,
            last_failure_time: current_timestamp(),
        }
    }
    
    pub async fn check_operation<F, T, E>(&mut self, operation: F) -> Result<Option<T>, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        match self.state {
            CircuitBreakerState::Closed => {
                match operation.await {
                    Ok(result) => {
                        self.current_failures = 0;
                        Ok(Some(result))
                    }
                    Err(e) => {
                        self.current_failures += 1;
                        self.last_failure_time = current_timestamp();
                        
                        if self.current_failures >= self.failure_threshold {
                            self.state = CircuitBreakerState::Open;
                        }
                        
                        Err(e)
                    }
                }
            }
            CircuitBreakerState::Open => {
                if current_timestamp() - self.last_failure_time > self.reset_timeout {
                    self.state = CircuitBreakerState::HalfOpen;
                    self.check_operation(operation).await
                } else {
                    Ok(None) // Fail-open
                }
            }
            CircuitBreakerState::HalfOpen => {
                match operation.await {
                    Ok(result) => {
                        self.state = CircuitBreakerState::Closed;
                        self.current_failures = 0;
                        Ok(Some(result))
                    }
                    Err(e) => {
                        self.state = CircuitBreakerState::Open;
                        self.last_failure_time = current_timestamp();
                        Err(e)
                    }
                }
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
