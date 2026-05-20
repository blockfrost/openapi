use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountContent {
    /// Bech32 stake address
    #[serde(rename = "stake_address")]
    pub stake_address: String,
    /// Delegation state of the account. **Note:** For registration state, use the `registered` field instead.
    #[serde(rename = "active")]
    pub active: bool,
    /// Registration state of an account
    #[serde(rename = "registered")]
    pub registered: bool,
    /// Epoch of the most recent action - registration or deregistration
    #[serde(rename = "active_epoch", deserialize_with = "Option::deserialize")]
    pub active_epoch: Option<i32>,
    /// Balance of the account in Lovelaces
    #[serde(rename = "controlled_amount")]
    pub controlled_amount: String,
    /// Sum of all rewards for the account in the Lovelaces
    #[serde(rename = "rewards_sum")]
    pub rewards_sum: String,
    /// Sum of all the withdrawals for the account in Lovelaces
    #[serde(rename = "withdrawals_sum")]
    pub withdrawals_sum: String,
    /// Sum of all  funds from reserves for the account in the Lovelaces
    #[serde(rename = "reserves_sum")]
    pub reserves_sum: String,
    /// Sum of all funds from treasury for the account in the Lovelaces
    #[serde(rename = "treasury_sum")]
    pub treasury_sum: String,
    /// Sum of available rewards that haven't been withdrawn yet for the account in the Lovelaces
    #[serde(rename = "withdrawable_amount")]
    pub withdrawable_amount: String,
    /// Bech32 pool ID to which this account is delegated
    #[serde(rename = "pool_id", deserialize_with = "Option::deserialize")]
    pub pool_id: Option<String>,
    /// Bech32 drep ID to which this account is delegated
    #[serde(rename = "drep_id", deserialize_with = "Option::deserialize")]
    pub drep_id: Option<String>,
}

impl AccountContent {
    pub fn new(stake_address: String, active: bool, registered: bool, active_epoch: Option<i32>, controlled_amount: String, rewards_sum: String, withdrawals_sum: String, reserves_sum: String, treasury_sum: String, withdrawable_amount: String, pool_id: Option<String>, drep_id: Option<String>) -> AccountContent {
        AccountContent {
            stake_address,
            active,
            registered,
            active_epoch,
            controlled_amount,
            rewards_sum,
            withdrawals_sum,
            reserves_sum,
            treasury_sum,
            withdrawable_amount,
            pool_id,
            drep_id,
        }
    }
}

