use crate::handlers::message_handler::{echo, health_check};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new().route("/", get(health_check)).route("/echo", post(echo))
}
