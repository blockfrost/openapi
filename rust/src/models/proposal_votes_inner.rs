use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalVotesInner {
    /// Hash of the voting transaction.
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Index of the certificate within the voting transaction.
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// The role of the voter. Can be one of constitutional_committee, drep, spo.
    #[serde(rename = "voter_role")]
    pub voter_role: VoterRole,
    /// The actual voter. CIP-129 Bech32 identifier: `drep1...`, `pool1...` or `cc_hot1...`.
    #[serde(rename = "voter")]
    pub voter: String,
    /// The Vote. Can be one of yes, no, abstain.
    #[serde(rename = "vote")]
    pub vote: Vote,
    /// Whether the vote counts toward the proposal's tally. Only the voter's latest vote counts, and a DRep vote stops counting if the DRep deregisters while the proposal is still live.
    #[serde(rename = "counted")]
    pub counted: bool,
}

impl ProposalVotesInner {
    pub fn new(tx_hash: String, cert_index: i32, voter_role: VoterRole, voter: String, vote: Vote, counted: bool) -> ProposalVotesInner {
        ProposalVotesInner {
            tx_hash,
            cert_index,
            voter_role,
            voter,
            vote,
            counted,
        }
    }
}
/// The role of the voter. Can be one of constitutional_committee, drep, spo.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoterRole {
    #[serde(rename = "constitutional_committee")]
    ConstitutionalCommittee,
    #[serde(rename = "drep")]
    Drep,
    #[serde(rename = "spo")]
    Spo,
}

impl Default for VoterRole {
    fn default() -> VoterRole {
        Self::ConstitutionalCommittee
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

