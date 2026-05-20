use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountWithdrawalContentInner {
    /// Hash of the transaction containing the withdrawal
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Withdrawal amount in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
    /// Slot of the transaction containing the withdrawal
    #[serde(rename = "tx_slot")]
    pub tx_slot: i32,
    /// Block creation time in UNIX time of the transaction containing the withdrawal
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Block height of the transaction containing the withdrawal
    #[serde(rename = "block_height")]
    pub block_height: i32,
}

impl AccountWithdrawalContentInner {
    pub fn new(tx_hash: String, amount: String, tx_slot: i32, block_time: i32, block_height: i32) -> AccountWithdrawalContentInner {
        AccountWithdrawalContentInner {
            tx_hash,
            amount,
            tx_slot,
            block_time,
            block_height,
        }
    }
}

