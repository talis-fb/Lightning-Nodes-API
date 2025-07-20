use std::sync::Arc;

use crate::models::{LightningNodes, LightningNodesView};
use crate::repository::{MempoolAPIRepository, NodesRepository};

pub struct FetchLastNodes {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl FetchLastNodes {
    #[tracing::instrument(skip(self), name = "update_last_nodes")]
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodes>> {
        tracing::info!("Fetching last nodes");
        let nodes = self.mempool_api_repository.get_last_nodes().await?;

        tracing::info!("Appending {} Nodes", nodes.len());

        let formatted_nodes: Vec<LightningNodesView> = nodes
            .clone()
            .into_iter()
            .map(|node| node.try_into())
            .filter_map(|result| result.ok())
            .collect();

        self.nodes_repository.append_nodes(formatted_nodes).await?;

        Ok(nodes)
    }
}
