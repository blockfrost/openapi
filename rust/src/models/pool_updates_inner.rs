use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolUpdatesInner {
    /// Transaction ID
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Action in the certificate
    #[serde(rename = "action")]
    pub action: Action,
}

impl PoolUpdatesInner {
    pub fn new(tx_hash: String, cert_index: i32, action: Action) -> PoolUpdatesInner {
        PoolUpdatesInner {
            tx_hash,
            cert_index,
            action,
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

