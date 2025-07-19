use std::sync::Arc;

use bb8_redis::RedisConnectionManager;

use crate::env;
use crate::models::{HealthResponse, HealthStatus};
use crate::repository::mempool::MempoolAPIRepositoryImpl;
use crate::repository::redis::RedisNodesRepository;
use crate::repository::{MempoolAPIRepository, NodesRepository};

#[derive(Clone)]
pub struct AppContext {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
    pub started_at: std::time::Instant,
    redis_conn_pool: Arc<bb8::Pool<RedisConnectionManager>>,
}

impl AppContext {
    pub async fn new() -> Self {
        let redis_url = &*env::REDIS_URL;
        let manager = RedisConnectionManager::new(redis_url.as_str()).unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        let nodes_repository = RedisNodesRepository {
            connection_pool: pool.clone(),
        };

        Self {
            mempool_api_repository: Arc::new(MempoolAPIRepositoryImpl),
            nodes_repository: Arc::new(nodes_repository),
            redis_conn_pool: Arc::new(pool),
            started_at: std::time::Instant::now(),
        }
    }

    pub async fn health_check(&self) -> HealthResponse {
        let redis_connected = tokio::select! {
            _ = self.redis_conn_pool.get() => true,
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => false,
        };

        let status = if redis_connected {
            HealthStatus::Ok
        } else {
            HealthStatus::Pending
        };

        HealthResponse {
            status,
            uptime: self.started_at.elapsed().as_secs(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            redis_connected,
        }
    }
}
