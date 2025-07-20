use axum::Json;
use axum::extract::State;

use crate::context::AppContext;
use crate::errors::ApiError;
use crate::models::{HealthResponse, HealthStatus, LightningNodes, LightningNodesView};
use crate::use_cases::fetch_last_nodes::FetchLastNodes;
use crate::use_cases::get_last_nodes::GetLastNodes;

#[axum::debug_handler]
pub async fn get_nodes(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<LightningNodesView>>, ApiError> {
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
    let result = FetchLastNodes {
        mempool_api_repository: ctx.mempool_api_repository.clone(),
        nodes_repository: ctx.nodes_repository.clone(),
    }
    .exec()
    .await?;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn health(State(ctx): State<AppContext>) -> Result<Json<HealthResponse>, ApiError> {
    let result = ctx.health_check().await;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn ready(State(ctx): State<AppContext>) -> Result<Json<HealthResponse>, ApiError> {
    match ctx.health_check().await.status {
        HealthStatus::Ok => Ok(Json(ctx.health_check().await)),
        HealthStatus::Pending => Err(ApiError::new(503, "Service not ready".to_string())),
    }
}
