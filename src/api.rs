use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::{repository, services};

pub fn app_router() -> Router {
    let nodes_repository = repository::InMemoryNodesRepository::default();
    let mempool_api_repository = repository::MockMempoolAPIRepository::default();
    let app_service = services::AppService {
        mempool_api_repository: Arc::new(mempool_api_repository),
        nodes_repository: Arc::new(nodes_repository),
    };


    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .with_state(app_service)
}

mod endpoints {
    use axum::{extract::State, Json};
    use axum::http::StatusCode;
    use crate::{entity::LightningNodes, services::AppService};

    pub async fn get_nodes(State(service): State<AppService>) -> Result<Json<Vec<LightningNodes>>, StatusCode> {
        let nodes = service.get_last_nodes().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(nodes))
    }
}