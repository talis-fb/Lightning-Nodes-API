mod api;
mod context;
mod handlers;
mod models;
mod repository;

#[tokio::main]
async fn main() {
    let app = api::app_router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
