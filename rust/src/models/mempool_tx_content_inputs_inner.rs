use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MempoolTxContentInputsInner {
    /// Input address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Hash of the UTXO transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// Whether the input is a collateral consumed on script validation failure
    #[serde(rename = "collateral")]
    pub collateral: bool,
    /// Whether the input is a reference transaction input
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<bool>,
}

impl MempoolTxContentInputsInner {
    pub fn new(tx_hash: String, output_index: i32, collateral: bool) -> MempoolTxContentInputsInner {
        MempoolTxContentInputsInner {
            address: None,
            tx_hash,
            output_index,
            collateral,
            reference: None,
        }
    }
}

