use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentMirsInner {
    /// Source of MIR funds
    #[serde(rename = "pot")]
    pub pot: Pot,
    /// Index of the certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Bech32 stake address
    #[serde(rename = "address")]
    pub address: String,
    /// MIR amount in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
}

impl TxContentMirsInner {
    pub fn new(pot: Pot, cert_index: i32, address: String, amount: String) -> TxContentMirsInner {
        TxContentMirsInner {
            pot,
            cert_index,
            address,
            amount,
        }
    }
}
/// Source of MIR funds
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Pot {
    #[serde(rename = "reserve")]
    Reserve,
    #[serde(rename = "treasury")]
    Treasury,
}

impl Default for Pot {
    fn default() -> Pot {
        Self::Reserve
    }
}

