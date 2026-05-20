use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetPolicyInner {
    /// Concatenation of the policy_id and hex-encoded asset_name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Current asset quantity
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl AssetPolicyInner {
    pub fn new(asset: String, quantity: String) -> AssetPolicyInner {
        AssetPolicyInner {
            asset,
            quantity,
        }
    }
}

