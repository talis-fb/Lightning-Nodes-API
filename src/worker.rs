use std::time::Duration;

use tokio::time::MissedTickBehavior;

use crate::context::AppContext;
use crate::handlers::UpdateLastNodes;

pub async fn run(ctx: AppContext, seconds_interval: u64) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(seconds_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

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

        match res {
            Ok(nodes) => {
                eprintln!("Updated nodes with: {:?} nodes", nodes.len());
            }
            Err(e) => {
                eprintln!("Error updating nodes: {}", e);
            }
        }
    }
}
