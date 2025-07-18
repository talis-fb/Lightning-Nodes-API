use thiserror::Error;

use crate::errors::ApiError;

#[derive(Error, Debug)]
#[error("There is no saved nodes")]
pub struct NodesNotFound;
impl From<NodesNotFound> for ApiError {
    fn from(err: NodesNotFound) -> Self {
        ApiError::new(502, err.to_string())
    }
}
