use chrono::{DateTime, NaiveDateTime, SecondsFormat, TimeZone, Utc};
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

impl TryFrom<LightningNodes> for LightningNodesView {
    type Error = anyhow::Error;
    fn try_from(value: LightningNodes) -> Result<Self, Self::Error> {
        let first_seen = {
            let dt = match Utc.timestamp_opt(value.firstSeen as i64, 0) {
                chrono::LocalResult::None => Err(anyhow::anyhow!("Invalid date")),
                chrono::LocalResult::Ambiguous(_, _) => Err(anyhow::anyhow!("Ambiguous date")),
                chrono::LocalResult::Single(date) => Ok(date),
            }?;
            dt.to_rfc3339_opts(SecondsFormat::Secs, true)
        };

        let capacity = {
            let decimal = value.capacity % 100_000_000;
            let integer = value.capacity / 100_000_000;
            match integer {
                0 => format!("{decimal}"),
                _ => format!("{}.{:08}", integer, decimal),
            }
        };

        Ok(LightningNodesView {
            publicKey: value.publicKey,
            alias: value.alias,
            capacity,
            firstSeen: first_seen,
        })
    }
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
