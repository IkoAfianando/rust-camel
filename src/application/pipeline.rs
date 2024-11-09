use crate::domain::{
    models::{error::DomainError, exchange::Exchange},
    ports::processor::Processor,
};
use std::sync::Arc;

pub struct ProcessorPipeline {
    processors: Vec<Arc<dyn Processor>>,
}

impl ProcessorPipeline {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Arc<dyn Processor>) {
        self.processors.push(processor);
    }

    pub async fn process(&self, exchange: Exchange) -> Result<Exchange, DomainError> {
        let mut current_exchange = exchange;
        for processor in &self.processors {
            current_exchange = processor.process(current_exchange).await?;
        }
        Ok(current_exchange)
    }
}
