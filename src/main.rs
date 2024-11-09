pub use rust_camel::{
    application::{
        pipeline::ProcessorPipeline,
        processors::{
            enricher::EnricherProcessor, filter::FilterProcessor, logging::LoggingProcessor,
            transform::TransformProcessor,
        },
        services::message_service::MessageService,
    },
    domain::models::exchange::Exchange,
    infrastructure::repositories::message_repository::InMemoryMessageRepository,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create repository
    let repository = Arc::new(InMemoryMessageRepository::new());

    // Create processors
    let logging_processor = Arc::new(LoggingProcessor::new("DEBUG".to_string()));
    let enricher_processor = Arc::new(EnricherProcessor::new());
    let transform_processor = Arc::new(TransformProcessor::new());
    let filter_processor = Arc::new(FilterProcessor::new());

    // Create pipeline
    let mut pipeline = ProcessorPipeline::new();
    pipeline.add_processor(logging_processor);
    pipeline.add_processor(enricher_processor);
    pipeline.add_processor(transform_processor);
    pipeline.add_processor(filter_processor);
    let pipeline = Arc::new(pipeline);

    // Create message service
    let message_service = Arc::new(MessageService::new(repository, pipeline));

    // Create channel for message processing
    let (tx, mut rx) = mpsc::channel(100);
    let service = message_service.clone();

    // Spawn processing task
    tokio::spawn(async move {
        while let Some(exchange) = rx.recv().await {
            match service.process_message(exchange).await {
                Ok(processed_exchange) => {
                    info!("Successfully processed exchange: {:?}", processed_exchange);
                }
                Err(e) => {
                    error!("Error processing exchange: {}", e);
                }
            }
        }
    });

    // Send test messages
    let exchange = Exchange::new("This is an IMPORTANT message".to_string());
    tx.send(exchange).await?;

    // Wait for processing to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}
