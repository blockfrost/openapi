use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentRedeemersInner {
    /// Index of the redeemer within the transaction
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// Validation purpose
    #[serde(rename = "purpose")]
    pub purpose: Purpose,
    /// Script hash
    #[serde(rename = "script_hash")]
    pub script_hash: String,
    /// Redeemer data hash
    #[serde(rename = "redeemer_data_hash")]
    pub redeemer_data_hash: String,
    /// Datum hash
    #[serde(rename = "datum_hash")]
    pub datum_hash: String,
    /// The budget in Memory to run a script
    #[serde(rename = "unit_mem")]
    pub unit_mem: String,
    /// The budget in CPU steps to run a script
    #[serde(rename = "unit_steps")]
    pub unit_steps: String,
    /// The fee consumed to run the script
    #[serde(rename = "fee")]
    pub fee: String,
}

impl TxContentRedeemersInner {
    pub fn new(tx_index: i32, purpose: Purpose, script_hash: String, redeemer_data_hash: String, datum_hash: String, unit_mem: String, unit_steps: String, fee: String) -> TxContentRedeemersInner {
        TxContentRedeemersInner {
            tx_index,
            purpose,
            script_hash,
            redeemer_data_hash,
            datum_hash,
            unit_mem,
            unit_steps,
            fee,
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

