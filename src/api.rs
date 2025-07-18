use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::context::AppContext;
use crate::models::LightningNodes;

pub fn app_router() -> Router {
    let app_context = AppContext::new();

    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .route("/update", get(endpoints::update_last_nodes))
        .with_state(app_context)
}

mod endpoints {
    use super::*;
    use crate::handlers::{GetLastNodes, UpdateLastNodes};

    #[axum::debug_handler]
    pub async fn get_nodes(
        State(ctx): State<AppContext>,
    ) -> Result<Json<Vec<LightningNodes>>, String> {
        let nodes = GetLastNodes {
            nodes_repository: ctx.nodes_repository.clone(),
        }
        .exec()
        .await?;
        return Ok(Json(nodes));
    }

    #[axum::debug_handler]
    pub async fn update_last_nodes(
        State(ctx): State<AppContext>,
    ) -> Result<Json<Vec<LightningNodes>>, String> {
        let result = UpdateLastNodes {
            mempool_api_repository: ctx.mempool_api_repository.clone(),
            nodes_repository: ctx.nodes_repository.clone(),
        }
        .exec()
        .await?;
        return Ok(Json(result));
    }
}
