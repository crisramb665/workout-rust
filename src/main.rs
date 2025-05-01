use axum::{
    Json, Router,
    routing::{get, post},
};
// use hyper::server::Server;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn health_check() -> &'static str {
    "OK"
}

async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(health_check)).route("/echo", post(echo));
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Listening on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
