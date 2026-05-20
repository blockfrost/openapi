use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochStakeContentInner {
    /// Stake address
    #[serde(rename = "stake_address")]
    pub stake_address: String,
    /// Bech32 prefix of the pool delegated to
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Amount of active delegated stake in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
}

impl EpochStakeContentInner {
    pub fn new(stake_address: String, pool_id: String, amount: String) -> EpochStakeContentInner {
        EpochStakeContentInner {
            stake_address,
            pool_id,
            amount,
        }
    }
}

