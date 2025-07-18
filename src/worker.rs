use crate::context::AppContext;
use crate::handlers::UpdateLastNodes;

pub async fn run(ctx: AppContext, seconds_interval: u64) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(seconds_interval));
    loop {
        interval.tick().await;

        // TODO:
        // Log errors
        let res = UpdateLastNodes {
            mempool_api_repository: ctx.mempool_api_repository.clone(),
            nodes_repository: ctx.nodes_repository.clone(),
        }
        .exec()
        .await;

        eprintln!("Updated nodes with: {:?} nodes", res.unwrap().len());
    }
}
