use axum::Router;
use axum::routing::get;

use crate::context::AppContext;

mod endpoints;

pub async fn app_router() -> Router {
    let app_context = AppContext::new().await;

    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .route("/update", get(endpoints::update_last_nodes))
        .with_state(app_context)
}
