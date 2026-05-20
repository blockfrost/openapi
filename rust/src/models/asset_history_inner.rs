use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetHistoryInner {
    /// Hash of the transaction containing the asset action
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Action executed upon the asset policy
    #[serde(rename = "action")]
    pub action: Action,
    /// Asset amount of the specific action
    #[serde(rename = "amount")]
    pub amount: String,
}

impl AssetHistoryInner {
    pub fn new(tx_hash: String, action: Action, amount: String) -> AssetHistoryInner {
        AssetHistoryInner {
            tx_hash,
            action,
            amount,
        }
    }
}
/// Action executed upon the asset policy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "minted")]
    Minted,
    #[serde(rename = "burned")]
    Burned,
}

impl Default for Action {
    fn default() -> Action {
        Self::Minted
    }
}

