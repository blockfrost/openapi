use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockContentAddressesInner {
    /// Address that was affected in the specified block
    #[serde(rename = "address")]
    pub address: String,
    /// List of transactions containing the address either in their inputs or outputs. Sorted by transaction index within a block, ascending.
    #[serde(rename = "transactions")]
    pub transactions: Vec<models::BlockContentAddressesInnerTransactionsInner>,
}

impl BlockContentAddressesInner {
    pub fn new(address: String, transactions: Vec<models::BlockContentAddressesInnerTransactionsInner>) -> BlockContentAddressesInner {
        BlockContentAddressesInner {
            address,
            transactions,
        }
    }
}

