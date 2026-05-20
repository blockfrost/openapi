use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepVotesInner {
    /// Hash of the vote transaction.
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Index of the certificate within the vote transaction.
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Governance Action Identifier (CIP-0129) of the proposal being voted on.
    #[serde(rename = "proposal_id")]
    pub proposal_id: String,
    /// Hash of the proposal transaction.
    #[serde(rename = "proposal_tx_hash")]
    pub proposal_tx_hash: String,
    /// Index of the certificate within the proposal transaction.
    #[serde(rename = "proposal_cert_index")]
    pub proposal_cert_index: i32,
    /// The Vote. Can be one of yes, no, abstain.
    #[serde(rename = "vote")]
    pub vote: Vote,
}

impl DrepVotesInner {
    pub fn new(tx_hash: String, cert_index: i32, proposal_id: String, proposal_tx_hash: String, proposal_cert_index: i32, vote: Vote) -> DrepVotesInner {
        DrepVotesInner {
            tx_hash,
            cert_index,
            proposal_id,
            proposal_tx_hash,
            proposal_cert_index,
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

