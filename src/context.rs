use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::LightningNodes;
use crate::repository::{
    InMemoryNodesRepository, MempoolAPIRepository, MockMempoolAPIRepository, NodesRepository
};

#[derive(Clone)]
pub struct AppContext {
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
    pub nodes_repository: Arc<dyn NodesRepository>,
}

impl AppContext {
    pub fn new() -> Self {
        let mock_mempool_api_repository = MockMempoolAPIRepository(Vec::from([LightningNodes {
            alias: "node1".to_string(),
            public_key: "dsad".to_string(),
            capacity: 21321,
            first_seen: 21312,
        }]));
        let nodes_repository = InMemoryNodesRepository::default();

        Self {
            mempool_api_repository: Arc::new(mock_mempool_api_repository),
            nodes_repository: Arc::new(nodes_repository),
        }
    }
}
