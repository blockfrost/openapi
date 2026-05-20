use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalWithdrawalsInner {
    /// Bech32 stake address
    #[serde(rename = "stake_address")]
    pub stake_address: String,
    /// Withdrawal amount in Lovelaces
    #[serde(rename = "amount")]
    pub amount: String,
}

impl ProposalWithdrawalsInner {
    pub fn new(stake_address: String, amount: String) -> ProposalWithdrawalsInner {
        ProposalWithdrawalsInner {
            stake_address,
            amount,
        }
    }
}

