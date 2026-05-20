use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressContentTotal {
    /// Bech32 encoded address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "received_sum")]
    pub received_sum: Vec<models::TxContentOutputAmountInner>,
    #[serde(rename = "sent_sum")]
    pub sent_sum: Vec<models::TxContentOutputAmountInner>,
    /// Count of all transactions on the address
    #[serde(rename = "tx_count")]
    pub tx_count: i32,
}

impl AddressContentTotal {
    pub fn new(address: String, received_sum: Vec<models::TxContentOutputAmountInner>, sent_sum: Vec<models::TxContentOutputAmountInner>, tx_count: i32) -> AddressContentTotal {
        AddressContentTotal {
            address,
            received_sum,
            sent_sum,
            tx_count,
        }
    }
}

