use axum::{routing::get, Router};

use crate::handlers;

pub fn create_router() -> Router{
    Router::new()
        .route("/", get(handlers::hello))
}

