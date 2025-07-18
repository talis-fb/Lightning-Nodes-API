use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

pub mod api_integration;
pub mod redis;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    code: u32,
    detail: String,
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            code: 500,
            detail: err.to_string().into(),
        }
    }
}

impl IntoResponse for ApiError {
    // TODO:
    // Set the real response status code to "code" field
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(self).into_response()
    }
}

impl ApiError {
    pub fn new(code: u32, detail: String) -> Self {
        ApiError { code, detail }
    }
}
