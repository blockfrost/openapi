use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetsInner {
    /// Asset identifier
    #[serde(rename = "asset")]
    pub asset: String,
    /// Current asset quantity
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl AssetsInner {
    pub fn new(asset: String, quantity: String) -> AssetsInner {
        AssetsInner {
            asset,
            quantity,
        }
    }
}

