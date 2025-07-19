use std::sync::Arc;

use crate::models::LightningNodes;
use crate::repository::{MempoolAPIRepository, NodesRepository};

pub struct UpdateLastNodes {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl UpdateLastNodes {
    #[tracing::instrument(skip(self), name = "update_last_nodes")]
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodes>> {
        tracing::info!("Fetching last nodes");
        let nodes = self.mempool_api_repository.get_last_nodes().await?;

        tracing::info!("Appending {} Nodes", nodes.len());
        self.nodes_repository.append_nodes(nodes).await?;

        self.nodes_repository.get_last_nodes().await
    }
}

pub struct GetLastNodes {
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl GetLastNodes {
    #[tracing::instrument(skip(self), name = "get_last_nodes")]
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodes>> {
        tracing::info!("Getting last nodes");
        self.nodes_repository.get_last_nodes().await
    }
}
