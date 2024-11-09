use actix_web::{test, web};
use crate::{
    application::{
        pipeline::ProcessorPipeline,
        processors::logging::LoggingProcessor,
        services::message_service::MessageService,
    },
    infrastructure::repositories::message_repository::InMemoryMessageRepository,
    interfaces::api::rest::{AppState, MessageRequest, ProcessMessageRequest, create_message, process_message},
    interfaces::api::health::health_check,
};
use serde_json::Value;
use std::sync::Arc;

async fn setup_integration_test_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let repository = Arc::new(InMemoryMessageRepository::new());
    let logging_processor = Arc::new(LoggingProcessor::new("TEST".to_string()));

    let mut pipeline = ProcessorPipeline::new();
    pipeline.add_processor(logging_processor);
    let pipeline = Arc::new(pipeline);

    let message_service = Arc::new(MessageService::new(repository, pipeline));
    let state = web::Data::new(AppState {
        message_service: message_service.clone(),
    });

    test::init_service(
        actix_web::App::new()
            .app_data(state)
            .service(
                web::scope("/api")
                    .route("/messages", web::post().to(create_message))
                    .route("/messages/process", web::post().to(process_message))
            )
            .route("/health", web::get().to(health_check))
    ).await
}

#[actix_rt::test]
async fn test_full_message_flow() {
    // Arrange
    let app = setup_integration_test_app().await;
    let test_message = "Integration test message";

    // Act - Create message
    let create_req = test::TestRequest::post()
        .uri("/api/messages")
        .set_json(&MessageRequest {
            body: test_message.to_string(),
        })
        .to_request();

    let create_resp: Value = test::call_and_read_body_json(&app, create_req).await;
    let message_id = create_resp["id"].as_str().unwrap();

    // Act - Process message
    let process_req = test::TestRequest::post()
        .uri("/api/messages/process")
        .set_json(&ProcessMessageRequest {
            message_id: message_id.to_string(),
            additional_data: Some("integration test data".to_string()),
        })
        .to_request();

    let process_resp: Value = test::call_and_read_body_json(&app, process_req).await;

    // Assert
    assert_eq!(process_resp["id"].as_str().unwrap(), message_id);
    assert_eq!(process_resp["body"].as_str().unwrap(), test_message);
    // assert!(process_resp["headers"].as_object().unwrap().contains_key("processed_by"));
}