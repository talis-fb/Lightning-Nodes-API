use crate::entity::LightningNodes;
use crate::repository::{MempoolAPIRepository, NodesRepository};

pub async fn update_last_nodes(
    mempool_api_repository: impl MempoolAPIRepository,
    nodes_repository: impl NodesRepository,
) -> Result<Vec<LightningNodes>, String> {
    let new_last_nodes = mempool_api_repository.get_last_nodes();

    nodes_repository.append_nodes(new_last_nodes);

    Ok(nodes_repository.get_last_nodes().await)
}

pub async fn get_last_nodes(
    nodes_repository: &dyn NodesRepository,
) -> Result<Vec<LightningNodes>, String> {
    Ok(nodes_repository.get_last_nodes().await)
}
