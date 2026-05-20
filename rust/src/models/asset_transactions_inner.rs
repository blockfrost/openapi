use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetTransactionsInner {
    /// Hash of the transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Transaction index within the block
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// Block height
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Block creation time in UNIX time
    #[serde(rename = "block_time")]
    pub block_time: i32,
}

impl AssetTransactionsInner {
    pub fn new(tx_hash: String, tx_index: i32, block_height: i32, block_time: i32) -> AssetTransactionsInner {
        AssetTransactionsInner {
            tx_hash,
            tx_index,
            block_height,
            block_time,
        }
    }
}

