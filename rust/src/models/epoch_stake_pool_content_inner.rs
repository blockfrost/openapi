use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochStakePoolContentInner {
    /// Stake address
    #[serde(rename = "stake_address")]
    pub stake_address: String,
    /// Amount of active delegated stake in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
}

impl EpochStakePoolContentInner {
    pub fn new(stake_address: String, amount: String) -> EpochStakePoolContentInner {
        EpochStakePoolContentInner {
            stake_address,
            amount,
        }
    }
}

