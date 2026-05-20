use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NutlinkAddressTickerInner {
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

impl NutlinkAddressTickerInner {
    pub fn new(tx_hash: String, block_height: i32, tx_index: i32, payload: Option<serde_json::Value>) -> NutlinkAddressTickerInner {
        NutlinkAddressTickerInner {
            tx_hash,
            block_height,
            tx_index,
            payload,
        }
    }
}

