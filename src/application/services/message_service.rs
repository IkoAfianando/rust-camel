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

    pub async fn get_and_process_message(
        &self,
        id: &uuid::Uuid,
        additional_data: Option<String>,
    ) -> Result<Option<Exchange>, DomainError> {
        // Retrieve the message from the repository
        let mut exchange = match self.repository.find_by_id(id).await? {
            Some(exchange) => exchange,
            None => return Err(DomainError::ProcessorError("Message not found".to_string())),
        };

        // Add additional data if provided
        if let Some(data) = additional_data {
            exchange.set_property("additional_data", &data);
        }

        // Always process through pipeline
        self.process_message(exchange).await.map(Some)
    }
}
