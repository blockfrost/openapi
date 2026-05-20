use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolDelegatorsInner {
    /// Bech32 encoded stake addresses
    #[serde(rename = "address")]
    pub address: String,
    /// Currently delegated amount
    #[serde(rename = "live_stake")]
    pub live_stake: String,
}

impl PoolDelegatorsInner {
    pub fn new(address: String, live_stake: String) -> PoolDelegatorsInner {
        PoolDelegatorsInner {
            address,
            live_stake,
        }
    }
}

