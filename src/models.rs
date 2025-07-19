use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct LightningNodes {
    pub publicKey: String,
    pub alias: String,
    pub capacity: u64,
    pub firstSeen: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct LightningNodesView {
    pub publicKey: String,
    pub alias: String,
    pub capacity: String,
    pub firstSeen: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HealthStatus {
    Ok,
    Pending,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthResponse {
    pub uptime: u64,
    pub status: HealthStatus,
    pub version: String,
    pub redis_connected: bool,
}
