use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentRequiredSignersInner {
    /// Hash of the witness
    #[serde(rename = "witness_hash")]
    pub witness_hash: String,
}

impl TxContentRequiredSignersInner {
    pub fn new(witness_hash: String) -> TxContentRequiredSignersInner {
        TxContentRequiredSignersInner {
            witness_hash,
        }
    }
}

