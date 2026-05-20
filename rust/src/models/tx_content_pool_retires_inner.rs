use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentPoolRetiresInner {
    /// Index of the certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Bech32 stake pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Epoch in which the pool becomes retired
    #[serde(rename = "retiring_epoch")]
    pub retiring_epoch: i32,
}

impl TxContentPoolRetiresInner {
    pub fn new(cert_index: i32, pool_id: String, retiring_epoch: i32) -> TxContentPoolRetiresInner {
        TxContentPoolRetiresInner {
            cert_index,
            pool_id,
            retiring_epoch,
        }
    }
}

