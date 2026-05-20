use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAddressesTotal {
    /// Bech32 encoded stake address
    #[serde(rename = "stake_address")]
    pub stake_address: String,
    #[serde(rename = "received_sum")]
    pub received_sum: Vec<models::AccountAddressesTotalReceivedSumInner>,
    #[serde(rename = "sent_sum")]
    pub sent_sum: Vec<models::AccountAddressesTotalReceivedSumInner>,
    /// Count of all transactions for all addresses associated with the account
    #[serde(rename = "tx_count")]
    pub tx_count: i32,
}

impl AccountAddressesTotal {
    pub fn new(stake_address: String, received_sum: Vec<models::AccountAddressesTotalReceivedSumInner>, sent_sum: Vec<models::AccountAddressesTotalReceivedSumInner>, tx_count: i32) -> AccountAddressesTotal {
        AccountAddressesTotal {
            stake_address,
            received_sum,
            sent_sum,
            tx_count,
        }
    }
}

