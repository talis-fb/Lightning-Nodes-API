use std::sync::Arc;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::models::LightningNodes;

#[async_trait]
pub trait NodesRepository: Send + Sync {
    async fn get_last_nodes(&self) -> Vec<LightningNodes>;
    async fn append_nodes(&self, nodes: Vec<LightningNodes>);
}

#[async_trait]
pub trait MempoolAPIRepository: Send + Sync {
    async fn get_last_nodes(&self) -> Vec<LightningNodes>;
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

#[derive(Default)]
pub struct MockMempoolAPIRepository(pub Vec<LightningNodes>);

#[async_trait]
impl MempoolAPIRepository for MockMempoolAPIRepository {
    async fn get_last_nodes(&self) -> Vec<LightningNodes> {
        self.0
            .iter()
            .cloned()
            .map(|mut node| {
                node.first_seen = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                node
            })
            .collect()
    }
}
