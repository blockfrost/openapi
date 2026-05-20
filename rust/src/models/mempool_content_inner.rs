use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MempoolContentInner {
    /// Hash of the transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
}

impl MempoolContentInner {
    pub fn new(tx_hash: String) -> MempoolContentInner {
        MempoolContentInner {
            tx_hash,
        }
    }
}

