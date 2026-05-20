use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetAddressesInner {
    /// Address containing the specific asset
    #[serde(rename = "address")]
    pub address: String,
    /// Asset quantity on the specific address
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl AssetAddressesInner {
    pub fn new(address: String, quantity: String) -> AssetAddressesInner {
        AssetAddressesInner {
            address,
            quantity,
        }
    }
}

