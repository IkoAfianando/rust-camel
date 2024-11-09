use actix_web::test;
use serde_json::Value;

use crate::tests::helpers::setup_test_app;

#[actix_rt::test]
async fn test_health_check() {
    // Arrange
    let app = setup_test_app().await;

    // Act
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp: Value = test::call_and_read_body_json(&app, req).await;

    // Assert
    assert_eq!(resp["status"], "ok");
    assert!(resp["version"].is_string());
}
