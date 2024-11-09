use crate::application::services::message_service::MessageService;
use crate::domain::models::error::DomainError;
use crate::domain::models::exchange::Exchange;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRequest {
    pub(crate) body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessMessageRequest {
    pub(crate) message_id: String,
    pub(crate) additional_data: Option<String>, // Optional field for additional processing data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    id: String,
    body: String,
    headers: std::collections::HashMap<String, String>,
    created_at: String,
    updated_at: String,
}

impl From<Exchange> for MessageResponse {
    fn from(exchange: Exchange) -> Self {
        MessageResponse {
            id: exchange.id.to_string(),
            body: exchange.body,
            headers: exchange.headers,
            created_at: exchange.created_at.to_rfc3339(),
            updated_at: exchange.updated_at.to_rfc3339(),
        }
    }
}

pub struct AppState {
    pub message_service: Arc<MessageService>,
}

pub async fn create_message(
    state: web::Data<AppState>,
    message: web::Json<MessageRequest>,
) -> impl Responder {
    info!("Received request to create message: {}", message.body);

    let exchange = Exchange::new(message.body.clone());

    match state.message_service.process_message(exchange).await {
        Ok(processed_exchange) => {
            info!("Successfully processed message: {}", processed_exchange.id);
            HttpResponse::Ok().json(MessageResponse::from(processed_exchange))
        }
        Err(e) => {
            info!("Error processing message: {}", e);
            HttpResponse::InternalServerError().body(format!("Error processing message: {}", e))
        }
    }
}

pub async fn process_message(
    state: web::Data<AppState>,
    req: web::Json<ProcessMessageRequest>,
) -> impl Responder {
    info!("Received request to process message ID: {}", req.message_id);

    match uuid::Uuid::parse_str(&req.message_id) {
        Ok(uuid) => {
            match state
                .message_service
                .get_and_process_message(&uuid, req.additional_data.clone())
                .await
            {
                Ok(exchange) => {
                    info!(
                        "Successfully processed message: {}",
                        exchange.as_ref().unwrap().id
                    );
                    HttpResponse::Ok().json(MessageResponse::from(exchange.unwrap()))
                }
                Err(e) => {
                    info!("Error processing message: {}", e);
                    match e {
                        DomainError::ProcessorError(msg) if msg.contains("not found") => {
                            HttpResponse::NotFound().body(msg)
                        }
                        _ => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
                    }
                }
            }
        }
        Err(_) => {
            info!("Invalid UUID format: {}", req.message_id);
            HttpResponse::BadRequest().body("Invalid UUID format")
        }
    }
}
