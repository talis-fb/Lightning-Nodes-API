use crate::context::AppContext;

mod api;
mod context;
mod errors;
mod handlers;
mod models;
mod repository;
mod worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_context = AppContext::new().await;

    let api_server = tokio::spawn(api::run(app_context.clone(), 3000));
    let worker = tokio::spawn(worker::run(app_context, 10));

    tokio::select! {
        v = api_server => Err(anyhow::anyhow!("API server finished: {:?}", v)),
        v = worker => Err(anyhow::anyhow!("Worker finished: {:?}", v)),
    }
}
