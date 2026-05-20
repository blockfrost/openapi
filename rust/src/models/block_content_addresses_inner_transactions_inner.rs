use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockContentAddressesInnerTransactionsInner {
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
}

impl BlockContentAddressesInnerTransactionsInner {
    pub fn new(tx_hash: String) -> BlockContentAddressesInnerTransactionsInner {
        BlockContentAddressesInnerTransactionsInner {
            tx_hash,
        }
    }
}

