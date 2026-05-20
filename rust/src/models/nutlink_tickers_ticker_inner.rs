use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NutlinkTickersTickerInner {
    /// Address of a metadata oracle
    #[serde(rename = "address")]
    pub address: String,
    /// Hash of the transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Block height of the record
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Transaction index within the block
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// Content of the ticker
    #[serde(rename = "payload", deserialize_with = "Option::deserialize")]
    pub payload: Option<serde_json::Value>,
}

impl NutlinkTickersTickerInner {
    pub fn new(address: String, tx_hash: String, block_height: i32, tx_index: i32, payload: Option<serde_json::Value>) -> NutlinkTickersTickerInner {
        NutlinkTickersTickerInner {
            address,
            tx_hash,
            block_height,
            tx_index,
            payload,
        }
    }
}

