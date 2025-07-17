use crate::entity::LightningNodes;
use crate::repository::NodesRepository;
use crate::repository::MempoolAPIRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppService {
    pub nodes_repository: Arc<dyn NodesRepository>,
    pub mempool_api_repository: Arc<dyn MempoolAPIRepository>,
}

impl AppService {
    pub fn update_last_nodes(&mut self) -> Result<Vec<LightningNodes>, String> {
        let new_last_nodes = self.mempool_api_repository.get_last_nodes();
        
        self.nodes_repository.append_nodes(new_last_nodes);

        Ok(self.nodes_repository.get_last_nodes())
    }

    pub fn get_last_nodes(&self) -> Result<Vec<LightningNodes>, String> {
        Ok(self.nodes_repository.get_last_nodes())
    }

}