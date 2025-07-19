use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::AsyncCommands;

use crate::errors::redis::NodesNotFound;
use crate::models::LightningNodesView;
use crate::repository::NodesRepository;

pub struct RedisNodesRepository {
    pub connection_pool: Pool<RedisConnectionManager>,
}

const REDIS_NODES_KEY: &str = "nodes";

#[async_trait]
impl NodesRepository for RedisNodesRepository {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodesView>> {
        let mut conn = self.connection_pool.get().await?;

        let json_nodes: Option<String> = conn.get(REDIS_NODES_KEY).await?;

        if json_nodes.is_none() {
            return Err(NodesNotFound.into());
        }

        let json_nodes = json_nodes.unwrap();

        let nodes: Vec<LightningNodesView> = serde_json::from_str(&json_nodes)?;
        Ok(nodes)
    }

    async fn append_nodes(&self, nodes: Vec<LightningNodesView>) -> anyhow::Result<()> {
        let mut conn = self.connection_pool.get().await?;

        // TODO: Use a binary format instead
        let json_nodes = serde_json::to_string(&nodes)?;

        conn.set::<&str, &str, ()>(REDIS_NODES_KEY, &json_nodes)
            .await?;

        Ok(())
    }
}
