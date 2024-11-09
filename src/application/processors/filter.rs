use crate::domain::{
    models::{error::DomainError, exchange::Exchange},
    ports::processor::Processor,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct FilterProcessor {
    predicate: Arc<dyn Fn(&Exchange) -> bool + Send + Sync>,
}

impl FilterProcessor {
    pub fn new() -> Self {
        // Default filter accepts all messages
        Self {
            predicate: Arc::new(|_| true),
        }
    }

    pub fn with_predicate<F>(predicate: F) -> Self
    where
        F: Fn(&Exchange) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Arc::new(predicate),
        }
    }
}

#[async_trait]
impl Processor for FilterProcessor {
    async fn process(&self, exchange: Exchange) -> Result<Exchange, DomainError> {
        if (self.predicate)(&exchange) {
            Ok(exchange)
        } else {
            Err(DomainError::ProcessorError(
                "Message filtered out".to_string(),
            ))
        }
    }
}
