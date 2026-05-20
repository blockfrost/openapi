use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentUtxo {
    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "inputs")]
    pub inputs: Vec<models::TxContentUtxoInputsInner>,
    #[serde(rename = "outputs")]
    pub outputs: Vec<models::TxContentUtxoOutputsInner>,
}

impl TxContentUtxo {
    pub fn new(hash: String, inputs: Vec<models::TxContentUtxoInputsInner>, outputs: Vec<models::TxContentUtxoOutputsInner>) -> TxContentUtxo {
        TxContentUtxo {
            hash,
            inputs,
            outputs,
        }
    }
}

