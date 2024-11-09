use crate::{
    interfaces::api::rest::{MessageRequest, ProcessMessageRequest},
    tests::helpers::setup_test_app,
};
use actix_web::{http::StatusCode, test};
use serde_json::Value;
use crate::application::processors::enricher::EnricherProcessor;
use crate::domain::models::exchange::Exchange;
use crate::domain::ports::processor::Processor;

#[actix_rt::test]
async fn test_create_message() {
    // Arrange
    let app = setup_test_app().await;
    let test_message = "Test message content";

    // Act
    let req = test::TestRequest::post()
        .uri("/api/messages")
        .set_json(&MessageRequest {
            body: test_message.to_string(),
        })
        .to_request();

    let resp: Value = test::call_and_read_body_json(&app, req).await;

    // Assert
    assert!(!resp["id"].as_str().unwrap().is_empty());
    assert_eq!(resp["body"].as_str().unwrap(), test_message);
    assert!(resp["headers"].is_object());
    assert!(resp["created_at"].is_string());
}

#[actix_rt::test]
async fn test_process_message() {
    // Arrange
    let app = setup_test_app().await;

    // First create a message
    let create_req = test::TestRequest::post()
        .uri("/api/messages")
        .set_json(&MessageRequest {
            body: "Test message".to_string(),
        })
        .to_request();

    let create_resp: Value = test::call_and_read_body_json(&app, create_req).await;
    let message_id = create_resp["id"].as_str().unwrap();

    // Then process it
    let process_req = test::TestRequest::post()
        .uri("/api/messages/process")
        .set_json(&ProcessMessageRequest {
            message_id: message_id.to_string(),
            additional_data: Some("test data".to_string()),
        })
        .to_request();

    let process_resp: Value = test::call_and_read_body_json(&app, process_req).await;

    // Debug print to see what headers are actually present
    println!("Response headers: {:?}", process_resp["headers"]);

    // Assert
    assert!(process_resp["headers"].as_object().unwrap().contains_key("processed_by"),
            "Headers should contain 'processed_by'. Got headers: {:?}",
            process_resp["headers"]);
}

// Add a test to verify enricher behavior specifically
#[actix_rt::test]
async fn test_enricher_processor() {
    let enricher = EnricherProcessor::new();
    let exchange = Exchange::new("test message".to_string());

    let processed = enricher.process(exchange).await.unwrap();
    assert!(processed.headers.contains_key("processed_by"),
            "Enricher should add processed_by header");
    assert_eq!(processed.headers.get("processed_by").unwrap(), "enricher");
}

#[actix_rt::test]
async fn test_process_nonexistent_message() {
    // Arrange
    let app = setup_test_app().await;

    // Act
    let req = test::TestRequest::post()
        .uri("/api/messages/process")
        .set_json(&ProcessMessageRequest {
            message_id: uuid::Uuid::new_v4().to_string(),
            additional_data: None,
        })
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
