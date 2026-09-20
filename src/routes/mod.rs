use axum::{Router, routing::get};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::hello::hello))
        .route("/health", get(handlers::health::health))
}
