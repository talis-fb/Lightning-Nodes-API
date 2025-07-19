use std::sync::Arc;

use crate::models::{LightningNodes, LightningNodesView};
use crate::repository::NodesRepository;

pub struct GetLastNodes {
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl GetLastNodes {
    #[tracing::instrument(skip(self), name = "get_last_nodes")]
    pub async fn exec(self) -> anyhow::Result<Vec<LightningNodesView>> {
        tracing::info!("Getting last nodes");
        self.nodes_repository.get_last_nodes().await
    }
}
