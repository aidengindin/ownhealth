pub mod domain;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listner");
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}


