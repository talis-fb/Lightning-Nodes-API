// use axum::response::IntoResponse;

// impl<T> IntoResponse for anyhow::Result<T> {
//     fn into_response(self) -> axum::response::Response {
//         axum::response::Json(self).into_response()
//     }
// }

use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    code: u32,
    message: String,
    detail: Option<String>,
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            code: 500,
            message: "Internal Server Error".to_string(),
            detail: err.to_string().into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(self).into_response()
    }
}
