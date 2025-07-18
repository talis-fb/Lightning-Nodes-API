use std::sync::Arc;

use crate::models::LightningNodes;
use crate::repository::{MempoolAPIRepository, NodesRepository};

pub struct UpdateLastNodes {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl UpdateLastNodes {
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodes>> {
        let nodes = self.mempool_api_repository.get_last_nodes().await?;
        self.nodes_repository.append_nodes(nodes).await?;
        self.nodes_repository.get_last_nodes().await
    }
}

pub struct GetLastNodes {
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl GetLastNodes {
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodes>> {
        self.nodes_repository.get_last_nodes().await
    }
}
