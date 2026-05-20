use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolVotesInner {
    /// Hash of the proposal transaction.
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Index of the certificate within the proposal transaction.
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// The Vote. Can be one of yes, no, abstain.
    #[serde(rename = "vote")]
    pub vote: Vote,
}

impl PoolVotesInner {
    pub fn new(tx_hash: String, cert_index: i32, vote: Vote) -> PoolVotesInner {
        PoolVotesInner {
            tx_hash,
            cert_index,
            vote,
        }
    }
}
/// The Vote. Can be one of yes, no, abstain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Vote {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "abstain")]
    Abstain,
}

impl Default for Vote {
    fn default() -> Vote {
        Self::Yes
    }
}

