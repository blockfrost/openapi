use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposal {
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
    /// An object describing the content of this GovActionProposal in a readable way.
    #[serde(rename = "governance_description", deserialize_with = "Option::deserialize")]
    pub governance_description: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The deposit amount paid for this proposal.
    #[serde(rename = "deposit")]
    pub deposit: String,
    /// Bech32 stake address of the reward address to receive the deposit when it is repaid.
    #[serde(rename = "return_address")]
    pub return_address: String,
    /// The epoch at which the proposal was ratified. Null if the proposal has not been ratified.
    #[serde(rename = "ratified_epoch", deserialize_with = "Option::deserialize")]
    pub ratified_epoch: Option<i32>,
    /// The epoch at which the proposal was enacted. Null if the proposal has not been enacted.
    #[serde(rename = "enacted_epoch", deserialize_with = "Option::deserialize")]
    pub enacted_epoch: Option<i32>,
    /// The epoch at which the proposal was dropped. A proposal is dropped if it expires or if any of its dependencies expire.
    #[serde(rename = "dropped_epoch", deserialize_with = "Option::deserialize")]
    pub dropped_epoch: Option<i32>,
    /// The epoch at which the proposal expired. Null if the proposal has not expired.
    #[serde(rename = "expired_epoch", deserialize_with = "Option::deserialize")]
    pub expired_epoch: Option<i32>,
    /// The epoch at which this governance action will expire.
    #[serde(rename = "expiration")]
    pub expiration: i32,
}

impl Proposal {
    pub fn new(id: String, tx_hash: String, cert_index: i32, governance_type: GovernanceType, governance_description: Option<std::collections::HashMap<String, serde_json::Value>>, deposit: String, return_address: String, ratified_epoch: Option<i32>, enacted_epoch: Option<i32>, dropped_epoch: Option<i32>, expired_epoch: Option<i32>, expiration: i32) -> Proposal {
        Proposal {
            id,
            tx_hash,
            cert_index,
            governance_type,
            governance_description,
            deposit,
            return_address,
            ratified_epoch,
            enacted_epoch,
            dropped_epoch,
            expired_epoch,
            expiration,
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

