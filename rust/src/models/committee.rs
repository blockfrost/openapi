use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Committee {
    /// CIP-129 Governance Action Identifier of the `NewCommittee` action that seated this committee. `null` for the Conway-genesis committee.
    #[serde(rename = "gov_action_id", deserialize_with = "Option::deserialize")]
    pub gov_action_id: Option<String>,
    /// Hash of the transaction containing the `NewCommittee` action that seated this committee. `null` for the Conway-genesis committee.
    #[serde(rename = "proposal_tx_hash", deserialize_with = "Option::deserialize")]
    pub proposal_tx_hash: Option<String>,
    /// Index of the proposal within its transaction. `null` for the Conway-genesis committee.
    #[serde(rename = "proposal_index", deserialize_with = "Option::deserialize")]
    pub proposal_index: Option<i32>,
    /// True iff an enacted `NoConfidence` governance action has dissolved this committee.
    #[serde(rename = "is_dissolved")]
    pub is_dissolved: bool,
    #[serde(rename = "quorum")]
    pub quorum: Box<models::CommitteeQuorum>,
    /// Members of the committee.
    #[serde(rename = "members")]
    pub members: Vec<models::CommitteeMembersInner>,
}

impl Committee {
    pub fn new(gov_action_id: Option<String>, proposal_tx_hash: Option<String>, proposal_index: Option<i32>, is_dissolved: bool, quorum: models::CommitteeQuorum, members: Vec<models::CommitteeMembersInner>) -> Committee {
        Committee {
            gov_action_id,
            proposal_tx_hash,
            proposal_index,
            is_dissolved,
            quorum: Box::new(quorum),
            members,
        }
    }
}

