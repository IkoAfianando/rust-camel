use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::models::{exchange::Exchange, error::DomainError};

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn save(&self, exchange: &Exchange) -> Result<Uuid, DomainError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Exchange>, DomainError>;
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError>;
}


