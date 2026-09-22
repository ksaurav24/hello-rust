use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: T,
    pub timestamp: DateTime<Utc>,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    pub request_id: String,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub offset: u32,
    pub limit: u32,
    pub count: u32,
    pub total: Option<u64>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(
        data: T,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            success: true,
            data,
            timestamp: Utc::now(),
            meta: ResponseMeta {
                request_id: request_id.into(),
                pagination: None,
            },
        }
    }
}