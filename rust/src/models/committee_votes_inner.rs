use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitteeVotesInner {
    /// Hash of the vote transaction.
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// CIP-129 bech32 encoded hot credential of the committee member that cast this vote (`cc_hot1...`).
    #[serde(rename = "voter_hot_id")]
    pub voter_hot_id: String,
    /// CIP-129 Governance Action Identifier of the proposal being voted on.
    #[serde(rename = "proposal_id")]
    pub proposal_id: String,
    /// Hash of the proposal transaction.
    #[serde(rename = "proposal_tx_hash")]
    pub proposal_tx_hash: String,
    /// Index of the proposal within its transaction.
    #[serde(rename = "proposal_index")]
    pub proposal_index: i32,
    /// Type of the governance action being voted on.
    #[serde(rename = "governance_type")]
    pub governance_type: GovernanceType,
    /// The Vote. Can be one of yes, no, abstain.
    #[serde(rename = "vote")]
    pub vote: Vote,
    /// Voting anchor URL — pointer to the off-chain rationale published by the voter.
    #[serde(rename = "metadata_url", deserialize_with = "Option::deserialize")]
    pub metadata_url: Option<String>,
    /// Hex hash of the off-chain anchor document.
    #[serde(rename = "metadata_hash", deserialize_with = "Option::deserialize")]
    pub metadata_hash: Option<String>,
    /// Block height of the vote transaction.
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Block creation time in UNIX time of the vote transaction.
    #[serde(rename = "block_time")]
    pub block_time: i32,
}

impl CommitteeVotesInner {
    pub fn new(tx_hash: String, voter_hot_id: String, proposal_id: String, proposal_tx_hash: String, proposal_index: i32, governance_type: GovernanceType, vote: Vote, metadata_url: Option<String>, metadata_hash: Option<String>, block_height: i32, block_time: i32) -> CommitteeVotesInner {
        CommitteeVotesInner {
            tx_hash,
            voter_hot_id,
            proposal_id,
            proposal_tx_hash,
            proposal_index,
            governance_type,
            vote,
            metadata_url,
            metadata_hash,
            block_height,
            block_time,
        }
    }
}
/// Type of the governance action being voted on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GovernanceType {
    #[serde(rename = "hard_fork_initiation")]
    HardForkInitiation,
    #[serde(rename = "new_committee")]
    NewCommittee,
    #[serde(rename = "new_constitution")]
    NewConstitution,
    #[serde(rename = "info_action")]
    InfoAction,
    #[serde(rename = "no_confidence")]
    NoConfidence,
    #[serde(rename = "parameter_change")]
    ParameterChange,
    #[serde(rename = "treasury_withdrawals")]
    TreasuryWithdrawals,
}

impl Default for GovernanceType {
    fn default() -> GovernanceType {
        Self::HardForkInitiation
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

