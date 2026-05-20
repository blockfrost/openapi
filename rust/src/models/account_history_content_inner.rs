use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountHistoryContentInner {
    /// Epoch in which the stake was active
    #[serde(rename = "active_epoch")]
    pub active_epoch: i32,
    /// Stake amount in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
    /// Bech32 ID of pool being delegated to
    #[serde(rename = "pool_id")]
    pub pool_id: String,
}

impl AccountHistoryContentInner {
    pub fn new(active_epoch: i32, amount: String, pool_id: String) -> AccountHistoryContentInner {
        AccountHistoryContentInner {
            active_epoch,
            amount,
            pool_id,
        }
    }
}

