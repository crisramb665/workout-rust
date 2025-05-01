use crate::models::message::Message;
use crate::services::message_service;
use axum::Json;

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    // Call the service layer to handle the business logic
    let response = message_service::process_message(payload).await;

    // Return the processed message
    Json(response)
}
