use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsInner {
    /// Governance Action Identifier (CIP-0129)
    #[serde(rename = "id")]
    pub id: String,
    /// Hash of the proposal transaction.
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Index of the certificate within the proposal transaction.
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Type of proposal.
    #[serde(rename = "governance_type")]
    pub governance_type: GovernanceType,
}

impl ProposalsInner {
    pub fn new(id: String, tx_hash: String, cert_index: i32, governance_type: GovernanceType) -> ProposalsInner {
        ProposalsInner {
            id,
            tx_hash,
            cert_index,
            governance_type,
        }
    }
}
/// Type of proposal.
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

