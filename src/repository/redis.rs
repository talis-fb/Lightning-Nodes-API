use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::AsyncCommands;

use crate::models::LightningNodes;
use crate::repository::NodesRepository;

pub struct RedisNodesRepository {
    pub connection_pool: Pool<RedisConnectionManager>,
}

const REDIS_NODES_KEY: &str = "nodes";


// TODO: Handle when database is not connecting
// * Add timeout
// * Add retry
// * Expose a health endpoint

#[async_trait]
impl NodesRepository for RedisNodesRepository {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>> {
        let mut conn = self.connection_pool.get().await?;

        let json_nodes: Option<String> = conn.get(REDIS_NODES_KEY).await?;

        if json_nodes.is_none() {
            return Ok(vec![]);
        }

        let nodes: Vec<LightningNodes> = serde_json::from_str(&json_nodes.unwrap()).unwrap();
        Ok(nodes)
    }

    async fn append_nodes(&self, nodes: Vec<LightningNodes>) -> anyhow::Result<()> {
        let mut conn = self.connection_pool.get().await?;

        // TODO: Use a binary format instead
        let json_nodes = serde_json::to_string(&nodes)?;

        conn.set::<&str, &str, ()>(REDIS_NODES_KEY, &json_nodes)
            .await?;

        Ok(())
    }
}
