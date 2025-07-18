use thiserror::Error;

use crate::errors::ApiError;

// TODO:
// * embed errors inside these ones to show the inner error on response and debug

#[derive(Error, Debug)]
#[error("API not available")]
pub struct ApiNotAvailableError(String);
impl From<ApiNotAvailableError> for ApiError {
    fn from(err: ApiNotAvailableError) -> Self {
        ApiError::new(502, err.to_string())
    }
}

#[derive(Error, Debug)]
#[error("Error parsing API response")]
pub struct ApiResponseParsingError;
impl From<ApiResponseParsingError> for ApiError {
    fn from(err: ApiResponseParsingError) -> Self {
        ApiError::new(502, err.to_string())
    }
}
