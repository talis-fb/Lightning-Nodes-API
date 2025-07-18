use std::sync::Arc;

use bb8_redis::RedisConnectionManager;

use crate::repository::mempool::MempoolAPIRepositoryImpl;
use crate::repository::redis::RedisNodesRepository;
use crate::repository::{MempoolAPIRepository, NodesRepository};

#[derive(Clone)]
pub struct AppContext {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl AppContext {
    pub async fn new() -> Self {
        // TODO: setup envs
        let manager = RedisConnectionManager::new("redis://localhost:6379").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        let nodes_repository = RedisNodesRepository {
            connection_pool: pool,
        };

        Self {
            mempool_api_repository: Arc::new(MempoolAPIRepositoryImpl),
            nodes_repository: Arc::new(nodes_repository),
        }
    }
}
