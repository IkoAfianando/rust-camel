use crate::application::pipeline::ProcessorPipeline;
use crate::domain::{
    models::{error::DomainError, exchange::Exchange},
    ports::repository::MessageRepository,
};
use std::sync::Arc;

pub struct MessageService {
    repository: Arc<dyn MessageRepository>,
    pipeline: Arc<ProcessorPipeline>,
}

impl MessageService {
    pub fn new(repository: Arc<dyn MessageRepository>, pipeline: Arc<ProcessorPipeline>) -> Self {
        Self {
            repository,
            pipeline,
        }
    }

    pub async fn process_message(&self, exchange: Exchange) -> Result<Exchange, DomainError> {
        // Process the message through the pipeline
        let processed_exchange = self.pipeline.process(exchange).await?;

        // Save the processed message
        self.repository.save(&processed_exchange).await?;

        Ok(processed_exchange)
    }
}
