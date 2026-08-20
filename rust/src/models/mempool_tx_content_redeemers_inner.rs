use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MempoolTxContentRedeemersInner {
    /// Index of the redeemer within the transaction
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// Validation purpose
    #[serde(rename = "purpose")]
    pub purpose: Purpose,
    /// The budget in Memory to run a script
    #[serde(rename = "unit_mem")]
    pub unit_mem: String,
    /// The budget in CPU steps to run a script
    #[serde(rename = "unit_steps")]
    pub unit_steps: String,
}

impl MempoolTxContentRedeemersInner {
    pub fn new(tx_index: i32, purpose: Purpose, unit_mem: String, unit_steps: String) -> MempoolTxContentRedeemersInner {
        MempoolTxContentRedeemersInner {
            tx_index,
            purpose,
            unit_mem,
            unit_steps,
        }
    }
}
/// Validation purpose
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Purpose {
    #[serde(rename = "spend")]
    Spend,
    #[serde(rename = "mint")]
    Mint,
    #[serde(rename = "cert")]
    Cert,
    #[serde(rename = "reward")]
    Reward,
    #[serde(rename = "vote")]
    Vote,
    #[serde(rename = "propose")]
    Propose,
}

impl Default for Purpose {
    fn default() -> Purpose {
        Self::Spend
    }
}

