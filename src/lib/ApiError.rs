use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error_code: String,
    pub message: String,
    pub meta: ErrorResponseMeta,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]

pub struct ErrorResponseMeta {
    pub request_id: String,
}



impl ApiError {
    pub fn new(code: String, message: impl Into<String>, request_id: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            meta: ErrorResponseMeta {
                request_id: request_id.into(),
            },
            message: message.into(),
            error_code: code,
        }
    }
}
