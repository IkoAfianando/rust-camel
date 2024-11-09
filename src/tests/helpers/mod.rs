use std::sync::Arc;
use actix_web::{test, web, App};
use crate::{
    application::{
        pipeline::ProcessorPipeline,
        processors::logging::LoggingProcessor,
        services::message_service::MessageService,
    },
    infrastructure::repositories::message_repository::InMemoryMessageRepository,
    interfaces::api::rest::{AppState, create_message, process_message},
    interfaces::api::health::health_check,
};
use crate::application::processors::enricher::EnricherProcessor;

// Change return type to Service
pub async fn setup_test_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    // Create test dependencies
    let repository = Arc::new(InMemoryMessageRepository::new());

    // Create processors with proper configuration
    let logging_processor = Arc::new(LoggingProcessor::new("TEST".to_string()));
    let enricher_processor = Arc::new(EnricherProcessor::new());  // This will set processed_by

    let mut pipeline = ProcessorPipeline::new();
    pipeline.add_processor(logging_processor);
    pipeline.add_processor(enricher_processor);  // Make sure enricher is in the pipeline
    let pipeline = Arc::new(pipeline);

    let message_service = Arc::new(MessageService::new(repository, pipeline));
    let state = web::Data::new(AppState {
        message_service: message_service.clone(),
    });

    // Create test app
    test::init_service(
        App::new()
            .app_data(state)
            .service(
                web::scope("/api")
                    .route("/messages", web::post().to(create_message))
                    .route("/messages/process", web::post().to(process_message))
            )
            .route("/health", web::get().to(health_check))
    ).await
}