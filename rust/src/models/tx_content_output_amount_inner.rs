use crate::models;
use serde::{Deserialize, Serialize};

/// TxContentOutputAmountInner : The sum of all the UTXO per asset
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentOutputAmountInner {
    /// The unit of the value
    #[serde(rename = "unit")]
    pub unit: String,
    /// The quantity of the unit
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl TxContentOutputAmountInner {
    /// The sum of all the UTXO per asset
    pub fn new(unit: String, quantity: String) -> TxContentOutputAmountInner {
        TxContentOutputAmountInner {
            unit,
            quantity,
        }
    }
}

