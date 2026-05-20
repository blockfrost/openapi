use crate::models;
use serde::{Deserialize, Serialize};

/// StakeDistributionParty : Signer registered to a signature round. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StakeDistributionParty {
    /// The unique identifier of the signer
    #[serde(rename = "party_id", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// Stake share as computed in the 'stake distribution' by the Cardano Node, multiplied by a billion (1.0e9)
    #[serde(rename = "stake", skip_serializing_if = "Option::is_none")]
    pub stake: Option<i64>,
}

impl StakeDistributionParty {
    /// Signer registered to a signature round. 
    pub fn new() -> StakeDistributionParty {
        StakeDistributionParty {
            party_id: None,
            stake: None,
        }
    }
}

