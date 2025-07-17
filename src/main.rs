
mod repository;
mod api;
mod worker;
mod entity;
mod services;

#[tokio::main]
async fn main() {

    let app = api::app_router();


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
