use crate::domain::{
    models::{error::DomainError, exchange::Exchange},
    ports::processor::Processor,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct TransformProcessor {
    transform_fn: Arc<dyn Fn(String) -> Result<String, DomainError> + Send + Sync>,
}

impl TransformProcessor {
    pub fn new() -> Self {
        // Default transformer just returns the original string
        Self {
            transform_fn: Arc::new(|s| Ok(s)),
        }
    }

    pub fn with_transformer<F>(transform_fn: F) -> Self
    where
        F: Fn(String) -> Result<String, DomainError> + Send + Sync + 'static,
    {
        Self {
            transform_fn: Arc::new(transform_fn),
        }
    }
}

#[async_trait]
impl Processor for TransformProcessor {
    async fn process(&self, mut exchange: Exchange) -> Result<Exchange, DomainError> {
        // Transform the message body
        let transformed_body = (self.transform_fn)(exchange.body)?;
        exchange.body = transformed_body;

        // Add transformation metadata
        exchange.set_header("transformed", "true");
        exchange.set_header("transformed_at", chrono::Utc::now().to_rfc3339().as_str());

        Ok(exchange)
    }
}