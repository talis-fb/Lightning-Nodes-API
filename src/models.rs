use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightningNodes {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    #[serde(rename = "firstSeen")]
    pub first_seen: u64,
}
