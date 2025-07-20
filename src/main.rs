use bipa::context::AppContext;

#[cfg(all(feature = "disable_api", feature = "disable_worker"))]
compile_error!("You cannot disable both api and worker features");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bipa::logging::setup_logging();
    bipa::env::print_envs();

    let app_context = AppContext::new().await;

    let api_server = if cfg!(feature = "disable_api") {
        tokio::spawn(futures::future::pending())
    } else {
        tokio::spawn(bipa::api::run(app_context.clone()))
    };

    let worker = if cfg!(feature = "disable_worker") {
        tokio::spawn(futures::future::pending())
    } else {
        tokio::spawn(bipa::worker::run(app_context))
    };

    tokio::select! {
        v = api_server => Err(anyhow::anyhow!("API server finished: {:?}", v)),
        v = worker => Err(anyhow::anyhow!("Worker finished: {:?}", v)),
        _ = tokio::signal::ctrl_c() => Err(anyhow::anyhow!("Ctrl+C")),
    }
}
