use async_trait::async_trait;
use std::collections::HashMap;
use crate::domain::{
    models::{exchange::Exchange, error::DomainError},
    ports::processor::Processor,
};

pub struct EnricherProcessor {
    metadata: HashMap<String, String>,
}

impl EnricherProcessor {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(metadata: HashMap<String, String>) -> Self {
        Self { metadata }
    }

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

#[async_trait]
impl Processor for EnricherProcessor {
    async fn process(&self, mut exchange: Exchange) -> Result<Exchange, DomainError> {
        // Add all metadata as headers
        for (key, value) in &self.metadata {
            exchange.set_header(key, value);
        }

        // Add processing metadata
        exchange.set_header("processed_by", "enricher");
        exchange.set_header("processed_at", chrono::Utc::now().to_rfc3339().as_str());

        Ok(exchange)
    }
}