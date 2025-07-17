use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::entity::LightningNodes;

#[async_trait]
pub trait NodesRepository: Send + Sync {
    async fn get_last_nodes(&self) -> Vec<LightningNodes>;
    async fn append_nodes(&self, nodes: Vec<LightningNodes>);
}

#[derive(Default)]
pub struct InMemoryNodesRepository(pub Arc<RwLock<Vec<LightningNodes>>>);

#[async_trait]
impl NodesRepository for InMemoryNodesRepository {
    async fn get_last_nodes(&self) -> Vec<LightningNodes> {
        (*self.0.read().await).clone()
    }

    async fn append_nodes(&self, nodes: Vec<LightningNodes>) {
        let mut guard = self.0.write().await;
        *guard = nodes;
    }
}

#[async_trait]
pub trait MempoolAPIRepository: Send + Sync {
    fn get_last_nodes(&self) -> Vec<LightningNodes>;
}

#[derive(Default)]
pub struct MockMempoolAPIRepository(pub Arc<Vec<LightningNodes>>);

#[async_trait]
impl MempoolAPIRepository for MockMempoolAPIRepository {
    fn get_last_nodes(&self) -> Vec<LightningNodes> {
        (*self.0).clone()
    }
}
