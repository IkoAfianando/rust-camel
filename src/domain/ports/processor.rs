use async_trait::async_trait;
use crate::domain::models::{exchange::Exchange, error::DomainError};

#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&self, exchange: Exchange) -> Result<Exchange, DomainError>;
}