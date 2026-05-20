use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignerRegistrationsListItemMessage {
    /// Stake share as computed in the 'stake distribution' by the Cardano Node, multiplied by a billion (1.0e9)
    #[serde(rename = "stake")]
    pub stake: i64,
    /// The unique identifier of the signer
    #[serde(rename = "party_id", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
}

impl SignerRegistrationsListItemMessage {
    pub fn new(stake: i64) -> SignerRegistrationsListItemMessage {
        SignerRegistrationsListItemMessage {
            stake,
            party_id: None,
        }
    }
}

