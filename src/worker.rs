use std::time::Duration;

use tokio::time::MissedTickBehavior;
use tracing::{Instrument, info_span};

use crate::context::AppContext;
use crate::env;
use crate::use_cases::fetch_last_nodes::FetchLastNodes;

pub async fn run(ctx: AppContext) -> anyhow::Result<()> {
    let seconds_interval = *env::WORKER_INTERVAL_SECONDS;

    let mut interval = tokio::time::interval(Duration::from_secs(seconds_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    tracing::info!("[Ok] Starting worker");

    loop {
        interval.tick().await;

        async {
            let response = FetchLastNodes {
                mempool_api_repository: ctx.mempool_api_repository.clone(),
                nodes_repository: ctx.nodes_repository.clone(),
            }
            .exec()
            .await;

            match response {
                Ok(nodes) => tracing::info!("Successfully fetched new {} nodes", nodes.len()),
                Err(e) => tracing::error!("Error fetching nodes: {}", e),
            }
        }
        .instrument(info_span!("worker_tick"))
        .await;
    }
}
