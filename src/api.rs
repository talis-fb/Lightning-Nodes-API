use std::time::Duration;

use axum::Router;
use axum::extract::Request;
use axum::response::Response;
use axum::routing::get;
use tower_http::trace::TraceLayer;
use tracing::{Span, error, info, info_span};

use crate::context::AppContext;

mod endpoints;

pub async fn app_router(ctx: AppContext) -> Router {
    Router::new()
        .route("/ping", get("pong"))
        .route("/nodes", get(endpoints::get_nodes))
        .route("/update", get(endpoints::update_last_nodes))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(|res: &Response, latency: Duration, _span: &Span| {
                    match res.status().as_u16() {
                        200..=399 => {
                            info!(status = %res.status(), latency_ms = %latency.as_millis())
                        }
                        _ => error!(status = %res.status(), latency_ms = %latency.as_millis()),
                    }
                }),
        )
        .with_state(ctx)
}

pub async fn run(ctx: AppContext, port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let api_router = app_router(ctx).await;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Starting HTTP server on {}", listener.local_addr().unwrap());
    axum::serve(listener, api_router).await?;
    Ok(())
}
