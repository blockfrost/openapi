use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentStakeAddrInner {
    /// Index of the certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Delegation stake address
    #[serde(rename = "address")]
    pub address: String,
    /// Registration boolean, false if deregistration
    #[serde(rename = "registration")]
    pub registration: bool,
}

impl TxContentStakeAddrInner {
    pub fn new(cert_index: i32, address: String, registration: bool) -> TxContentStakeAddrInner {
        TxContentStakeAddrInner {
            cert_index,
            address,
            registration,
        }
    }
}

