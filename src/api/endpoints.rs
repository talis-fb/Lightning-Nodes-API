use axum::Json;
use axum::extract::State;

use crate::context::AppContext;
use crate::errors::ApiError;
use crate::handlers::{GetLastNodes, UpdateLastNodes};
use crate::models::{HealthResponse, LightningNodes};

#[axum::debug_handler]
pub async fn get_nodes(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<LightningNodes>>, ApiError> {
    let nodes = GetLastNodes {
        nodes_repository: ctx.nodes_repository.clone(),
    }
    .exec()
    .await?;
    Ok(Json(nodes))
}

#[axum::debug_handler]
pub async fn update_last_nodes(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<LightningNodes>>, ApiError> {
    let result = UpdateLastNodes {
        mempool_api_repository: ctx.mempool_api_repository.clone(),
        nodes_repository: ctx.nodes_repository.clone(),
    }
    .exec()
    .await?;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn health(
    State(ctx): State<AppContext>,
) -> Result<Json<HealthResponse>, ApiError> {
    let result = ctx.health_check().await;
    Ok(Json(result))
}
