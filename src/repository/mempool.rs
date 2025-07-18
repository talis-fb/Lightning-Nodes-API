use std::time::UNIX_EPOCH;

use async_trait::async_trait;

use crate::models::LightningNodes;
use crate::repository::MempoolAPIRepository;

pub struct MempoolAPIRepositoryImpl;

#[async_trait]
impl MempoolAPIRepository for MempoolAPIRepositoryImpl {
    async fn get_last_nodes(&self) -> anyhow::Result<Vec<LightningNodes>> {
        let response = reqwest::Client::new()
            .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
            .send()
            .await?;

        let nodes = response.json::<Vec<LightningNodes>>().await?;
        Ok(nodes)
    }
}
