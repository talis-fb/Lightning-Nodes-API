use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub mod api_integration;
pub mod redis;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    code: u16,
    detail: String,
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            code: 500,
            detail: err.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = axum::response::Json(self).into_response();
        *response.status_mut() = status_code;
        response
    }
}

impl ApiError {
    pub fn new(code: u16, detail: String) -> Self {
        ApiError { code, detail }
    }
}
