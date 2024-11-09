use async_trait::async_trait;
use tracing::{info};
use crate::domain::{
    models::{exchange::Exchange, error::DomainError},
    ports::processor::Processor,
};

pub struct LoggingProcessor {
    prefix: String,
}

impl LoggingProcessor {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

#[async_trait]
impl Processor for LoggingProcessor {
    async fn process(&self, exchange: Exchange) -> Result<Exchange, DomainError> {
        info!{
            "{}: Processing exchange {} with body: {}",
            self.prefix, exchange.id, exchange.body
        }
        Ok(exchange)
    }
}