use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::models::LightningNodes;
use crate::repository::{MempoolAPIRepository, NodesRepository};

#[derive(Default)]
pub struct InMemoryNodesRepository(pub Arc<RwLock<Vec<LightningNodes>>>);

#[async_trait]
impl NodesRepository for InMemoryNodesRepository {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>> {
        Ok((*self.0.read().await).clone())
    }

    async fn append_nodes(&self, nodes: Vec<LightningNodes>) -> anyhow::Result<()> {
        let mut guard = self.0.write().await;
        *guard = nodes;
        Ok(())
    }
}

#[derive(Default)]
pub struct MockMempoolAPIRepository(pub Vec<LightningNodes>);

#[async_trait]
impl MempoolAPIRepository for MockMempoolAPIRepository {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>> {
        Ok(self.0.clone())
    }
}
