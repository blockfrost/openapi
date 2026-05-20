use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDelegationContentInner {
    /// Epoch in which the delegation becomes active
    #[serde(rename = "active_epoch")]
    pub active_epoch: i32,
    /// Hash of the transaction containing the delegation
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Rewards for given epoch in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
    /// Bech32 ID of pool being delegated to
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Slot of the transaction containing the delegation
    #[serde(rename = "tx_slot")]
    pub tx_slot: i32,
    /// Block creation time in UNIX time of the transaction containing the delegation
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Block height of the transaction containing the delegation
    #[serde(rename = "block_height")]
    pub block_height: i32,
}

impl AccountDelegationContentInner {
    pub fn new(active_epoch: i32, tx_hash: String, amount: String, pool_id: String, tx_slot: i32, block_time: i32, block_height: i32) -> AccountDelegationContentInner {
        AccountDelegationContentInner {
            active_epoch,
            tx_hash,
            amount,
            pool_id,
            tx_slot,
            block_time,
            block_height,
        }
    }
}

