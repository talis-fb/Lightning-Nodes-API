use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use tokio::sync::RwLock;

use crate::entity::LightningNodes;
use crate::{handlers, repository};

#[derive(Clone)]
pub struct AppContext {
    pub mempool_api_repository: Arc<RwLock<dyn repository::MempoolAPIRepository>>,
    pub nodes_repository: Arc<RwLock<dyn repository::NodesRepository>>,
}

impl AppContext {
    pub fn new(
        mempool_api_repository: impl repository::MempoolAPIRepository + 'static,
        nodes_repository: impl repository::NodesRepository + 'static,
    ) -> Self {
        Self {
            mempool_api_repository: Arc::new(RwLock::new(mempool_api_repository)),
            nodes_repository: Arc::new(RwLock::new(nodes_repository)),
        }
    }
}

pub fn app_router() -> Router {
    let app_context = AppContext::new(
        repository::MockMempoolAPIRepository(Arc::new(vec![LightningNodes {
            alias: "node1".to_string(),
            public_key: "dsad".to_string(),
            capacity: 21321,
            first_seen: 21312,
        }])),
        repository::InMemoryNodesRepository(Arc::new(RwLock::new(vec![LightningNodes {
            alias: "node1".to_string(),
            public_key: "dsad".to_string(),
            capacity: 21321,
            first_seen: 21312,
        }]))),
    );

    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .with_state(app_context)
}

mod endpoints {
    use std::sync::Arc;

    use axum::Json;
    use axum::extract::State;
    use axum::http::StatusCode;

    use crate::api::AppContext;
    use crate::entity::LightningNodes;
    use crate::handlers;
    use crate::repository::{MempoolAPIRepository, NodesRepository};

    #[axum::debug_handler]
    pub async fn get_nodes(
        State(ctx): State<AppContext>,
    ) -> Result<Json<Vec<LightningNodes>>, String> {
        let nodes = &*ctx.nodes_repository.read().await; //.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        handlers::get_last_nodes(nodes)
            .await
            .map(|nodes| Json(nodes))
    }
}
