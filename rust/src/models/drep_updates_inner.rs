use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepUpdatesInner {
    /// Transaction ID
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Index of the certificate within the update transaction.
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Action in the certificate
    #[serde(rename = "action")]
    pub action: Action,
    /// Deposit in Lovelaces paid at this registration. `null` on `deregistered` and `updated` rows.
    #[serde(rename = "deposit", deserialize_with = "Option::deserialize")]
    pub deposit: Option<String>,
}

impl DrepUpdatesInner {
    pub fn new(tx_hash: String, cert_index: i32, action: Action, deposit: Option<String>) -> DrepUpdatesInner {
        DrepUpdatesInner {
            tx_hash,
            cert_index,
            action,
            deposit,
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
    #[serde(rename = "updated")]
    Updated,
}

impl Default for Action {
    fn default() -> Action {
        Self::Registered
    }
}

