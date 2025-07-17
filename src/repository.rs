use crate::entity::LightningNodes;

pub trait NodesRepository {
    fn get_last_nodes(&self) -> Vec<LightningNodes>;
    fn append_nodes(&self, nodes: Vec<LightningNodes>);
}


#[derive(Default)]
pub struct InMemoryNodesRepository(pub Vec<LightningNodes>);

impl NodesRepository for InMemoryNodesRepository {
    fn get_last_nodes(&self) -> Vec<LightningNodes> {
        self.0.clone()
    }

    fn append_nodes(&self, nodes: Vec<LightningNodes>) {
        self.0 = nodes.into_iter().collect();
    }
}


pub trait MempoolAPIRepository {
    fn get_last_nodes(&self) -> Vec<LightningNodes>;
}


#[derive(Default)]
pub struct MockMempoolAPIRepository(pub Vec<LightningNodes>);

impl MempoolAPIRepository for MockMempoolAPIRepository {
    fn get_last_nodes(&self) -> Vec<LightningNodes> {
        self.0.clone()
    }
}