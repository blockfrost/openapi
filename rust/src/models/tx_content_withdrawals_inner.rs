use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentWithdrawalsInner {
    /// Bech32 withdrawal address
    #[serde(rename = "address")]
    pub address: String,
    /// Withdrawal amount in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
}

impl TxContentWithdrawalsInner {
    pub fn new(address: String, amount: String) -> TxContentWithdrawalsInner {
        TxContentWithdrawalsInner {
            address,
            amount,
        }
    }
}

