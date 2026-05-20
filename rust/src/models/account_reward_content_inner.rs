use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountRewardContentInner {
    /// Epoch of the associated reward
    #[serde(rename = "epoch")]
    pub epoch: i32,
    /// Rewards for given epoch in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
    /// Bech32 pool ID being delegated to
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Type of the reward
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl AccountRewardContentInner {
    pub fn new(epoch: i32, amount: String, pool_id: String, r#type: Type) -> AccountRewardContentInner {
        AccountRewardContentInner {
            epoch,
            amount,
            pool_id,
            r#type,
        }
    }
}
/// Type of the reward
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "leader")]
    Leader,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "pool_deposit_refund")]
    PoolDepositRefund,
}

impl Default for Type {
    fn default() -> Type {
        Self::Leader
    }
}

