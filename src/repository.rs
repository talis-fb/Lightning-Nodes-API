use async_trait::async_trait;

use crate::models::LightningNodes;

pub mod in_memory;
pub mod mempool;
pub mod redis;

#[async_trait]
pub trait NodesRepository: Send + Sync {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>>;
    async fn append_nodes(&self, nodes: Vec<LightningNodes>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait MempoolAPIRepository: Send + Sync {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>>;
}
