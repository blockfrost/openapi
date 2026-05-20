use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochContent {
    /// Epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
    /// Unix time of the start of the epoch
    #[serde(rename = "start_time")]
    pub start_time: i32,
    /// Unix time of the end of the epoch
    #[serde(rename = "end_time")]
    pub end_time: i32,
    /// Unix time of the first block of the epoch
    #[serde(rename = "first_block_time")]
    pub first_block_time: i32,
    /// Unix time of the last block of the epoch
    #[serde(rename = "last_block_time")]
    pub last_block_time: i32,
    /// Number of blocks within the epoch
    #[serde(rename = "block_count")]
    pub block_count: i32,
    /// Number of transactions within the epoch
    #[serde(rename = "tx_count")]
    pub tx_count: i32,
    /// Sum of all the transactions within the epoch in Lovelaces
    #[serde(rename = "output")]
    pub output: String,
    /// Sum of all the fees within the epoch in Lovelaces
    #[serde(rename = "fees")]
    pub fees: String,
    /// Sum of all the active stakes within the epoch in Lovelaces
    #[serde(rename = "active_stake", deserialize_with = "Option::deserialize")]
    pub active_stake: Option<String>,
}

impl EpochContent {
    pub fn new(epoch: i32, start_time: i32, end_time: i32, first_block_time: i32, last_block_time: i32, block_count: i32, tx_count: i32, output: String, fees: String, active_stake: Option<String>) -> EpochContent {
        EpochContent {
            epoch,
            start_time,
            end_time,
            first_block_time,
            last_block_time,
            block_count,
            tx_count,
            output,
            fees,
            active_stake,
        }
    }
}

