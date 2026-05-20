use crate::models;
use serde::{Deserialize, Serialize};

/// AccountAddressesAssetsInner : The sum of all assets of all addresses associated with a given account
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAddressesAssetsInner {
    /// The unit of the value
    #[serde(rename = "unit")]
    pub unit: String,
    /// The quantity of the unit
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl AccountAddressesAssetsInner {
    /// The sum of all assets of all addresses associated with a given account
    pub fn new(unit: String, quantity: String) -> AccountAddressesAssetsInner {
        AccountAddressesAssetsInner {
            unit,
            quantity,
        }
    }
}

