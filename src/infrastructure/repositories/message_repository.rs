use crate::domain::{
    models::{error::DomainError, exchange::Exchange},
    ports::repository::MessageRepository,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryMessageRepository {
    messages: Mutex<HashMap<Uuid, Exchange>>,
}

impl InMemoryMessageRepository {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MessageRepository for InMemoryMessageRepository {
    async fn save(&self, exchange: &Exchange) -> Result<Uuid, DomainError> {
        let mut messages = self
            .messages
            .lock()
            .map_err(|e| DomainError::RepositoryError(format!("Failed to acquire lock: {}", e)))?;
        messages.insert(exchange.id, exchange.clone());
        Ok(exchange.id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Exchange>, DomainError> {
        let messages = self
            .messages
            .lock()
            .map_err(|e| DomainError::RepositoryError(format!("Failed to acquire lock: {}", e)))?;
        Ok(messages.get(&id).cloned())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        let mut messages = self
            .messages
            .lock()
            .map_err(|e| DomainError::RepositoryError(format!("Failed to acquire lock: {}", e)))?;
        messages.remove(id);
        Ok(())
    }
}
