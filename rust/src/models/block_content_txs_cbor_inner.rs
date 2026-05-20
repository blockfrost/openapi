use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockContentTxsCborInner {
    /// Hash of the transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// CBOR representation of the transaction data
    #[serde(rename = "cbor")]
    pub cbor: String,
}

impl BlockContentTxsCborInner {
    pub fn new(tx_hash: String, cbor: String) -> BlockContentTxsCborInner {
        BlockContentTxsCborInner {
            tx_hash,
            cbor,
        }
    }
}

