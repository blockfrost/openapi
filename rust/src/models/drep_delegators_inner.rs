use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepDelegatorsInner {
    /// Bech32 encoded stake addresses
    #[serde(rename = "address")]
    pub address: String,
    /// Currently delegated amount
    #[serde(rename = "amount")]
    pub amount: String,
}

impl DrepDelegatorsInner {
    pub fn new(address: String, amount: String) -> DrepDelegatorsInner {
        DrepDelegatorsInner {
            address,
            amount,
        }
    }
}

