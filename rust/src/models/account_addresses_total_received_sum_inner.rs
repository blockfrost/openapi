use crate::models;
use serde::{Deserialize, Serialize};

/// AccountAddressesTotalReceivedSumInner : The sum of all the UTXO per asset for all addresses associated with the account
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAddressesTotalReceivedSumInner {
    /// The unit of the value
    #[serde(rename = "unit")]
    pub unit: String,
    /// The quantity of the unit
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl AccountAddressesTotalReceivedSumInner {
    /// The sum of all the UTXO per asset for all addresses associated with the account
    pub fn new(unit: String, quantity: String) -> AccountAddressesTotalReceivedSumInner {
        AccountAddressesTotalReceivedSumInner {
            unit,
            quantity,
        }
    }
}

