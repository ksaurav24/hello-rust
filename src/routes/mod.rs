use axum::{Router, routing::get};

use crate::{config::{Config}, handlers};

pub fn create_router(config:Config) -> Router {
    Router::new()
        .route("/", get(handlers::hello::hello))
        .route("/health", get(handlers::health::health))
        .with_state(config)
}
