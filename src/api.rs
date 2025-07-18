use axum::Router;
use axum::routing::get;

use crate::context::AppContext;

mod endpoints;

pub async fn app_router(ctx: AppContext) -> Router {
    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .route("/update", get(endpoints::update_last_nodes))
        .with_state(ctx)
}

pub async fn run(ctx: AppContext, port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let api_router = app_router(ctx).await;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, api_router).await?;
    Ok(())
}
