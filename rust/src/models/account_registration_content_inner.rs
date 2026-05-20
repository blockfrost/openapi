use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountRegistrationContentInner {
    /// Hash of the transaction containing the (de)registration certificate
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Action in the certificate
    #[serde(rename = "action")]
    pub action: Action,
    /// Deposit in Lovelaces paid at this registration. `null` on `deregistered` rows.
    #[serde(rename = "deposit", deserialize_with = "Option::deserialize")]
    pub deposit: Option<String>,
    /// Slot of the transaction containing the (de)registration certificate
    #[serde(rename = "tx_slot")]
    pub tx_slot: i32,
    /// Block creation time in UNIX time of the transaction containing the (de)registration certificate
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Block height of the transaction containing the (de)registration certificate
    #[serde(rename = "block_height")]
    pub block_height: i32,
}

impl AccountRegistrationContentInner {
    pub fn new(tx_hash: String, action: Action, deposit: Option<String>, tx_slot: i32, block_time: i32, block_height: i32) -> AccountRegistrationContentInner {
        AccountRegistrationContentInner {
            tx_hash,
            action,
            deposit,
            tx_slot,
            block_time,
            block_height,
        }
    }
}
/// Action in the certificate
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "deregistered")]
    Deregistered,
}

impl Default for Action {
    fn default() -> Action {
        Self::Registered
    }
}

