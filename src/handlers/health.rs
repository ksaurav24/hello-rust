use axum::{extract::State,Json};
use serde::Serialize;

use crate::config::{Config, Environment};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub environment: Environment
}

pub async fn health(
    State(config): State<Config>
) -> Json<HealthResponse> {
    println!("Health Requested");
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Server is healthy".to_string(),
        environment: config.environment,
    })
}
