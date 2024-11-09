use std::collections::HashMap;
use actix_web::{web, App, HttpServer};
pub use rust_camel::{
    application::{
        pipeline::ProcessorPipeline,
        processors::{
            enricher::EnricherProcessor, filter::FilterProcessor, logging::LoggingProcessor,
            transform::TransformProcessor,
        },
        services::message_service::MessageService,
    },
    infrastructure::repositories::message_repository::InMemoryMessageRepository,
    interfaces::api::rest::{create_message, process_message, AppState},
    interfaces::api::health::{health_check},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create repository
    let repository = Arc::new(InMemoryMessageRepository::new());

    // Create processors
    let logging_processor = Arc::new(LoggingProcessor::new("DEBUG".to_string()));
    
    let mut metadata = HashMap::new();
    metadata.insert("service_name".to_string(), "rust-camel".to_string());
    let enricher_processor = Arc::new(EnricherProcessor::with_metadata(metadata));
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

    // Create app state
    let state = web::Data::new(AppState {
        message_service: message_service.clone(),
    });

    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::scope("/api")
                .route("/messages", web::post().to(create_message))
                .route("/messages/process", web::post().to(process_message)),
        )
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
