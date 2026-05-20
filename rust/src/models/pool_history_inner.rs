use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolHistoryInner {
    /// Epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
    /// Number of blocks created by pool
    #[serde(rename = "blocks")]
    pub blocks: i32,
    /// Active (Snapshot of live stake 2 epochs ago) stake in Lovelaces
    #[serde(rename = "active_stake")]
    pub active_stake: String,
    /// Pool size (percentage) of overall active stake at that epoch
    #[serde(rename = "active_size")]
    pub active_size: f64,
    /// Number of delegators for epoch
    #[serde(rename = "delegators_count")]
    pub delegators_count: i32,
    /// Total rewards received before distribution to delegators
    #[serde(rename = "rewards")]
    pub rewards: String,
    /// Pool operator rewards
    #[serde(rename = "fees")]
    pub fees: String,
}

impl PoolHistoryInner {
    pub fn new(epoch: i32, blocks: i32, active_stake: String, active_size: f64, delegators_count: i32, rewards: String, fees: String) -> PoolHistoryInner {
        PoolHistoryInner {
            epoch,
            blocks,
            active_stake,
            active_size,
            delegators_count,
            rewards,
            fees,
        }
    }
}

