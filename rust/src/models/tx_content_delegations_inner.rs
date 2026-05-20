use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentDelegationsInner {
    /// Index of the certificate within the transaction
    #[serde(rename = "index")]
    pub index: i32,
    /// Index of the certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Bech32 delegation stake address
    #[serde(rename = "address")]
    pub address: String,
    /// Bech32 ID of delegated stake pool
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Epoch in which the delegation becomes active
    #[serde(rename = "active_epoch")]
    pub active_epoch: i32,
}

impl TxContentDelegationsInner {
    pub fn new(index: i32, cert_index: i32, address: String, pool_id: String, active_epoch: i32) -> TxContentDelegationsInner {
        TxContentDelegationsInner {
            index,
            cert_index,
            address,
            pool_id,
            active_epoch,
        }
    }
}

