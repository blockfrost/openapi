use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolListRetireInner {
    /// Bech32 encoded pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Retirement epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
}

impl PoolListRetireInner {
    pub fn new(pool_id: String, epoch: i32) -> PoolListRetireInner {
        PoolListRetireInner {
            pool_id,
            epoch,
        }
    }
}

