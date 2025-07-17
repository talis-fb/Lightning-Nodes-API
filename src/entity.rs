use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightningNodes {
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    pub first_seen: u64,
}
